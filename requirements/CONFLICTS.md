# MELFINA — REQUIREMENTS-LEVEL CONFLICTS

Companion to `requirements/REQUIREMENTS_MASTER.md`. Requirements-level treatment of
the tensions surfaced by research and by the autonomous-personal-intelligence
vision.

**Rule (from the mission):** *do not resolve a tension by pretending one side is
always right.* Each tension is expressed as one or more of:
- a **configurable dimension** (a user-set setting with a safe default),
- a **constraint** (a hard bound that applies regardless of setting),
- a **safeguard** (a mechanism that catches a failure mode),
- an **open question** (deferred to a later phase or to real-world use).

No implementation is designed here. Evidence tiers as in the master:
**[E] [G] [DI] [H] [U]**.

The ten research-level tensions (**RC1–RC10**) are carried from
`research/CONFLICTS.md` and mapped to requirements. The autonomous-personal-
intelligence vision adds seven (**VC1–VC7**); the dynamic-self-directed correction
adds five more (**VC8–VC12**, Part B-2).

---

## Part A — Research-level tensions (RC1–RC10)

### RC1 — Structure vs. novelty
- **Configurable dimension:** a "how much structure vs open space" setting per
  area (day shape, projects, practice). Default: light structure, few anchors.
  *(MEL-REQ-175)*
- **Constraint:** whatever the setting, system *behaviour* stays predictable;
  novelty is expressed in the user's own content and in surface appearance the
  user can restyle, not in system behaviour changing. *(MEL-REQ-010, 191)* [E/DI]
- **Open:** does "anchored flexibility" (few fixed points + free space) actually
  work for this user? `[H]` → OQ-4.

### RC2 — Externalise memory vs. recording-becomes-compulsion
- **Constraint:** record once, treat as settled; no re-verification prompts; no
  "review your entries" ritual. *(MEL-REQ-021, 055, 062)* [E]
- **Constraint:** capture is near-frictionless and defers organisation.
  *(MEL-REQ-020, 066)* [E]
- **Safeguard:** retrieval never invents; stored vs inferred is labelled.
  *(MEL-REQ-093)* [E]
- **Configurable dimension:** history retention period (default modest, visible,
  adjustable). *(MEL-REQ-065)* [G]

### RC3 — Reminders vs. notification overload
- **Configurable dimension:** per-source, per-context interruption settings;
  delivery windows. Default: quiet, predictable windows / task boundaries.
  *(MEL-REQ-011, 041)* [E]
- **Constraint:** no notification-driven behaviour change; assume alert decay;
  don't loop identical alerts. *(MEL-REQ-042, 043)* [E]
- **Constraint:** user-declared "now" is preferred over inferred timing;
  correctness never depends on context inference. *(MEL-REQ-044)* [E]
- **Open:** what reminder content/timing keeps prospective-memory support
  effective for a reminder-fatigued / demand-avoidant user? `[U]` → OQ-3.

### RC4 — Simplify/reduce-decisions vs. loss of information & control
- **Configurable dimension:** disclosure level (how much is shown by default).
- **Constraint:** hiding is predictable, rule-based, user-reversible, and never
  driven by the system deciding what matters. *(MEL-REQ-051)* [E/G]
- **Constraint:** few things shown at once + a default "next" + frictionless "not
  now". *(MEL-REQ-030)* [E]

### RC5 — Measurement/feedback vs. compulsive monitoring & perfectionism
- **Constraint (default):** no streaks, points, scores, comparative or aggregate
  metrics; no guilt/shortfall UI. *(MEL-REQ-012, 045, 047, 049)* [E]
- **Configurable dimension (opt-in only):** a minimal, private, non-comparative,
  non-accumulating, partial-credit, disableable progress view — *if* the user
  turns it on. *(MEL-REQ-046)*
- **Constraint (music):** no error counts / mistake highlighting / comparative
  recordings by default — perfectionistic *concerns* drive MPA and overlap OCD.
  *(MEL-REQ-080)* [E]
- **Open:** is there *any* progress representation that helps this user without
  harm? Current answer: none demonstrated. `[D]` → OQ-6.

### RC6 — Automation vs. autonomy, predictability, agency
- **Constraint:** the autonomy triad — cognitive ≠ decision ≠ execution; the
  action pipeline THINK→…→VERIFY. *(MEL-REQ-017, 018)* [E]
- **Configurable dimension:** per-area autonomy/proactivity level; the
  routine/reversible/pre-authorised class boundary. Default: conservative, HITL for
  anything consequential. *(MEL-REQ-019, 178)* [E]
- **Constraint:** fully usable with AI disabled. *(MEL-REQ-154)* [E]
- **Safeguard:** autonomous-action budgets as hard cutoffs; loop detection;
  emergency stop. *(MEL-REQ-115, 183, 145)* [E]
- **Open:** the default consequential/routine boundary. `[E/U]` → OQ-12.

### RC7 — Completion/closure vs. "not-just-right" dissonance
- **Constraint:** no forced binary completion; partial progress counts; "dropped"
  ≠ "failed". *(MEL-REQ-027)* [E]
- **Constraint:** no "perfect closure" spectacle (all-green, zero-inbox
  celebration). *(MEL-REQ-013)* [E]
- **Design direction:** closure via *hand-off to MELFINA* (a credible plan / a
  scheduled resurfacing), distinct from "finished" — Masicampo + Leroy: a credible
  path, not actual completion, relieves the load. *(MEL-REQ-028)* [E]

### RC8 — Prompting/accountability vs. pressure → avoidance & shame
- **Constraint:** offer, don't instruct; no "overdue"/countdown/disappointment; no
  guilt UI; ask-don't-assume when initiating. *(MEL-REQ-005, 029, 047, 106)* [E/G]
- **Configurable dimension:** accountability features exist only if the user opts
  in and shapes them. *(MEL-REQ-050)* [E]

### RC9 — Rich context capture vs. total-capture noise & capture friction
- **Constraint:** no total capture; design for retrieval/reconstruction, not
  completeness; archive may be lossy. *(MEL-REQ-025, 092)* [E]
- **Constraint:** capture is one action, no forced fields. *(MEL-REQ-020)* [E]
- **Configurable dimension:** optional resumption-cue capture on leaving a task.
  *(MEL-REQ-024)* [E]

### RC10 — Predictable interface vs. stale/unmotivating interface
- **Constraint:** system behaviour and layout stay stable; system-initiated visual
  change is opt-in. *(MEL-REQ-010, 191)* [E]
- **Configurable dimension:** the user restyles surface appearance whenever they
  want novelty. *(MEL-REQ-191)* [E/DI]

---

## Part B — Vision-level tensions (VC1–VC7)

### VC1 — Extreme capability vs. extreme lightweightness
*"As powerful as reasonably possible while as lightweight as reasonably possible."*
- **Constraint:** minimal always-on core; low idle footprint; fast startup;
  minimal background processes. *(MEL-REQ-192, 196, 197)* [B]
- **Design direction (hypothesis):** a large capability *surface* delivered as
  **on-demand, dormant-when-unused capabilities** so that adding capability does
  not raise always-on cost; smallest-adequate reasoning components. *(MEL-REQ-128,
  198, 199)* `[H]` → OQ-5.
- **Constraint:** capability composition preferred over hard-coded features; no
  duplicate functionality; no unnecessary frameworks/abstractions. *(MEL-REQ-193,
  194, 195)* [B]
- **Open:** does capability composition genuinely reduce *total* system complexity,
  or relocate it? `[H]` → OQ-5.
- **Not resolved by:** picking a "small" implementation technology — that is an
  architecture decision and is explicitly out of scope. *(MEL-REQ-200)*

### VC2 — Autonomous intelligence vs. user agency
*The system should think and decide independently where valuable — without that
becoming loss of user control or uncontrolled action.*
- **Constraint:** the autonomy triad (cognitive / decision / execution), never
  collapsed. Cognitive autonomy broad; execution authority minimal and explicit.
  *(MEL-REQ-016, 017)* [E]
- **Constraint:** the action pipeline; consequential actions are HITL; routine/
  reversible/pre-authorised actions MAY be HOTL. *(MEL-REQ-018, 019)* [E]
- **Constraint:** reach a decision internally ≠ authority to execute it.
  *(MEL-REQ-102)* [E]
- **Safeguard:** the system may conclude "no action warranted" and record that;
  over-helpfulness is a known agent failure mode. *(MEL-REQ-103)* [E]
- **Constraint:** MELFINA does not author its own objectives or success metrics.
  *(MEL-REQ-149)* [E]
- **Configurable dimension:** per-area autonomy and proactivity levels.
  *(MEL-REQ-178)* [E]
- **Open:** where is the *useful-autonomy sweet spot* for this user — how much
  independent thinking/deciding adds value before it erodes agency or trust? Only
  real use answers this. `[U]` → OQ-7, OQ-12.

### VC3 — Local-only core vs. capability ceiling
*A fully local core caps reasoning power at what local models/components can do.*
- **Constraint:** the core is local-only — no remote dependency, no cloud
  component it defers to. *(MEL-REQ-014, 164, 166)* [B/E]
- **Constraint:** where a task exceeds local capability, MELFINA produces a
  clearly-marked lower-capability local result or declines — it MUST NOT silently
  require the network. *(MEL-REQ-155)* [E]
- **Configurable dimension (MAY, future):** a separately-installed, isolated,
  off-by-default network capability (reference lookup, backup to the user's own
  remote storage, an optional larger remote model) that can never form the lethal
  trifecta with core data and is never a core dependency. *(MEL-REQ-167)* [E]
- **Open:** where does the local reasoning ceiling actually sit for MELFINA's
  needs, and is any remote add-on ever worth its risk? `[U]` → OQ-11.

### VC4 — Adaptation/learning vs. stability & predictability
*The system should adapt with the user, but not drift, not become unpredictable,
not entrench temporary states.*
- **Constraint:** non-parametric-first — learned behaviour is explicit, inspectable,
  editable, forgettable items; no silent model drift. *(MEL-REQ-118)* [E]
- **Constraint:** learned patterns surface as suggestions the user confirms.
  *(MEL-REQ-119)* [E]
- **Constraint:** distinguish stable preferences from temporary states;
  observations from conclusions. *(MEL-REQ-121)* [E/DI]
- **Constraint:** no personality drift; core behavioural rules are versioned and
  change only by explicit user act; adaptation cannot weaken the safeguards.
  *(MEL-REQ-122, 186)* [E]
- **Constraint:** the five learning types (configuration / preference-learning /
  skill-acquisition / strategy-tuning / core-modification) are separately governed;
  core-modification is not autonomous. *(MEL-REQ-149, 150)* [E]
- **Open:** the line between "helpful adaptation" and "drift" is not sharply
  definable in advance. `[U]` → OQ-14.

### VC5 — Proactive assistance vs. interruption / surveillance
- **Constraint:** proactivity defaults low, tunable per category; ask-don't-assume;
  know when *not* to interrupt / act; "not now" without penalty. *(MEL-REQ-105–107)*
  [E]
- **Constraint:** proactive assistance must not require pervasive monitoring and
  must not manifest as frequent unsolicited notifications. *(MEL-REQ-109)* [E]
- **Configurable dimension:** MELFINA learns which proactive acts the user welcomes
  vs ignores — via the §25 adaptation safeguards. *(MEL-REQ-108)* [E]
- **Open:** the right default proactivity level per area. `[U]` → OQ-7.

### VC6 — Rich functionality vs. cognitive complexity
*A very capable system risks becoming a feature-heavy load — the exact
anti-requirement (MEL-AR-11).*
- **Constraint:** few things shown at once; progressive disclosure with a stable
  obvious control; no default dashboard; plain literal language. *(MEL-REQ-030, 051,
  052)* [E/G]
- **Design direction:** capability lives behind on-demand invocation, not on
  screen; the surface the user sees stays small even as the capability set grows.
  *(MEL-REQ-128, 199)* `[H]`
- **Open:** can a system this capable actually *feel* simple to this user? `[U]` →
  OQ-5.

### VC7 — Self-improvement/"evolution" vs. safety, reliability, control
- **Constraint:** improvement is bounded, versioned, inspectable, reversible,
  testable, auditable, permission-gated. *(MEL-REQ-148)* [E]
- **Constraint:** no self-authored objectives/metrics/core-rules; no autonomous
  core-code modification; no self-replication. *(MEL-REQ-149)* [E]
- **Constraint:** verification hierarchy respected — formal/external checks over
  intrinsic self-assessment; watch for self-confirming loops and model collapse.
  *(MEL-REQ-148, 131, 136)* [E]
- **Safeguard:** adaptation/improvement cannot alter the compulsion/manipulation/
  runaway safeguards. *(MEL-REQ-186)* [E]
- **Open:** how to *measure* whether an improvement is actually an improvement for
  this user, before adopting it. `[U]` → OQ-14, OQ-15.

---

## Part B-2 — Dynamic-self-directed tensions (VC8–VC12)

*Added by MISSION 001's dynamic-self-directed correction. Full requirements:
`REQUIREMENTS_MASTER.md` PART IV-B (`MEL-REQ-204…253`).*

### VC8 — Dynamic strategy selection vs. predictability & trust
*MELFINA choosing its own approach (depth, speed, context, decomposition,
verification effort, autonomy) per situation vs. the user needing consistent,
trustworthy, non-surprising behaviour.*
- **Constraint:** dynamic ≠ arbitrary — every choice is bounded by the grounding
  factors (goals · constraints · permissions · evidence · learned context ·
  invariants · user boundaries · verification · consequences · uncertainty) and
  must be justifiable against them. *(MEL-REQ-207, 252)* [DI]
- **Constraint:** given the same situation + grounding, strategic choices are
  consistent and explainable; surprising strategic behaviour is a defect.
  *(MEL-REQ-253)* [E]
- **Constraint:** every strategy choice is inspectable / explainable on request.
  *(MEL-REQ-208)* [E]
- **Configurable dimension:** the user can constrain how much strategic latitude
  MELFINA takes, per area (a bound on dynamism, not a fixed mode).
- **Open:** how much strategic variability reads as "unpredictable" for this
  user. `[U]` → OQ-16.

### VC9 — Capability creation vs. lightweightness & one-maintainer comprehension
*Self-generated capabilities could bloat the system and break "one person can hold
the whole thing in their head" (`MEL-REQ-192`), or "no unnecessary dependencies".*
- **Constraint:** compose/adapt existing capabilities before generating new code.
  *(MEL-REQ-220)* [DI]
- **Constraint:** detect and retire obsolete/redundant/harmful capabilities; keep
  an archive + restore path. *(MEL-REQ-224)* [E]
- **Constraint:** no duplicate functionality. *(MEL-REQ-195)* [B]
- **Constraint:** capabilities load on demand, dormant when unused — creation adds
  to the *surface*, not the always-on cost. *(MEL-REQ-128, 199, 243)* `[H]`
- **Open:** whether a system that grows its own capabilities can stay within
  one-maintainer comprehension. `[H]` → OQ-18.

### VC10 — Taking self-modification seriously vs. safety / invariant preservation
*The mission asks that self-modification (up to code and architecture) be
investigated as a serious long-term capability — while it is not automatically
safe and the safety invariants may not be provably sufficient.*
- **Constraint:** a *graded* model — nine tiers, escalating authorisation +
  containment; tiers 6–9 (code / subsystem / architecture / self-replacement) are
  **never autonomous**. *(MEL-REQ-233, 236)* [E for the grading]
- **Constraint:** every tier carries the full control set (authorise, test,
  sandbox, version, rollback, verify, contain failure, preserve invariants,
  prevent escalation, recover). *(MEL-REQ-234)* [E]
- **Constraint:** self-modification only from a safe, quiescent state.
  *(MEL-REQ-237)* [E]
- **Constraint — THE META-INVARIANT:** MELFINA cannot redefine the rules
  governing its own self-modification; those rules are external to its reasoning,
  monotonic (authority only narrows without an explicit human act), versioned,
  audited. *(MEL-REQ-235)* [E]
- **Acknowledged limit:** the meta-invariant is *necessary but not proven
  sufficient* against a highly capable self-modifier — so high tiers are kept
  rare + human-driven + heavily audited, not trusted to invariants alone.
  *(MEL-REQ-239)* `[U]`
- **Distinction:** introspection (reading self) is free and useful; intercession
  (altering self) is the gated part. *(MEL-REQ-238)* [E]
- **Open:** does the meta-invariant actually hold in practice? `[U]` → OQ-19. The
  right authorisation granularity for the nine tiers → OQ-20.

### VC11 — Dynamic context selection vs. the predictable-deterministic core
*Active, situational choices about what context to bring vs. `MEL-REQ-010`
(predictable, deterministic core).*
- **Constraint:** context selection is fully auditable — what was used, what was
  deliberately excluded, why — *especially* because it partly relies on the
  model's weak self-knowledge. *(MEL-REQ-213)* [E]
- **Constraint:** acquisition stays local-only; discard ≠ delete. *(MEL-REQ-214,
  215)* [E]
- **Design direction:** the *data operations* (retrieval by exact criteria,
  scheduling, storage) stay deterministic; the *situational judgement* about what
  to bring is the dynamic part, and it is logged. *(MEL-REQ-174, 213)* [E/DI]

### VC12 — "Best is situational" vs. stable, relied-upon behaviour
*`MEL-REQ-251` (do not optimise one fixed notion of best) vs. the user's need to
rely on MELFINA behaving a certain way.*
- **Constraint:** every trade-off decision is grounded (`MEL-REQ-252`) and
  consistent + explainable (`MEL-REQ-253`).
- **Constraint:** the safety / local-only / user-authority / hazard-surface MUSTs
  are **not** things MELFINA trades off — they are invariants, outside the
  situational-trade-off space. *(Part I §2; MEL-REQ-207, 235)* [DI]
- **Framing:** "best is situational" applies to *how MELFINA works a problem*
  (depth, speed, breadth, autonomy-within-bounds), not to *whether it respects the
  user, stays local, or keeps the safeguards*.

---

## Part C — What the later phases must decide (not decided here)

| Decision | Phase | Note |
|---|---|---|
| The neutral primitive set / life model | HUMAN / CENTRAL MODEL | OQ-1, OQ-2; ~15 SHOULD requirements depend on it |
| Default consequential/routine action boundary | SYSTEM DESIGN + REAL-WORLD USE | OQ-12 |
| Default per-area autonomy & proactivity levels | REAL-WORLD USE + ITERATION | OQ-7 |
| Storage substrate, file format, encoding | ARCHITECTURE / STORAGE | E-class; MUST stay open until then |
| Interaction medium(s) | INTERFACE | E-class; "multi-modal, user-choosable" is the only constraint |
| Which local reasoning components / model sizes | ARCHITECTURE + AI LAYER | OQ-11; constrained by MEL-REQ-155, 198 |
| Permission-grant mechanism (how capabilities are represented) | ARCHITECTURE / LOW-LEVEL FOUNDATIONS | constrained by MEL-REQ-180 (object-capability style) |
| Whether/when to build an isolated network add-on | ITERATION | OQ-11; MEL-REQ-167 sets the constraints if ever |
| Whether any progress representation is offered at all | REAL-WORLD USE | OQ-6; default is none |
| The dynamic-dispatch / metareasoning mechanism (how strategy is chosen) | ARCHITECTURE + CORE ENGINE | constrained by MEL-REQ-204–209, 252–253 |
| The capability lifecycle (create → test → promote → version → retire) mechanism | ARCHITECTURE + AI LAYER | constrained by MEL-REQ-219–227 |
| The external permission monitor that enforces the meta-invariant | LOW-LEVEL FOUNDATIONS + ARCHITECTURE | constrained by MEL-REQ-235 (external, deterministic, monotonic) |
| Authorisation granularity for the nine self-change tiers | SYSTEM DESIGN + ITERATION | OQ-20; MEL-REQ-233 |
| Whether MELFINA ever performs tier 6+ self-modification, or those stay human-only | ITERATION (much later) | E-class; MEL-REQ-236 keeps them non-autonomous regardless |
| The fixed regression / verification harness for capabilities | ARCHITECTURE + TESTING/EVALUATION | constrained by MEL-REQ-221, 226 |
