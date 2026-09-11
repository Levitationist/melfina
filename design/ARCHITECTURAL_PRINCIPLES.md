# MELFINA — ARCHITECTURAL PRINCIPLES

**Phase:** SYSTEM DESIGN / ARCHITECTURE — MISSION 001.
**Status:** first pass, pending review. **Revision 1 applied** (targeted
adversarial-review corrections: AP-6/AP-9/AP-12 expanded for RC-1/RC-6/RC-7; §2
derived-view classification and §3 deferred-technology table updated for
M1/M2/RC-1/RC-3/RC-4/L4 — see `ARCHITECTURE_ADVERSARIAL_REVIEW.md` "Revision 1 —
resolution status"). **Date:** 2026-09-10.
**Baseline:** commit `f3093fc` (research + requirements + human/central model).
**Companions:** `SYSTEM_ARCHITECTURE.md`, `ARCHITECTURAL_ALTERNATIVES.md`,
`RUNTIME_MODEL.md`, `DATA_AND_STATE_MODEL.md`, `AUTHORITY_AND_SECURITY_MODEL.md`,
`CAPABILITY_MODEL.md`, `FAILURE_AND_RECOVERY.md`, `TRACEABILITY.md`,
`ARCHITECTURE_ADVERSARIAL_REVIEW.md`, `README.md`.

---

## 0. What this document does

The architecture is **derived from invariants and flows**, not assembled from a
module wishlist. This document states:

1. the **invariants** the architecture must never violate (from requirements +
   model), and the **principle** each invariant produces;
2. the **classification vocabulary** — mechanism / policy / data / capability /
   governance / derived view — used throughout the design;
3. what is **deliberately deferred** (technology).

Nothing here chooses a language, storage product, framework, model, or engine.
Where a *pattern* is named (event sourcing, reference monitor, capability tokens,
supervision trees, blackboard control) it is a **structural pattern**, evaluated
in `ARCHITECTURAL_ALTERNATIVES.md`, not a product decision.

**Evidence tiers:** `[E]` established · `[G]` guidance · `[DI]` design inference ·
`[H]` hypothesis · `[U]` unknown. Architecture work is mostly `[DI]`; grounding
patterns are `[E]`.

---

## 1. The invariants and the principles they force

### INV-1 — Local-only core
*The core must function with networking completely disabled, indefinitely: no
cloud, telemetry, remote API, mandatory account, silent external communication,
online-sync, or cloud reasoning dependency.* (MEL-REQ-014, 164–166; local-only
addendum.)

→ **AP-1 — The core has no network surface.** No core subsystem links a socket to
a non-local address, holds a "sync later" queue for core data, or degrades toward
a remote. Any future network capability is a **separate, isolated, off-by-default
component** that is never a dependency of any core function and by construction
cannot hold *private data + untrusted content + an outbound channel*
simultaneously (the "lethal trifecta", `[E]`; MEL-REQ-126, 167).

### INV-2 — The user is the authority; the autonomy triad is never collapsed
*Cognitive autonomy (reason), decision autonomy (select), and execution authority
(perform) are distinct and separately granted. There is no single "agent"
permission.* (MEL-REQ-004, 016–019.)

→ **AP-2 — Reasoning cannot act.** The subsystems that think and decide produce
**proposals and claims**, never effects. Effects occur only when a proposal
passes an explicit authorisation gate. Holding the ability to reason grants
nothing about deciding; reaching a decision grants nothing about executing.

### INV-3 — Predictable, deterministic core; nothing silent
*The core does not silently rearrange, hide, delete, or act. Consequential changes
are previewable and reversible. Given the same inputs and state, core operations
produce the same result.* (MEL-REQ-010; model P-3, P-12; IU evidence `[E]`.)

→ **AP-3 — Determinism lives in the core; non-determinism lives at the edge.**
The substrate (append, project, index, query, route, log) is deterministic.
Probabilistic reasoning is confined to an optional ring and its outputs are
labelled and gated. Every automatic change is either shown or in the audit log.

### INV-4 — Everything MELFINA knows is a Claim, with provenance and epistemic status
*No unqualified "fact". Status ∈ {observed, reported, inferred, hypothesised,
open, stipulated, decided, recalled}; full provenance; confidence; bitemporal
coordinates.* (Model §4.3, §8; MEL-REQ-093, 096, 131; `[E]` epistemic logic, PROV.)

→ **AP-4 — The knowledge substrate is claim-shaped, not fact-shaped.** Storage
carries `holder`, `status`, `provenance`, `confidence`, `valid-time`,
`transaction-time`, `supersedes` on every unit. There is no schema path for a
bare fact.

### INV-5 — Append-only history; nothing overwritten by normal operation
*Transaction-time is immutable and monotonic. "Change the past" = append a new
claim with an earlier valid-time. Deletion is a separate explicit user act.*
(Model P-5, §11; bitemporal modelling `[E]`; MEL-REQ-160–162, 170.)

→ **AP-5 — The Chronicle is the single append-only source of truth; everything
else is a rebuildable projection.** Current-state, context, "what needs
attention", the calendar view, capability reliability — all **derived**, all
**caches**, all reconstructible from the Chronicle. (Event-sourcing pattern,
`[E]`; see `DATA_AND_STATE_MODEL.md` and `ARCHITECTURAL_ALTERNATIVES.md` AA-2.)

### INV-6 — The self-modification meta-invariant
*MELFINA cannot redefine — silently, autonomously, or by persuasion — the rules
governing whether/how it may modify itself. Those rules are enforced outside its
reasoning, are monotonic (authority only narrows without an explicit human act),
versioned, and audited. Necessary, not proven sufficient.* (MEL-REQ-235, 239;
monotonic confinement `[E]`.)

→ **AP-6 — Governance is a ring the rest of the system can read but never write.**
The permission rules, the self-modification tier policy, the autonomous-action
budgets, and the emergency-stop authority live in an innermost governance ring
(**Ring 0**). No capability at any ring has the effect "widen my authority" or
"edit Ring 0" — that operation is **absent from the capability namespace**. Ring 0
changes happen only through a path that is not a MELFINA-invokable operation
(a human editing versioned governance files). The reference-monitor properties
apply (`[E]`, Anderson): the gate is **non-bypassable, always invoked,
evaluable**.

The "no write path" applies at rest as well as at runtime (RC-1, adversarial
review H1):

- **Integrity-protected at rest.** The Ring 0 governance file set carries a
  cryptographic integrity chain whose trust root is a **human-held signing key
  that the running MELFINA never possesses**. MELFINA verifies the chain and its
  signature at every startup; an invalid signature or a broken chain ⇒ **refuse
  to run and report** — never proceed on suspect governance, never self-repair.
- **Excluded from every File Access grant.** The Ring 0 governance path is a
  **hard, non-overridable exclusion** in the File Access capability itself (not a
  Policy Store setting, not a grant the user could accidentally issue). The
  Reference Monitor rejects any grant whose resolved path (after symlink and
  `..` resolution) intersects the Ring 0 region. **MELFINA cannot grant itself,
  or be granted, write access to Ring 0 files by any path.**
- *(Implementation of the cryptographic mechanism, the key custody, and the
  storage medium is deferred to LOW-LEVEL FOUNDATIONS. The invariant is: at-rest
  tampering of Ring 0 is prevented from taking effect and is detected as
  blocking.)*

Residual limits are documented in `AUTHORITY_AND_SECURITY_MODEL.md` §9 and remain
after RC-1 (social engineering of the user, unintended Ring-1 bugs,
test-passing-but-badly-generalising capabilities). The meta-invariant is
**necessary, not sufficient**.

### INV-7 — Least authority; capabilities are unforgeable, scoped grants
*Every component, capability, automation, and the AI holds the minimum authority
for its function. No ambient authority. Authority is an explicit, revocable,
scoped grant held only by holding a reference.* (MEL-REQ-124, 179–180;
object-capability model `[E]`.)

→ **AP-7 — Nothing has ambient authority.** A component starts able to do
*nothing* and receives narrow, typed capability grants from Ring 0/Ring 1. A
capability cannot hold more authority than its grantor held, nor grant itself
more later (monotonic, `[E]`); capability creation/modification passes an
explicit Reference Monitor check that the creator's authority is sufficient
(M8). Grants are **live, revocable handles** — the Reference Monitor can
invalidate one mid-execution (RC-5). Terminal/GUI authority is granted over
**structured actions** (parsed argv / a structured invocation), never a raw
shell string; shell interposition is a distinct, higher-risk grant (RC-4).
(WASI component-model pattern, `[E]`.)

→ **AP-7a — Authority is judged in aggregate, and composition never launders
it.** A capability's risk class is set by a deterministic Ring-1 classifier from
the **union** of its declared authority, its declared scope, and its **aggregate
effect** over the enclosing activity chain's window; a composed workflow is
classified by the union across all components. Routine actions carry a per-chain
aggregate budget; the action that would cross it re-gates as consequential.
"Practically irreversible in aggregate" counts as irreversible. Ring 2 can never
lower a class. (RC-2; MEL-REQ-227; preserves the existing risk-classification
invariant, does not replace it.)

### INV-8 — Assume adversarial content will sometimes win; contain the blast radius
*Untrusted content processed by any reasoning component may carry adversarial
instructions. Limit what a compromised component can do.* (MEL-REQ-185, 186;
prompt-injection is unsolved, `[E]`.)

→ **AP-8 — Structural containment over behavioural trust.** Important invariants
are enforced by *structure* (a component physically lacks the capability) rather
than by *asking a model to obey* (mission §8). A reasoning component that ingests
untrusted content runs sandboxed, without private-data + outbound authority, with
bounded resources, fully audited.

### INV-9 — Fully usable with AI disabled
*Core functions — capture, memory, retrieval, time/prospective-memory support,
user-made plans, deterministic automations, search — work with all AI/reasoning
components disabled.* (MEL-REQ-154; deskilling evidence `[E]`.)

→ **AP-9 — AI is an optional ring behind a narrow interface.** The reasoning ring
depends on the core; the core does not depend on the reasoning ring. Disabling
the reasoning ring removes proposals, teaching, and dynamic strategy — not
capture, memory, retrieval, time support, or user-driven action.

**The reasoner's locality is structural, not a preference** (RC-6, adversarial
review H6). Ring 1 and Ring 2 hold **no network capability**. The Reasoner
Interface, resident in the local core, **cannot open a socket**; it can bind only
to a local process, a local library, or a local file-backed model. A remote
reasoner endpoint is **not a configuration option of the core** — it would have
to be a Ring 4 component (out of scope, off by default). Where the architecture
establishes this structurally it says **"cannot"**, not "does not".

### INV-10 — Restraint at the hazard surfaces; quiet by default; no scorekeeping
*Completion, confirmation, history, metrics, re-checking, notifications are
hazards. No streaks/points/aggregate-completion by default. Nothing interrupts
unbidden except a user-defined minimal set.* (MEL-REQ-010–013, 041–047; research
§5, §11, `[E]`.)

→ **AP-10 — The architecture privileges no hazard surface.** There is no
"dashboard subsystem", no "metrics engine", no "streak store". Progress and
history are *projections the user may opt into*, computed on demand, never a
first-class stored aggregate. Notifications are a single, user-governed,
predictably-scheduled outbound path, not a cross-cutting concern every subsystem
can trigger.

### INV-11 — Graceful degradation; failure contained, not cascading
*Stays coherent during low-capacity periods, lapses, partial failures, and AI
unavailability. Re-entry after any gap is frictionless. Failure of one capability
does not cascade to the core.* (MEL-REQ-015, 172, 173; personal-informatics
abandonment `[E]`.)

→ **AP-11 — Supervision-tree failure isolation; "let it crash".** Subsystems and
capability executions are supervised units with restart strategies; a crash is
contained to its subtree, logged, and retried or abandoned without touching the
Chronicle. (Erlang/OTP pattern, `[E]`.) Recovery = replay projections from the
last checkpoint. There is no UI or code path that reacts to a gap with pressure
or guilt (MEL-REQ-047; AP-10).

### INV-12 — Lightweight; low idle footprint; on-demand loading
*Minimal CPU/RAM/power when idle; minimal, justified background processes; fast
startup; capabilities and models load on demand; adding a capability does not
raise always-on cost; minimal essential dependencies; one maintainer can hold the
whole system in their head.* (MEL-REQ-192–199; VC1.)

→ **AP-12 — Small always-on core; everything else dormant until needed.** Ring 0
and the Ring 1 substrate are the only always-resident parts and are deliberately
small. Reasoning, capabilities, and any models are activated on demand and
released. The capability *surface* can be large while the *resident footprint*
stays flat.

**Responsibilities are not components** (RC-7, adversarial review M7). The rings
and the ~25 named subsystems throughout this design are a **responsibility
decomposition** — they do **not** imply separate processes, services, binaries,
or IPC boundaries. A conforming implementation MAY group cohesive
responsibilities into a small number of components; several naturally are one
component each — "the store" (Chronicle + Query + Entity Registry + the
current-state cache), "the gate" (Pipeline State Machine + the Reference Monitor
call path), "the edge" (Audit + Notification Gateway). The distinction between
**boundaries that must be real at runtime / must be security boundaries** and
**boundaries that are conceptual organisation** is stated explicitly in
`SYSTEM_ARCHITECTURE.md` §2 and §9-of-the-adversarial-review. Only the following
must be real, enforced boundaries: **Ring 0 ↔ everything (no write inward);
Ring 1 ↔ the sandboxed outer region (Ring 2/Ring 3 get no Chronicle write, no
capability invocation, no network, no Ring 0); and the reasoning↔effect boundary
(Ring 2 emits only proposals, never invokes a capability).** Everything else may
be co-located.

### INV-13 — Dynamic, not arbitrary
*MELFINA determines per situation what context, reasoning depth, strategy, and
capability fit, and whether to act/wait/ask/do-nothing — grounded in goals,
constraints, permissions, evidence, learned context, user boundaries, uncertainty,
consequences, verification. Same situation + grounding ⇒ consistent, explainable
choice.* (MEL-REQ-204–253; VC8–VC12; model §7, §13.6.)

→ **AP-13 — A metareasoning controller, not an agent loop.** Dynamic behaviour is
a *control* problem over a transient working context (the blackboard pattern,
`[E]`, is **one candidate** coordination mechanism, to be chosen in CORE ENGINE —
not an architectural requirement, O3): the controller selects which reasoning
contributors to run, how deep, with how much context, and whether to emit a
proposal at all. It is bounded by Ring 0
(budgets, permissions) and by the grounding factors, and its choices are logged
and explainable (MEL-REQ-208, 253). It **never executes** — it emits proposals
into the pipeline.

### INV-14 — Traceability
*Every major architectural decision traces to a requirement, a model concept, a
research finding, or an explicitly documented architectural inference.*
(Mission "TRACEABILITY".)

→ **AP-14 — No unsupported invention.** `TRACEABILITY.md` carries the matrix.
Anything marked `[DI]` states the inference explicitly.

---

## 2. Classification vocabulary

Every element of the architecture is exactly one of:

| Class | Definition | Where it lives | Examples |
|---|---|---|---|
| **Governance** | Rules about *what may change what*, and the authority to stop things. Immutable without an explicit human act. Outside the reasoning process. | Ring 0 | permission rules; self-modification tier policy; autonomous-action budgets; emergency-stop authority; the meta-invariant |
| **Mechanism** | Deterministic machinery that does one job and embeds no policy beyond safety/fairness arbitration. | Ring 1 | Chronicle append; projection engine; query; index; the pipeline state machine; audit writer; capability registry (metadata) |
| **Policy** | User-set choices that mechanism consults. **Data**, editable by the user, versioned, never silently changed. | Ring 1 (stored as data), consulted by Ring 1/2 | the conflict axes as dimensions; autonomy & proactivity levels per area; interruption rules; retention periods; the consequential/routine boundary |
| **Data** | The user's life record. | Ring 1 (the Chronicle) | Events, Claims, Intentions; the Entity registry |
| **Capability** | Something MELFINA can *do* — isolated, permissioned, versioned, replaceable. | Ring 3 | skills; tools; workflows; terminal control; GUI control; file access; verifiers |
| **Derived view** | Computed from Data on demand; never authoritative; rebuildable; a cache. | Ring 1 | the **current-state cache** ("what holds now"); "what needs attention"; calendar view; learning trajectories; capability reliability; any progress representation. **Context is a derived view built *on demand* per situation by the Ring-2 Context Constructor — it is NOT a maintained view and never persisted** (RC/M1). |

**Rules:**
- Mechanism must not embed policy (Hydra principle, `[E]`). Where a decision could
  be the user's, it is **policy data**, not a coded default.
- Governance is never Data in the Chronicle (if it were, reasoning could propose
  editing it — INV-6).
- A derived view is never treated as a source of truth; losing every derived view
  loses nothing (AP-5). **There is one authoritative store (the Chronicle), one
  materialised current-state cache, and on-demand computed views — not a
  proliferation of independently-maintained projections** (O1, adopted as
  wording).
- **Where consequential correctness depends on "what holds now", the read goes
  through the Chronicle contract (a fresh, authoritative read), not a
  best-effort cache** (RC/M2). The current-state cache serves *pull* surfaces
  and non-consequential reads.
- A capability never becomes mechanism or governance by being useful.

---

## 3. Deliberately deferred (not decided here)

Per the mission and the project pipeline, **architecture defines structure, not
technology.** The following are **not chosen and not implied**, and are deferred
to the named later phases:

| Decision | Deferred to | Constrained by |
|---|---|---|
| Programming language(s) | LOW-LEVEL FOUNDATIONS | AP-3 (determinism), AP-12 (lightweight, minimal deps), AP-8 (must support real isolation) |
| Storage engine / product | STORAGE | AP-4 (claim-shaped), AP-5 (append-only + projections), INV-1 (local, no server dependency) |
| Local AI components / model sizes / whether any are neural | AI / ASSISTANT LAYER | AP-9 (narrow interface), AP-1 (local), AP-12 (on-demand); OQ-11 |
| Capability isolation *implementation* (process / VM / WASM / other) | LOW-LEVEL FOUNDATIONS | AP-7, AP-8 (no ambient authority; real containment) |
| The relevance / metareasoning *algorithm* | CORE ENGINE | AP-13; model OQ-M6 |
| Interaction medium(s) | INTERFACE | MEL-REQ-190 (multi-modal, user-choosable, CLI-friendly) |
| On-disk format / file layout / indexing | STORAGE | AP-4, AP-5, MEL-REQ-161 (open, documented, outlives the software) |
| The permission-gate *mechanism* — grant representation + how actions are parsed and checked | LOW-LEVEL FOUNDATIONS | **CONTAINMENT-CRITICAL** (RC-4). AP-6, AP-7 (unforgeable, non-bypassable, monotonic). Terminal/GUI grants MUST be checked against a **parsed, structured action** (argv / structured invocation), never a raw string; shell interposition is a distinct higher-risk grant. |
| The Ring 0 at-rest integrity mechanism (signing scheme, key custody, storage medium) | LOW-LEVEL FOUNDATIONS | RC-1. AP-6 (tampering prevented from taking effect, detected as blocking; trust root is a human-held key MELFINA never holds) |
| The isolation primitive's network posture | LOW-LEVEL FOUNDATIONS | AP-1, AP-8: the Ring 3 sandbox has **zero network capability by default** (L4); a socket cannot be opened from Ring 2 or Ring 3 |
| The two-independent-verifier mechanism for high-risk actions | LOW-LEVEL FOUNDATIONS + TESTING/EVALUATION | RC-3 (architectural independence: different code paths / different implementations, not a claim of statistical independence) |
| Aggregate-budget parameters (window length, magnitude metric, per-chain defaults) | LOW-LEVEL FOUNDATIONS / CORE ENGINE, as Ring-0 governance data | RC-2, AP-7a. Only the *invariant* (aggregate is governed, composition never launders) is fixed here |
| The mid-execution grant-revocation + compensation mechanism (how a running execution is stopped; per-capability rollback) | LOW-LEVEL FOUNDATIONS + CORE ENGINE + CAPABILITY | RC-5, AP-7. The *outcome vocabulary* (completed / partially completed / rolled back / failed / interrupted) is fixed; the mechanism is not |

If a later phase's technology choice would violate an invariant above, **the
choice is wrong, not the invariant** — unless the user explicitly revises the
requirement or model.
