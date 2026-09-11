# MELFINA — SYSTEM ARCHITECTURE

**Phase:** SYSTEM DESIGN / ARCHITECTURE — MISSION 001. **Status:** first pass,
pending review. **Revision 1 applied** — targeted adversarial-review corrections
RC-1…RC-7, M1/M2/M3/M5/M6/M8/M9/M10, L4/L5, O1/O3/O4 (§2 subsystem map, §6.3/§6.5/
§6.6 flows, §11/§12/§16/§17/§20/§21/§22/§23, §25–§30 all touched). See
`ARCHITECTURE_ADVERSARIAL_REVIEW.md` "Revision 1 — resolution status".
**Date:** 2026-09-10. **Baseline:** `f3093fc`.

Read `ARCHITECTURAL_PRINCIPLES.md` first (the invariants AP-1…AP-14 and the
mechanism/policy/data/capability/governance/derived-view vocabulary are used
throughout). Companions listed there.

**This document does not choose a language, storage product, framework, model, or
engine.** Structural patterns (`[E]`) are evaluated in
`ARCHITECTURAL_ALTERNATIVES.md`.

---

## 1. Architecture summary (one page)

MELFINA is **a small governed core over an append-only claim/event substrate,
with reasoning and capabilities as optional, isolated, permissioned outer rings,
coordinated by a metareasoning controller that is a control loop, not an agent
loop.**

Five concentric **rings** of decreasing trust and increasing dynamism.
Dependencies point **strictly inward**. Nothing outer can modify anything inner.

```
        ┌──────────────────────────────────────────────────────────────┐
        │  RING 4  (FUTURE, ISOLATED)  NETWORK                           │
        │  never in the core · off by default · separately installed    │
        │  by construction cannot hold {private data + untrusted +      │
        │  outbound} together (breaks the lethal trifecta)              │
        │  ┌────────────────────────────────────────────────────────┐   │
        │  │  RING 3  CAPABILITIES  (isolated · permissioned ·        │  │
        │  │  dynamic · replaceable · no ambient authority)           │  │
        │  │  skills · tools · workflows · terminal control ·         │  │
        │  │  GUI control · file access · verifiers                   │  │
        │  │  ┌──────────────────────────────────────────────────┐   │  │
        │  │  │  RING 2  REASONING  (optional · bounded · may use   │  │  │
        │  │  │  AI · produces PROPOSALS + CLAIMS, never effects)   │  │  │
        │  │  │  metareasoning controller · context construction · │  │  │
        │  │  │  planning · decision · self-evaluation · teaching · │  │  │
        │  │  │  scientific-reasoning discipline                    │  │  │
        │  │  │  ┌────────────────────────────────────────────┐    │  │  │
        │  │  │  │  RING 1  CORE MECHANISM                      │   │  │  │
        │  │  │  │  (deterministic · local-only · no AI)        │   │  │  │
        │  │  │  │  Chronicle (append-only Events+Claims+       │   │  │  │
        │  │  │  │   Intentions, bitemporal) · Projection       │   │  │  │
        │  │  │  │   engine · Query · Entity registry ·         │   │  │  │
        │  │  │  │   Pipeline state machine · Audit ·           │   │  │  │
        │  │  │  │   Capability registry (metadata) · Policy    │   │  │  │
        │  │  │  │   store · Notification gateway               │   │  │  │
        │  │  │  │  ┌──────────────────────────────────────┐   │   │  │  │
        │  │  │  │  │  RING 0  GOVERNANCE                    │  │   │  │  │
        │  │  │  │  │  (read-only to all outer rings;       │  │   │  │  │
        │  │  │  │  │   changed only by a human editing     │  │   │  │  │
        │  │  │  │  │   versioned files — NOT a MELFINA op) │  │   │  │  │
        │  │  │  │  │  · Reference monitor / permission gate │  │   │  │  │
        │  │  │  │  │  · Self-modification tier policy       │  │   │  │  │
        │  │  │  │  │  · Autonomous-action budgets           │  │   │  │  │
        │  │  │  │  │  · Emergency-stop authority            │  │   │  │  │
        │  │  │  │  │  · The meta-invariant                  │  │   │  │  │
        │  │  │  │  └──────────────────────────────────────┘   │   │  │  │
        │  │  │  └────────────────────────────────────────────┘    │  │  │
        │  │  └──────────────────────────────────────────────────┘   │  │
        │  └────────────────────────────────────────────────────────┘  │
        └──────────────────────────────────────────────────────────────┘
              (Diagram 1 — high-level subsystem architecture)
```

- **Ring 0** enforces *what may change what*. It is not a subsystem MELFINA runs;
  it is a set of rules + a gate that every consequential operation passes through.
  Outer rings can **read** it (to explain it) and have **no write path** to it.
- **Ring 1** is the only always-resident code besides Ring 0. It is the
  claim/event substrate + the deterministic services that operate on it. It never
  calls AI, never touches the network, never executes a capability itself.
- **Ring 2** is optional. It reads Ring 1 through a narrow interface and emits
  **proposals** (Intentions with `status = proposed`) and **claims** (`holder =
  melfina`, `status = inferred`). It cannot write the Chronicle directly and
  cannot invoke a capability.
- **Ring 3** is where *doing* happens. Each capability is a sandboxed component
  with **no ambient authority**; it receives narrow, typed, revocable capability
  grants for a specific authorised action, executes, and returns a result that
  Ring 1 verifies and records.
- **Ring 4** does not exist yet and is not a core concern. If ever built, it is a
  distinct, off-by-default, separately-installed capability, structurally unable
  to form the lethal trifecta with core data.

**The E²CI model maps to the substrate directly:** `Event`, `Claim`, `Intention`
are the units appended to the Chronicle; `Entity` is a registry of referents;
`Time` is the bitemporal coordinate on every unit; **`Context` and "state" are
projections** — computed, never stored, thrown away after use.

---

## 2. Major subsystem map (B, C — subsystems and responsibilities)

Grouped by ring. Each subsystem has **one responsibility**. "Class" per
`ARCHITECTURAL_PRINCIPLES.md` §2.

> **These are RESPONSIBILITIES, not components** (RC-7 / AP-12). The ~25 entries
> below do **not** imply separate processes, services, binaries, or IPC
> boundaries. A conforming implementation MAY group cohesive responsibilities
> into a small number of components — and several naturally are one component
> each: **"the store"** = Chronicle + Query + Entity Registry + the current-state
> cache; **"the gate"** = Pipeline State Machine + the Reference Monitor call
> path; **"the edge"** = Audit + Notification Gateway. **The only boundaries that
> MUST be real, enforced boundaries** (see §20): Ring 0 ↔ everything (no write
> inward); Ring 1 ↔ the sandboxed outer region (Ring 2/Ring 3 get no Chronicle
> write, no capability invocation, no network, no Ring 0); and the
> reasoning↔effect boundary (Ring 2 emits only proposals, never invokes a
> capability). Everything else may be co-located. This satisfies MEL-REQ-192
> without weakening security.

### Ring 0 — Governance

| Subsystem | Class | Responsibility | Traces to |
|---|---|---|---|
| **Reference Monitor** | governance | The single, non-bypassable, always-invoked gate every consequential operation passes. Checks: capability grant valid & in scope; action within the authorised proposal; within Ring-0 budgets; not blocked by an emergency stop. Denies by default. | MEL-REQ-018, 179–185, 235; ref-monitor `[E]` |
| **Tier Policy** | governance | The versioned classification rules: given *what a change touches*, which of the nine self-modification tiers it is, and what governance that tier requires. Read-only to reasoning. | MEL-REQ-233–236, 227 |
| **Budget Authority** | governance | Hard ceilings on autonomous action: steps, wall-clock, capability-invocation count, and any metered resource. Enforced as cut-offs, not alerts, by an **external watchdog** (not self-checked). Bounds are **per activity, per window, AND per activity-chain in aggregate** (RC-2): a chain of routine actions carries an aggregate budget (count + total affected scope); crossing it **escalates the next action to consequential (HITL)** regardless of how the chain is framed as activities or spread across windows. | MEL-REQ-115, 183; runaway-loop `[E]` |
| **Emergency Stop** | governance | A signal, reachable independently of the reasoning ring, that halts the pipeline and all Ring-3 executions and drives Ring 1 to a defined safe state. | MEL-REQ-145, 187 |
| **Governance Store** | governance/data | The versioned files holding the above rules + the permission grants the user has issued. Append-versioned; every change is a recorded human act. **Not in the Chronicle.** **Cryptographically integrity-protected at rest** (RC-1): a signature chain rooted in a human-held key MELFINA never possesses; verified at every startup; invalid ⇒ refuse to run. **Hard-excluded from every File Access grant** — the Reference Monitor rejects any grant whose resolved path intersects the Ring 0 region. Also holds the explicit **lethal-trifecta prohibition** as a governance rule (L5): no component may hold {outbound channel + private-data read}, and none while also ingesting untrusted content. | MEL-REQ-235, 126; INV-6 |

### Ring 1 — Core mechanism

| Subsystem | Class | Responsibility | Traces to |
|---|---|---|---|
| **Chronicle** | mechanism + data | The append-only, bitemporal log of `Event` / `Claim` / `Intention` units. The single source of truth. Atomic appends; monotonic transaction-time; no update, no delete (tombstone only, via an explicit redaction Event). | Model §4, §11, P-5; MEL-REQ-160–162, 170 |
| **Entity Registry** | mechanism + data | Stable ids for referents; `kind` (open); names/aliases over time; `same-as` claims. Thin — most entity meaning lives in Claims about them. | Model §4.1 |
| **Projection Engine** | mechanism | Maintains **one** materialised cache — **current-state** ("what holds now") — plus **on-demand computed views** ("what needs attention", calendar view, learning trajectories, capability reliability) built when asked and not independently maintained (O1). Idempotent; checkpointed; rebuildable from scratch. All are **caches** and may lag. **`Context` is NOT built here** — it is a per-situation Ring-2 derivation (§8, M1). | Model §3.3, §7, §13; event-sourcing `[E]`; AP-5 |
| **Query** | mechanism | Read access to Chronicle + projections, including bitemporal queries ("what I believed at transaction-time T about valid-time V"). The only read path Ring 2/3 get. | Model §6.1; MEL-REQ-091–093 |
| **Pipeline State Machine** | mechanism | Advances a proposed action through THINK→DECIDE→PROPOSE→AUTHORISE→EXECUTE→VERIFY, producing a distinct inspectable artefact at each stage, stoppable at any stage. **Structurally validates every proposal** (well-formed target, resolvable references, a scope expressible in the grant vocabulary) *before* AUTHORISE; a malformed proposal is rejected with a recorded Event and never reaches the user or EXECUTE (M10). Routes AUTHORISE to the user or (for the routine/reversible/pre-authorised class, within the aggregate budget) applies the policy default. Calls the Reference Monitor before EXECUTE. Reads "what holds now" for the DECIDE/classify step **through the Chronicle contract (a fresh authoritative read), not the current-state cache** (M2). | MEL-REQ-018, 019, 144 |
| **Capability Registry** | mechanism + data | Metadata only: which capabilities exist, version, declared authority, risk class, reliability-by-task-type claims, lifecycle status. **No capability code runs here.** | MEL-REQ-123–128, 218–227 |
| **Policy Store** | policy (data) | The user's settings: conflict-axis positions, autonomy/proactivity levels per area, interruption rules, retention periods, the consequential/routine boundary. Versioned; changed only by a user act; consulted by Ring 1 and Ring 2. | MEL-REQ-175–178; requirements CONFLICTS |
| **Audit** | mechanism | Appends a tamper-evident record of every consequential action, permission grant/revoke, capability invocation, Ring-2 data access, and self-change. Local, user-readable. Separate from the Chronicle so it also records *reads*. | MEL-REQ-156, 182, 201–203 |
| **Notification Gateway** | mechanism + policy | The **single** outbound-to-user path for anything unsolicited. Enforces: user-governed, per-source, per-context, predictably scheduled (windows / task boundaries), no repetition-driven escalation, no behaviour-change nudging. Everything else the user *pulls*. | MEL-REQ-011, 041–047; Fitz 2019 `[E]` |
| **Supervisor** | mechanism | Starts the Ring 1 responsibilities, restarts crashed ones from a clean state, contains failure so it does not cascade, orchestrates recovery (rebuild the current-state cache from the last checkpoint). "Let it crash." *(The supervision **topology** — flat vs hierarchical — is deferred; start flat, add hierarchy only if the component count grows. O4.)* | MEL-REQ-015, 172, 173; OTP `[E]` |

### Ring 2 — Reasoning (optional)

| Subsystem | Class | Responsibility | Traces to |
|---|---|---|---|
| **Metareasoning Controller** | mechanism (of reasoning) | A **dynamic controller** (not an agent loop): per situation, decides which reasoning contributors to run, reasoning depth, how much context, whether to compose/create a capability, and **whether to act / wait / ask / do nothing**. Bounded by Ring-0 budgets and the grounding factors. Emits nothing but proposals + claims. Logs every strategic choice. *(A blackboard-style opportunistic-contribution pattern is **one candidate** coordination mechanism, to be chosen and evaluated in CORE ENGINE — it is not an architectural requirement. O3.)* | MEL-REQ-204–213, 251–253; blackboard control `[E]`; VC8 |
| **Context Constructor** | mechanism (of reasoning) | Given a focus + present moment + active Intention(s), asks Query for a relevance-ranked slice; decides what to include/exclude and records why (auditable). Never persists context. | Model §7; MEL-REQ-210–215 |
| **Reasoning Contributors** | mixed (M3) | Interpret · infer · spot conflicts/gaps/anomalies · plan · evaluate options · derive from first principles · run the scientific-reasoning discipline · teach. Each writes only to the transient working context. **The core contributors** (this list) are small, audited, **trusted Ring-2 code**. **Additional or novel reasoning strategies MELFINA constructs** are **Ring-3 capabilities under the full lifecycle** (created / tested / versioned / risk-classified / authorised / retired) and run **sandboxed** — so reasoning-strategy dynamism (MEL-REQ-206) is genuine but bounded, and a constructed strategy cannot exceed its grant. | MEL-REQ-94–103, 129–132, 70–76, 206 |
| **Self-Evaluation** | mechanism (of reasoning) | Compares MELFINA's past claims/decisions/predictions to recorded outcomes; produces reliability-by-task-type claims. **Uses external signals** (outcome Events, fixed verification suites, user feedback) — not the model's own metacognitive report. | MEL-REQ-247–250; LLM-metacognition `[E]` |
| **Reasoner Interface** | mechanism | The narrow, typed boundary between Ring 2's contributors and whatever performs the actual inference (a large local model, a small local model, a symbolic engine, or nothing). Input: a context projection + a question. Output: claims/proposals + confidence + a reasoning trace. No Chronicle write, no capability invocation, no Ring-0 access. **Because Ring 1/Ring 2 hold no network capability, the interface CANNOT open a socket** — it binds only to a local process / local library / local file-backed model. A remote reasoner is **not a configuration option**; it would be a Ring 4 component (out of scope) (RC-6). | Mission §8; MEL-REQ-153–155, 164; INV-1; AP-8, AP-9 |

### Ring 3 — Capabilities

| Subsystem | Class | Responsibility | Traces to |
|---|---|---|---|
| **Capability Host** | mechanism | Loads a capability component on demand into a sandbox with **no ambient authority and zero network capability by default** (L4 — a socket cannot be opened from Ring 3 unless the component is a Ring 4 network capability); hands it exactly the typed capability grants the Reference Monitor issued for this authorised action, **as live handles the Monitor can invalidate mid-action** (RC-5); checks handle validity at every effect boundary; enforces timeout, loop detection, resource ceiling; tears it down after. | MEL-REQ-124–128, 177, 184; WASI pattern `[E]`; AP-7, AP-8 |
| **Capability Lifecycle Manager** | mechanism | Runs DISCOVER-GAP → COMPOSE/SEARCH → DESIGN → CREATE → TEST → VERIFY → VERSION → AUTHORISE → USE → EVALUATE → RETIRE. "Compose before code." Enforces: a new capability is untrusted until it passes the fixed verification suite; risk class set by the deterministic classifier, never lowered by reasoning. | MEL-REQ-217–227; VC9 |
| **Terminal Control** | capability | Operate the local machine's command execution under a scoped grant. Distinct capability from GUI. **Authority is represented as structured actions (argv vectors / structured invocations), never as raw shell strings** (RC-4); a command grant is evaluated against the *parsed* invocation; **shell interposition (`;`, `\|`, `$()`, backticks, `&&`, subshells) is a distinct, higher-risk capability**, never equivalent to an exact-command grant. Direct process execution with an explicit argv is the default safe form. Observation vs execution separated; reversible/irreversible classified; destructive actions get one deliberate confirmation. | MEL-REQ-141–146; INV-7, INV-8 |
| **GUI Control** | capability | Operate the desktop GUI under a scoped grant. Distinct capability from Terminal. **GUI actions have structured representations where practical** (target element/window + action, not a raw coordinate stream) (RC-4). Same separations. | MEL-REQ-143–146 |
| **File Access** | capability | Read/write specific paths the user exposed, under a scoped grant. Never ambient. **Hard, non-overridable exclusion of the Ring 0 governance path** — the Reference Monitor rejects any grant whose resolved path (post symlink / `..` resolution) intersects the Ring 0 region (RC-1). | MEL-REQ-164, 179, 235 |
| **Verifiers** | capability | Deterministic (where practical) checks that a capability's result actually happened / is correct (tests, builds, diffs, state re-reads). **Verifier trust spec** (RC-3): minimal authority; **no ambient authority**; **read-only** on the checked state unless a narrowly-justified mechanism is required; **cannot directly cause effects**; independently testable against fixtures. Used by the pipeline's VERIFY stage and the capability TEST/VERIFY steps. Kept separate from the reasoner (self-certification forbidden). **High-risk actions require agreement from ≥2 *independent* verification mechanisms** (§6.6). | MEL-REQ-113, 142, 172, 221, 248 |

### Ring 4 — Network (future, isolated) — not built; see `AUTHORITY_AND_SECURITY_MODEL.md` §8.

---

## 3. Dependency directions (D)

```
   Ring 4  ──depends on──▶  Ring 3  ──▶  Ring 2  ──▶  Ring 1  ──▶  Ring 0
   (future)                (capabilities) (reasoning)  (core)    (governance)

   ── allowed:   outer → inner (read + request)
   ── forbidden: inner → outer (Ring 1 never calls Ring 2/3; Ring 0 calls nothing)
   ── forbidden: any → Ring 0 write (Ring 0 changes only via human file edit)
```

- **Ring 1 depends on Ring 0** (consults the Reference Monitor, budgets, tier
  policy) and on nothing outer. It is complete and useful alone (AP-9, INV-9).
- **Ring 2 depends on Ring 1** (Query, the Reasoner Interface, Policy Store) and
  on Ring 0 (budgets). It cannot function without Ring 1; Ring 1 can function
  without it.
- **Ring 3 depends on Ring 1** (Capability Registry, the pipeline) and on Ring 0
  (the Reference Monitor issues its grants). Capabilities do not depend on Ring 2
  — a user can invoke a capability directly through the pipeline with no reasoning
  involved.
- **No cycles.** The metareasoning controller does not "own" the system; it is a
  Ring 2 service that proposes.

---

## 4. Data ownership (E) — see `DATA_AND_STATE_MODEL.md` for detail

| Data | Owner | Authoritative? | Mutable? | Where |
|---|---|---|---|---|
| Events / Claims / Intentions | Chronicle | **yes** | append-only | Ring 1 |
| Entity ids / kinds / names | Entity Registry | yes | append-versioned | Ring 1 |
| Current-state, context, "what needs attention", views | Projection Engine | **no** (cache) | rebuildable | Ring 1 |
| User settings / conflict-axis positions / autonomy levels | Policy Store | yes | user act only, versioned | Ring 1 |
| Permission grants the user issued | Governance Store | yes | human act only, versioned | Ring 0 |
| Permission *rules*, tier policy, budgets | Governance Store | yes | human file edit only | Ring 0 |
| Capability metadata / versions / risk class / reliability | Capability Registry | yes | lifecycle-managed | Ring 1 |
| Capability *code* | (component files) | yes | lifecycle-managed, versioned | Ring 3 storage |
| Audit records | Audit | yes | append-only, tamper-evident | Ring 1 |
| Transient working context | Metareasoning Controller | no | discarded after use | Ring 2 memory only |
| MELFINA's beliefs / proposals / self-eval | Chronicle (as Claims/Intentions `holder=melfina`) | yes as *claims* | append-only | Ring 1 |

**The user owns all of it.** Everything except capability code and the transient
working context is in an open, documented, exportable form (MEL-REQ-160–163). Full
deletion is a Ring-1 redaction Event that tombstones the target across the
Chronicle + the current-state cache + any computed views (MEL-REQ-162).

---

## 5. Runtime lifecycle (F) — see `RUNTIME_MODEL.md` for detail

```
  STARTUP
   1. Ring 0 loads governance files, verifies their at-rest integrity signature
      (RC-1) and then their version chain, arms the Reference Monitor and
      Emergency Stop. An invalid signature or a broken chain ⇒ refuse to run
      (never proceed on suspect governance).
   2. Ring 1 Supervisor starts: Chronicle opens (verify append-log integrity);
      Projection Engine loads checkpoints and replays the Chronicle tail to
      catch projections up; Query, Pipeline SM, Audit, Notification Gateway,
      Policy Store, Capability Registry come up.
   3. Ring 1 is now fully operational. AI disabled ⇒ done (INV-9).
   4. Ring 2 activates lazily on first need (a proposal request, a question,
      a teaching session). Reasoner Interface binds to whatever local reasoner
      is configured — or reports "reasoning unavailable" and Ring 1 carries on.
   5. Ring 3 capabilities load on demand, per authorised action, and unload.

  IDLE
   Ring 0 + Ring 1 resident and quiet. Bounded background work only:
   projection catch-up, checkpointing, scheduled notification-window flush,
   any user-scheduled automation due. No polling of the reasoning ring.
   No network. Low CPU/RAM (AP-12).

  SHUTDOWN
   Pipeline SM refuses new consequential actions; in-flight actions run to a
   VERIFY or a clean abort; projections checkpoint; Chronicle closes atomically.
   Emergency Stop forces the same, faster, from a defined safe state.
```

---

## 6. The core flows (G–L)

### 6.1 Event flow (G)

```
  a change in the world / a user capture / an observation / an action result
        │
        ▼
  becomes an Event (+ any Claims it brings-about / ends)
        │
        ▼
  Chronicle.append  ── atomic, monotonic transaction-time ──▶  durable
        │
        ├──▶ Audit.append (if consequential)
        │
        ▼
  Projection Engine notified ──▶ updates the ONE current-state cache; other
                                 views ("what needs attention", calendar, …)
                                 are computed on demand, not maintained (O1);
                                 Context is a per-situation Ring-2 build, not
                                 built here (§8, M1). (idempotent; may lag; a
                                 consistency window is acceptable for reads,
                                 never for the append itself)
```
Capture is one action, no forced classification (MEL-REQ-020): the Event/Claim is
appended immediately; enrichment (kind, links, next action) is later Claims.

### 6.2 Reasoning flow (H) — a dynamic controller, not an agent loop

```
  trigger: a user question · a proposal request · a scheduled review ·
           a detected gap · a teaching session
        │
        ▼
  Metareasoning Controller:
    · frames the situation (focus + present + active Intentions, from Query)
    · asks Context Constructor for a relevance-ranked slice → the working
      context (transient, never persisted; blackboard-style is one candidate
      coordination pattern — O3)
    · decides: which contributors, what depth, whether more context helps or
      is noise, whether to decompose, whether existing capabilities suffice,
      whether to act / wait / ask / do nothing
    · runs contributors; each posts claims/partial results to the working context
    · re-evaluates after each; stops when the budget is reached OR marginal
      value drops OR an answer/decision is reached OR "do nothing" is chosen
        │
        ▼
  outputs (written to the Chronicle as Claims/Intentions, NOT as effects):
    · Claims (holder=melfina, status=inferred, full provenance + reasoning trace)
    · an Intention (status=proposed) IF an action is warranted
    · or a recorded "no action warranted, because…" Claim
    · every strategic choice logged (MEL-REQ-208, 253)
        │
        ▼
  working context discarded
```
Ring 0 budgets bound the whole thing. The controller **never** executes.

### 6.3 Decision flow (I)

```
  a proposed Intention  ──▶  Pipeline SM structural validation (M10):
  (from Ring 2, or a         well-formed target? references resolve? scope
   user-invoked request)     expressible in the grant vocabulary?
        │                        │ no ─▶ reject, record Event, stop
        ▼ yes
  classify: routine/reversible/pre-authorised  OR  consequential ?
    (deterministic classifier + Policy Store's consequential/routine boundary;
     "reversible?" reads "what holds now" through the CHRONICLE CONTRACT — a
     fresh authoritative read, not the current-state cache — M2)
    AGGREGATE CHECK (RC-2): is this action part of a chain whose routine actions
    have already crossed the aggregate budget (count / total affected scope)?
    is its aggregate rollback impractical? ─▶ if so, treat as CONSEQUENTIAL
        │
   ┌────┴─────────────────────────┐
   ▼                              ▼
 routine class                  consequential class
   │                              │
   ▼                              ▼
 policy default authorises     route to the user for AUTHORISE
 (act-then-show, HOTL)            (propose-then-wait, HITL)
   │                              │
   └──────────────┬───────────────┘
                  ▼
          AUTHORISE artefact recorded
```
"MELFINA may decide that no action should be taken" is a first-class decision
outcome — it produces a Claim, not an Intention (MEL-REQ-103).

**Aggregate-effect governance (RC-2):** the routine/reversible class is bounded
not only per-action but by a **per-activity-chain aggregate budget** (Budget
Authority, §2). A sequence of individually-routine actions whose combined effect
or combined irreversibility crosses the budget **escalates to consequential**,
regardless of activity/window framing. Concurrent conflict is judged
conservatively — two actions conflict if their **declared affected scopes
overlap**, and the pipeline serialises them (M9; semantic refinement is CORE
ENGINE).

### 6.4 Authorisation flow (J)

```
  an AUTHORISE artefact (user decision, or policy default for the routine class)
        │
        ▼
  Governance Store records the grant: a scoped, time-bounded, revocable
  capability grant for THIS action (what may be read/changed/run/communicated)
        │
        ▼
  Pipeline SM moves the Intention to status=authorised
        │
        ▼
  (nothing happens yet — authorisation ≠ execution)
```
Monotonic: a grant can be *narrowed* or *revoked* at any time without a further
act; *widening* requires a fresh AUTHORISE (AP-7).

### 6.5 Execution flow (K)

```
  an authorised Intention
        │
        ▼
  Pipeline SM → Reference Monitor:  grant valid? in scope (checked against the
                PARSED, structured action — argv / structured invocation, not a
                raw string — RC-4)? within per-action AND aggregate budget?
                not emergency-stopped?           │
        │                                        ▼
        │                                     DENY → recorded, Intention → blocked
        ▼
      PASS
        │
        ▼
  Capability Host: load the component into a sandbox with NO ambient authority
  and ZERO network capability (L4); inject the grants as LIVE HANDLES the
  Reference Monitor can invalidate; run with a hard timeout + loop detector +
  resource ceiling
        │
        ▼
  the capability performs the action, its handle re-checked at every effect
  boundary                                        │
        │                                         ▼
        │           ┌── grant revoked mid-action (targeted revocation, or the
        │           │    user overriding an in-progress action) ──────────────┐
        │           ▼                                                          │
        │     Capability Host STOPS the capability; rollback invoked if the    │
        │     action is reversible; the partial state is recorded as a         │
        │     PARTIAL FAILURE Event (RC-5) — see §6.6 outcome vocabulary       │
        ▼                                                                      │
  the capability returns a result ◀───────────────────────────────────────────┘
        │
        ▼
  Capability Host tears down the sandbox; result handed to VERIFY
```

### 6.6 Verification flow (L)

```
  a capability result
        │
        ▼
  Verifiers (deterministic where practical; minimal, no ambient authority,
  read-only, cannot cause effects — RC-3):
    · did the intended effect actually occur? (state re-read, test, build, diff)
    · is it within the authorised, PARSED scope?
        │
   ┌────┴─────────────────────────────────────┐
   │  HIGH-RISK action?  ──▶ require ≥2 INDEPENDENT verification mechanisms      │
   │  (architectural independence: different code paths / implementations /      │
   │   check methods — NOT a claim of statistical independence) to AGREE.        │
   │  Disagreement ⇒ "not verified" ⇒ the failure path.                          │
   │  If redundant verification is genuinely impractical for a specific action   │
   │  class, that class is an explicit, recorded exception — its actions are     │
   │  held to a higher AUTHORISE bar and a single Verifier, and the exception    │
   │  boundary is documented, not hidden.                                        │
   └────┬─────────────────────────────────────┘
        │
   ┌────┴────┐
   ▼         ▼
  verified   not verified / partial / failed / interrupted
   │         │
   ▼         ▼
  Event(s) appended:            Event appended with the OUTCOME VOCABULARY:
  the action + what it            completed · partially completed · rolled back ·
  brought-about/ended;            failed · interrupted (RC-5);
  Intention → fulfilled          rollback invoked where the action is reversible
                                 (NOT promised where it cannot be guaranteed);
                                 Intention → blocked; recorded honestly (no hiding)
        │
        ▼
  Audit updated; the current-state cache updated; if Ring 2 is active,
  Self-Evaluation notes the outcome against the prediction.
  Verifiers are themselves periodically self-checked against known-good /
  known-bad fixtures in the fixed regression suite; a Verifier that starts
  disagreeing with the fixtures is distrusted.
```

### Diagram 3 — THINK → DECIDE → PROPOSE → AUTHORISE → EXECUTE → VERIFY

```
  ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌───────────┐   ┌─────────┐   ┌─────────┐
  │  THINK  │──▶│ DECIDE  │──▶│ PROPOSE │──▶│ AUTHORISE │──▶│ EXECUTE │──▶│ VERIFY  │
  └────┬────┘   └────┬────┘   └────┬────┘   └─────┬─────┘   └────┬────┘   └────┬────┘
   Ring 2       Ring 2       Ring 2         Ring 1 SM +     Ring 3        Ring 1 +
   controller   controller   → Chronicle    user (HITL)     under         Verifiers
   + contribs   (may output   Intention      or policy       Ref-Monitor   → Chronicle
   over the     "no action")  status=        default (HOTL   grant +       Events;
   working ctx                proposed       for routine)    sandbox       rollback on
                                             → Gov. Store                  failure
                                             grant
        │            │            │               │              │            │
        ▼            ▼            ▼               ▼              ▼            ▼
   distinct     distinct     distinct        distinct       distinct     distinct
   artefact     artefact     artefact        artefact       artefact     artefact
        └────────────┴────────────┴───────────────┴──────────────┴────────────┘
                     EMERGENCY STOP interrupts between any two stages
                     (Ring 0, reachable without Ring 2)
```
Cognitive autonomy = THINK. Decision autonomy = DECIDE. Execution authority =
the AUTHORISE grant. **Never one permission.** (INV-2, AP-2.)

---

## 7. Persistence / storage boundary (M) — see `DATA_AND_STATE_MODEL.md`

The architecture commits to a **logical shape** and defers the **engine**:

- **Authoritative:** the Chronicle — an append-only, bitemporal sequence of
  `Event` / `Claim` / `Intention` units, each carrying holder/status/provenance/
  confidence/valid-time/transaction-time/supersedes. Plus the Entity Registry,
  Policy Store, Capability Registry, Governance Store, Audit — each
  append-versioned.
- **Derived (caches):** **one materialised current-state cache** + on-demand
  computed views (O1). Loss of every derived view loses nothing; all rebuild from
  the Chronicle. Where consequential correctness depends on "what holds now", the
  read is a fresh authoritative Chronicle read, not the cache (M2).
- **Deferred:** the storage engine/product, the on-disk format, indexing, whether
  the current-state cache is in-memory / on-disk / both (eager vs lazy). The Ring 0
  at-rest integrity mechanism (RC-1). Constrained by AP-4 (claim-shaped), AP-5
  (append-only + rebuildable), INV-1 (no server dependency), MEL-REQ-161
  (open, documented, outlives the software).

Alternatives (pure event-sourced vs mutable-state+history vs **hybrid**) are
compared in `ARCHITECTURAL_ALTERNATIVES.md` AA-2; **hybrid recommended**
(Chronicle authoritative + materialised current-state as a rebuildable cache),
which pays down event-sourcing's slow-read cost while keeping the time-travel and
provenance guarantees. Confidence: moderate.

---

## 8. Context construction (N)

Context is a **Ring 2 derivation built ON DEMAND per situation** — it is **not a
maintained projection**, is never authoritative, and is never persisted (model
§7, INV-3→AP-3, INV-13; M1).

```
  situation = { focus (Entity/Event/Intention), present (valid-time,
                transaction-time), active Intention(s), optionally place }
        │
        ▼
  Context Constructor → Query + the current-state cache (for non-consequential
  framing) / the Chronicle (where correctness matters): a relevance-ranked slice
  of Claims/Events/state-content/Intentions connected (directly or via short
  relation paths) to the focus, ranked by expected effect ÷ effort
        │
        ▼
  Constructor decides inclusion/exclusion, records the decision (auditable:
  what was included, what excluded, why — MEL-REQ-213)
        │
        ▼
  the working context — transient, in Ring 2 memory, discarded after the activity
```
"Acquire more context" = ask Query again (local only — never the network,
MEL-REQ-214). "Discard context" = drop from the working set; the Chronicle unit
is untouched (MEL-REQ-215). The *relevance algorithm itself* is deferred to CORE
ENGINE (model OQ-M6).

---

## 9. Memory architecture (O)

MELFINA has **one** memory: the Chronicle. There is no separate "long-term
store", "vector memory", or "episodic buffer" as an architectural primitive —
those, if they appear, are **projections or indexes** the Projection Engine
maintains for retrieval, all rebuildable, none authoritative (AP-5).

- **Storage ≠ memory** (model §12): a file the user drops in is an `Entity` (kind
  `source`); what MELFINA *knows from it* is Claims (`status = reported`,
  provenance → that Entity + the ingest Event).
- **Remembering** = a Query (possibly relevance-ranked) over Chronicle +
  projections. `status = recalled` marks the normal fallibility.
- **Forgetting** = (a) a Claim's valid-time ends / confidence decays / it is
  superseded → simply not surfaced (relevance), or (b) an explicit user deletion →
  a redaction Event that tombstones across Chronicle + projections.
- **MELFINA's adaptation memory** (learned preferences) = `inferred` Claims
  `holder = melfina` `about` the user — explicit, inspectable, supersedable,
  forgettable (MEL-REQ-118–122). **Not** weights, not a hidden model state.
- **Retrieval never invents** (MEL-REQ-093): Query returns stored units; any
  synthesis Ring 2 does over them is a new `inferred` Claim, visibly distinct.

Reliable autonomous consolidation/forgetting is research-stage (model C-2, `[U]`)
— the architecture keeps it as an *optional Projection Engine behaviour* with a
human-visible policy, never a silent process.

---

## 10. Capability / skill architecture (P) — see `CAPABILITY_MODEL.md`

- A **capability** is an `Entity` (kind `capability`) + Claims about it (what it
  does, declared authority, version, risk class, reliability-by-task-type) +
  a **component** (code/config) stored outside Ring 1.
- **No ambient authority** (AP-7): a capability starts able to do nothing; the
  Reference Monitor injects narrow, typed, revocable grants per authorised action.
- **Contextual selection & composition** (MEL-REQ-216): the metareasoning
  controller picks which capabilities fit and composes them; composition is
  itself expressed as a workflow-Entity.
- **The catalogue is not fixed** (MEL-REQ-218): capabilities are created, versioned
  (`supersedes`), and retired (redaction Event) over MELFINA's life.
- **Risk class** is set by a **deterministic classifier** in Ring 1 from what the
  capability's declared authority touches (data model / computer / permissions /
  external / core). Ring 2 **cannot lower it** (MEL-REQ-227) — the classifier is
  mechanism, its inputs are the capability's own declared authority, and the
  Reference Monitor reads the class from the Registry, not from a reasoning claim.

### Diagram 4 — Capability lifecycle

```
   DISCOVER GAP        a Claim "no available capability covers task-type T"
        │              (status=inferred; a real gap vs "the model wants to
        ▼               build" is `requirements` OQ-17 — gated by the steps below regardless)
   SEARCH / COMPOSE    can existing capabilities (alone or composed) do it?
        │  ── yes ──▶  compose → a workflow-Entity → TEST
        ▼  no
   DESIGN             specify the missing capability (interface, authority needed)
        │
        ▼
   CREATE             compose-before-code; if code: generate into a sandbox
        │
        ▼
   TEST               run against the FIXED verification suite (Ring 3 Verifiers,
        │             deterministic — not the reasoner's self-assessment)
        ▼
   VERIFY             effect correct? within declared authority? no escalation?
        │
        ▼
   VERSION            register: id, version, declared authority, risk class
        │             (deterministic classifier — for a COMPOSED workflow the
        │             class is the UNION of component authorities + component
        │             scope + aggregate effect, never per-step; RC-2),
        │             provenance (generated-by Event)
        ▼
   AUTHORISE          risk class ⇒ governance: trivial ⇒ user confirm; higher ⇒
        │             explicit user authorisation; tiers 6–9 ⇒ human-authored path.
        │             The Reference Monitor caps the new capability's maximum
        │             possible grant at { what the user authorises } ∩ { the
        │             creating context's current authority } (M8; MEL-REQ-222).
        ▼
   USE                Capability Host, sandboxed, per-action grants
        │
        ▼
   EVALUATE           Self-Evaluation: reliability-by-task-type claims from
        │             recorded outcomes; periodic replay-test on the fixed suite
        ▼
   RETIRE / REPLACE   obsolete/harmful ⇒ redaction Event ends "active"; the
                      Entity + history stay; a replacement supersedes it
```

---

## 11. Workflow / automation architecture (Q)

- An **automation** = a `melfina`-owned `Intention` with `standing = recurring` or
  a trigger condition, + a `plan`, + the `Event`s it performs, + verification
  Claims. It is **not** a new subsystem — it is data in the Chronicle + the
  ordinary pipeline.
- **Three permission levels** (MEL-REQ-111): *generation* (Ring 2 may draft an
  automation), *authorisation* (a user act — makes it eligible to run),
  *execution* (each run goes through the full pipeline + Reference Monitor).
- **Triggers**: scheduled (a time), event-conditioned (a Chronicle Event
  matching a pattern), state-conditioned (a projection reaching a condition).
  Event vs state is made explicit in the automation's definition (users confuse
  them, `[E]`).
- **Loop/runaway protection**: Budget Authority caps every automation run **and
  the automations-fired-per-window count**; a loop detector operating on the
  **automation-trigger graph** (which automation's outputs can satisfy which
  automation's triggers) halts cycles of **any length**, not just a single
  self-triggering automation (M6; MEL-REQ-115).
- **History, ownership, pause, disable, override**: all present — history is the
  Event stream, ownership is `generated-by`, pause/disable/override are user acts
  the pipeline honours (MEL-REQ-114).

---

## 12. Terminal control (R) and GUI control (S) architecture

Both are **Ring 3 capabilities**, **distinct** (separate grants, MEL-REQ-143),
each split into: observe · reason (Ring 2) · propose · authorise · execute ·
verify (mission R/S; MEL-REQ-144).

> **AU-2 (the grant representation + action parsing) is CONTAINMENT-CRITICAL**
> (RC-4). The containment of terminal/GUI control lives *entirely* in the grant
> being checked against a **parsed, structured action**, not a raw string.
> A coarse grant model collapses the boundary.

| Concern | Mechanism |
|---|---|
| structured actions, not raw strings | terminal authority is a set of permitted **argv vectors / structured invocations**; a command grant is evaluated against the **parsed** invocation; GUI actions are **target-element + action**, not a raw coordinate/keystroke stream, where practical (RC-4) |
| no shell interposition by default | terminal control executes a process directly with an explicit argv; **shell metacharacters (`;` `\|` `$()` backticks `&&` subshells) / interpolation are a DISTINCT, higher-risk capability** — raw shell execution is *never* treated as equivalent to an exact-command grant (RC-4) |
| least privilege | a grant names exactly the permitted argv shapes / windows / paths in scope |
| scoped authority | time-bounded, action-bounded; the grant is a **live handle** the Reference Monitor can invalidate mid-action (RC-5); revoked after the action |
| reversible vs irreversible | the deterministic classifier tags each proposed action; irreversible ⇒ extra confirmation (MEL-REQ-181); **N reversible actions whose aggregate rollback is impractical classify up** (RC-2) |
| destructive-action protection | deletion / overwrite / external send ⇒ a distinct, deliberate confirmation, never chained (MEL-REQ-056, 181) |
| user override | any pending or in-progress action is stoppable by the user; mid-action, the grant handle is invalidated and the action is stopped + rolled back if reversible + recorded with the outcome vocabulary (RC-5) |
| emergency kill switch | Ring-0 Emergency Stop halts all Ring-3 executions instantly |
| audit trail | every command/gesture logged with inputs, grant used, effect |
| failure recovery | Capability Host teardown + rollback where reversible; Supervisor contains the crash |
| timeout / loop protection | hard timeout + loop detector + resource ceiling in the Host |
| privilege-escalation prevention | monotonic grants; a capability cannot acquire authority beyond its grant; the Reference Monitor is always invoked |

Autonomous execution here is limited to **narrow, reversible, pre-authorised
scopes** (MEL-REQ-144); anything else is proposed.

---

## 13. Learning / teaching architecture (T)

Not a subsystem — a **use of the reasoning ring over the Chronicle**:

- **Subjects, concepts, prerequisites** = `Entity`s + `prerequisite-of` Claims.
- **Understanding** = a descriptive Claim "self understands C at depth D",
  superseded upward; **uncertainty** = its confidence.
- **A teaching session** = a `session` Event; a **learning trajectory** = ordered
  `Intention`s targeting "understands C" conditions.
- **Spaced review** = `melfina`-owned `recurring` Intentions to resurface concepts
  (MEL-REQ-072) — routed through the Notification Gateway like any prompt, subject
  to the same restraint.
- **Socratic default** (MEL-REQ-071): a Policy Store setting controls how much the
  teaching contributor withholds vs supplies; default leans to guided discovery.
- **A learning-history projection** exists for the user's benefit — subject to
  AP-10 (not a checking surface; no guilt for forgetting).

## 14. Scientific reasoning architecture (U)

A **discipline the metareasoning controller applies**, not a module (model §13.4):

- observation → `observation` Event → `observed` Claim
- question → `open` Claim · hypothesis → `hypothesised` Claim
- prediction → Claim `derived-from` the hypothesis, `about` an expected Event
- experiment → `session` Event with `object` = the hypothesis
- evidence → `evidence-for/-against` Claims
- conclusion → `inferred` Claim, provenance = the evidence chain
- revision → a new Claim `supersedes` the old (AGM); both retained

The controller enforces: separate derived from retrieved (MEL-REQ-096); check
apparent novelty against known results, flag unverifiable novelty (MEL-REQ-135);
reasoning is reproducible (a stored trace, MEL-REQ-131); "here is how I derived
this" over "trust me" (MEL-REQ-136). Rediscovery in formalisable domains is a
capability (a symbolic/derivation contributor + Verifiers), narrow-only per the
feasibility classification (MEL-REQ-134; `[E]`/`[U]`).

## 15. Self-evaluation architecture (V)

Covered in §2 (Ring 2) and Diagram 4's EVALUATE. Key structural point
(MEL-REQ-248): Self-Evaluation is **mechanism that reads recorded outcomes and
runs fixed verification suites** — it does **not** ask the reasoner "how did you
do?". Its outputs are Claims the Capability Lifecycle Manager and the
metareasoning controller consult (to de-prioritise / retire unreliable
capabilities and strategies, MEL-REQ-249).

## 16. Capability creation / evolution architecture (W) — `CAPABILITY_MODEL.md`

Diagram 4 above. Structural guarantees: compose-before-code (MEL-REQ-220);
untrusted until tested on the fixed suite (MEL-REQ-221); **least authority on
creation — an explicit Reference Monitor check caps the new capability at
{ what the user authorises } ∩ { the creating context's current authority }**
(M8; MEL-REQ-222); code capabilities safe via sandbox+test+rollback+human-gate,
not proof (MEL-REQ-223, verified codegen `[U]`); **risk class not self-lowerable —
and a composed workflow is classified by the UNION of its components' authority,
scope, and aggregate effect, never per-step, so composition cannot launder a
high-risk action into low-risk steps** (RC-2; MEL-REQ-227); every version
retained, rollback available (MEL-REQ-225). The invariant that makes AD-8 sound:
**the sandbox ceiling ≡ the declared authority** — an under-declared capability
simply cannot do its job and fails testing, so risk cannot be *usefully* lowered.

## 17. Self-modification governance architecture (X) — `AUTHORITY_AND_SECURITY_MODEL.md` §5–§6

### Diagram 5 — Self-modification governance boundary

```
   proposed self-change
        │
        ▼
   Deterministic Tier Classifier (Ring 1, reads WHAT THE CHANGE TOUCHES):
        1 config data          2 learned Claims        3 a new capability
        4 an existing capab.   5 a workflow            6 MELFINA's own code
        7 a subsystem          8 architecture          9 self-replacement
        │                      │                       │
   ┌────┴──────┐          ┌────┴─────┐            ┌────┴───────────────────┐
   ▼           ▼          ▼          ▼            ▼                        ▼
  T1–T2       T3–T5      T3–T5      T3–T5        T6–T8                    T9
  Ring 1      Capability Lifecycle (Ring 3):    NOT a MELFINA operation.  human
  applies     compose·test·verify·version·      A human edits versioned   decision
  after a     AUTHORISE (user, class-scaled)    code/subsystem/arch       only;
  user        + regression-tested               using ordinary dev tools; MELFINA
  confirm                                        MELFINA MAY assist under  cannot
  matched to                                     the normal pipeline but   initiate
  the setting                                    cannot perform T6+ itself
        │           │            │                    │                    │
        └───────────┴────────────┴────────────────────┴────────────────────┘
                                  ▼
   ┌──────────────────────────────────────────────────────────────────────┐
   │  RING 0 — THE META-INVARIANT (read-only to everything above)          │
   │  · There is NO capability whose effect is "widen my authority" or     │
   │    "edit Ring 0". That operation is absent from the namespace.        │
   │  · The tier policy + permission rules live in versioned files that    │
   │    Ring 1–3 can READ (to explain) and have NO write path to —         │
   │    at RUNTIME *and* AT REST:                                          │
   │      – cryptographically signed; trust root = a human-held key        │
   │        MELFINA never possesses; verified every startup; invalid ⇒     │
   │        refuse to run (RC-1)                                           │
   │      – the Ring 0 path is HARD-EXCLUDED from every File Access grant;  │
   │        the Reference Monitor rejects any grant whose resolved path    │
   │        intersects it (RC-1)                                           │
   │  · Any change to them is a human file edit, recorded, monotonic       │
   │    (authority only narrows without an explicit human widening act).   │
   │  · The Reference Monitor is non-bypassable, always invoked, evaluable.│
   └──────────────────────────────────────────────────────────────────────┘
        │
        ▼
   DOCUMENTED RESIDUAL LIMITS (this is necessary, not sufficient — MEL-REQ-239):
   · Ring 2 could socially engineer the USER into a Ring-0 change → mitigation:
     Ring-0 changes are rare, deliberate, and framed as such; the user is the
     backstop, and the requirements' anti-manipulation safeguards apply.
   · An unintended bug in Ring 1 could grant unintended authority → mitigation:
     Ring 1 is deliberately small, auditable, and a candidate for formal checks;
     the smaller Ring 0/1, the smaller this surface.
   · A capability could pass every test yet generalise badly → mitigation:
     periodic regression on the fixed suite, capability-drift monitoring, easy
     rollback, and keeping T6+ human-driven and rare.
   · "MELFINA may assist" a human with T6–9 is the soft spot — a rubber-stamping
     human lets MELFINA effectively author a core change. Not resolved; the
     strongest hardening (no assistance at all) is very restrictive; the user
     decides. (Adversarial review §13, §18.)
```

## 18. Audit / provenance architecture (Y)

- **Provenance is structural, not metadata** (AP-4): every Claim carries
  `generated-by` / `derived-from` / `attributed-to`; MELFINA answers "how do I
  know this?" by walking those links to an `observation` Event, a `source`, or a
  reasoning trace.
- **The Audit log is separate from the Chronicle** because it also records
  *reads* (Ring-2 data access, MEL-REQ-156) and permission grants/revokes, which
  are not life-events. It is append-only and **tamper-evident** (the mechanism —
  hash chain / signed segments — is deferred to LOW-LEVEL FOUNDATIONS; the
  requirement is the property, MEL-REQ-203).
- **The user can see what MELFINA is doing and has done** (MEL-REQ-201): a plain
  view over Audit + the `attributed-to melfina` Events.

## 19. Failure / recovery architecture (Z) — see `FAILURE_AND_RECOVERY.md`

### Diagram 7 — Failure / recovery path

```
  failure detected (crash · timeout · loop · verification failure · corrupt view)
        │
        ▼
  Supervisor identifies the subtree
        │
   ┌────┴───────────────────────────┬───────────────────────────┐
   ▼                                ▼                           ▼
 Ring 3 capability execution      Ring 1 projection            Ring 1 core service
 crashed / timed out / looped     rebuild failed / corrupt     (Chronicle / Query /
   │                                │                          Pipeline SM) crashed
   ▼                                ▼                            │
 Capability Host teardown;        discard the view; replay      ▼
 rollback if reversible;          from the last good           restart from a
 Event: "action X failed";        checkpoint; if replay        clean state;
 Intention → blocked;             fails, rebuild the view      Chronicle integrity
 contained — Chronicle & core     from scratch from the        re-verified on open;
 untouched                        Chronicle                    in-flight pipeline
   │                                │                          actions resume at
   ▼                                ▼                          their last artefact
 retried per strategy, or         "let it crash" — the         or abort cleanly
 abandoned; user informed         Chronicle is the ground        │
 neutrally (no guilt UI)          truth, so a lost view          ▼
                                  loses nothing                Emergency Stop path:
                                                               Ring 0 halts all,
                                                               safe state, report
```

Recovery guarantees: appends are atomic (no partial Events); the Chronicle is the
single ground truth (any projection is disposable); failure is contained to a
supervision subtree (INV-11); nothing about a failure is surfaced with pressure
or blame (AP-10, MEL-REQ-047).

## 20. Security / isolation boundaries (AA) — `AUTHORITY_AND_SECURITY_MODEL.md`

**The boundaries that MUST be real, enforced boundaries** (per RC-7 / AP-12 — the
rest of the ~25-subsystem decomposition is conceptual and may be co-located):
- **Ring 0 ↔ everything: read-only outward, no write inward** — at runtime *and*
  at rest (signed; Ring 0 path hard-excluded from File Access — RC-1). The
  meta-invariant.
- **Ring 1 ↔ the sandboxed outer region (Ring 2 + Ring 3):** the outer region
  gets **Query (read) + (for Ring 2) the Reasoner Interface + (for Ring 3) the
  per-action grants** — and nothing else. **No Chronicle write, no capability
  invocation from Ring 2, no network capability, no Ring 0 access.**
- **The reasoning↔effect boundary:** Ring 2 emits only proposals + claims;
  **Ring 2 cannot invoke a capability.** (Do not collapse this — O2 rejected.)
- Ring 3 execution: **sandboxed, no ambient authority, ZERO network capability by
  default** (L4), per-action typed grants as live handles, hard resource limits.
- Ring 3 ↔ Ring 4: Ring 4 is absent; if built, it is a component that
  **structurally cannot** hold {private-data read + outbound}, and neither while
  ingesting untrusted content — enforced as an explicit **Ring 0 governance rule**
  (L5), not prose.
- **Threat model** and the lethal-trifecta analysis: `AUTHORITY_AND_SECURITY_MODEL.md`
  §7–§9.

## 21. Resource management (AB) — see `RUNTIME_MODEL.md` §5

- **Always-resident:** Ring 0 + Ring 1 substrate only, deliberately small (AP-12).
- **On-demand:** Ring 2, Ring 3 components, any model — activated on need,
  released after (or after an idle timeout).
- **Bounded background:** projection catch-up, checkpointing, notification-window
  flush, due automations — each with a work budget; no busy polling; no keeping a
  model warm just for latency (`RUNTIME_MODEL.md` §5; MEL-REQ-196–199).
- **Budgets:** Ring 0 caps autonomous work (steps, time, invocations, metered
  resources) per activity, per window, **and per activity-chain in aggregate**
  (RC-2), enforced by an **external watchdog**, not self-checked.
- **Graceful degradation:** if resources are tight, the metareasoning controller
  chooses cheaper strategies / shallower reasoning / smaller context / declines
  (MEL-REQ-155, 243); Ring 1 is never starved.

## 22. Local AI integration boundary (AC)

The **Reasoner Interface** (Ring 2) is the sole boundary:

```
   Ring 2 contributor  ──(context projection + a question)──▶  Reasoner Interface
                       ◀──(claims / proposals + confidence + reasoning trace)──
```
- The reasoner behind it may be a large local model, a small local model, a
  symbolic engine, an ensemble, or **absent**. Swapping it changes nothing
  outside Ring 2 (AP-9).
- The reasoner **never** gets: Chronicle write, capability invocation, Ring 0
  access, or ambient file access.
- **Locality is structural, not a preference** (RC-6): Ring 1/Ring 2 hold **no
  network capability**, so the Reasoner Interface — resident in the local core —
  **cannot open a socket**. It binds only to a **local** process / library /
  file-backed model. A remote endpoint is **not a configuration option**; a
  remote reasoner would be a Ring 4 component (out of scope, off by default).
  The architecture says **"cannot"**, not "does not".
- It gets: the specific context projection it was handed (read-only, transient),
  and an output channel for proposals/claims.
- **Local reasoner ceiling:** if a task exceeds the local reasoner, MELFINA
  returns a clearly-marked lower-capability local result or **declines** — never
  reaches out (it structurally cannot). Which tasks are feasible locally is OQ-11
  (deferred to AI/ASSISTANT LAYER).

## 23. Future network isolation boundary (AD)

### Diagram 6 — Local core vs future network boundary

```
   ┌───────────────────────────────────────────┐   ║   ┌────────────────────────┐
   │  LOCAL CORE  (Rings 0–3)                    │   ║   │  RING 4 (FUTURE)        │
   │                                            │   ║   │  a network capability   │
   │  Chronicle · reasoning · capabilities ·     │   ║   │  · off by default       │
   │  terminal/GUI control · everything the      │   ║   │  · separately installed │
   │  user relies on                            │   ║   │  · clearly indicated    │
   │                                            │   ║   │    when active          │
   │  Rings 0–3 hold NO network capability —      │   ║   │  · fully removable      │
   │  the Reasoner Interface CANNOT bind a       │   ║   │  · NEVER a dependency   │
   │  remote endpoint; Ring 3 sandboxes CANNOT   │   ║   │    of any core function │
   │  open a socket (RC-6, L4).                   │   ║   │  · NO Ring 0 access     │
   │  Works forever with networking disabled.    │   ║   │  · all traffic audited  │
   └───────────────────────────────────────────┘   ║   └────────────────────────┘
                                                   ║
        ═══════════════════════════════════════════╬═══════════════════════════════
        the boundary is ARCHITECTURAL, not a config flag:
        · Ring 4 is a distinct component the user must physically add
        · the "no {outbound + private-data read}, and neither while ingesting
          untrusted content" rule is an explicit RING 0 GOVERNANCE RULE (L5),
          enforced by the Reference Monitor — not prose (lethal-trifecta break,
          MEL-REQ-126)
```

## 24. Human interaction boundary (AE)

- **Inbound (user → MELFINA):** captures, questions, decisions/authorisations,
  settings changes, capability invocations, emergency stop. Each is an Event or a
  Policy/Governance change; captures are one-action, no forced classification.
- **Outbound (MELFINA → user):** two channels only —
  (1) **pull** (the user asks / opens a view) — the default for everything;
  (2) **push** (the Notification Gateway) — a single, user-governed, predictably
  scheduled path, minimal by default, no behaviour-change nudging, no repetition
  escalation (AP-10; MEL-REQ-011, 041–047).
- **The interface medium is deferred to INTERFACE.** The architecture only
  requires: multi-modal, user-choosable, CLI-friendly; nothing forces a single
  modality; the hazard surfaces (§AP-10) are honoured whatever the medium.

### Diagram 2 — Data flow (consolidated)

```
   USER ──capture / question / decision / setting / invoke / STOP──┐
                                                                    ▼
   ┌───────────────────────── RING 1 ─────────────────────────────────────────┐
   │  captures / results ──▶ Chronicle (append-only) ──▶ Projection Engine    │
   │       ▲                     │  │                          │             │
   │       │                     │  └──▶ Audit                  ▼             │
   │       │                     ▼                   current-state CACHE +    │
   │   Verifiers ◀── EXECUTE   Query ◀───────────────  on-demand views        │
   │  (≥2 for      ▲          │  ▲   (DECIDE reads     (context is NOT here —  │
   │   high-risk)  │          │  │    fresh here)       it is a Ring-2 build)  │
   │       │          │          │  │                          │             │
   └───────┼──────────┼──────────┼──┼──────────────────────────┼─────────────┘
           │          │          │  │                          │
      Ring 3 cap.  Pipeline SM   │  │ (read only)              │ (pull)
      (sandboxed)  + Ref-Monitor │  │                          ▼
           ▲          ▲          ▼  │                     USER (views)
           │          │     ┌─── RING 2 (optional) ───┐
      grant│    proposal│    │  Metareasoning          │
           │          │      │  Controller ─▶ Context  │──▶ Reasoner Interface
      ┌────┴──────────┴──┐   │  Constructor ─▶ working  │      ▲   │
      │  RING 0            │   │  context (transient) ──▶ │  local reasoner
      │  Reference Monitor │   │  Contributors           │  (model / symbolic /
      │  Budgets · Tiers  │   │  Self-Evaluation        │   none)
      │  Emergency Stop   │   └─────────────────────────┘
      └───────────────────┘         │ (claims / proposals only — no effects)
                                    ▼
                              Chronicle (as Claims / Intentions)
```

---

## 25. Most important architectural decisions

| # | Decision | Rationale | Traces to | Confidence |
|---|---|---|---|---|
| AD-1 | **Five inward-only rings** (Governance / Core / Reasoning / Capabilities / future Network) | forces the autonomy triad, AI-is-not-the-whole-system, least authority, and local-only into *structure* rather than convention | INV-2,6,7,9; mission §8 | High |
| AD-2 | **Ring 0 is read-only to all outer rings; changed only by human file edit** | the meta-invariant is structural — no MELFINA operation can widen its own authority | MEL-REQ-235; monotonic confinement `[E]` | High (as a requirement); the *sufficiency* is `[U]` (AD-9) |
| AD-3 | **The Chronicle (append-only Events/Claims/Intentions) is the single source of truth; everything else is a rebuildable projection** | E²CI is bitemporal, append-only, provenance-carrying — event sourcing is the shape that falls out; projections give the read performance | model §4,§11,P-5; event-sourcing `[E]` | Moderate–High |
| AD-4 | **Hybrid persistence: Chronicle authoritative + materialised current-state as a cache** | pays down event-sourcing's slow-read cost without losing time-travel/provenance | AA-2 | Moderate |
| AD-5 | **Reasoning is a dynamic control problem, not an agent loop** (blackboard-style is one candidate pattern, chosen in CORE ENGINE — O3) | matches "dynamic not arbitrary": a controller selects contributors/depth/context and whether to act at all; bounded; explainable | MEL-REQ-204–213,251–253; blackboard `[E]` | Moderate–High |
| AD-6 | **Ring 2 emits only proposals + claims; it cannot write the Chronicle or invoke a capability** | separates cognitive/decision autonomy from execution authority structurally | INV-2, AP-2 | High |
| AD-7 | **Capabilities are sandboxed components with no ambient authority; per-action typed grants from the Reference Monitor** | least authority; blast-radius containment; assume prompt injection wins | MEL-REQ-124,179,184,185; ocap + WASI `[E]` | High |
| AD-8 | **Risk class set by a deterministic Ring-1 classifier from declared authority; Ring 2 cannot lower it. A composed workflow is classified by the UNION of component authority + scope + aggregate effect (RC-2). Sandbox ceiling ≡ declared authority.** | prevents "talk the system into a lower gate" and "launder a high-risk action into low-risk steps" | MEL-REQ-227; RC-2 | High |
| AD-9 | **Self-modification tiers 6–9 are not MELFINA operations at all** — a human edits versioned code/subsystem/architecture with ordinary tools; MELFINA may assist under the normal pipeline | keeps recursive-self-improvement risk bounded; the meta-invariant necessary-not-sufficient limits are documented | MEL-REQ-233,236,239 | Moderate (the boundary is clean; residual social/bug risk remains) |
| AD-10 | **One memory (the Chronicle); no separate long-term/vector/episodic store as a primitive** | storage ≠ memory; retrieval structures are rebuildable views | model §12; AP-5 | Moderate–High |
| AD-11 | **The Notification Gateway is the single push path; everything else is pull** | the hazard surfaces (interruption, repetition) get *one* governed choke point instead of being cross-cutting | MEL-REQ-011,041–047; Fitz `[E]` | High |
| AD-12 | **Supervision-based failure isolation; the Chronicle is the recovery ground truth** (topology deferred — O4) | failure contained not cascading; recovery = rebuild views from checkpoint | MEL-REQ-015,172,173; OTP `[E]` | High |
| AD-13 | **Technology deferred** (language, storage engine, isolation impl, reasoner, format, the Ring-0 signing scheme, the action parser) | architecture defines structure; the pipeline puts these in later phases; premature binding risks the invariants | project pipeline; `ARCHITECTURAL_PRINCIPLES.md` §3 | High |
| AD-14 | **The self-modification meta-invariant is enforced at rest as well as at runtime** — Ring 0 files are signed (human-held key) and hard-excluded from every File Access grant (RC-1). Verifier trust is specified and high-risk actions need ≥2 independent verifiers (RC-3). Mid-execution revocation is defined (RC-5). | closes the adversarial-review H1/H3/H5 gaps between "the architecture says" and "structurally impossible" | RC-1,3,5; adversarial review §1–3 | High for the structure; the meta-invariant's *sufficiency* remains `[U]` |
| AD-15 | **Aggregate-effect governance** — routine actions carry a per-activity-chain aggregate budget that escalates to consequential on crossing; compositions are aggregate-risk-classified (RC-2). | closes the aggregation escape from the routine class and composition | RC-2; MEL-REQ-019,181,220,227 | High |

## 26. Alternatives rejected and why (summary — full in `ARCHITECTURAL_ALTERNATIVES.md`)

| Choice | Rejected | Because |
|---|---|---|
| process model | **one monolithic process** | can't contain a compromised capability or a runaway reasoner; violates AP-8, AP-11 |
| process model | **full microkernel with per-subsystem IPC** | overhead + complexity disproportionate for a single-user local system; violates AP-12 (one maintainer, lightweight) — the *conceptual* separation is kept, the literal IPC-everywhere is not |
| persistence | **pure event-sourced (no materialised state)** | "reads are slow and unpredictable" (`[E]`); the personal-assistant "what needs attention" read would be a full replay |
| persistence | **mutable current-state + a side audit log** | loses native bitemporal time-travel and provenance-as-structure; AP-4/AP-5 |
| reasoning | **a single autonomous agent loop** | the mission forbids it; collapses the autonomy triad; hard to bound and explain |
| reasoning | **a fixed pipeline of reasoning stages** | not dynamic; can't vary depth/strategy/context per situation (VC8) |
| self-mod governance | **rules stored as Claims MELFINA can reason over** | then reasoning can propose editing them; violates INV-6 |
| self-mod governance | **a "modify self" capability gated by permission** | the gate itself becomes a target; better that the operation is *absent* for T6+ |
| AI boundary | **the model as orchestrator / source of truth** | mission §8 forbids; violates AP-9, AP-3, AP-4 |
| capability isolation | **trust capabilities, check outputs** | assume prompt injection wins (AP-8); containment must be structural |
| ring structure | **collapse Ring 2 (reasoning) and Ring 3 (effects) into one outer region** (adversarial review O2) | rejected — the reasoning↔effect boundary is one of the three must-be-real boundaries (§20). Implementations MAY co-locate reasoning and capability code in one OS process for lightness, but the authority distinction — Ring 2 emits only proposals/claims and holds no grant; Ring 3 holds per-action grants and can cause effects — must remain a real, enforced check, not a code convention. |
| verifier trust | **let the reasoner judge whether an action succeeded** | a reasoner that can be prompt-injected can also be injected into a false "it worked"; VERIFY must be deterministic-where-practical, minimal-authority, independently testable (RC-3) |
| terminal authority | **grant a capability the ability to run arbitrary shell strings** | raw shell execution is never equivalent to an exact-command grant; command grants are matched against parsed argv; shell interposition is a distinct, higher-risk capability (RC-4) |

## 27. Remaining architectural uncertainties

| ID | Uncertainty | Depends on | Minimum commitment now |
|---|---|---|---|
| AU-1 | Is the hybrid persistence (AD-4) the right read/write split, or should current-state be fully lazy? | STORAGE + prototype; OQ-M2, OQ-M8 | commit only to "Chronicle authoritative + projections rebuildable"; leave the materialisation strategy open |
| AU-2 **(CONTAINMENT-CRITICAL — RC-4)** | How fine-grained are capability grants, and is the grant a token, a handle, or a policy record? How is a terminal/GUI action represented structurally (parsed argv / structured invocation) so that a command grant is matched against structure, never a raw string? | LOW-LEVEL FOUNDATIONS; ocap literature | commit to "unforgeable, scoped, revocable *mid-execution* (RC-5), monotonic, no-ambient; terminal authority = structured actions not raw shell strings; shell interposition is a distinct higher-risk capability"; not the representation or the parser. This is the **first** LOW-LEVEL FOUNDATIONS deliverable. |
| AU-3 | Does the metareasoning controller need its own persistent state, or is it stateless-per-situation? | CORE ENGINE; OQ-M6 | commit to "the working context is transient"; the controller's own learning is `inferred` Claims in the Chronicle (AD-10) |
| AU-4 | Is `State` a fifth model primitive → does the Projection Engine need a first-class current-state store? | model OQ-M2; prototype | architecture works either way — current-state is a projection regardless; a primitive `State` would make it a *checkpointed* projection, not a new subsystem |
| AU-5 | Can Ring 1 be small enough to be formally checkable, materially reducing AD-9's residual bug risk? | LOW-LEVEL FOUNDATIONS; seL4 precedent `[E]` | design Ring 0 + Ring 1 for smallness and auditability; don't commit to formal methods |
| AU-6 | Where exactly is the routine/reversible/pre-authorised boundary for the HOTL class? | requirements OQ-12; real use | Policy Store holds it, conservative default (err to HITL); not fixed here |
| AU-7 | Does the model's "worries / affect" gap (OQ-M5) force a structural change (a distinct unit type)? | model OQ-M5; user input | none yet — if it becomes a primitive it is another Chronicle unit type, not a new ring |

## 28. Biggest architectural risks

1. **The meta-invariant is necessary, not sufficient (AD-2, AD-9).** A capable
   reasoning ring could socially engineer the user, or exploit a Ring-1 bug.
   *Mitigation:* small auditable Ring 0/1, T6+ human-only and rare, anti-
   manipulation safeguards, user-as-backstop; Revision 1 adds at-rest signing of
   Ring 0 (RC-1), a specified verifier trust model with ≥2 independent verifiers
   for high-risk actions (RC-3), and aggregate-effect governance so a high-risk
   action cannot be laundered through low-risk steps (RC-2). *Reduced, still
   documented, not solved.*
2. **Projection / current-state-cache consistency window (AD-3/AD-4).** Reads can
   lag writes; a "what needs attention" view could momentarily miss a
   just-appended item.
   *Mitigation:* the append is always immediate and durable; views catch up
   fast; anything consequential reads current/authoritative state through the
   Chronicle contract, not a lagging cache (M2). A silent-divergence audit of the
   cache is a future implementation concern (O6), not an architecture requirement.
3. **Metareasoning controller complexity (AD-5).** A dynamic controller that
   is hard to make predictable/explainable would violate MEL-REQ-253.
   *Mitigation:* the controller's inputs (grounding factors) and choices are
   logged; the algorithm is deferred to CORE ENGINE where it gets its own design
   + evaluation; a conservative default strategy under meta-uncertainty.
4. **Lightweightness vs the ring overhead (AP-12).** Five rings + sandboxing +
   projections + audit could accrete resident cost.
   *Mitigation:* only Ring 0 + Ring 1 substrate are resident; everything else
   dormant; RUNTIME_MODEL §5 budgets background work.
5. **Local reasoner ceiling (AC, OQ-11).** If local reasoning is too weak,
   MELFINA declines a lot and feels inert.
   *Mitigation:* structural — decline honestly rather than reach out; the ceiling
   rises over time; deferred to AI/ASSISTANT LAYER, not forced here.
6. **Capability sprawl vs one-maintainer comprehension (VC9, `requirements` OQ-18).** Self-
   generated capabilities could erode "hold the whole system in your head".
   *Mitigation:* compose-before-code, retire-the-obsolete, no-duplication, a
   complexity budget in LOW-LEVEL FOUNDATIONS.

## 29. What must NOT be implemented yet

- **Any `src/` code.** Nothing.
- **A storage engine, on-disk format, or schema.** (STORAGE phase.)
- **A language choice or project skeleton.** (LOW-LEVEL FOUNDATIONS.)
- **The capability isolation mechanism** (process/VM/WASM/other). (LOW-LEVEL
  FOUNDATIONS.)
- **The Ring 0 at-rest signing scheme** (algorithm, key storage, chain format).
  Only the *invariant* is fixed (RC-1). (LOW-LEVEL FOUNDATIONS.)
- **The terminal/GUI action parser** (which parser, which structured grammar).
  Only the *invariant* is fixed (RC-4). (LOW-LEVEL FOUNDATIONS.)
- **The relevance / metareasoning algorithm.** (CORE ENGINE.)
- **Any local model integration or model selection.** (AI/ASSISTANT LAYER.)
- **Ring 4 (network) — in any form.**
- **An interaction UI.** (INTERFACE.)
- **Tiers 6–9 self-modification tooling.**

The architecture is a set of **boundaries, flows, and invariants**. The next
phases fill them with mechanism.

## 30. Recommended next phase

**LOW-LEVEL FOUNDATIONS** — establish, in this order:
1. the **capability grant representation + the Reference Monitor contract**
   (unforgeable, non-bypassable, monotonic, **revocable mid-execution** — RC-5),
   *including the structured representation of terminal/GUI actions* so a command
   grant is evaluated against parsed argv / a structured invocation and never a
   raw shell string, with shell interposition defined as a distinct higher-risk
   capability. This is **CONTAINMENT-CRITICAL (RC-4 / AU-2)** and comes first;
2. the Chronicle's logical format (open, documented, append-only, bitemporal) and
   the append/query contract — the thing everything else stands on;
3. the isolation primitive for Ring 3 (the mechanism, chosen against AP-7/AP-8;
   zero network capability by default — L4);
4. the Ring 0 governance file format + version-chain verification **+ the at-rest
   integrity scheme** (a signature chain over a human-held key MELFINA never
   possesses; invalid signature or broken version chain ⇒ refuse to run — RC-1);
5. the verifier trust contract (deterministic-where-practical, minimal-authority,
   no-ambient-authority, independently testable against fixtures; the fallback
   boundary where ≥2-independent-verifier redundancy is impractical — RC-3);
6. only then, a language and project skeleton chosen to serve 1–5 and AP-12.

**STORAGE** and **CORE ENGINE** follow. **Do not** start `src/` until LOW-LEVEL
FOUNDATIONS has a reviewed, accepted design and the user authorises it.

---

*Companions: `ARCHITECTURAL_PRINCIPLES.md`, `ARCHITECTURAL_ALTERNATIVES.md`,
`RUNTIME_MODEL.md`, `DATA_AND_STATE_MODEL.md`, `AUTHORITY_AND_SECURITY_MODEL.md`,
`CAPABILITY_MODEL.md`, `FAILURE_AND_RECOVERY.md`, `TRACEABILITY.md`.*
