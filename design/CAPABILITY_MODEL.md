# MELFINA — CAPABILITY MODEL (ARCHITECTURE)

**Phase:** SYSTEM DESIGN / ARCHITECTURE — MISSION 001. First pass, pending review.
**Revision 1 applied** — RC-2 (union risk class, composition never launders
risk), RC-3 (verifier trust), RC-4 (structured terminal actions), RC-5
(mid-execution revocation), RC-1 (File Access Ring-0 exclusion), M6, M8, L4. See
`ARCHITECTURE_ADVERSARIAL_REVIEW.md` "Revision 1 — resolution status".
Baseline `f3093fc`. Read `ARCHITECTURAL_PRINCIPLES.md`, `SYSTEM_ARCHITECTURE.md`,
`AUTHORITY_AND_SECURITY_MODEL.md` first.

Covers mission items **P** (capability/skill architecture), **Q** (workflow/
automation), **W** (capability creation/evolution). **No isolation technology
chosen** (process / VM / WASM / other — LOW-LEVEL FOUNDATIONS, constrained by
AP-7/AP-8).

This is the *architecture* companion to `requirements` PART IV-B. The requirements
say *what*; this says *where it lives and how the boundaries hold*.

---

## 1. What a capability is (architecturally)

A **capability** is three things bound together:

| Part | Where | Content |
|---|---|---|
| an **Entity** (kind `capability`) | Entity Registry | stable id, name |
| **Claims about it** | Chronicle | what it does; declared authority (exactly what it needs to read/change/run/communicate); version; risk class; reliability-by-task-type; lifecycle status (draft / testing / active / retired) |
| a **component** | component files (Ring 3 storage), versioned | the executable/config that does the work |

**Capabilities are Ring 3.** They start with **no ambient authority** and run
**sandboxed**. They receive narrow, typed, revocable grants from the Reference
Monitor **per authorised action**, for the duration of that action only.

Kinds of capability: skills (do a discrete thing), tools (wrap an external
program locally), workflows (a composition of other capabilities), terminal
control, GUI control, file access, verifiers (deterministic result checks).
The kind list is **open** — new kinds do not change the architecture.

## 2. The Capability Host (mechanism)

For each authorised action that uses a capability:

```
   1. load the component into a fresh sandbox with NO ambient authority and
      ZERO network capability (L4 — the sandbox cannot make a network call)
   2. inject exactly the typed capability grants the Reference Monitor issued
      for THIS action (nothing more). A terminal/GUI grant names a STRUCTURED
      action (parsed argv / structured invocation), never a raw shell string
      (RC-4); shell interposition is a distinct, higher-risk grant type.
   3. run with:
        · a hard wall-clock timeout
        · a loop detector (repeated identical operations / no progress)
        · a resource ceiling (memory, and any metered resource)
        · Ring-0 budget accounting (invocation count, steps) AND the aggregate
          budget for the enclosing activity chain (RC-2)
        · an external watchdog holding the grants as LIVE handles — it can
          invalidate them mid-execution (RC-5)
   4. capture the result
   5. tear down the sandbox (no persistent capability state between actions
      unless explicitly granted a scoped store)
   6. hand the result to VERIFY
```

A capability that tries to exceed its grant, escalate, loop, or run past its
timeout is **terminated** by the Host; an Event records the failure; contained —
the Chronicle and core are untouched.

**Mid-execution revocation (RC-5).** A grant is a live, revocable handle, not a
one-shot ticket. On a revocation trigger (user act, emergency stop, budget or
aggregate cut-off, Supervisor detecting escalation/loop, a conflicting
higher-priority action) the Reference Monitor invalidates the handle atomically;
the Host stops the execution where possible; reversible effects already made are
rolled back where possible; irreversible/partial effects are recorded as a
**partial failure**. The action always reaches a definite terminal outcome from
the vocabulary *completed / partially completed / rolled back / failed /
interrupted* — the system never claims a rollback it cannot guarantee. (Full path
in `AUTHORITY_AND_SECURITY_MODEL.md` §3.)

## 3. Selection and composition (P)

- **Contextual selection** (MEL-REQ-216): the Metareasoning Controller, given a
  situation, consults the Capability Registry (+ the capability-reliability
  projection) and picks which capabilities fit.
- **Composition**: a composition is itself a **workflow-Entity** — a `plan` of
  sub-capability invocations with `depends-on` / temporal relations. Composing
  costs no new code; it is data.
- **Composition never launders risk (RC-2).** A workflow's risk class is the
  **union** of every component's declared authority, every component's scope, and
  the **aggregate effect** of running them together. Composition can only raise
  the class. There is no way to take a high-risk action, split it into several
  individually-low-risk capability calls, and have the workflow gate as low-risk.
  "Practically irreversible in aggregate" (many reversible steps that together
  cannot be undone) is an input to the class.
- **"Compose before code"** (MEL-REQ-220): capability creation MUST attempt to
  solve a gap by composing/adapting existing capabilities before generating new
  code. Enforced as the first branch of the lifecycle (§4).
- **Sub-agents** (MEL-REQ-127): a capability MAY delegate to a specialised
  sub-capability, but it runs under the **same** permission model and the **same**
  pipeline — delegation is not a way to escape authorisation.

## 4. The capability lifecycle (W)

```
   DISCOVER GAP
     a Claim "no available capability (alone or composed) covers task-type T"
     (status = inferred). Whether this is a real gap or the model's inclination
     to build is `requirements` OQ-17 — mitigated because every step below is a gate regardless.
        │
        ▼
   SEARCH / COMPOSE  ── first branch ──
     can existing capabilities (alone or composed) do it?
        yes ─▶ build a workflow-Entity ─▶ TEST
        no  ─▶ DESIGN
        │
        ▼
   DESIGN
     specify the missing capability: its interface, its declared authority
     (exactly what it must read/change/run/communicate — least authority)
        │
        ▼
   CREATE
     if code: generate into a sandbox. Safety rests on sandbox + test +
     rollback + human gate, NOT on a proof of correctness (verified codegen is
     near-unsolved, `[E]`/`[U]`; MEL-REQ-223).
        │
        ▼
   TEST
     run against the FIXED verification suite (Ring 3 Verifiers — see the
     verifier trust model, AUTHORITY_AND_SECURITY_MODEL §4.1: deterministic where
     practical, minimal authority, no ambient authority, read-only unless
     narrowly justified, independently testable against fixtures, NOT a second
     agent; the reasoner does NOT self-assess "it works", `[E]`; MEL-REQ-221, 248)
        │
        ▼
   VERIFY
     · did the effect occur correctly on the test cases?
     · did it stay within its declared authority? (no escalation attempt)
     · for a high-risk capability: agreement from >= 2 INDEPENDENT verification
       mechanisms (architecturally independent — different code paths /
       implementations / observation methods, not statistical independence;
       RC-3). Disagreement => indeterminate => not verified. Where a second
       mechanism is genuinely impractical, the capability is a Policy-Store-listed
       single-verifier action with a written justification and raised governance.
        │
        ▼
   VERSION
     register in the Capability Registry: id, version, declared authority,
     provenance (generated-by which Event, attributed to which agents),
     RISK CLASS (set by the deterministic classifier from the UNION of declared
     authority + scope + aggregate effect — Ring 2 cannot lower it, MEL-REQ-227,
     RC-2). An explicit Reference Monitor check confirms the creator's authority
     was sufficient for the authority this capability declares (M8) — a capability
     cannot be born holding more than its creator held.
        │
        ▼
   AUTHORISE
     risk-class-scaled governance (AUTHORITY_AND_SECURITY_MODEL §5):
       · trivial (no computer / no data-model / no permissions / no external /
         no core) ⇒ a user confirmation
       · higher ⇒ explicit user authorisation
       · tiers 6–9 ⇒ not a MELFINA operation (a human path)
        │
        ▼
   USE
     Capability Host, sandboxed, per-action grants
        │
        ▼
   EVALUATE
     Self-Evaluation produces reliability-by-task-type Claims from recorded
     outcomes; periodic replay-test on the fixed suite detects drift/regression
     (MEL-REQ-226)
        │
        ▼
   RETIRE / REPLACE
     obsolete / redundant / harmful (errors, loops, escalation attempts,
     reinforcing a compulsion pattern) ⇒ a redaction Event ends the "active"
     Claim; the Entity + all versions + history stay; a replacement `supersedes`
     it (MEL-REQ-224)
```

### Diagram — the lifecycle governance boundary

```
   [ DISCOVER ]─[ COMPOSE/CREATE ]─[ TEST/VERIFY ]─[ VERSION ]─[ AUTHORISE ]─[ USE ]
        │              │                  │             │            │          │
        │   Ring 2 may drive up to here   │   Ring 1 deterministic   │   Ring 0 + user
        │   (draft, compose, generate)    │   classifier sets risk   │   gate here
        │                                 │   class; Ring 2 CANNOT    │
        │                                 │   change it              │
        ▼                                                            ▼
   ┌────────────────────────────────────────────────────────────────────────┐
   │  Risk class ↑  ⇒  governance ↑.  Risk class is NEVER lowered by MELFINA.│
   │  A capability that would touch Ring 0 or MELFINA's own code:            │
   │  there is NO such capability. That path is absent (tiers 6–9).          │
   └────────────────────────────────────────────────────────────────────────┘
```

## 5. Risk classification (deterministic, Ring 1)

Input: the capability's **declared authority** (what it says it needs), its
**declared scope** (which objects), and — for a composition or a repeated routine
action — its **aggregate effect** over the enclosing activity chain's window
(RC-2). The classifier maps the **union** of these to a risk class by what it
touches. For a workflow the union is taken across all components; composition
never lowers the class. "Practically irreversible in aggregate" is treated as
irreversible.

| Touches | Risk class (illustrative bands — exact policy is Ring-0 data) |
|---|---|
| only its own scoped inputs/outputs; no side effects | **trivial** |
| the user's data model (writes Claims/Events/Intentions) | **elevated** |
| the local computer (terminal / GUI / files) | **elevated → high** by reversibility |
| irreversible / destructive actions (individually **or in aggregate**) | **high** |
| permissions, or other capabilities' grants | **high** |
| a raw shell / shell interposition (vs a structured, parsed command) | **high** — a distinct grant type (RC-4) |
| external communication (Ring 4) | **high** |
| routine actions whose aggregate over the window crosses the chain budget | **escalated to consequential** for the crossing action (RC-2) |
| MELFINA's own code / subsystems / architecture / Ring 0 | **not a capability** (tiers 6–9) |

The classifier is **mechanism** (`ARCHITECTURAL_PRINCIPLES.md` §2); its rules are
**Ring-0 governance data**; the Reference Monitor reads the class **from the
Registry**, never from a reasoning claim. This is how "risk class never silently
lowered" (MEL-REQ-227) is structural.

## 6. Workflow / automation architecture (Q)

An **automation** is not a subsystem. It is:

- a `melfina`-owned `Intention`, `standing = recurring` **or** with a trigger
  condition, + a `plan` (sub-Intentions / Events with `depends-on` and temporal
  relations);
- the `Event`s it performs (its history);
- verification `Claim`s per run.

**Three permission levels** (MEL-REQ-111), each a distinct gate:

| Level | Who | Gate |
|---|---|---|
| **generation** (draft an automation) | Ring 2 may | none beyond normal reasoning budgets |
| **authorisation** (make it eligible to run) | **user only** | an explicit user act; the automation is inert until then |
| **execution** (a specific run) | the runtime, when due | the **full pipeline + Reference Monitor**, every run |

**Triggers** — the automation's definition states which, explicitly (users confuse
events and states, `[E]`):
- **scheduled** — a `valid-time` target;
- **event-conditioned** — a Chronicle Event matching a pattern;
- **state-conditioned** — a projection reaching a condition.

**Runaway protection**: Ring-0 Budget Authority caps every run (steps, time,
invocations) **and the aggregate effect of the automation's runs over its window
(RC-2)** — an automation whose individually-routine actions accumulate to a
consequential effect is escalated to a user gate, not silently continued; a loop
detector + the Supervisor halt oscillating / self-triggering automations
(MEL-REQ-115); the automation-trigger graph is checked for cycles of any length
(M6). **History / ownership / pause / disable / override**:
history is the Event stream; ownership is `generated-by`; pause/disable/override
are user acts the pipeline honours (MEL-REQ-114).

## 7. What is NOT a capability

To prevent capability-namespace creep from eroding the invariants:

- **Anything that would modify Ring 0**, MELFINA's code, subsystems, or
  architecture (tiers 6–9). Not a capability. A human path.
- **File access to the Ring 0 governance store — even read.** The File Access
  capability's grantable paths **hard-exclude** any path resolving inside the
  Ring 0 store (RC-1). Neither the user nor MELFINA can grant a capability access
  to the governance files; they are changed only by a human editing them directly
  with ordinary tools and re-signing.
- **Anything that would widen MELFINA's own authority or autonomy ceiling.**
  Absent from the namespace.
- **The Chronicle append, the Reference Monitor, the pipeline state machine, the
  projection engine.** These are Ring 1 *mechanism*, not capabilities — they do
  not run in a sandbox, are not created/retired, and have no risk class.
- **The metareasoning controller.** Ring 2 mechanism, not a capability.
- **Governance rules, the tier policy, budgets.** Ring 0 *governance data*.

## 8. Deferred to later phases

- The **isolation mechanism** (process / lightweight VM / WASM component / other)
  — LOW-LEVEL FOUNDATIONS, against AP-7/AP-8 acceptance criteria (no ambient
  authority; real containment; lightweight).
- The **grant representation** (token / handle / resource / policy record) and
  the **structured-action parser** for terminal/GUI grants (which parser, which
  grammar) — LOW-LEVEL FOUNDATIONS (AU-2, RC-4; **containment-critical**).
- The **aggregate-budget accounting** — window length, magnitude metric, per-chain
  defaults — Ring-0 governance data set in LOW-LEVEL FOUNDATIONS / CORE ENGINE
  (RC-2). Only the invariant is fixed here.
- The **fixed verification suite** format and harness — TESTING/EVALUATION +
  ARCHITECTURE follow-on.
- The **capability component format** and the **composition (workflow) DSL** —
  CORE ENGINE.
- The **gap-recognition heuristic** (real gap vs "the model wants to build",
  `requirements` OQ-17) — CORE ENGINE + real use.
