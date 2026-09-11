# MELFINA — RUNTIME MODEL

**Phase:** SYSTEM DESIGN / ARCHITECTURE — MISSION 001. First pass, pending review.
**Revision 1 applied** — RC-1 (startup signature check), RC-6 (local-only
reasoner bind), RC-5 (live grant handles), RC-2 (aggregate-budget watchdog), M9
(declared-scope conflict), L4, O1 wording. See
`ARCHITECTURE_ADVERSARIAL_REVIEW.md` "Revision 1 — resolution status".
Baseline `f3093fc`. Read `ARCHITECTURAL_PRINCIPLES.md` and `SYSTEM_ARCHITECTURE.md`
first. **No technology chosen.**

Covers mission items **F** (lifecycle), **G** (event flow), **H** (reasoning
flow), **AB** (resource management), and the idle/degradation behaviour.

---

## 1. Lifecycle states

```
   (not running) ──start──▶ STARTING ──▶ CORE-READY ──▶ FULL
                                │             │            │
                                │             ▼            ▼
                                │        (AI disabled)   REASONING-ACTIVE
                                │             │            │
                                └─────────────┴────────────┘
                                              │
                            ┌─────────────────┼──────────────────┐
                            ▼                 ▼                  ▼
                          IDLE           DEGRADED           STOPPING
                       (quiet, bounded  (resource-tight,   (drain, checkpoint,
                        background)      or a subsystem     close atomically)
                                         down — still           │
                                         coherent)              ▼
                                                            (not running)

         EMERGENCY-STOP: reachable from any state, Ring 0, halts to a safe state.
```

| State | Meaning |
|---|---|
| **STARTING** | Ring 0 loads + verifies governance **signature (RC-1) and version chain**; Ring 1 Supervisor brings up the substrate; Chronicle integrity checked; the current-state cache is caught up from checkpoint. An invalid governance signature or broken chain ⇒ refuse to reach CORE-READY. |
| **CORE-READY** | Ring 0 + Ring 1 fully operational. Capture, memory, retrieval, time support, user-made plans, deterministic automations, search — all work. **This is a complete, usable system** (AP-9, INV-9). |
| **FULL** | as CORE-READY, with Ring 2/3 available on demand. |
| **REASONING-ACTIVE** | a Ring-2 activity is in progress (a question, a proposal, a teaching session). Transient. |
| **IDLE** | no user interaction, no in-flight pipeline action, no active reasoning. Ring 0 + Ring 1 resident and quiet; only bounded background work (§4). |
| **DEGRADED** | resources are tight, or a non-core subsystem is down. The metareasoning controller shifts to cheaper strategies / shallower reasoning / smaller context / declining (MEL-REQ-155, 243). Ring 1 is never starved; the Chronicle is never at risk. |
| **STOPPING** | the Pipeline SM refuses new consequential actions; in-flight actions run to VERIFY or abort cleanly; projections checkpoint; Chronicle closes atomically. |

**No state ever requires the network** (INV-1).

## 2. Startup sequence (F)

```
  1. Ring 0
     · load governance files (permission rules, tier policy, budgets, the grants
       the user has issued)
     · verify **at-rest integrity (RC-1)**: a cryptographic integrity chain over
       the governance files, rooted in a human-held signing key MELFINA never
       possesses. An invalid signature ⇒ refuse to start.
     · verify the **version chain** (each change is a recorded human act; a broken
       chain ⇒ refuse to start, report — do not proceed on suspect governance)
     · both checks are hard preconditions for STARTING → CORE-READY; there is no
       "continue anyway". The signing scheme / key storage / chain format are
       LOW-LEVEL FOUNDATIONS (only the invariant is fixed here).
     · arm the Reference Monitor and the Emergency-Stop authority

  2. Ring 1 (Supervisor starts these; order matters where noted)
     a. Chronicle: open the append log; verify integrity (append-order, atomicity
        markers); a detected corruption ⇒ enter a read-only recovery mode and
        report, never silently repair
     b. Entity Registry, Policy Store, Governance-grant view: load
     c. Projection Engine: load the last good checkpoints; replay the Chronicle
        tail since each checkpoint to catch projections up
     d. Query, Pipeline SM, Audit, Notification Gateway, Capability Registry: up

  3. → CORE-READY. If AI is configured off, this is the terminal ready state.

  4. Ring 2 (lazy): on first need, the Reasoner Interface binds to the configured
     **local** reasoner. The interface holds no network capability (RC-6), so a
     remote endpoint is not a configurable option — it is unreachable through the
     core capability model. If no local reasoner is available or it fails to load,
     MELFINA reports "reasoning unavailable" and stays at CORE-READY — no error,
     no degradation of Ring 1.

  5. Ring 3 (lazy): a capability component loads only when the pipeline reaches
     EXECUTE for an authorised action that uses it, and unloads after. The Ring 3
     sandbox has **zero network capability by default** (L4); network access, if
     ever granted, is a distinct high-risk capability and a Ring-4 concern.
```

**Startup must be fast** (MEL-REQ-197). The slow part is projection catch-up;
it is bounded by "Chronicle tail since last checkpoint", and checkpointing is
frequent enough (§4) that catch-up is short. If catch-up is long (e.g. after a
crash), CORE-READY can be declared with projections *still catching up* — reads
that need a not-yet-caught-up view fall back to a direct Chronicle query.

## 3. Event flow (G) — detail

Every change enters as an `Event` (possibly with `Claim`s it `brings-about` /
`ends`). Sources: a user capture; an `observation` (user or capability); the
result of an executed action; a scheduled/triggered automation firing; a
`decision` the user makes; a Ring-0 governance change (recorded, not in the
Chronicle but mirrored as an audit Event).

```
   source ──▶ construct Event (+ Claims)
          ──▶ Chronicle.append
                 · assign monotonic transaction-time
                 · atomic: the whole unit lands or nothing does
                 · durable before the append returns
          ──▶ Audit.append  (if consequential: any action, any grant, any
                             capability run, any Ring-2 data access, any
                             self-change)
          ──▶ notify Projection Engine
                 · one idempotent handler keeps **the current-state cache** up to
                   date (via brings-about/ends). Other views ("what needs
                   attention", calendar, learning trajectory, capability
                   reliability, an opted-in progress view) are **built on demand**
                   from the Chronicle or that cache — not separately maintained
                   materialised projections (O1).
                 · a handler failure is contained (FAILURE_AND_RECOVERY §3);
                   the append already succeeded
          ──▶ if a rule/automation matches the Event, its Intention is queued
              into the pipeline (which then goes through DECIDE/AUTHORISE/…)
```

**Capture is one action, no forced classification** (MEL-REQ-020): the Event is
appended immediately; `kind`, links, and a next action are *later* Claims the
user or MELFINA may add. The user never waits on classification.

**Ordering:** the Chronicle is totally ordered by transaction-time. `valid-time`
can be anything (past/future). "What was true then, as I believed it then" vs "as
I believe it now" is a bitemporal query (`DATA_AND_STATE_MODEL.md` §4).

## 4. Idle behaviour and bounded background work (AB)

In **IDLE**, the only things running are Ring 0 + the Ring 1 substrate, quiet.
Background work is a **fixed, small set**, each with a work budget and a schedule,
never busy-polling:

| Background task | When | Budget |
|---|---|---|
| Projection catch-up | after appends, until caught up | yields between units; pauses under load |
| Checkpointing | periodic + before shutdown | bounded I/O per run |
| Notification-window flush | at the user's configured windows / task boundaries | delivers the queued minimal set, then quiet |
| Due automations | when a scheduled/triggered automation is due | each run capped by Ring-0 budgets; goes through the pipeline |
| Spaced-review resurfacing | when a `melfina` recurring Intention for a concept is due | routed through the Notification Gateway; subject to restraint |
| Capability regression tests | periodic, low priority | opportunistic; skipped under load or on battery |
| Aggregate-budget accounting (RC-2) | on every routine action and every automation run | maintains the per-activity-chain aggregate tally over its window; when a chain crosses its aggregate budget the next action in it is re-gated as **consequential** |
| Budget / revocation watchdog (RC-2, RC-5) | always, alongside the Ring 1 substrate | an external monitor (not the reasoner, not the running capability) that enforces wall-clock, step, and aggregate budgets and can invalidate a live grant mid-execution |

**Not** running in idle: the reasoning ring (no polling — it activates on a
trigger), any model (released after use / an idle timeout), any capability
component, anything network.

**Resource posture** (AP-12, MEL-REQ-196–199):
- **Resident:** Ring 0 + Ring 1 substrate — deliberately small; the target is
  low idle CPU and a modest RAM floor (concrete numbers are a LOW-LEVEL
  FOUNDATIONS / CORE ENGINE concern, set as budgets there).
- **On-demand:** Ring 2, Ring 3 components, models — loaded on need, released
  after use or an idle timeout.
- **Bounded:** background work as above; the metareasoning controller's total
  effort per activity is capped by Ring-0 budgets.
- **Degradation:** under resource pressure MELFINA does *less* (cheaper
  strategies, shallower reasoning, smaller context, more declining) — it never
  drops correctness or starves Ring 1 (MEL-REQ-155, 243; INV-11).
- **No premature optimisation** (mission §9, MEL-REQ-200): the architecture sets
  *boundaries* (what is resident, what is on-demand, what is budgeted); concrete
  performance tuning is a later phase.

## 5. Reasoning flow (H) — detail

See `SYSTEM_ARCHITECTURE.md` §6.2 for the diagram. Runtime specifics:

- **Trigger → activation:** a user question, a proposal request, a scheduled
  review, a detected capability gap, a teaching session. On trigger, Ring 2
  activates (binds the Reasoner Interface if not already).
- **The working context** lives in Ring 2 memory only, for the duration of the
  activity. It is a transient, on-demand construction — built by the Context
  Constructor from Query results, added to by contributors, discarded at the end.
  It is **never** persisted (model §7; INV-3). ("Blackboard" is one candidate
  mechanism for how the controller and contributors share it — O3, not an
  architectural requirement.)
- **The controller's loop** (bounded by Ring-0 budgets): frame the situation →
  build initial context → decide strategy (depth, contributors, whether more
  context helps or is noise, whether to decompose, whether existing capabilities
  suffice, whether to act/wait/ask/do-nothing) → run contributors → re-evaluate →
  **stop** when the budget is hit, or marginal value drops below a threshold, or
  an answer/decision is reached, or "do nothing" is chosen.
- **Every strategic choice is logged** (MEL-REQ-208) so the user can see *why*
  MELFINA reasoned the way it did, not just the output.
- **Outputs** are appended to the Chronicle as Claims (`holder = melfina`,
  `status = inferred`, full provenance + a reasoning trace) and, if an action is
  warranted, one `Intention` (`status = proposed`). If no action is warranted, a
  Claim records that and why (MEL-REQ-103).
- **Determinism note:** the *controller's decisions* should be consistent for the
  same situation + grounding (MEL-REQ-253); the *contributors* may be
  probabilistic (a model), but their outputs are labelled and gated, never
  effects (AP-3, AP-2). Whether the controller itself can be made deterministic
  enough is Risk 3 (`SYSTEM_ARCHITECTURE.md` §28), deferred to CORE ENGINE.

## 6. Concurrency posture

- **The Chronicle is the serialisation point.** Appends are totally ordered;
  concurrent producers queue. This gives a simple, predictable consistency model
  for a single-user system without distributed-consensus machinery.
- **Projections are built by consumers of the append stream** — they can lag, and
  multiple projection handlers run independently (a slow one doesn't block a fast
  one).
- **The pipeline processes one consequential action to a definite state**
  (VERIFY or a clean abort) — it does not interleave two consequential actions
  that could conflict; it may run independent, non-conflicting actions
  concurrently. **Conflict is judged conservatively (M9):** two actions conflict
  if their **declared scopes overlap** (the objects/resources each action's grant
  names). Absent stronger semantics, overlap ⇒ serialise. A stronger conflict
  model (semantic commutativity) is a later refinement, never a relaxation of
  this default.
- **Capability grants are live, revocable handles (RC-5).** The Reference Monitor
  (via the watchdog, §4) can invalidate a grant mid-execution: after revocation
  no new effect is permitted; a running execution is stopped where possible;
  reversible effects already made are rolled back where possible; irreversible or
  partial effects are recorded as a **partial failure**. User-visible outcome
  state distinguishes *completed / partially completed / rolled back / failed /
  interrupted* (see `FAILURE_AND_RECOVERY.md` §4, §8).
- **Capability executions are isolated** (each in its own sandbox) and supervised.
- The concrete concurrency mechanism (threads / processes / async / actors) is a
  LOW-LEVEL FOUNDATIONS choice, constrained by AP-11 (failure isolation) and
  AP-12 (lightweight).

## 7. Time in the runtime

- **Wall-clock** is used for transaction-time (monotonic; the runtime must handle
  clock adjustments without breaking monotonicity — a LOW-LEVEL FOUNDATIONS
  detail).
- **`valid-time`** is data (user- or reasoning-asserted), not the runtime clock.
- **Scheduled work** (automations, notification windows, spaced review) is driven
  off wall-clock against `valid-time` targets and Allen-relation constraints in
  the data.
- There is **no dependence on network time** (INV-1).
