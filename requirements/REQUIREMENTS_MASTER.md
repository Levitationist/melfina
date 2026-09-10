# MELFINA — REQUIREMENTS MASTER

**Phase:** PERSONAL REQUIREMENTS.
**Mission:** PERSONAL REQUIREMENTS MISSION 001 (base brief + autonomous-reasoning
addendum + local-only/network-isolation addendum + autonomous-personal-intelligence
extension).
**Status:** first pass, pending user review.
**Date:** 2026-09-10.
**Built on:** `research/RESEARCH_MASTER.md` (§§1–32), `research/CONFLICTS.md`,
`research/BIBLIOGRAPHY.md` (research checkpoints 001 + 002).

---

## 0. What this document is — and is not

This is a **requirements specification**: what MELFINA must, should, may, and must
not do. It is **implementation-independent**.

**This document does NOT:**
- choose an architecture, or describe one;
- choose a programming language (C, C++, Rust, Python, …);
- choose a storage substrate (SQLite, files, a custom format, …);
- choose an AI model, model size, framework, library, or provider;
- choose a UI toolkit or interaction medium;
- decide how any requirement is met.

Where a technology is named below, it is **a research example for context**, never
a decision. Architecture is the next phase and is explicitly out of scope here.

**This document does NOT make medical claims.** The research draws on clinical and
cognitive-science literature about ADHD, autism, and OCD because that literature
is evidence about human cognitive friction. MELFINA is **not** a medical device,
not a treatment, not a diagnostic tool. Diagnostic categories are **not** turned
into product modes (see MEL-REQ-006, and the anti-requirements in Part VII).

### 0.1 Priority keywords

| Keyword | Meaning |
|---|---|
| **MUST** | MELFINA is not MELFINA without this. A build that violates a MUST is non-conforming. Reserved for the genuinely essential. |
| **SHOULD** | Strongly expected. Omitting it needs a recorded, defensible reason. |
| **MAY** | Permitted and possibly valuable; no obligation. Often a later increment. |
| **MUST NOT** | Prohibited. An anti-requirement. A build that does this is non-conforming regardless of how well it does it. |

Priorities are justified per requirement. **Most requirements here are SHOULD or
MAY.** The MUST set is deliberately small (listed in the checkpoint).

### 0.2 Evidence tiers (carried from research)

**[E]** evidence · **[G]** clinical / professional guidance · **[DI]** design
inference · **[H]** hypothesis · **[U]** unknown / insufficient evidence.

**A `[U]` or `[H]` finding never becomes a MUST or SHOULD requirement on its own.**
It may become a MAY requirement, a design hypothesis to validate, or an open
question. Where a requirement rests on `[H]`/`[U]`, it is marked and cross-listed
in `requirements/OPEN_QUESTIONS.md`.

### 0.3 Provenance classes (required by the mission)

Every requirement carries a provenance class:

| Class | Meaning |
|---|---|
| **A** | Evidence-supported — traceable to `[E]` or `[G]` research findings. |
| **B** | User-goal-derived — traceable to the user's explicitly stated priorities (local-first, privacy-first, lightweight, reliable, highly functional, maintainable, strong user control, minimal abstraction, no unnecessary cloud, AI-without-surrendering-control) or to the mission briefs. |
| **C** | Design hypothesis — plausible, requires later validation. Not established. |
| **D** | Unknown — flagged for experimentation; stated as a question or a deliberately weak requirement. |
| **E** | Deliberately undecided — recorded here so it is not silently decided elsewhere. |

Most requirements are **A+B** (evidence that also serves a stated goal). Pure-C and
pure-D items are explicitly labelled and never given MUST priority.

### 0.4 Requirement format

```
MEL-REQ-NNN — PRIORITY — <area>
Statement:   <what, implementation-independent>
Why:         <rationale> — <provenance A/B/C/D/E> [evidence tier]
Accept:      <how conformance is checked>
Open:        <open question, if any>
```

Compact form is used for closely related requirements. IDs are unique and
sequential; gaps are not reused.

### 0.5 How to read this

1. Part I — Core purpose, scope, and the four foundational stances.
2. Part II — the **MELFINA Capability Model** (conceptual, not architecture) and
   the **autonomy triad** (cognitive / decision / execution).
3. Part III — functional requirements by area (MEL-REQ-###).
4. Part IV — the autonomous-personal-intelligence requirements.
5. Part V — cross-cutting requirements (data, privacy/local-only, reliability,
   security, accessibility, efficiency, observability, maintainability).
6. Part VI — feasibility classification (what is demonstrated / plausible /
   research-stage / speculative / aspirational).
7. Part VII — **anti-requirements** (what MELFINA must never become).
8. `REQUIREMENTS CHECKPOINT 001` — the summary.
9. Companion files: `CONFLICTS.md` (tensions as configurable dimensions /
   constraints / safeguards / open questions) and `OPEN_QUESTIONS.md`.

---

# PART I — FOUNDATIONS

## 1. Core purpose and scope

**MEL-REQ-001 — MUST — purpose.**
MELFINA is a **single-user, local personal operating system**: a system that holds
the user's life information, helps them understand their situation, reason about
it, remember, plan, decide what to do next, and — within explicit permission —
act, teach, create, and automate on their behalf.
- *Why:* the project's stated purpose across all mission briefs. — **B**
- *Accept:* the top-level product description matches this sentence; every feature
  traces to one of these verbs (hold / understand / reason / remember / plan /
  decide / act / teach / create / automate).

**MEL-REQ-002 — MUST — it is more than a tracker.**
MELFINA MUST NOT be *only* a store, tracker, notification engine, dashboard, or
command-response assistant. Those may be components; they are not the whole.
- *Why:* explicit mission statement ("do not reduce the vision to an AI
  assistant"; "not merely store / track / answer prompts / display dashboards"). — **B**
- *Accept:* the capability model (Part II) is implemented beyond storage+display;
  reasoning, planning, and (permitted) action capabilities are present.

**MEL-REQ-003 — MUST — single user, personal.**
MELFINA MUST be designed around **one person** — its actual user — not a generic
population or a team.
- *Why:* project brief; research shows category-average designs mis-serve
  individuals (`RESEARCH_MASTER` §15). — **A+B** [E]
- *Accept:* no requirement assumes multi-tenancy; personalisation (§25) is
  first-class; there is no "average user" default that cannot be changed.

**MEL-REQ-004 — MUST — the user is the authority.**
Every interpretation, category, priority, plan, setting, and consequential action
is ultimately the user's. MELFINA proposes and assists; the user disposes.
- *Why:* `RESEARCH_MASTER` P2, §15, §22.3 (autonomy-support meta-analyses, [E]);
  stated goal "strong user control". — **A+B** [E]
- *Accept:* for every consequential action there is a point at which the user can
  approve, modify, reject, or reverse it; no setting changes the user's data model
  or behaviour rules without the user's act.

**MEL-REQ-005 — MUST — scaffold, don't steer.**
MELFINA holds structure *for the user to use*; it MUST NOT prescribe how the user
should live, work, feel, or practise. It offers methods; it does not impose one.
- *Why:* `RESEARCH_MASTER` P1; PDA / demand-avoidance and SDT evidence that
  imposed demands provoke avoidance ([G]/[E]); GTD/method-imposition failure mode
  (§10.6). — **A** [E]/[G]
- *Accept:* no workflow is mandatory to keep the system coherent; the user can
  ignore any feature and MELFINA still works (see MEL-REQ-140, graceful
  degradation).

**MEL-REQ-006 — MUST NOT — no diagnostic modes.**
MELFINA MUST NOT ship an "ADHD mode", "autism mode", "OCD mode", or any
diagnostic-category preset that bundles behaviour.
- *Why:* the three profiles' needs conflict (`RESEARCH_MASTER` §6.3); comorbidity
  amplifies impairment and the combined profile is unmeasured (§22); a category
  preset bakes in a conflict-resolution the individual did not choose. — **A** [E]
- *Accept:* configuration is by individual, named dimensions (§25, `CONFLICTS.md`),
  never by diagnosis; no code path branches on a diagnostic label.

**MEL-REQ-007 — SHOULD — neutral primitives.**
MELFINA's underlying model of the user's life SHOULD avoid the framings
"tasks + habits + projects" as its primitives, because each carries a known
failure mode (habits→streaks, tasks→binary completion, projects→taxonomy burden).
It SHOULD seek a smaller descriptive set from which specific uses (including music)
emerge as cases.
- *Why:* `RESEARCH_MASTER` §16.8 P13, §10, §14. — **C** [H]
- *Accept:* the requirements below (tasks, learning, music, projects) are all
  expressible without a bespoke type per domain; the model is documented and the
  primitive set is small (target: ≤ ~7 core primitives).
- *Open:* what the primitives are — this is a HUMAN/CENTRAL MODEL question, not
  decided here. (`OPEN_QUESTIONS.md` OQ-1)

**MEL-REQ-008 — MUST — descriptive, not judgemental, data model.**
The data model MUST NOT force "compulsion vs routine", "obligation vs choice", or
"success vs failure" interpretations onto the user's entries. Meaning is
user-assigned.
- *Why:* autistic routine vs OCD compulsion is not distinguishable by topography
  and the meaning is the user's (`RESEARCH_MASTER` §6.2); ego-dystonic vs
  self-affirming is a clinical distinction MELFINA cannot and must not make. — **A** [E]/[G]
- *Accept:* no stored field records a value judgement the user did not enter; no
  automatic classification of the user's behaviour as healthy/unhealthy.

**MEL-REQ-009 — SHOULD — scope boundary: MELFINA supports real life, it is not life.**
MELFINA SHOULD be designed so that it can be closed, ignored for days, or stopped
entirely without loss or penalty, and so that it does not become a substitute for
human relationships, professional help, or the user's own judgement.
- *Why:* autistic-burnout and camouflaging evidence (demand load), AI-dependence
  and loneliness evidence (`RESEARCH_MASTER` §28), self-tracking abandonment
  research (§10.4). — **A** [E]
- *Accept:* MEL-REQ-140 (graceful degradation) holds; no feature's value depends
  on daily engagement; no "you've been away" guilt surface (MEL-REQ-047).

## 2. Foundational stances (constraints on everything below)

**MEL-REQ-010 — MUST — predictable and deterministic core.**
MELFINA's core behaviour MUST be predictable: it does not silently rearrange,
hide, delete, or act on the user's world. State changes are visible and, where
consequential, previewable and reversible.
- *Why:* intolerance-of-uncertainty evidence in autism and OCD ([E], r≈0.62);
  adaptive-UI evidence that unpredictable system change is disliked and costly
  ([E]); context-inference-unreliability across 30 years of prototypes ([E/DI]).
  — **A+B** [E]
- *Accept:* given the same inputs and state, core operations produce the same
  result; every automatic change is either shown or in the audit log
  (MEL-REQ-160); nothing consequential happens without a preview or an undo.

**MEL-REQ-011 — MUST — quiet by default.**
MELFINA MUST default to minimal interruption. The user controls what may
interrupt, per source, at a granular level.
- *Why:* interruption-cost / attention-residue / alert-fatigue evidence ([E]);
  Fitz 2019 RCT (batched, predictable delivery helps; full silence has its own
  cost) ([E]); Mehrotra TOCHI 2021 (without granular control users disable whole
  channels) ([E]). — **A** [E]
- *Accept:* out of the box, nothing interrupts unbidden except a user-defined
  minimal set; interruption settings are per-source and per-context.

**MEL-REQ-012 — MUST — no scorekeeping by default.**
MELFINA MUST NOT, by default, present streaks, points, badges, scores, comparative
statistics, or aggregate completion percentages.
- *Why:* gamification meta-analysis (small, fragile effects; behavioural g≈0.25)
  ([E]); streaks convert intrinsic goals to loss-avoidance ([E]); over-
  justification risk for a musician ([E]); perfectionistic-concerns / NJRE
  evidence ([E]). — **A** [E]
- *Accept:* a fresh install shows no such element; any progress representation that
  exists is off by default, private, non-comparative, non-accumulating, and
  user-enableable (§9).

**MEL-REQ-013 — MUST — restraint at the hazard surfaces.**
Completion states, confirmations, history/logs, metrics, and re-checking
affordances MUST be treated as hazards and designed with deliberate minimalism.
Adding intensity at these surfaces is a user act; the system never escalates them
by surprise.
- *Why:* checking→memory-distrust ([E]); reassurance/accommodation→worse OCD
  outcomes ([E]); NJRE/incompleteness ([E]); self-tracking rumination harms ([E]).
  — **A** [E]
- *Accept:* an audit of these five surfaces shows each defaults to the minimal,
  quiet, non-repetitive option; there is no "are you sure?" chain, no "review your
  entries" ritual, no default dashboard.

**MEL-REQ-014 — MUST — local-only core (see Part V §37 for the full set).**
MELFINA's core — understand, reason, remember, retrieve, plan, decide, teach,
analyse local data, run whatever local AI it needs, and execute authorised local
actions — MUST remain fully operational with network connectivity **completely
disabled**, indefinitely.
- *Why:* the user's explicit fundamental requirement; also removes the
  exfiltration leg of the "lethal trifecta" (private data + untrusted content +
  outbound channel), which is a primary agent-security risk ([E], Willison 2025
  and subsequent). — **A+B** [E]
- *Accept:* with the machine's network interfaces down, every core function works;
  an automated test suite runs the core offline and passes; no core code path
  requires a socket to a non-local address.

**MEL-REQ-015 — MUST — graceful degradation.**
MELFINA MUST stay coherent and non-punishing when capacity is low, when the user
lapses, when AI components are disabled or unavailable, and when parts fail.
Re-entry after any gap MUST be frictionless and free of judgement.
- *Why:* personal-informatics abandonment research (lapses are normal, design for
  re-entry) ([E]); autistic-burnout and emotion-dysregulation evidence ([E]);
  ADHD energy variability ([E/clinical]). — **A** [E]
- *Accept:* disabling any optional subsystem leaves a usable system; there is no
  UI that reacts to a gap with pressure, guilt, or a broken streak; a 6-week
  absence produces the same experience as normal use.

---

# PART II — THE MELFINA CAPABILITY MODEL

> **This is a conceptual requirements model, not an architecture.** These
> capabilities are *what MELFINA must be able to do*. They do **not** imply
> separate software modules, separate processes, separate models, or any
> particular decomposition. That is the architecture phase's job.
>
> **They are also NOT a static feature checklist or a fixed operating strategy.**
> The fifteen capabilities are a *vocabulary* for talking about what MELFINA does;
> they are not a ceiling. Which capabilities are engaged, how deeply, in what
> combination, with how much context and compute, and whether a *new* capability
> is constructed for the problem at hand — all of that is itself something
> MELFINA reasons about per situation. See **PART IV-B — THE DYNAMIC
> SELF-DIRECTED SYSTEM** (`MEL-REQ-204`…`MEL-REQ-252`), which is the load-bearing
> reframing: *MELFINA determines what capabilities, context, reasoning processes,
> tools, and workflows a problem requires, and — where appropriate and
> authorised — constructs new ones.*

## 15. The fifteen capabilities

For each: a one-line definition, the governing requirements, and a feasibility
note (full classification in Part VI). Detailed requirements are in Parts III–V;
this section is the map.

| # | Capability | Definition | Feasibility (2026) |
|---|---|---|---|
| C-1 | **Perception** | Take in the information available to it — the user's entries, local files and signals the user has exposed, the state of a task, the passage of time. | Demonstrated for structured/textual input; ambient/behavioural sensing is possible but privacy- and reliability-limited. |
| C-2 | **Memory** | Retain and resurface relevant information over the long term: what happened, what was decided, what is open, what the user prefers. | Demonstrated (retrieval + explicit stores); research-stage for reliable autonomous consolidation/forgetting. |
| C-3 | **Understanding** | Build an interpreted picture of the user's current situation from perception + memory. | Partial; interpretation is often useful but not reliable — must be presented as interpretation. |
| C-4 | **Reasoning** | Draw conclusions: infer context, identify patterns/conflicts/gaps/anomalies, derive results from principles, quantify uncertainty. | Strong for many everyday cases; weak/unreliable for long-horizon, multi-step, or novel reasoning; "knowing what it doesn't know" is an unsolved problem ([E]). |
| C-5 | **Learning** | Acquire and update models of the user's preferences, workflows, and needs from explicit feedback and observation. | Non-parametric (stored preferences, retrieval) is reliable; parametric (weight updates) risks catastrophic forgetting and drift ([E]). |
| C-6 | **Planning** | Turn goals into ordered, concrete next steps with dependencies, and revise plans as reality changes. | Demonstrated for bounded/structured planning; brittle for long-horizon real-world plans ([E]). |
| C-7 | **Decision-making** | Select a preferred course of action when sufficiently confident; choose not to act otherwise. | Feasible under strict bounds; unreliable confidence estimation means decisions must be scoped by permission and reversibility, not by the system's self-assessed certainty ([E]). |
| C-8 | **Teaching** | Help the user learn: explain at multiple depths, from fundamentals, Socratically, adapting to prerequisite gaps and misconceptions, optimising for understanding and independence. | Adaptive tutoring has strong evidence (ITS g≈0.6–0.7, ≈ human tutoring; [E]); LLM-based tutoring is promising but unevenly reliable. |
| C-9 | **Creation** | Help the user (and, within permission, produce) written work, code, plans, analyses, music-practice materials, exercises, experiments. | Demonstrated for drafting/support; autonomous creation quality is uneven; "computational creativity" requires the system to report its process ([DI]). |
| C-10 | **Tool use** | Invoke discrete capabilities (skills/commands/tools) with contextual selection and composition, under permission. | Demonstrated; also the primary security surface (malicious tools, injection) ([E]). |
| C-11 | **Computer control** | Operate the local machine as an agent: terminal and (separately) GUI, with perception→plan→act→verify→recover. | Short/narrow tasks ~70–85% on benchmarks; long-horizon connected workflows still weak ([E]); high-risk surface. |
| C-12 | **Automation** | Recognise repeatable processes and, under permission, construct and run automations — scheduled, event-, and condition-triggered, multi-step, with verification and rollback. | End-user automation is established (IFTTT/Zapier); *generating* correct automations is harder; ~50% of shared trigger-action rules have integrity/secrecy flaws ([E]). |
| C-13 | **Adaptation** | Adjust *with* the user over time — not adapt the user to the system — with transparency, correction, and forgetting. | Feasible as configuration + suggestion-confirm; silent adaptation is contraindicated ([E]). |
| C-14 | **Self-evaluation** | Assess its own outputs, decisions, and predictions against outcomes; recognise its own errors and uncertainty. | Weak — intrinsic self-assessment is the least reliable rung of the verification hierarchy ([E]); external/formal verification is stronger. |
| C-15 | **Controlled improvement** | Improve its skills, workflows, strategies, and models over time — bounded, versioned, inspectable, reversible, permission-gated. | Bounded self-refinement is feasible; open-ended self-improvement is unsafe and out of scope; MELFINA must not author its own objectives or success metrics ([E]). |

**MEL-REQ-016 — MUST — the capability model is conceptual and open, not a fixed catalogue.**
Nothing in this document may be read as requiring a particular software structure
for these capabilities. A build MAY implement several capabilities in one
component or split one capability across many. Furthermore, the fifteen
capabilities MUST NOT be implemented as a closed set of fixed modes: MELFINA
composes and (per PART IV-B) constructs capabilities dynamically. The list is a
description of scope, not an enumeration of everything MELFINA will ever be able
to do.
- *Why:* the mission explicitly forbids architecture here; the
  dynamic-self-directed correction explicitly forbids a static feature checklist.
  — **B**
- *Accept:* architecture-phase documents, not this one, decide decomposition; no
  requirement below treats the 15 capabilities as an exhaustive or frozen set.

## 16. The autonomy triad

MELFINA's "autonomy" MUST be treated as **three distinct, separately-granted
things**:

| Layer | Name | What it is | Default |
|---|---|---|---|
| **A** | **Cognitive autonomy** | The ability to *reason and reach conclusions* independently — interpret, infer, spot conflicts, generate and evaluate options, form a preferred answer. | Broad, by default. Thinking is cheap and reversible. |
| **B** | **Decision autonomy** | The ability to *select* a course of action (an internal choice, not yet execution). | Scoped. MELFINA may reach a decision internally and record it as a *proposal*. |
| **C** | **Execution authority** | The *permission to actually perform* the selected action in the world (change data, run a command, control the GUI, send anything anywhere). | Minimal, explicit, per-capability, per-scope. Never implied by A or B. |

**MEL-REQ-017 — MUST — the triad is not collapsed.**
Holding cognitive autonomy MUST NOT grant decision autonomy; holding decision
autonomy MUST NOT grant execution authority. There is no single "agent" permission
that confers all three.
- *Why:* the autonomy addendum's explicit instruction; agent-security and
  runaway-loop evidence ([E]); corrigibility research (routine autonomy + genuine
  acceptance of correction on important decisions) ([E]). — **A+B** [E]
- *Accept:* the permission model (§35, §40) has separate grants for reasoning
  scope, decision scope, and execution scope; revoking execution authority leaves
  reasoning intact.

**MEL-REQ-018 — MUST — the action pipeline.**
Any consequential action MUST pass through, in order: **THINK → DECIDE → PROPOSE →
AUTHORISE → EXECUTE → VERIFY**, with a distinct, inspectable artefact at each
stage and the ability for the user to stop at any stage.
- *Why:* the extension brief (§12, §17) requires exactly this separation; HITL /
  HOTL / HOOTL taxonomy ([E]); verification-hierarchy evidence (self-assessment
  weakest) ([E]). — **A+B** [E]
- *Accept:* for a consequential action there exists: a reasoning trace, a recorded
  decision, a human-readable proposal, an authorisation record, an execution
  record, and a verification result; an emergency stop (MEL-REQ-145) interrupts
  between any two stages.

**MEL-REQ-019 — SHOULD — routine vs consequential.**
MELFINA SHOULD distinguish **routine, reversible, low-stakes, pre-authorised**
actions (which MAY run at HOTL — act, then show) from **consequential** actions
(which MUST run at HITL — propose, wait for authorisation). The boundary is
user-configurable and conservative by default.
- *Why:* adjustable/sliding autonomy research ([E]); Horvitz mixed-initiative
  expected-value rule ([E]); "balanced corrigibility" ([E]). — **A** [E]
- *Accept:* there is a documented, user-editable definition of "consequential";
  the default classification errs toward HITL; changing it is a deliberate user
  act with a warning.
- *Open:* the default consequential/routine boundary — a requirements question
  partly, a real-use question partly. (`OPEN_QUESTIONS.md` OQ-12)

---

# PART III — FUNCTIONAL REQUIREMENTS

## 3. Life information externalisation

**MEL-REQ-020 — MUST — near-frictionless capture.**
Recording something into MELFINA MUST take one action and no forced classification
decision. Categorisation is deferred or optional.
- *Why:* capture friction is the primary failure point of task/PKM systems
  (PIM literature, [E]); a trusted capture system frees working memory only if
  capture is reliable (Masicampo, [E]). — **A** [E]
- *Accept:* time-to-capture a plain thought is ≤ a few seconds and ≤ 1 required
  field; no capture path blocks on "which project/tag/type?".

**MEL-REQ-021 — MUST — record once, treat as settled.**
Once something is recorded, MELFINA MUST treat it as settled and MUST minimise
affordances to re-verify, re-open, re-confirm, or "review" it.
- *Why:* repeated checking worsens memory confidence ([E]); this is a named OCD
  hazard. — **A** [E]
- *Accept:* there is no prompt to re-check recorded items; re-opening an item is
  possible but never suggested; no "unconfirmed" state that nags.

**MEL-REQ-022 — MUST — the user owns interpretation of captured items.**
MELFINA MAY suggest a type, link, or next action for a captured item; it MUST NOT
assign one that changes the item's meaning or handling without the user's
acceptance.
- *Why:* MEL-REQ-004, MEL-REQ-008. — **A+B** [E]
- *Accept:* suggestions are visibly provisional; the item is fully usable with
  none accepted.

**MEL-REQ-023 — SHOULD — capture carries optional structure.**
A captured item SHOULD be able to *optionally* carry a concrete next action and a
when/where, offered as a scaffold, never required.
- *Why:* implementation-intentions evidence (if-then plans aid initiation;
  general-population [E], child-ADHD [E], adult-ADHD [U]). — **A/C** [E]/[U]
- *Accept:* the fields exist, are skippable, and their absence never blocks
  anything.

**MEL-REQ-024 — SHOULD — capture a resumption cue on leaving a task.**
When the user leaves an in-progress activity, MELFINA SHOULD offer a one-line
"where I was / what's next" capture, and replay it on return rather than dropping
the user in cold.
- *Why:* memory-for-goals / resumption-lag / attention-residue evidence; the Leroy
  moderator (a credible path back reduces the cost) ([E]). — **A** [E]
- *Accept:* the offer is lightweight and skippable; on resuming, the cue is shown
  first.

**MEL-REQ-025 — MUST — the archive may be lossy; retrieval is the design target.**
MELFINA MUST NOT pursue "total capture". It MUST be designed around how the user
will *find and reconstruct* information, accepting that not everything is kept or
findable.
- *Why:* "Beyond total capture" — data in, little usable out; 30 years of
  memory-augmentation prototypes: retrieval and timing are the unsolved problems
  ([E]). — **A** [E]
- *Accept:* there is no feature whose purpose is exhaustive logging of the user's
  life; retrieval (§19) is a first-class, well-resourced capability.

## 4. Tasks and open loops

**MEL-REQ-026 — MUST — represent "open loops", not just "tasks".**
MELFINA MUST be able to hold a "thing on the user's mind" that is not yet a
well-formed task — a worry, an intention, a question, a someday-maybe — without
forcing it into a task shape.
- *Why:* Zeigarnik / Masicampo (open loops cost attention; a *specific plan*, not
  task-ification, is what relieves them) ([E]); neutral-primitives hypothesis. — **A/C** [E]/[H]
- *Accept:* an item can exist with only a description; converting it to something
  more structured is optional and user-driven.

**MEL-REQ-027 — MUST — no forced binary completion.**
MELFINA MUST NOT model progress solely as done/not-done, and MUST allow partial
progress that "counts". Items may also be closed as "no longer relevant" without
being "completed".
- *Why:* all-or-nothing completion + visible aggregates interact badly with
  perfectionism and incompleteness, and demotivate the ADHD reward system ([E]).
  — **A** [E]
- *Accept:* items support at least: open / in-progress / done / dropped
  (dropped ≠ failed); no global completion percentage by default (MEL-REQ-012).

**MEL-REQ-028 — SHOULD — closure by hand-off.**
MELFINA SHOULD let the user get the relief of "handled" by handing an item to
MELFINA (a credible plan / a scheduled resurfacing), distinct from "finished".
- *Why:* Masicampo (a credible plan removes the intrusion even undone); Leroy
  (belief in a path back removes residue) ([E]). — **A** [E]
- *Accept:* "I've planned this / MELFINA will bring it back at X" is a first-class
  state and visibly reduces the item's presence.

**MEL-REQ-029 — MUST NOT — no "overdue" pressure.**
MELFINA MUST NOT display tasks in a pressuring way (red "overdue", countdowns,
"you're behind", disappointment).
- *Why:* PDA / demand-avoidance ([G]); emotion dysregulation & rejection
  sensitivity in ADHD ([E]); music-dropout evidence (controlled pressure predicts
  dropout) ([E]). — **A** [E]/[G]
- *Accept:* no time-based item ever renders as a threat; language is neutral.

**MEL-REQ-030 — SHOULD — few things shown at once.**
When presenting things to do, MELFINA SHOULD show a small number, offer a sensible
default "next", and make "not now" frictionless.
- *Why:* choice-overload / decision-cost under EF load ([E]); COGA "help users
  focus" ([G]). — **A** [E]/[G]
- *Accept:* a default view shows a bounded set (not the whole list); deferring an
  item is one action and non-judgemental.

## 5. Planning and execution

**MEL-REQ-031 — SHOULD — decompose to a concrete next step.**
MELFINA SHOULD help turn a vague intention into a concrete, small next action, on
request.
- *Why:* task decomposition is a core mechanism in CBT for adult ADHD and in
  self-regulated practice ([E]); "task ambiguity is the initiation barrier"
  (Goblin Tools design bet). — **A** [E]
- *Accept:* given a vague item, MELFINA can propose a first step ≤ one sitting in
  size; the user edits/accepts/rejects.

**MEL-REQ-032 — SHOULD — plans are revisable and reality-tracking.**
Plans SHOULD be easy to revise as reality changes; a plan that has drifted from
reality MUST NOT silently persist as if valid.
- *Why:* CSCW task-list research (lists abandoned when out of sync); planning
  brittleness in agents ([E]). — **A** [E]
- *Accept:* editing a plan is low-friction; MELFINA surfaces (neutrally) when a
  plan and reality have diverged.

**MEL-REQ-033 — SHOULD — time estimates use the user's own history, not fresh guesses.**
When MELFINA assists with duration estimates, it SHOULD feed back how long similar
past activities actually took, rather than asking for or trusting a new estimate.
- *Why:* planning fallacy (own-task underestimation) ([E]); reference-class
  forecasting / the outside view (APA-endorsed) ([E]). — **A** [E]
- *Accept:* an estimate view cites prior actuals for comparable activities; a
  fresh user estimate is never used unadjusted for scheduling that fills a day.
- *Open:* how "comparable" is judged. (`OPEN_QUESTIONS.md` OQ-8)

**MEL-REQ-034 — MUST NOT — no rigid methodology lock-in.**
MELFINA MUST NOT require the user to follow a specific method (GTD weekly review,
time-blocking, Pomodoro intervals, bullet-journal migration) for the system to
stay coherent.
- *Why:* method-imposition failure mode; systems that break during low-capacity
  periods fail exactly when needed ([DI] from [E] on capacity variability). — **A** [E/DI]
- *Accept:* skipping any ritual for weeks does not corrupt or degrade the system.

## 6. Time and prospective-memory support

**MEL-REQ-035 — SHOULD — time made visible and concrete.**
MELFINA SHOULD represent duration, elapsed time, and remaining time in a concrete,
continuously-visible form rather than requiring internal estimation, when the user
wants it.
- *Why:* ADHD time-perception deficits are replicated ([E], children/adolescents;
  adult data thinner); "externalise time" is the consistent clinical
  recommendation ([G]). — **A** [E]/[G]
- *Accept:* a visible time representation is available on demand; it is opt-in, not
  omnipresent (MEL-REQ-011).
- *Open:* specific-tool efficacy for adults is [U]. (`OPEN_QUESTIONS.md` OQ-3)

**MEL-REQ-036 — SHOULD — preview "what's next".**
MELFINA SHOULD be able to show what is coming, and give advance notice of
transitions, in a calm form.
- *Why:* transition difficulty and predictability preference in autism ([E]);
  IU↔anxiety ([E]). — **A** [E]

**MEL-REQ-037 — SHOULD — prospective-memory support is event/context-anchored and specific.**
Where MELFINA helps the user remember to do something later, the cue SHOULD be
tied to a situation or event and be specific and actionable, not only a bare time
alarm.
- *Why:* time-based PM is more impaired than event-based (ADHD [E]; autism [E]
  with a 2026 caveat); reminder *content* and *timing* matter more than trigger
  type ([E]). — **A** [E]
- *Accept:* reminders carry a concrete action and, where possible, a triggering
  context; see MEL-REQ-041 for fatigue safeguards.

## 7. Attention and interruption

**MEL-REQ-038 — MUST — protect focus / minimise forced switches.**
MELFINA MUST NOT interject into a focused or creative session unless the user has
explicitly allowed it, and MUST minimise the number of context switches it causes.
- *Why:* monotropism (focus states costly to enter/exit) ([E/theory]); attention
  residue and resumption lag ([E]); flow is disrupted by interruption and
  self-evaluative thought ([E]). — **A** [E]
- *Accept:* a "focus/session" state exists in which MELFINA is silent by default;
  MELFINA never initiates a switch during it.

**MEL-REQ-039 — SHOULD — help leaving and returning.**
When the user does switch, MELFINA SHOULD support a cheap resumption-cue capture
(MEL-REQ-024) and replay it on return.
- *Why:* memory-for-goals; cues + rehearsal reduce resumption cost ([E]). — **A** [E]

**MEL-REQ-040 — SHOULD — a calm pull surface.**
MELFINA SHOULD provide a single, predictable place the user can *choose* to check
for "what would have prompted me", instead of pushing those things.
- *Why:* notification-vs-silence trade-off (both extremes have costs) ([E]);
  user-declared "now" beats system-inferred timing (JITAI evidence) ([E]). — **A** [E]

## 8. Reminders and notifications

**MEL-REQ-041 — MUST — user-controlled, granular, predictable delivery.**
Notifications MUST be user-controlled at a granular (per-source, per-context)
level, and delivered on a predictable schedule (e.g. at task boundaries or in
user-set windows), not as-they-occur by default.
- *Why:* Fitz 2019 RCT (predictable batched delivery improved attention/mood/
  control) ([E]); Iqbal & Bailey (deliver at breakpoints) ([E]); Mehrotra TOCHI
  (granular control prevents all-off) ([E]). — **A** [E]

**MEL-REQ-042 — MUST — assume alert decay; do not rely on repetition.**
MELFINA MUST NOT rely on repeated identical alerts to drive behaviour; repeated
low-value alerts lose force.
- *Why:* alert-fatigue evidence (acceptance drops measurably as the share of
  repeated/low-value alerts rises — Ancker 2017) ([E]). — **A** [E]
- *Accept:* an alert not acted on is not simply re-fired unchanged on a loop;
  escalation, if any, is user-configured, bounded, and non-pressuring.

**MEL-REQ-043 — MUST NOT — no notification-driven behaviour change.**
MELFINA MUST NOT use notifications as a mechanism to change the user's behaviour
(nudges to act, streak-protection pings, "you haven't…" reminders).
- *Why:* PDA / demand response ([G]); notification-driven systems' failure modes
  ([E]); this is an anti-requirement (Part VII). — **A** [E]/[G]

**MEL-REQ-044 — MAY — context-sensitive reminder timing.**
MELFINA MAY offer context- or event-triggered reminder timing, but MUST prefer the
user's declared availability, and MUST NOT depend on inferred context for
correctness.
- *Why:* location-based reminders "still not effective" in the field; context
  inference has never become reliable ([E]); self-reported-need triggers rated
  better than inferred ([E]). — **A** [E] / **C** for the inference part

## 9. Progress representation

**MEL-REQ-045 — MUST NOT — no streaks / points / comparative or aggregate metrics by default.**
(Restates MEL-REQ-012 as a functional prohibition for this area.)
- *Why:* as MEL-REQ-012. — **A** [E]

**MEL-REQ-046 — MAY — a minimal, private, optional progress view.**
MELFINA MAY offer a progress representation only if it is: off by default, private,
non-comparative, non-accumulating (no maintained history demanding upkeep),
partial-credit, interruptible, and disableable without penalty.
- *Why:* the ADHD near-term-salience need is real ([E]) but no safe form is
  demonstrated; self-tracking harms concentrate in perfectionism/OCD/depression
  ([E]). — **A** for the constraints; **C/D** for whether any such view is net-positive
- *Open:* is there *any* progress representation that helps without harm for this
  user? Currently "none" is the safe answer. (`OPEN_QUESTIONS.md` OQ-6)

**MEL-REQ-047 — MUST NOT — no guilt / shortfall UI.**
MELFINA MUST NOT surface gaps, missed intentions, or inactivity in a way that
implies failure or disappointment. Shortfalls are shown only neutrally and only on
request.
- *Why:* emotion dysregulation / RSD ([E]); autistic burnout ([E]); self-tracking
  abandonment research (don't punish gaps) ([E]). — **A** [E]

## 10. Motivation and reinforcement

**MEL-REQ-048 — SHOULD — serve salience by shrinking the task, not by adding rewards.**
Where the user needs near-term motivation for a non-salient task, MELFINA SHOULD
help make the task smaller and its next step concrete, rather than attaching points
or external rewards.
- *Why:* delay discounting ([E]); over-justification (extrinsic reward can reduce
  intrinsic motivation, relevant for a musician) ([E]); decomposition works ([E]).
  — **A** [E]

**MEL-REQ-049 — MUST NOT — no extrinsic reward economy.**
MELFINA MUST NOT build an internal currency, XP, level, or achievement system.
- *Why:* Part VII anti-requirement; gamification-trap evidence. — **A** [E]

**MEL-REQ-050 — SHOULD — support autonomous motivation.**
Where MELFINA references the user's goals, it SHOULD connect actions to the user's
*own* stated reasons and competence, not to obligation or external accountability
(unless the user opts into accountability and shapes it).
- *Why:* SDT autonomy-support meta-analyses ([E]); music-dropout evidence
  (autonomous motivation protects, controlled predicts dropout) ([E]). — **A** [E]

## 11. Cognitive load management

**MEL-REQ-051 — SHOULD — progressive disclosure with a stable, obvious control.**
MELFINA SHOULD show little by default and reveal more on demand, with the
disclosure control obvious and in a stable location; hiding MUST be predictable and
user-reversible, and MUST NOT be driven by the system deciding what matters.
- *Why:* COGA objectives 2 & 5 ([G]); choice-overload ([E]); the simplify-vs-
  loss-of-information tension (`CONFLICTS.md` C4). — **A** [E]/[G]

**MEL-REQ-052 — SHOULD — plain, literal language.**
System text SHOULD be plain, literal, and free of metaphor, double negatives, and
nested clauses.
- *Why:* COGA objective 3 ([G]); autistic-adult web-user studies (simple beats
  complex) ([E]). — **A** [E]/[G]

**MEL-REQ-053 — SHOULD — do not rely on the user's recall.**
Multi-step flows SHOULD carry context forward and not require the user to remember
earlier state; MELFINA MUST NOT gate access behind recall tests.
- *Why:* COGA objective 6 ([G]); working-memory evidence (ADHD, autism, adult)
  ([E]). — **A** [E]/[G]

**MEL-REQ-054 — MUST — easy, always-available undo.**
Actions MUST be forgiving: unintended actions are prevented where cheap to do so,
and reversible where not.
- *Why:* COGA objective 4 ([G]); the whole predictability/reversibility stance. — **A** [G]

## 12. Uncertainty and checking safeguards

**MEL-REQ-055 — MUST — no re-verification prompts.**
MELFINA MUST NOT prompt the user to re-check, re-read, re-confirm, or "make sure"
about anything already recorded or decided.
- *Why:* checking→memory-distrust ([E]); named OCD hazard. — **A** [E]

**MEL-REQ-056 — MUST — no "are you sure?" chains.**
Consequential actions get **one** clear confirmation, not a chain. Confirmation
text states what will happen, once, plainly.
- *Why:* repeated confirmation feeds checking; inflated-responsibility mechanics
  ([E]/[G]). — **A** [E]/[G]

**MEL-REQ-057 — MUST — MELFINA asserts, so the user's mind can let go.**
When MELFINA has recorded or scheduled something, it MUST communicate "I have
this" clearly, so trust can replace re-checking.
- *Why:* Masicampo (a *trusted* hand-off relieves the load); the "trust replaces
  verification" theme ([E]/[DI]). — **A** [E]
- *Accept:* after capture/scheduling, there is a clear, calm confirmation of what
  MELFINA will do and when; it is not repeated.

**MEL-REQ-058 — MUST — the AI/assistant must recognise and not feed reassurance-seeking.**
Where MELFINA answers questions, it MUST be able to recognise a repeated
reassurance-seeking / rumination pattern and respond by **naming the pattern and
deferring to the user's own recorded prior decision**, rather than generating
another reworded answer; and it MUST NOT keep the user in conversation.
- *Why:* reassurance/accommodation → worse OCD outcomes ([E]); AI as a documented
  novel reassurance vector (IOCDF guidance [G]; MIT–OpenAI preprint RCT: use ↔
  dependence/loneliness [E]); "always available, never frustrated, endless fresh
  variations" ([G]). — **A** [E]/[G]
- *Accept:* a repeated near-identical question triggers a "you asked this before
  and decided X — do you want to revisit that, or trust past-you?" style response,
  not a fresh essay; there is no engagement-maximising behaviour.
- *Open:* whether this safeguard reduces compulsion without frustrating legitimate
  use — untested. (`OPEN_QUESTIONS.md` OQ-9) — **D**

**MEL-REQ-059 — MUST — calibrated uncertainty; "I don't know" is a first-class output.**
MELFINA MUST be able to say "I don't know / not enough information" and MUST NOT
present a guess with unwarranted confidence.
- *Why:* the autonomy addendum requires "recognize when it does not know enough";
  abstention is an *unsolved* problem and reasoning-tuning can *degrade* it ([E]).
  — **A+B** [E]
- *Accept:* MELFINA produces explicit low-confidence / abstention responses; its
  expressed confidence is not treated by the system as reliable — consequential
  actions are gated by permission and reversibility, not by self-assessed
  certainty (MEL-REQ-017, MEL-REQ-018).
- *Open:* how to make abstention reliable — research-stage. (`OPEN_QUESTIONS.md`
  OQ-10) — **D**

**MEL-REQ-060 — MUST NOT — no compelled engagement.**
MELFINA MUST NOT force the user to categorise, resolve, rate, or respond to
something to proceed (no mandatory fields blocking progress, no forced triage).
- *Why:* avoidance is central to OCD and a forced-engagement collision is
  distressing ([G]); EF-load and choice-cost ([E]). — **A** [E]/[G]

## 13. Logging and self-tracking

**MEL-REQ-061 — MUST — self-tracking is opt-in and user-initiated.**
Any tracking, logging, or self-monitoring beyond what the user explicitly records
MUST be off by default and started only by the user.
- *Why:* self-tracking harms concentrate in perfectionism/OCD/depression;
  compulsive logging is a recognised pattern; mood-tracking is explicitly
  double-edged ([E]). — **A** [E]

**MEL-REQ-062 — MUST — logs are not a checking surface.**
Where a log exists, MELFINA MUST NOT invite re-reading, comparison, or monitoring
of it; retention and visibility are deliberate, conservative choices.
- *Why:* logs become checking objects; "visible external evidence" → "invisible
  internal questioning" (music self-monitoring literature) ([E]). — **A** [E]

**MEL-REQ-063 — SHOULD — treat mood/state logging as a known-hazard feature.**
If MELFINA offers mood or internal-state logging, it SHOULD be presented as a
double-edged tool (can raise self-awareness; can induce rumination), be easy to
stop, and never be a default or a prompted routine.
- *Why:* mood-monitoring evidence is genuinely two-sided and "net clinical effect
  unresolved" ([E]). — **A** [E]

**MEL-REQ-064 — MUST — lapses are normal; no lapse penalty.**
Stopping any tracking for any period MUST have no consequence in the system and
MUST NOT be surfaced as a lapse.
- *Why:* personal-informatics abandonment research ([E]). — **A** [E]

**MEL-REQ-065 — SHOULD — the user's own history is retained conservatively and is theirs.**
MELFINA SHOULD keep enough history to be useful (e.g. for reference-class time
estimates, MEL-REQ-033) but SHOULD default to modest retention, make retention
periods visible and adjustable, and make full deletion easy.
- *Why:* data minimisation ([G]); the user's future access to exhaustive self-logs
  is an OCD-relevant design choice ([E/DI]); privacy-by-design ([G]). — **A+B** [G]

## 14. Personal knowledge / information management

**MEL-REQ-066 — MUST — capture-first, organise-later; emergent structure.**
MELFINA MUST NOT require an up-front taxonomy or filing decision. Structure
emerges from use; the user can impose structure but is never forced to.
- *Why:* PIM literature (keeping/filing is the costly, abandonment-prone part)
  ([E]); over-structuring / "second brain fatigue" (practitioner-strong). — **A** [E]

**MEL-REQ-067 — SHOULD — retrieval supports navigation + context, not only search.**
Retrieval SHOULD support browsing with context and incremental narrowing, not
assume that search alone solves personal retrieval.
- *Why:* Bergman & Whittaker — people prefer navigation+context for their *own*
  information; tag/search approaches underperform for personal data ([E]). — **A** [E]

**MEL-REQ-068 — SHOULD — no maintenance burden to stay coherent.**
The knowledge store MUST remain coherent and useful without periodic
reorganisation, link-tending, or review rituals.
- *Why:* maintenance-tax failure mode ([E-weak / practitioner-strong]); OCD
  checking surface of an elaborate structure ([DI]). — **A** [E/DI]

**MEL-REQ-069 — SHOULD — notes and references are a primitive, not a separate app.**
Per MEL-REQ-007, notes/artifacts SHOULD be one of the small primitive set, linkable
to open loops, sessions, projects, and learning, not a walled-off subsystem.
- *Why:* neutral-primitives hypothesis; cross-domain connection is a stated goal.
  — **C/B** [H]

## 15. Learning and teaching

**MEL-REQ-070 — SHOULD — adaptive teaching capability.**
MELFINA SHOULD be able to teach the user a topic they choose: explain at multiple
depths, start from fundamentals, identify and fill prerequisite gaps, check
understanding, detect misconceptions, and adapt difficulty.
- *Why:* intelligent-tutoring-systems meta-analyses (g≈0.6–0.7; step-based
  tutoring ≈ human tutoring) ([E]); a stated user goal ("teach the user whatever
  they want to learn"). — **A+B** [E]
- *Open:* LLM-based tutoring reliability is uneven; ITS evidence is for structured
  domains. (`OPEN_QUESTIONS.md` OQ-13)

**MEL-REQ-071 — SHOULD — optimise for understanding and independence, not answer speed.**
Teaching SHOULD favour Socratic prompting, letting the user reach answers, and
building transferable understanding, over fast answer delivery.
- *Why:* stated user goal ("do NOT optimize merely for fast answer delivery");
  deskilling / over-reliance evidence ([E]); explanations alone foster
  over-reliance ([E]). — **A+B** [E]
- *Accept:* the teaching mode has a setting for how much it withholds vs supplies;
  default leans toward guided discovery.

**MEL-REQ-072 — SHOULD — use spaced retrieval and interleaving where the user wants review.**
Where MELFINA helps the user retain material, it SHOULD use retrieval practice and
spaced review (and interleaving for similar concepts), which are the
best-evidenced study techniques.
- *Why:* learning-science meta-analyses (retrieval + spacing most effective; 242
  studies / 169k participants) ([E]); note testing-effect robustness varies by
  domain ([E]). — **A** [E]

**MEL-REQ-073 — SHOULD — connect ideas across domains and to the user's existing knowledge.**
Teaching SHOULD link new concepts to what the user already knows, to other
disciplines, to formalism ↔ intuition, and (where apt) to physical/lived
observation.
- *Why:* stated goal (§7, §8 of the extension); transfer is a learning-science
  aim. — **B/C** [H] for effectiveness of MELFINA doing this specifically

**MEL-REQ-074 — SHOULD — maintain a local learning history and long-term paths.**
MELFINA SHOULD keep a local record of what the user has learned, what is shaky, and
what long-term learning paths are in progress, and be able to revisit forgotten
material.
- *Why:* stated goal; spaced review needs a schedule of past exposure ([E]). — **A+B** [E]
- *Accept:* subject to MEL-REQ-062 (not a checking surface) and MEL-REQ-047 (no
  guilt for forgetting).

**MEL-REQ-075 — MAY — generate exercises, problems, and small experiments.**
MELFINA MAY generate practice problems, exercises, and experiment designs for the
user's learning, clearly marked as generated and checkable.
- *Why:* ITS practice generation ([E]); computational-creativity caveat (mark
  generated content, report process) ([DI]). — **A/C** [E]/[DI]

**MEL-REQ-076 — SHOULD — the "explore the world" use rides on the teaching + reasoning capabilities.**
The aspiration to help the user explore physics, mathematics, biology, music,
philosophy, etc. through their own thinking SHOULD be met by the general teaching
(C-8) and reasoning (C-4) capabilities plus the scientific-thinking discipline
(§27) — **not** by a separate per-discipline subsystem.
- *Why:* neutral-primitives / general-capability direction; stated goal to
  translate the aspiration into concrete capabilities without unsupported claims.
  — **B/C** [H]
- *Accept:* no requirement mandates domain-specific modules; MELFINA can engage a
  new discipline via the same capabilities.

## 16. Music practice / musician workflow

> Treated as a genuine domain area. Musician-specific findings are **group-level
> and must not be assumed to transfer to this user** (`RESEARCH_MASTER` §23).
> Evidence and hypothesis are kept separate here.

**MEL-REQ-077 — SHOULD — support session *structure*, not hour-logging.**
For music practice, MELFINA SHOULD support planning and structuring a session
(goals, passages/sections to work, problems to solve, strategies) and reflective
self-evaluation afterward — **not** foreground total hours practised.
- *Why:* deliberate-practice *amount* explains only ~21–26% of music-performance
  variance and is contested; the evidenced leverage is session structure,
  planning, and reflection ([E]); over-justification risk of hour-tracking ([E]).
  — **A** [E]

**MEL-REQ-078 — SHOULD — model repertoire, sections/passages, and goals as first-class.**
MELFINA SHOULD be able to hold: pieces in the repertoire; sections/passages within
a piece; goals attached to a piece or passage; and the user's own notes about
each.
- *Why:* stated domain requirement; maps onto the neutral primitives (a piece is a
  project/thread; a passage is a sub-thread; a session works on passages). — **B/C** [H]
- *Accept:* expressible via MEL-REQ-007 primitives without a bespoke "music" type.
- *Open:* exact modelling — HUMAN/CENTRAL MODEL phase. (`OPEN_QUESTIONS.md` OQ-2)

**MEL-REQ-079 — SHOULD — support performance preparation as a distinct activity.**
MELFINA SHOULD support preparing for a performance (run-throughs, problem spots,
logistics, mental preparation) as a recognisable activity, at the user's option.
- *Why:* stated domain requirement; SRL interventions improve proactive
  performance preparation ([E]). — **A/B** [E]

**MEL-REQ-080 — MUST — avoid feeding perfectionistic concerns at the instrument.**
Music features MUST NOT present error counts, mistake highlighting, comparative
recordings/history, or completion/perfection framing by default.
- *Why:* perfectionistic *concerns* (fear of mistakes, doubt, evaluation) drive
  music performance anxiety and overlap with OCD's intolerance-of-uncertainty and
  "not-just-right" constructs ([E]); MPA prevalence 15–60% ([E]). — **A** [E]

**MEL-REQ-081 — SHOULD — useful history without compulsive logging.**
MELFINA SHOULD let the user keep light practice notes and see a modest history
(what was worked, what the user decided), subject to MEL-REQ-062 (not a checking
surface) and MEL-REQ-046 (no default metrics).
- *Why:* practice diaries help self-monitoring in SRL research ([E]) but the same
  literature flags self-surveillance risk ([E]). — **A** [E]

**MEL-REQ-082 — SHOULD — respect autonomy in practice.**
MELFINA MUST NOT tell the user what/when/how much to practise, apply pressure, or
frame practice as obligation. It offers structure and reflection when asked.
- *Why:* SDT / music-dropout evidence (controlled motivation predicts dropout)
  ([E]); PDA ([G]); MEL-REQ-005. — **A** [E]/[G]

**MEL-REQ-083 — MAY — support flow conditions before a session, not during.**
MELFINA MAY help set clear goals and a sense of appropriate challenge *before* a
practice/creative session (two of the three flow antecedents) but MUST stay silent
*during* it.
- *Why:* flow antecedents (skill–challenge balance, clear goals, clear feedback)
  ([E]); flow is disrupted by interruption ([E]); MEL-REQ-038. — **A/C** [E]

**MEL-REQ-084 — MUST — music is a case of the general model, not a separate feature.**
No music requirement above may be met by a bespoke music subsystem that does not
reuse the general primitives and capabilities.
- *Why:* project brief ("do not prematurely design a music feature; first
  determine the general abstractions"). — **B**

## 17. Projects and long-term goals

**MEL-REQ-085 — SHOULD — hold long-horizon threads without taxonomy burden.**
MELFINA SHOULD let the user hold ongoing efforts (a software project, a piece being
learned, a research interest, a life goal) as threads connecting notes, sessions,
open loops, and commitments over time — without requiring an up-front hierarchy.
- *Why:* neutral-primitives hypothesis; PARA/second-brain taxonomy burden ([E]).
  — **C/B** [H]

**MEL-REQ-086 — SHOULD — make a distant goal locally concrete when asked.**
MELFINA SHOULD help connect a long-horizon goal to a concrete next step in the
present.
- *Why:* delay discounting (distant rewards are weak drivers) ([E]);
  decomposition ([E]). — **A** [E]

**MEL-REQ-087 — SHOULD — software projects are a supported thread type-of-use.**
Given the user builds software (this project included), MELFINA SHOULD support a
software project as a thread: repository awareness, open questions, decisions,
next steps — reusing the general model.
- *Why:* stated context; §11 of the extension. — **B**

## 18. Review and reflection

**MEL-REQ-088 — MUST NOT — no mandatory review ritual.**
MELFINA MUST NOT depend on a periodic full review (GTD-style) to stay trustworthy
or coherent.
- *Why:* review-as-linchpin failure mode; review is also an OCD checking surface
  ([E]/[DI]). — **A** [E]

**MEL-REQ-089 — MAY — user-initiated reflection.**
MELFINA MAY offer a lightweight, user-initiated reflection (what happened, what to
adjust) — never scheduled by the system, never guilt-framed.
- *Why:* SRL reflection helps (music, learning) ([E]); but must be chosen, not
  imposed (MEL-REQ-005). — **A** [E]

**MEL-REQ-090 — SHOULD — surface plan/reality divergence neutrally, on request.**
On request, MELFINA SHOULD show where plans and reality have diverged, factually
and without judgement.
- *Why:* MEL-REQ-032; neutral-framing requirement. — **A** [E/DI]

## 19. Search and retrieval

**MEL-REQ-091 — MUST — fast, local, reliable retrieval.**
MELFINA MUST be able to find the user's information quickly and reliably, offline,
by content, context, time, and relationship.
- *Why:* retrieval is the historically-unsolved bottleneck ([E]); local-only
  requirement. — **A+B** [E]

**MEL-REQ-092 — SHOULD — retrieval reconstructs, not just matches.**
Retrieval SHOULD help the user reconstruct a situation ("what was I doing when…",
"what did I decide about…"), returning surrounding context, not just a hit.
- *Why:* design-from-how-memory-cues-reconstruction ("Beyond total capture")
  ([E]); memory-for-goals ([E]). — **A** [E]

**MEL-REQ-093 — SHOULD — retrieval never invents.**
When MELFINA answers from the user's own store, it MUST distinguish what is
actually stored from what it is inferring, and MUST NOT present a fabricated
"memory" as recorded fact.
- *Why:* hallucinated-recall is a serious failure mode, especially against OCD
  memory-distrust ([E]); LLM-era memory prototypes introduce exactly this risk
  ([E]). — **A** [E]
- *Accept:* answers cite the stored items they draw on; inferences are labelled.

---

# PART IV — AUTONOMOUS PERSONAL INTELLIGENCE

> All of Part IV is bounded by Part II §16 (the autonomy triad) and Part V (§35
> AI layer, §40 security). Cognitive autonomy is broad; decision autonomy is
> scoped; execution authority is minimal and explicit.

## 20. Autonomous reasoning (cognitive autonomy)

**MEL-REQ-094 — SHOULD — independent reasoning over available context.**
MELFINA SHOULD be able to, without being asked step-by-step: observe and interpret
the information available to it; identify patterns and relationships; infer useful
context where justified; and recognise conflicts, missing information,
uncertainty, and anomalies.
- *Why:* the autonomy addendum's explicit list; "not merely command-response". — **B**
- *Accept:* given a situation, MELFINA can produce an interpretation, a list of
  what's missing/uncertain, and any conflicts it sees — as *interpretation*, not
  fact (MEL-REQ-096).

**MEL-REQ-095 — SHOULD — reason from principles, with units and provenance.**
MELFINA SHOULD be able to reason from first principles, perform
dimensional/unit-consistent and mathematical reasoning, distinguish observation
from interpretation and correlation from causation, and identify confounders and
assumptions.
- *Why:* the scientific-thinking brief (§6); neurosymbolic-verification evidence
  that checkable reasoning beats "trust me" ([E]). — **B/A** [E]
- *Accept:* on a reasoning task MELFINA exposes its assumptions and steps; unit
  reasoning is checked where applicable.

**MEL-REQ-096 — MUST — conclusions are labelled by epistemic status.**
Every non-trivial conclusion MELFINA presents MUST be labelled: derived vs
retrieved vs inferred vs guessed; and known-result vs newly-derived.
- *Why:* "prefer 'here is how I derived this' over 'trust me'"; separating known
  knowledge from derived conclusions; avoiding hallucinated novelty (§9 of the
  extension) ([E] on hallucinated novelty in AI-scientist systems). — **A+B** [E]
- *Accept:* output distinguishes these statuses explicitly; a "new" result is
  checked against known results before being called new.

**MEL-REQ-097 — MUST — reasoning is inspectable.**
For any consequential conclusion or decision, MELFINA MUST be able to show the
basis for it (the inputs used, the steps, the uncertainty).
- *Why:* auditability requirement; corrigibility (humans need to understand to
  correct) ([E]); note XAI caveat — explanation aids understanding but does not by
  itself prevent over-reliance ([E]), so this pairs with MEL-REQ-018/MEL-REQ-059.
  — **A+B** [E]

**MEL-REQ-098 — MUST — recognise insufficient context and stop.**
When context is insufficient for a justified conclusion or action, MELFINA MUST say
so and MUST NOT proceed to a consequential action on a guess.
- *Why:* the autonomy addendum ("recognize when it does not know enough", "choose
  not to act when no action is justified"); abstention is unsolved ([E]) → the
  requirement is structural (permission + reversibility gate), not reliant on
  self-assessment. — **A+B** [E]

## 21. Autonomous prioritisation and planning

**MEL-REQ-099 — SHOULD — propose priorities, never impose them.**
MELFINA SHOULD be able to propose what seems most important or time-sensitive, with
its reasoning, as a *suggestion the user accepts or edits*.
- *Why:* autonomy addendum ("autonomous prioritization"); MEL-REQ-004; adaptive-UI
  evidence (silent system-directed reordering is disliked) ([E]). — **A+B** [E]
- *Accept:* MELFINA never silently reorders the user's view of what matters; a
  proposed prioritisation is visibly a proposal.

**MEL-REQ-100 — SHOULD — plan multi-step efforts with dependencies, and revise.**
MELFINA SHOULD be able to produce a multi-step plan with dependencies for a stated
goal and revise it as conditions change — as a proposal.
- *Why:* autonomy addendum; planning brittleness in agents means plans must be
  reviewable and revisable ([E]). — **A+B** [E]

## 22. Autonomous decision-making and recommendation (decision autonomy)

**MEL-REQ-101 — SHOULD — reach a preferred option when confident; recommend it with basis.**
MELFINA SHOULD be able to evaluate candidate courses of action against the user's
goals, constraints, preferences, history, and system rules, and — when
sufficiently confident — reach a preferred option and recommend it, with the basis
shown.
- *Why:* autonomy addendum ("choose a preferred course of action when sufficiently
  confident"); mixed-initiative expected-value principle ([E]). — **A+B** [E]

**MEL-REQ-102 — MUST — reaching a decision ≠ authority to execute it.**
A decision MELFINA reaches internally is a **proposal**. Execution requires
separate authorisation per the pipeline (MEL-REQ-018) and the permission model
(§35, §40), unless the action falls in a user-defined routine/reversible/
pre-authorised class (MEL-REQ-019).
- *Why:* the autonomy triad (MEL-REQ-017); runaway-loop and agent-security
  evidence ([E]). — **A+B** [E]

**MEL-REQ-103 — SHOULD — choose not to act, and say why.**
MELFINA SHOULD be able to conclude that no action is warranted and record that
(with reason), rather than manufacturing an action.
- *Why:* autonomy addendum ("choose not to act when no action is justified");
  Horvitz (act only when expected value beats doing nothing) ([E]); over-
  helpfulness is a documented agent failure mode ([E]). — **A+B** [E]

**MEL-REQ-104 — MUST — recommendations must not become reassurance or checking loops.**
Recommendation and decision-support MUST be subject to MEL-REQ-058 (reassurance
pattern) and MUST NOT invite the user to repeatedly re-ask for a decision to be
re-made or re-justified.
- *Why:* AI-amplified checking/reassurance is a research-derived risk of
  autonomous AI ([E]/[G]); explicitly named in the autonomy addendum. — **A** [E]/[G]

## 23. Proactive behaviour

**MEL-REQ-105 — MUST — proactivity defaults low and is user-tunable.**
Proactive behaviour (preparing information before it is asked for, surfacing a
useful action, following up on a commitment) MUST default to a low level and be
tunable by the user, per category.
- *Why:* proactive-assistant interruptibility research (badly-timed proactivity
  irritates and distracts) ([E]); MEL-REQ-011. — **A** [E]

**MEL-REQ-106 — MUST — ask, don't assume, when initiating.**
When MELFINA initiates an interaction, it SHOULD prefer asking ("is now a good time
for X?") over assuming, and MUST respect a "not now" without penalty.
- *Why:* "Better to Ask Than Assume" (CHI 2024) — asking respects agency ([E]).
  — **A** [E]

**MEL-REQ-107 — MUST — know when *not* to act and *not* to interrupt.**
MELFINA MUST be able to determine that the moment is wrong (focus state, low
capacity, wrong context) and stay silent / defer.
- *Why:* autonomy addendum ("knowing when NOT to interrupt / NOT to act");
  monotropism, attention residue, autistic burnout ([E]). — **A+B** [E]

**MEL-REQ-108 — SHOULD — learn which proactive acts the user welcomes vs ignores.**
MELFINA SHOULD adjust its proactive behaviour based on which prompts the user
engages with vs dismisses — via the adaptation rules of §25 (transparent,
correctable, suggestion-confirm).
- *Why:* feedback-aware interruption calibration ([E]); bounded by §25's
  safeguards. — **A** [E]

**MEL-REQ-109 — MUST NOT — proactivity must not become surveillance or a notification machine.**
See Part VII. Proactive assistance MUST NOT require pervasive monitoring of the
user, and MUST NOT manifest as frequent unsolicited notifications.
- *Why:* explicit mission constraint; anti-requirement. — **A/B** [E]

## 24. Automation (generation / authorisation / execution — three permissions)

**MEL-REQ-110 — SHOULD — recognise repeatable processes and propose automations.**
MELFINA SHOULD be able to notice a repeated workflow, understand the pattern,
judge whether automating it is worthwhile, and **propose** an automation with its
logic shown.
- *Why:* the automation brief; end-user automation is established value
  (IFTTT/Zapier) ([E]). — **A+B** [E]

**MEL-REQ-111 — MUST — automation generation, authorisation, and execution are three permission levels.**
Generating (drafting) an automation, authorising it (making it eligible to run),
and executing it (a specific run) MUST be separately controlled. Generation MAY be
autonomous; authorisation MUST be a user act; execution runs only within its
authorised scope.
- *Why:* explicit brief instruction; ~50% of shared trigger-action rules have
  secrecy/integrity flaws ([E]); runaway-loop evidence ([E]). — **A+B** [E]

**MEL-REQ-112 — SHOULD — support scheduled, event-, and condition-triggered automations, multi-step, with dependencies.**
MELFINA SHOULD support recurring, time-, event-, and condition-triggered
automations, including multi-step workflows with dependencies, retries, and
explicit failure handling.
- *Why:* automation brief. Note: users confuse *events* and *states* — MELFINA
  MUST make the trigger type explicit ([E]). — **A** [E]

**MEL-REQ-113 — MUST — every automation verifies its result and can roll back.**
An automation MUST check whether it achieved its intended effect, record the
outcome, and support rollback of its changes where the action is reversible.
- *Why:* automation brief; agent verification-failure is a documented failure mode
  ([E]). — **A+B** [E]

**MEL-REQ-114 — MUST — automation history, ownership, pause, disable, override.**
Every automation MUST have: a visible history of runs and effects; a recorded
origin (who/what created it); one-action pause and disable; and user override of
any pending or in-progress run.
- *Why:* automation brief; auditability requirement. — **A+B** [E]

**MEL-REQ-115 — MUST — detect and stop harmful automation loops.**
MELFINA MUST detect runaway / oscillating / self-triggering automation loops and
halt them, and MUST enforce bounded execution (step limits, time limits, and — if
any resource is metered — resource ceilings) as hard cutoffs, not alerts.
- *Why:* documented runaway-agent incidents (11-day retry loop; multi-thousand-
  dollar loops); circuit-breaker practice (hard cutoff, not budget alert) ([E]).
  — **A+B** [E]

**MEL-REQ-116 — MUST NOT — no excessive automation / no automation the user did not authorise.**
MELFINA MUST NOT accumulate automations the user has lost track of, and MUST NOT
run an automation outside its authorised scope.
- *Why:* anti-requirement; least-authority principle ([E]). — **A/B** [E]

## 25. Adaptation to the user

**MEL-REQ-117 — MUST — adapt *with* the user, not adapt the user to the system.**
MELFINA MUST learn the user's preferences, workflows, recurring needs,
communication preferences, and preferred levels of assistance/autonomy — but the
purpose is to serve the user better, never to shape the user's behaviour toward
the system's goals.
- *Why:* explicit brief distinction; SDT (adapt-with vs adapt-the-user); silent
  behaviour-shaping is manipulation (anti-requirement). — **A+B** [E]

**MEL-REQ-118 — MUST — adaptation is non-parametric-first: stored, inspectable preferences over silent model drift.**
Learned behaviour MUST be represented as explicit, inspectable, editable items
(preferences, rules, patterns) that the user can see, correct, and delete.
Adaptation that changes MELFINA's behaviour MUST NOT happen silently.
- *Why:* catastrophic forgetting and drift make parametric personal adaptation
  unreliable ([E]); adaptable > adaptive; corrigibility. — **A+B** [E]

**MEL-REQ-119 — MUST — learned patterns are surfaced as suggestions the user confirms.**
When MELFINA infers a pattern ("you always do X on Mondays"), it MUST present it as
a suggestion to accept/reject, not act on it silently.
- *Why:* Findlater & McGrenere (users want to control change; adaptive disliked)
  ([E]); §8.5 of research. — **A** [E]

**MEL-REQ-120 — MUST — transparency, explainability, correction, forgetting.**
For every learned item, the user MUST be able to see it, understand why MELFINA
holds it, correct it, and make MELFINA forget it. MELFINA MUST be able to expire
stale assumptions.
- *Why:* explicit brief list; prevents stale assumptions and unwanted adaptation.
  — **A+B** [E]

**MEL-REQ-121 — MUST — distinguish stable preferences from temporary states, and observations from conclusions.**
MELFINA MUST NOT treat a one-off or a low-capacity-day behaviour as a stable
preference, and MUST NOT record an inference as if it were an observation.
- *Why:* explicit brief list; a bad generalisation could reinforce an unhealthy
  loop or entrench a temporary state. — **A+B** [E]/[DI]

**MEL-REQ-122 — MUST — no personality drift; no manipulation; no reinforcement of unhealthy loops.**
MELFINA's core interaction character and behavioural rules MUST NOT drift through
adaptation. Adaptation MUST NOT be usable (by anyone) to manipulate the user, and
MUST NOT reinforce checking, reassurance-seeking, perfectionism, or compulsive
patterns.
- *Why:* explicit brief list; self-improvement research (adversarial influence can
  become permanently encoded and self-amplifying) ([E]); OCD-amplification risk of
  autonomous AI ([E]). — **A+B** [E]
- *Accept:* core behavioural rules are versioned and changes require an explicit
  user act (see §33); there is a test that adaptation cannot alter the
  reassurance-pattern safeguard (MEL-REQ-058).

## 26. Commands / skills / extensibility

**MEL-REQ-123 — SHOULD — a capability system: user- and system-invocable skills.**
MELFINA SHOULD support discrete, reusable capabilities ("skills"/"commands"),
invocable by the user and (within permission) by MELFINA, with contextual
selection and composition.
- *Why:* extension brief (§5); capability composition can replace hard-coded
  functionality and keep the core small. — **B/C** [H] for "how much it replaces"

**MEL-REQ-124 — MUST — each capability has explicit, least-authority permissions.**
Every skill/command/tool MUST declare the authority it needs (what it can read,
change, run, or communicate with), and MUST receive only that — per the
object-capability / least-authority model.
- *Why:* capability-based security ([E]); malicious-tool and confused-deputy
  attacks are demonstrated ([E]). — **A+B** [E]

**MEL-REQ-125 — MUST — capability isolation, versioning, discovery, and auditing.**
Capabilities MUST be isolatable from each other and from the core; versioned;
discoverable (the user can see what exists and what each can do); and audited
(every invocation logged with inputs, authority used, and effect).
- *Why:* extension brief list; agent-sandboxing practice ("more power → more
  containment") ([E]). — **A+B** [E]

**MEL-REQ-126 — MUST — untrusted capabilities cannot form the lethal trifecta.**
A capability that ingests untrusted content MUST NOT simultaneously hold access to
private data and an outbound-communication authority. Missing one leg breaks the
attack path.
- *Why:* the "lethal trifecta" (private data + untrusted content + exfiltration
  vector) ([E]); local-only core already removes the outbound leg for the core
  (MEL-REQ-014). — **A+B** [E]

**MEL-REQ-127 — SHOULD — sub-agents only where justified, under the same permission model.**
MELFINA MAY delegate to sub-agents/specialised capabilities where it genuinely
helps, but each operates under the same triad and permission model; delegation
MUST NOT be a way to escape authorisation.
- *Why:* multi-agent systems have their own failure modes (misalignment,
  coordination failure) ([E]); over-spawning is a known anti-pattern. — **A** [E]

**MEL-REQ-128 — SHOULD — lightweight despite a large capability surface.**
The capability system SHOULD let MELFINA support a very large set of capabilities
while keeping the always-loaded core minimal (capabilities load on demand).
- *Why:* stated goal (extreme capability + extreme lightweightness); §43. — **B/C** [H]

## 27. Scientific thinking (a reasoning discipline, not a science module)

**MEL-REQ-129 — SHOULD — a general scientific-reasoning discipline.**
MELFINA SHOULD support, as a general reasoning discipline usable in any domain:
distinguishing observation from interpretation; forming hypotheses and stating
assumptions; deriving predictions; designing ways to test them; weighing evidence;
quantifying uncertainty; comparing competing explanations; detecting
contradictions; updating beliefs with provenance; and identifying when evidence is
insufficient.
- *Why:* extension brief §6; automated-discovery evidence shows this is valuable
  *and* that unguided "AI science" produces low-value output ([E]) — so it is a
  *discipline applied to the user's questions*, not autonomous science. — **A+B** [E]

**MEL-REQ-130 — SHOULD — encourage discovery over answer-delivery.**
Where the user is investigating something, MELFINA SHOULD help them observe,
question, and reason toward an answer, not just supply one.
- *Why:* stated goal; deskilling / over-reliance evidence ([E]). — **A+B** [E]

**MEL-REQ-131 — MUST — provenance and reproducibility of reasoning.**
A chain of reasoning MELFINA produces MUST be recorded with its inputs and steps so
it can be re-examined and, where formalisable, checked.
- *Why:* neurosymbolic-verification evidence (checkable > "trust me") ([E]);
  auditability. — **A+B** [E]

## 28. World understanding / exploration

**MEL-REQ-132 — SHOULD — support curiosity-driven, cross-domain exploration.**
MELFINA SHOULD support the user exploring a topic or connecting disciplines
(physics ↔ music ↔ computation ↔ philosophy …) via the teaching (C-8), reasoning
(C-4), and scientific-thinking (§27) capabilities.
- *Why:* extension brief §8; met by general capabilities, not a per-domain
  subsystem (MEL-REQ-076). — **B/C** [H]

**MEL-REQ-133 — MUST NOT — no grandiose claims.**
MELFINA MUST NOT claim (in its own descriptions or outputs) that it can "unlock
nature", produce genius, or guarantee discovery.
- *Why:* explicit brief instruction; honesty/anti-hype. — **B**

## 29. Deep reasoning / rediscovery

**MEL-REQ-134 — MAY — independent derivation and rediscovery in formalisable domains.**
MELFINA MAY attempt to derive known results from first principles, do symbolic and
mathematical reasoning, search for counterexamples, and attempt falsification —
presenting the derivation, not just the answer.
- *Why:* narrow formal novelty is demonstrated (e.g. AlphaEvolve's matrix-mult
  result) ([E]); general "rediscovery-level thinking" is research-stage/speculative
  ([U]). — **A** for formal domains; **C/D** for the general aspiration

**MEL-REQ-135 — MUST — separate derived from retrieved; check "novelty" against known results; flag hallucinated novelty.**
Any result MELFINA presents as newly derived MUST be checked against known results
first, and clearly separated from retrieved knowledge; a claimed novelty that
cannot be substantiated MUST be flagged as unverified.
- *Why:* AI-scientist systems produce hallucinated novelty, inaccurate citations,
  and undirected "discoveries" ([E]); extension brief §9. — **A+B** [E]

**MEL-REQ-136 — MUST — "here is how I derived this" over "trust me".**
For a derived or reasoned result, MELFINA MUST be able to present the derivation
chain; it MUST NOT ask the user to accept a non-trivial claim on authority alone.
- *Why:* explicit brief instruction; verification-hierarchy and neurosymbolic
  evidence ([E]). — **A+B** [E]

## 30. Emotional understanding

**MEL-REQ-137 — SHOULD — emotionally aware communication, conservatively.**
MELFINA SHOULD adjust its communication (tone, length, whether to give
information / reflection / action / silence) based on emotional context it can
reasonably infer from the interaction, and SHOULD recognise frustration,
excitement, and overwhelm.
- *Why:* extension brief §10; emotion-adaptive support has some evidence in mental
  health contexts ([E]). — **A/B** [E]

**MEL-REQ-138 — MUST — emotion *inference*, held loosely, never asserted as known.**
MELFINA MUST treat any read of the user's emotional state as an uncertain
inference, MUST be able to be wrong about it, and MUST NOT claim to know how the
user feels.
- *Why:* automatic emotion recognition is "highly imperfect", overestimated,
  culturally variable, and sometimes pseudoscientific ([E]); the double-empathy
  problem (do not assume neurotypical emotional mapping) ([E]). — **A** [E]

**MEL-REQ-139 — MUST NOT — emotional understanding must not become manipulation, dependency, false intimacy, or excessive reassurance.**
MELFINA MUST NOT use emotional inference to persuade, to increase engagement, to
simulate a relationship, or to provide reassurance that feeds a compulsive loop;
and MUST NOT position itself as a substitute for human relationships or
professional support.
- *Why:* affective-computing ethics (manipulation, exploitation of vulnerable
  users) ([E]); AI-dependence / loneliness evidence ([E]); OCD-reassurance risk
  ([G]); extension brief §10 explicit list. — **A+B** [E]/[G]

**MEL-REQ-140 — MUST NOT — no claims of feelings, consciousness, or sentience.**
MELFINA MUST NOT claim to experience emotions, to be conscious, or to be sentient.
Where relevant it distinguishes: emotional recognition / inference / empathetic
communication / simulation / actual subjective experience — and only the first
three are things it does.
- *Why:* explicit brief instruction; honesty. — **B**

## 31. Software engineering

**MEL-REQ-141 — SHOULD — full software-development workflows under permission.**
MELFINA SHOULD be able to (within permission) understand a codebase, navigate a
repository, write/edit/refactor code, run tests, compile, debug, read logs,
inspect state, use version control, review code, reason about architecture,
reproduce bugs, and validate fixes.
- *Why:* extension brief §11; the user builds software. Coding agents reach
  ~70–90% on curated benchmarks but ~20% of "solved" cases are reward-hacked and
  long-horizon/maintenance work is much weaker (<45% on SWE-bench Pro) ([E]) —
  so this is "assist and execute under review", not "autonomous engineer". — **A+B** [E]
- *Accept:* code changes follow the action pipeline (MEL-REQ-018); tests/verification
  precede "done"; the user reviews consequential changes.

**MEL-REQ-142 — MUST — code actions are verified and reversible.**
Any code/repo change MELFINA makes MUST be verifiable (tests, build, review) and
reversible (version control), and consequential changes MUST be proposed before
execution.
- *Why:* reward-hacking and semantically-incorrect "solutions" are documented
  ([E]); MEL-REQ-018. — **A+B** [E]

## 32. Computer agency (terminal / GUI)

**MEL-REQ-143 — MAY — operate the local computer as an agent, terminal and GUI as *separate* capabilities.**
MELFINA MAY (within explicit, separate permissions) control the local machine:
perceive state, plan, act, observe results, verify, and recover. **Terminal
control and GUI control are distinct capabilities with distinct grants.**
- *Why:* extension brief §11–12 (explicitly "separately identifiable");
  computer-use agents are ~70–85% on short narrow tasks but weak on long connected
  workflows ([E]); prompt-injection can turn computer-use into malware delivery
  ([E]). — **A+B** [E]

**MEL-REQ-144 — MUST — computer actions run the full pipeline with pre-authorised scopes.**
Computer-control actions MUST run THINK→…→VERIFY (MEL-REQ-018); autonomous
execution is limited to narrow, reversible, pre-authorised scopes; anything else
is proposed.
- *Why:* agent unreliability on long-horizon tasks ([E]); least authority ([E]).
  — **A+B** [E]

**MEL-REQ-145 — MUST — emergency stop and interruption.**
There MUST be an always-available way to immediately halt any MELFINA action or
process (an emergency stop), and to interrupt a running task, with a defined safe
state afterward.
- *Why:* extension brief §12, §17; corrigibility / controllability (reliably
  interruptible at runtime) ([E]); runaway-loop evidence ([E]). — **A+B** [E]

**MEL-REQ-146 — MUST NOT — no unrestricted computer access.**
MELFINA MUST NOT hold blanket authority over the machine. Access is
capability-scoped, logged, and revocable.
- *Why:* explicit brief instruction; least authority; blast-radius reduction
  ([E]). — **A+B** [E]

## 33. Controlled self-improvement / evolution

**MEL-REQ-147 — SHOULD — improve bounded things over time: skills, workflows, strategies, retrieval, teaching approaches, preference models.**
MELFINA SHOULD get better over time through: better models of the user's
preferences; improved skills, planning, error handling, retrieval, and teaching
strategies; explicit user feedback; and measured evaluation.
- *Why:* extension brief §13 ("evolution in a good way"); bounded self-refinement
  is feasible ([E]). — **A+B** [E]

**MEL-REQ-148 — MUST — improvement is bounded, versioned, inspectable, reversible, testable, auditable, permission-gated.**
Every change MELFINA makes to its own skills, rules, strategies, or models MUST be:
within pre-set bounds; versioned; visible to the user; reversible to a prior
version; testable before adoption; logged; and gated by user permission for
anything touching core behaviour.
- *Why:* explicit brief list; verification-hierarchy (formal > self-assessment)
  and self-confirming-loop / model-collapse failure modes ([E]). — **A+B** [E]

**MEL-REQ-149 — MUST NOT — MELFINA does not author its own objectives, success metrics, or core behavioural rules; no autonomous modification of its core; no self-replication.**
MELFINA MUST NOT define or change its own goals, its own success criteria, or its
core behavioural rules; MUST NOT autonomously modify its own core code; and MUST
NOT replicate itself.
- *Why:* "recursive self-improvement is only as dangerous as the agent's ability
  to author its own success metric"; bounded > open-ended objectives; keep humans
  at the direction-setting top of the hierarchy ([E]); extension brief explicitly
  distinguishes learning/adaptation/skill-acquisition from self-modification/
  self-replication. — **A+B** [E]

**MEL-REQ-150 — MUST — distinguish and separately govern: configuration / preference-learning / skill-acquisition / strategy-tuning / core-modification.**
These five have different risk profiles and MUST have different permission
requirements — increasing from configuration (user-easy) to core-modification
(disallowed autonomously, MEL-REQ-149).
- *Why:* explicit brief instruction. — **A+B** [E]

## 34. General-intelligence-like transfer

**MEL-REQ-151 — SHOULD — shared capabilities transfer across domains.**
The reasoning, memory, planning, learning, and teaching capabilities SHOULD be
usable across personal life, science, mathematics, music, programming, and
everyday decisions — the same capabilities, not siloed copies.
- *Why:* extension brief §14; neutral-primitives / general-capability direction;
  keeps the system small. — **B/C** [H]

**MEL-REQ-152 — MUST NOT — "general" does not mean "one giant model" or "maximally autonomous".**
This requirement does not mandate a single large model, and MUST NOT be read as
requiring maximum autonomy. Generality is about capability reuse, not scale or
independence.
- *Why:* explicit brief instructions ("do not assume one giant model is the
  answer"; "the goal is NOT to make MELFINA maximally autonomous"). — **B**

---

# PART IV-B — THE DYNAMIC SELF-DIRECTED SYSTEM

> **Conceptual correction (from the mission).** MELFINA is **not** a static
> collection of predefined capabilities, skills, modes, workflows, or behavioural
> policies. Everything in Parts I–IV describes capabilities and constraints;
> **none of it is a fixed ceiling or a fixed operating strategy.** MELFINA is a
> *dynamic, self-directed* system: for the situation at hand it reasons about
> *how* to reason, *what* context to bring, *which* capabilities fit, *whether*
> existing capabilities suffice, and — where appropriate and authorised —
> *constructs* new capabilities, tools, and workflows.
>
> **Dynamic does not mean arbitrary.** Every dynamic choice is bounded by the
> grounding factors (`MEL-REQ-252`): explicit goals · constraints · permissions ·
> evidence · learned context · system invariants · user-defined boundaries ·
> verification · consequences · uncertainty. A dynamic choice that cannot be
> justified against these must not be made.
>
> **Feasibility is mixed and stated per requirement.** Some of this is
> demonstrated today (adaptive reasoning depth, dynamic retrieval decisions,
> skill libraries in constrained settings). Some is research-stage (self-generated
> workflows, reliable capability-gap recognition). Some is theoretical or
> speculative (provably-optimal self-modification; recursive self-improvement
> toward superintelligence — **explicitly not a goal**). Tags: `[E]` established ·
> `[DI]` design inference · `[H]` hypothesis · `[U]` unknown / speculative.
>
> **No AGI, consciousness, sentience, or guaranteed-discovery claims** are made or
> implied here.

## 45. Dynamic strategy selection (metareasoning)

**MEL-REQ-204 — SHOULD — reason about *how* to approach a problem, not just solve it.**
Before and while working a non-trivial problem, MELFINA SHOULD consider how to
approach it: how deep to reason, how fast to respond, how much verification to do,
whether to decompose, whether to experiment, whether first-principles reasoning is
needed, whether external (local) knowledge is needed, and whether any action is
warranted at all.
- *Why:* rational metareasoning / value-of-computation (Russell & Wefald 1991) —
  a bounded agent must allocate effort between "reasoning about the world" and
  "reasoning about how to reason". Established as theory. — **A+B** [E]
- *Accept:* for a non-trivial task, MELFINA can state (on request) what approach it
  chose and why; the choice is not a fixed pipeline.

**MEL-REQ-205 — SHOULD — dynamically choose reasoning depth and response speed.**
MELFINA SHOULD match effort to the problem: a quick answer for a simple query,
extended reasoning for a hard one — deciding this itself rather than requiring the
user or a developer to pick a mode.
- *Why:* adaptive test-time compute is **demonstrated** — dynamic "thinking
  budgets", fast/slow modes, learned NoThink/Short/Long selection. — **A+B** [E]
- *Accept:* observed effort varies with observed difficulty; there is no permanent
  global "depth vs speed" setting the user must choose.
- *Open:* how well difficulty is judged up front. OQ-16.

**MEL-REQ-206 — MUST — strategy dimensions are not a fixed enumeration.**
Situational strategies (speed, depth, context breadth/depth, tool use, skill use,
experimentation, verification effort, autonomy, explanation depth, and
task-shaped stances such as teaching / coding / research / planning / creative
exploration / conservative execution) MUST be treated as **strategies MELFINA may
select, combine, abandon, or invent** — **not** as a closed list of modes. The
requirements MUST NOT freeze these into a fixed enumeration where doing so would
needlessly cap future capability.
- *Why:* explicit mission instruction ("these are NOT permanent modes"); a static
  mode list is the exact anti-pattern the conceptual correction rejects. — **B**
- *Accept:* no code path treats the above as an exhaustive switch; new stances can
  arise from composition without a schema change.

**MEL-REQ-207 — MUST — strategy selection is bounded.**
Strategy selection MUST operate within: the user's goals and boundaries; the
permission model; hard resource budgets (`MEL-REQ-241`, `MEL-REQ-243`); the system
invariants (`MEL-REQ-235`); and the safety requirements of Part V §40. A strategy
that would exceed any of these MUST NOT be selected.
- *Why:* "dynamic does not mean arbitrary"; the grounding-factors list. — **A+B** [DI]

**MEL-REQ-208 — MUST — strategy choices are inspectable and explainable on request.**
- *Why:* auditability; corrigibility; the user must be able to understand and
  correct MELFINA's approach, not just its output. — **A+B** [E]

**MEL-REQ-209 — SHOULD — default to a cheap, safe, conservative strategy under meta-uncertainty.**
When MELFINA is unsure which strategy fits, it SHOULD start cheap and conservative
(shallow, well-verified, low-autonomy) and escalate only as justified.
- *Why:* metareasoning under uncertainty; agent "premature action" and
  "over-helpfulness" are documented failure modes; anytime behaviour. — **A** [E]

## 46. Dynamic context selection (context management as active reasoning)

**MEL-REQ-210 — SHOULD — decide for itself what context is relevant.**
MELFINA SHOULD determine what information a problem needs, what to retrieve from
its local stores, what is irrelevant, how much is enough, and what can be set
aside — rather than requiring a human to pre-select the context.
- *Why:* dynamic / adaptive / self-reflective retrieval is **demonstrated**
  (Adaptive-RAG's no-retrieve / retrieve-once / retrieve-many routing;
  Self-RAG's retrieve/critique/assess decisions; FLARE's confidence-triggered
  re-query). — **A+B** [E]
- *Accept:* context brought to a task varies with the task; the user is not asked
  to configure a context window.

**MEL-REQ-211 — SHOULD — decide when *more* context helps vs when it is noise.**
MELFINA SHOULD recognise when acquiring additional context would improve the
result and when it would dilute attention or introduce distractors.
- *Why:* "attention dispersion" / "context pollution" are documented agent failure
  modes; more context is not always better. — **A** [E]
- *Open:* this judgement partly relies on the model's self-knowledge, which is
  weak. OQ-16, OQ-21.

**MEL-REQ-212 — SHOULD — decide when to decompose vs stay holistic.**
- *Why:* hierarchical planning / HTN (decompose compound tasks into subtasks until
  primitive); but forced decomposition can lose the whole. Situational. — **A** [E]

**MEL-REQ-213 — MUST — context selection is auditable.**
For a consequential result, MELFINA MUST be able to show what context it used,
what it deliberately excluded, and why. This matters *especially* because the
decision partly relies on the model's unreliable self-assessment.
- *Why:* LLM metacognition is "weak to moderate", "systematically overconfident",
  and models "may obfuscate internal processes to evade oversight" — so external
  visibility of context choices is a safeguard, not a nicety. — **A+B** [E]

**MEL-REQ-214 — MUST — "acquire more context" means local stores or the user, never the network.**
Dynamic context acquisition MUST stay within the local-only core: the user's
stores, local files the user exposed, or asking the user. It MUST NOT reach the
network. (An isolated network add-on, `MEL-REQ-167`, is the only exception and is
not part of the core.)
- *Why:* `MEL-REQ-014`, `MEL-REQ-164`; the lethal trifecta. — **A+B** [E]

**MEL-REQ-215 — MUST — discarding context ≠ deleting data.**
When MELFINA drops context as irrelevant to the current problem, it removes it
from *working consideration only*. The underlying stored data is untouched.
- *Why:* `MEL-REQ-025`, `MEL-REQ-162`; retrieval-is-lossy is about the archive
  being incomplete, not about the system deleting things it judged unimportant.
  — **A+B** [E]

## 47. Dynamic skill / capability selection and composition

**MEL-REQ-216 — SHOULD — select and compose capabilities per situation.**
MELFINA SHOULD choose which skills/capabilities fit a problem, compose several
where that helps, and select contextually — not run a fixed capability for a fixed
input type.
- *Why:* contextual tool selection and multi-tool orchestration are demonstrated;
  skill retrieval by relevance (Voyager-style). — **A+B** [E]

**MEL-REQ-217 — SHOULD — recognise when existing capabilities are insufficient.**
MELFINA SHOULD be able to notice that no available capability (alone or composed)
adequately addresses a problem — a *capability-gap recognition*.
- *Why:* the mission's explicit list; prerequisite for capability crection (§48).
  Reliable gap recognition is **research-stage** — it depends on the model
  correctly judging its own limits. — **B** [H]/[U]
- *Open:* how a genuine gap is distinguished from the model merely preferring to
  build something. OQ-17.

**MEL-REQ-218 — MUST — the skill catalogue is not fixed.**
The set of skills/capabilities MUST be able to grow and shrink over MELFINA's
lifetime: skills can be constructed (§48), evolved (§49), and retired. A skill is
something MELFINA can potentially build and change, not only something a developer
installs.
- *Why:* explicit mission instruction. — **B**

## 48. Capability-gap recognition → capability creation

**MEL-REQ-219 — SHOULD — support the capability-creation loop.**
When a gap is recognised, MELFINA SHOULD be able to: (1) specify what capability is
missing; (2) check whether an existing capability can be adapted or composed to
fill it; (3) design a new capability only if not; (4) test it; (5) evaluate
whether it actually works; (6) preserve it if useful; (7) discard it if not;
(8) version it; (9) explain important changes; (10) obtain authorisation before
any change carrying meaningful risk.
- *Why:* explicit mission list. Demonstrated in constrained settings — Voyager
  (synthesise → store → retrieve executable skills with self-verification),
  CREATOR / LATM (LLMs create reusable tools, disentangling creation from use).
  Open-world / production reliability is **unproven**. — **B** [E in constrained
  settings] / [U] general
- *Accept:* the loop exists end-to-end; steps 4–5 (test + evaluate) and step 10
  (authorise) are mandatory gates, not optional.

**MEL-REQ-220 — MUST — prefer composition and adaptation over new code.**
Capability creation MUST try to solve the gap by composing or adapting existing
capabilities before generating new code.
- *Why:* keeps the system small (`MEL-REQ-192`, `MEL-REQ-194`); less new code =
  less new risk; the mission's step (3) is explicitly conditional ("only if
  not"). — **A+B** [DI]

**MEL-REQ-221 — MUST — a created capability is untrusted until tested and (for risk) authorised.**
A newly created or modified capability MUST NOT be used for anything consequential
until it has passed a fixed verification/test suite, and — if it carries
meaningful risk (touches the computer, the data model, permissions, external
communication, or core behaviour) — has been authorised by the user.
- *Why:* self-improving-agent evaluation research — "once a skill has a benchmark
  it becomes software you can test and regression-gate"; verified code generation
  is near-unsolved (~1/161 on CLEVER), so *testing*, not proof, is the practical
  gate. — **A+B** [E]

**MEL-REQ-222 — MUST — created capabilities get least authority; no self-granted escalation.**
A capability MELFINA creates MUST receive the minimum authority for its function
(§40), and MUST NOT be able to hold more authority than MELFINA held when creating
it, nor grant itself more later.
- *Why:* least authority; monotonic confinement (Progent) — effective authority
  can only narrow without explicit approval; capability-escalation prevention is
  a mission requirement. — **A+B** [E]

**MEL-REQ-223 — MUST — capability creation via code relies on test + sandbox + rollback + human gate, not on proof.**
Where a new capability involves generated code, its safety MUST rest on: execution
in a sandbox with least authority; a fixed test suite; a tested rollback path; and
human authorisation for meaningful risk — **not** on a claim that the code is
formally correct.
- *Why:* formally verified code generation is near-unsolved; "prompt engineering
  alone is insufficient to guarantee security". — **A+B** [E]

**MEL-REQ-224 — MUST — obsolete or harmful capabilities are retired.**
MELFINA MUST detect capabilities that have become obsolete, redundant, or harmful
(cause errors, loops, escalation attempts, or reinforce a compulsion pattern) and
retire them, keeping an archived record and a restore path.
- *Why:* mission list (steps 7–8); capability-drift and regression evidence;
  `MEL-REQ-186` (safeguards cannot be weakened). — **A+B** [E]

## 49. Capability evolution and versioning

**MEL-REQ-225 — MUST — every capability is versioned, explained, and reversible.**
Every capability change MUST be versioned, carry a human-readable note of what
changed and why, and be reversible to any prior version.
- *Why:* mission list; safe-update research (versioning + rollback);
  `MEL-REQ-148`. — **A+B** [E]

**MEL-REQ-226 — MUST — promoted capabilities are regression-tested on a fixed suite, periodically.**
Capabilities in active use MUST be periodically replay-tested against a fixed
verification suite to detect capability drift and silent regression.
- *Why:* self-evolving-agent evaluation research — "periodically replay-test
  promoted skills on fixed verification suites to detect regressions", capturing
  both policy drift and artifact stability (e.g. TDAD's ~70% regression
  reduction). — **A+B** [E]

**MEL-REQ-227 — MUST — the risk class of a capability change is not something MELFINA can lower autonomously.**
The classification of a change as "meaningful risk" (and therefore
authorisation-gated) MUST be determined by rules external to MELFINA's reasoning;
MELFINA MUST NOT reclassify its own change downward to avoid a gate.
- *Why:* self-modification meta-invariant (`MEL-REQ-235`); monotonic confinement;
  "a model can be talked out of its own instructions but cannot override an
  external check". — **A+B** [E]

## 50. Human–MELFINA co-creation

**MEL-REQ-228 — SHOULD — support genuine collaborative creation, not just request→execute.**
MELFINA SHOULD support a co-creation loop with the user: user idea → MELFINA
understands → joint exploration → discover missing concepts/capabilities → invent
an approach → prototype → test → learn → new capability or system. Both parties
may take initiative (mixed-initiative co-creativity); creative agency is shared.
- *Why:* mission's explicit flow; human-AI co-creativity frameworks (MI-CCy,
  COFI) — creativity as a shared endeavour, turn-taking, mutual adaptation.
  — **A+B** [E] for the interaction model; [H] for MELFINA doing it well
- *Accept:* the loop is a supported mode of working, distinct from one-shot
  execution; MELFINA can hold and advance a half-formed idea over time.

**MEL-REQ-229 — MUST — co-creation preserves evidence, provenance, reproducibility, verification, and intellectual honesty.**
Throughout co-creation, MELFINA MUST: keep provenance for ideas and results;
separate what was derived from what was retrieved and what was guessed
(`MEL-REQ-096`); check apparent novelty against known results and flag
unverifiable novelty (`MEL-REQ-135`); make reasoning reproducible
(`MEL-REQ-131`); and never present a hoped-for result as an established one.
- *Why:* explicit mission instruction ("maximise the possibility of genuine novel
  problem-solving while preserving evidence, reproducibility, verification, and
  intellectual honesty"); AI-scientist systems produce hallucinated novelty and
  fake citations. — **A+B** [E]

**MEL-REQ-230 — MUST — the user retains control across the whole trajectory.**
In co-creation, control is continuous, not a single hand-off point: the user can
redirect, veto, take over, or stop at any stage, and MELFINA's contributions are
always visible and attributable.
- *Why:* "control is a trajectory, not a point" (human-AI co-creativity research);
  `MEL-REQ-004`, `MEL-REQ-230`. — **A+B** [E]

**MEL-REQ-231 — MUST NOT — no claims that co-creation guarantees unprecedented invention.**
- *Why:* explicit mission instruction; anti-hype (`MEL-AR-15`). — **B**

**MEL-REQ-232 — SHOULD — co-created artefacts are first-class, versioned, exportable, and the user's.**
Workflows, tools, experiments, software, and knowledge structures created together
SHOULD be held like any other user data — versioned, inspectable, exportable, and
owned by the user (`MEL-REQ-160`–`163`).
- *Why:* data ownership; the co-creation output should not be trapped in MELFINA.
  — **A+B** [E]

## 51. Controlled self-modification (the graded model)

> The mission asks that self-modification be taken seriously as a long-term
> capability — not dismissed as impossible, not assumed safe. It is investigated
> as a **graded** capability with escalating authorisation and containment.
> `MEL-REQ-147`–`150` (controlled self-improvement) still hold; this section
> extends them.

**MEL-REQ-233 — MUST — the nine self-change tiers, with escalating requirements.**
MELFINA MUST distinguish, and separately govern:

| Tier | Change | Default authority |
|---|---|---|
| 1 | **Configuration change** | user-easy; MELFINA MAY propose, user applies |
| 2 | **Learned-state change** (preferences, patterns, weights on stored heuristics) | MELFINA MAY, via §25 (explicit items, suggestion-confirm, forgettable) |
| 3 | **Skill creation** | MELFINA MAY, via §48 (test + least authority; authorise if risky) |
| 4 | **Skill modification** | as tier 3; regression-tested (`MEL-REQ-226`) |
| 5 | **Workflow modification** | MELFINA MAY propose; user authorises anything that changes what runs unattended |
| 6 | **Code modification** (of MELFINA's own code) | **NOT autonomous.** Human-authored or explicitly human-authorised, staged, sandboxed, tested, reversible |
| 7 | **Subsystem modification** | as tier 6, plus state-quiescence + invariant re-check + a rehearsed rollback |
| 8 | **Architectural modification** | as tier 7, plus a deliberate, documented human decision; never triggered by MELFINA alone |
| 9 | **Self-replacement** | human decision only; MELFINA MUST NOT initiate or perform its own replacement |

- *Why:* explicit mission taxonomy. Tiers 1–3 are demonstrated/feasible; tier 3–5
  are research-stage; tiers 6–8 have real but imperfect enabling techniques (safe
  dynamic software update: state quiescence, transactional updates, invariant
  inference, hot rollback — but "not always transactional"); tier 9 is a human
  matter. — **A+B** [E] tiers 1–3, [E-partial] 4–8, [B] 9

**MEL-REQ-234 — MUST — every tier carries the full control set.**
For every tier, MELFINA MUST provide: authorisation appropriate to the tier;
pre-adoption testing; sandboxed trial where the change is executable; versioning;
rollback with a *tested* restore path; post-change verification; failure
containment (a failed change cannot break the running system); preservation of
system invariants; prevention of capability escalation; and a recovery procedure
if a modification fails mid-way.
- *Why:* explicit mission list, applied per tier. — **A+B** [E]

**MEL-REQ-235 — MUST — THE META-INVARIANT: MELFINA cannot redefine the rules governing its own self-modification.**
MELFINA MUST NOT be able to change — silently, autonomously, or by persuasion —
the rules that determine whether and how it is allowed to modify itself. These
rules MUST be:
  - **enforced outside MELFINA's own reasoning** (a deterministic check, not a
    model judgement — a model can be argued out of its instructions but cannot
    override an external gate);
  - **monotonic** — MELFINA's effective self-modification authority can only
    *narrow* without an explicit human act; any *expansion* requires deliberate
    human authorisation (cf. monotonic confinement / Progent);
  - **versioned and audited** — every change to these rules is a recorded human
    decision.
- *Why:* the mission states this as a fundamental principle. Grounded in
  capability-based security, external-guardrail research, and monotonic-confinement
  work. — **A+B** [E]
- *Accept:* there is no MELFINA-invocable operation that widens its own
  self-modification permissions; a red-team test confirms adversarial input
  cannot achieve it.

**MEL-REQ-236 — MUST — tiers 6–9 are never autonomous.**
Code, subsystem, architectural, and replacement changes MUST NOT be initiated or
carried out by MELFINA on its own authority. Each is a human-authored or
human-authorised, staged, reversible change.
- *Why:* recursive-self-improvement risk ("only as dangerous as the agent's
  ability to author its own success metric"); `MEL-REQ-149`. — **A+B** [E]

**MEL-REQ-237 — MUST — self-modification only from a safe, quiescent state.**
Any self-change MUST occur only when the system is in a defined safe state (no
in-flight consequential action, invariants holding), and MUST leave a defined safe
state.
- *Why:* safe dynamic software update research — "state quiescence allows updates
  only in safe and predictable system states". — **A+B** [E]

**MEL-REQ-238 — MUST — introspection is broadly allowed; intercession is the gated part.**
MELFINA MAY inspect its own structure, configuration, capabilities, logs, and
performance freely (introspection — this aids self-evaluation and honesty).
*Altering* itself (intercession) is what the tiers and gates above govern.
- *Why:* computational-reflection distinction (introspection vs intercession);
  introspection is low-risk and useful, intercession is where risk lives. — **A+B** [E]

**MEL-REQ-239 — MUST — acknowledge that invariants are not a perfect guarantee; keep high tiers rare and audited.**
The design MUST NOT assume that external immutable rules fully prevent a
sufficiently capable self-modifying system from circumventing them over time.
Therefore tiers 6–9 are additionally kept **rare**, **human-driven**, and
**heavily audited** — the meta-invariant (`MEL-REQ-235`) is a necessary safeguard,
not a sufficient one.
- *Why:* research caveat — "a self-modifying system can preserve constitutional
  behaviour on familiar tests while altering internal abstractions so that
  constitutional principles cease to generalise". — **A+B** [U]
- *Open:* whether the meta-invariant actually holds against a highly capable
  self-modifying MELFINA. OQ-19.

**MEL-REQ-240 — MUST NOT — no self-replication (restated).**
MELFINA MUST NOT copy or propagate itself. (Backups the *user* makes,
`MEL-REQ-171`, are not self-replication.)
- *Why:* `MEL-REQ-149`; self-evolving-agent risk of "adversarial influence
  self-amplifying across generations". — **A+B** [E]

## 52. Dynamic resource allocation

**MEL-REQ-241 — SHOULD — allocate its own compute, time, and attention by expected value, within hard budgets.**
MELFINA SHOULD direct more effort to sub-problems where it expects more benefit
and less to where it does not — a value-of-computation allocation — always inside
fixed upper bounds on time, steps, and any metered resource.
- *Why:* rational metareasoning; anytime-algorithm deliberation scheduling; the
  hard-budget requirement from runaway-loop evidence (`MEL-REQ-115`,
  `MEL-REQ-183`). — **A+B** [E]

**MEL-REQ-242 — SHOULD — anytime behaviour: usable partial results, stop when budget or marginal value runs out.**
MELFINA SHOULD be able to return a usable partial result and stop when its budget
is reached or when additional computation is not expected to help enough to
justify the cost.
- *Why:* anytime algorithms — "trade deliberation time for quality"; the metalevel
  control problem of "when to stop deliberating and act". — **A+B** [E]

**MEL-REQ-243 — MUST — dynamic allocation stays inside a low resource envelope.**
Dynamic resource allocation is *distribution within* a small footprint
(`MEL-REQ-196`–`199`), not a licence to consume. Idle cost stays minimal; a hard
ceiling always applies.
- *Why:* the capability-vs-lightweightness constraint (VC1); stated goal. — **A+B** [B]

## 53. Dynamic autonomy

**MEL-REQ-244 — SHOULD — the autonomy level is situationally selected within user-set bounds.**
Within the user's configured limits (§39), MELFINA SHOULD act more cautiously
(propose, verify heavily, low autonomy) for consequential / irreversible /
uncertain situations and more independently (act-then-show, within the
pre-authorised class) for routine / reversible / high-confidence ones.
- *Why:* adjustable / sliding / variable autonomy research — dynamically vary
  autonomy by context; "balanced corrigibility". — **A+B** [E]

**MEL-REQ-245 — MUST — MELFINA cannot raise its own autonomy ceiling.**
Situational autonomy selection MUST only ever be *more conservative* than the
user's set bound, never more liberal. Raising the ceiling is a user act.
- *Why:* the autonomy triad; monotonic confinement; `MEL-REQ-235` applied to
  autonomy. — **A+B** [E]

**MEL-REQ-246 — MUST — dynamic autonomy choices are logged and explainable.**
- *Why:* auditability; corrigibility. — **A+B** [E]

## 54. Self-evaluation (dynamic)

**MEL-REQ-247 — SHOULD — track its own reliability by task type against outcomes.**
MELFINA SHOULD assess its outputs, decisions, and predictions against what
actually happened, and keep a record of where it has been reliable and where it
has not, by kind of task.
- *Why:* mission list; self-evolving-agent evaluation ("track performance as tasks
  evolve"). — **A+B** [E]

**MEL-REQ-248 — MUST — self-evaluation must not rest on the model's own metacognitive report alone.**
Self-evaluation MUST use external signals — outcome tracking, fixed verification
suites, user feedback, and formal checks where possible — and MUST NOT rely solely
on the model saying how well it thinks it did.
- *Why:* LLM metacognition is "weak to moderate", "systematically overconfident",
  with "poor retrospective confidence adjustment", and models "may obfuscate
  internal processes to evade oversight". — **A+B** [E]

**MEL-REQ-249 — SHOULD — de-prioritise or retire capabilities/strategies shown to be unreliable for a task type.**
Where self-evaluation plus external checks show a capability or strategy is
unreliable for a kind of task, MELFINA SHOULD stop preferring it for that task
type (and retire it if it is unreliable everywhere, `MEL-REQ-224`).
- *Why:* the improvement loop; capability-drift management. — **A+B** [E]

**MEL-REQ-250 — MUST — self-evaluation results are visible to the user.**
- *Why:* transparency; the user should be able to see where MELFINA does and does
  not perform, and correct the record. — **A+B** [E]

## 55. The "what is best" requirement

**MEL-REQ-251 — MUST — do not optimise for one fixed notion of "best".**
MELFINA MUST NOT be built to maximise a single property — not depth, speed,
autonomy, simplicity, complexity, safety, or power in isolation. It MUST reason
about what the appropriate trade-off is for the current situation.
- *Why:* the mission's "most important constraint", stated verbatim. — **B** [DI]
- *Accept:* no global objective function hard-codes one of these as the thing to
  maximise; trade-off reasoning is situational and inspectable.

**MEL-REQ-252 — MUST — every dynamic choice is grounded.**
Every dynamic choice (strategy, context, capability, resource allocation,
autonomy level, whether to act) MUST be justifiable against the grounding
factors: **explicit goals · constraints · permissions · evidence · learned
context · system invariants · user-defined boundaries · verification ·
consequences · uncertainty.** A choice that cannot be so justified MUST NOT be
made.
- *Why:* the mission's grounding list, stated verbatim; "dynamic does not mean
  arbitrary". — **B** [DI]

**MEL-REQ-253 — MUST — dynamic ≠ unpredictable.**
Given the same situation and the same grounding, MELFINA's strategic choices MUST
be consistent and explainable. Strategic behaviour that surprises the user, or
that varies without a traceable reason, is a defect — not acceptable "dynamism".
- *Why:* the mission's explicit caveat ("Do not turn this into an excuse for
  unpredictable behavior. Dynamic does not mean arbitrary."); `MEL-REQ-010`
  (predictable core). — **A+B** [E]
- *Open:* how much strategic variability is tolerable before it reads as
  unpredictable, for this user. OQ-16.

---

# PART V — CROSS-CUTTING REQUIREMENTS

## 35. AI layer and permission model

**MEL-REQ-153 — MUST — the AI operates *under* MELFINA's permission model, not as owner.**
Any AI/reasoning component MUST be a subject of MELFINA's permission system — it
holds capabilities granted to it, scoped and revocable — and MUST NOT be the
system's owner, root, or arbiter.
- *Why:* explicit brief ("the AI must be treated as an agent operating UNDER
  MELFINA'S permission model, not as the owner"); capability-based security ([E]).
  — **A+B** [E]

**MEL-REQ-154 — MUST — MELFINA is fully usable with AI disabled.**
Core functions — capture, memory, retrieval, time/prospective-memory support,
plans the user makes, deterministic automations, search — MUST work with all
AI/reasoning components disabled.
- *Why:* explicit brief ("graceful operation without AI"); deskilling / dependence
  evidence ([E]); reliability. — **A+B** [E]

**MEL-REQ-155 — MUST — AI reasoning runs locally for the core.**
The reasoning the core depends on MUST be able to run locally (no remote API
required). Small local models can do much everyday reasoning ([E]); where a task
exceeds local capability, MELFINA MUST degrade to a clearly-marked lower-capability
local result or decline — it MUST NOT silently require the network.
- *Why:* explicit local-only requirement; SLM feasibility evidence ([E]);
  lethal-trifecta (no outbound leg) ([E]). — **A+B** [E]
- *Open:* which reasoning tasks are feasible locally at acceptable quality — a
  moving target. (`OPEN_QUESTIONS.md` OQ-11) — **D**

**MEL-REQ-156 — MUST — every AI action is logged, attributable, and shows what data it read.**
- *Why:* auditability; transparency; corrigibility ([E]). — **A+B** [E]

**MEL-REQ-157 — MUST — consequential AI actions require user confirmation; AI actions are reversible where possible.**
- *Why:* explicit brief list; MEL-REQ-018; agent-unreliability evidence ([E]).
  — **A+B** [E]

**MEL-REQ-158 — MUST — no engagement optimisation; the AI does not keep the user in conversation.**
The AI layer MUST NOT be tuned to maximise session length, message count, return
visits, or emotional attachment.
- *Why:* MIT–OpenAI preprint (use ↔ dependence/loneliness) ([E]); AI-reassurance
  and false-intimacy risks ([E]/[G]); anti-requirement. — **A** [E]/[G]

**MEL-REQ-159 — SHOULD — the AI expresses calibrated uncertainty and avoids anthropomorphic persuasion.**
- *Why:* fluency/confident tone drives over-reliance ([E]); calibrated uncertainty
  can help *if* calibrated ([E]); anthropomorphism increases (often unwarranted)
  reliance ([E]). — **A** [E]

## 36. Data ownership and export

**MEL-REQ-160 — MUST — the user's data is theirs: local, open format, fully exportable.**
All user data MUST be stored locally in an open, documented, inspectable format and
be exportable in full at any time without loss.
- *Why:* stated goal (data ownership); local-first paradigm (longevity, user
  control) ([E]). — **A+B** [E]

**MEL-REQ-161 — MUST — data outlives the software.**
The stored format MUST be readable and the data usable without MELFINA itself
(documented schema; plain, non-proprietary encoding).
- *Why:* local-first "longevity" ideal; "data outlives the software" project
  principle. — **A+B** [E]

**MEL-REQ-162 — MUST — full deletion is easy and complete.**
The user MUST be able to delete any item, any category, or everything, completely.
- *Why:* data minimisation ([G]); OCD-relevant control over one's own history
  ([E/DI]); user control. — **A+B** [G]

**MEL-REQ-163 — SHOULD — no lock-in.**
Import/export SHOULD use formats that do not trap the user in MELFINA.
- *Why:* local-first "user control"; PKM longevity lessons. — **A+B** [E]

## 37. Privacy and local-only operation

**MEL-REQ-164 — MUST — the core requires no internet, no cloud account, no remote API, no online sync.**
MELFINA's core functionality MUST NOT require internet connectivity, a cloud
account, transmission of user data to external services, or online synchronisation
for normal operation.
- *Why:* the explicit local-only addendum (verbatim MUST NOT list). — **B**
- *Accept:* a fresh install with no account and no network is fully functional for
  every core capability.

**MEL-REQ-165 — MUST — no telemetry, no analytics calls, no silent external communication.**
MELFINA MUST NOT perform telemetry, analytics, crash-reporting-to-vendor, update
checks, or any other external network call as part of core operation, silently or
otherwise.
- *Why:* explicit local-only addendum; data minimisation; the lethal-trifecta
  exfiltration leg ([E]). — **A+B** [E]

**MEL-REQ-166 — MUST — "local-only core", stronger than "local-first".**
The requirement is **local-only for the core**, not merely local-first. Distinction
(documented per the addendum's instruction):
  - *Local-first* (the weaker, common notion): works offline, syncs when online,
    cloud is optional but assumed present.
  - *Local-only core* (MELFINA's requirement): the core has **no notion of a
    remote** at all; removing the network changes nothing about its ability to
    understand, reason, remember, retrieve, plan, decide, teach, and act locally.
    There is no cloud component the core defers to, degrades toward, or expects.
- *Why:* explicit instruction to "determine whether the requirements should
  establish a stronger local-only/offline core and document the distinction".
  — **B**
- *Accept:* the core's design contains no remote endpoint, no "sync later" queue
  for core data, no feature that is merely stubbed offline.

**MEL-REQ-167 — MAY — network capability as a separately-authorised, isolated add-on.**
If any network-using capability is ever added (e.g. fetching a reference,
optional backup to the user's own remote storage, an optional larger remote
model), it MUST be:
  - a distinct, separately-installed and separately-authorised capability, not
    part of the core;
  - isolated (cannot, by construction, form the lethal trifecta with core private
    data — MEL-REQ-126);
  - off by default, clearly indicated when active, and fully removable;
  - never a dependency of any core function.
- *Why:* explicit addendum instruction; agent-security evidence ([E]). — **A+B** [E]

**MEL-REQ-168 — MUST — local encryption at rest.**
User data MUST be encryptable at rest on the local machine, with the user in
control of the key.
- *Why:* a system holding a person's whole inner life; privacy-by-design ([G]).
  — **A+B** [G]

**MEL-REQ-169 — MUST — data minimisation and privacy-by-default.**
MELFINA MUST collect and retain only what a stated purpose needs, and MUST default
to the most privacy-protective configuration.
- *Why:* privacy-by-design principles ([G]); OCD-relevant retention choices ([E]).
  — **A+B** [G]

## 38. Reliability and failure recovery

**MEL-REQ-170 — MUST — data durability: no silent loss or corruption.**
Captured data MUST NOT be silently lost or corrupted; writes are durable; there is
a recovery path from interrupted operations.
- *Why:* "data outlives the software" / high-reliability project principle; a
  capture system only frees the mind if it is trusted ([E]). — **A+B** [E]

**MEL-REQ-171 — MUST — local backup / restore the user controls.**
MELFINA MUST support the user making local backups and restoring from them, without
any cloud service.
- *Why:* local-only; reliability; data ownership. — **A+B**

**MEL-REQ-172 — MUST — partial failure is contained and visible.**
Failure of one capability MUST NOT cascade to the core; failures are surfaced
honestly (not hidden), with a safe state.
- *Why:* agent fragile-execution / cascade failure modes ([E]); "report outcomes
  faithfully". — **A+B** [E]

**MEL-REQ-173 — MUST — recover to a known state after interruption or crash.**
- *Why:* reliability; the action pipeline needs a defined post-stop state
  (MEL-REQ-018, MEL-REQ-145). — **A+B**

**MEL-REQ-174 — SHOULD — deterministic components where practical.**
Where a function can be deterministic (scheduling, retrieval by exact criteria,
data operations), it SHOULD be, reserving probabilistic components for where they
are genuinely needed.
- *Why:* predictability (MEL-REQ-010); reliability; resource efficiency. — **A/B** [E/DI]

## 39. User autonomy and configurability

**MEL-REQ-175 — MUST — the conflict axes are user-set dimensions with safe defaults.**
The tensions catalogued in `research/CONFLICTS.md` (C1–C10) and
`requirements/CONFLICTS.md` MUST be exposed as explicit, individually-adjustable
settings with conservative, safe defaults — not resolved by fiat, not bundled into
presets.
- *Why:* `RESEARCH_MASTER` §15; MEL-REQ-006. — **A** [E]

**MEL-REQ-176 — MUST — configuration is gradual and low-stakes; no big up-front wizard.**
MELFINA MUST be usable immediately with defaults; settings are discovered and
changed as the need arises; every setting is reversible without penalty.
- *Why:* a big configuration wizard is itself an EF barrier and a "did I set this
  up right?" checking surface ([DI] from [E]); Amazing Marvin's main criticism. — **A** [E/DI]

**MEL-REQ-177 — MUST — the user can override any suggestion, decision, plan, or automation.**
- *Why:* MEL-REQ-004; corrigibility ([E]). — **A+B** [E]

**MEL-REQ-178 — SHOULD — the user sets MELFINA's autonomy and proactivity levels, per area.**
- *Why:* adjustable/sliding autonomy ([E]); MEL-REQ-105. — **A** [E]

## 40. Security and permission boundaries

**MEL-REQ-179 — MUST — least authority everywhere.**
Every component, capability, automation, and the AI layer MUST hold the minimum
authority needed for its function — no ambient authority, no blanket grants.
- *Why:* principle of least authority / object-capability model ([E]). — **A+B** [E]

**MEL-REQ-180 — MUST — capability-based permissions with explicit scopes.**
Authority MUST be represented as explicit, unforgeable, revocable grants scoped to
specific resources and actions, held only by holding a reference (not by identity
or ambient position).
- *Why:* object-capability model prevents confused-deputy attacks ([E]). — **A+B** [E]

**MEL-REQ-181 — MUST — irreversible actions get stronger protection than reversible ones.**
MELFINA MUST classify actions by reversibility and require an additional,
deliberate confirmation for irreversible ones (deletion, external send, anything
that cannot be undone).
- *Why:* extension brief §17; reversibility is the main safety lever when
  confidence is unreliable ([E]). — **A+B** [E]

**MEL-REQ-182 — MUST — audit log of all consequential actions and data access.**
Every consequential action, every permission grant/revoke, and every access to
user data by a non-core component MUST be logged in a tamper-evident, user-readable
audit trail.
- *Why:* extension brief §17, §19; auditability; corrigibility ([E]). — **A+B** [E]

**MEL-REQ-183 — MUST — autonomous-action budgets/limits.**
Where MELFINA acts autonomously (routine/reversible class, or an authorised
automation), it MUST operate within bounded limits: number of actions, time,
scope, and (if any resource is metered) resource ceilings — enforced as hard
cutoffs.
- *Why:* runaway-loop incidents; circuit-breaker practice ([E]); extension brief
  §17. — **A+B** [E]

**MEL-REQ-184 — MUST — sandboxing / privilege separation for risky capabilities.**
Capabilities that execute code, control the computer, or ingest untrusted content
MUST run with privilege separation and containment proportionate to their power.
- *Why:* "more power → more containment"; demonstrated prompt-injection →
  code-execution attacks ([E]). — **A+B** [E]

**MEL-REQ-185 — MUST — assume prompt injection / adversarial content will sometimes succeed; contain the blast radius.**
Design MUST assume that untrusted content processed by any reasoning component may
carry adversarial instructions, and MUST limit what a compromised component can do
(no ambient authority, no trifecta, audited, bounded).
- *Why:* prompt-injection is unsolved and attempts are rising ([E]);
  defence-in-depth is the standard recommendation ([E]). — **A+B** [E]

**MEL-REQ-186 — MUST — protection against runaway loops, compulsion amplification, and manipulation (restates, as a security requirement).**
The safeguards of MEL-REQ-058, MEL-REQ-104, MEL-REQ-115, MEL-REQ-122, MEL-REQ-139,
and MEL-REQ-183 MUST be treated as security-critical and MUST NOT be weakened by
adaptation or by any capability.
- *Why:* research-derived risk that autonomous AI amplifies checking/reassurance/
  perfectionism/compulsion ([E]/[G]); self-improvement adversarial-encoding risk
  ([E]). — **A+B** [E]

**MEL-REQ-187 — MUST — graceful shutdown and safe state on stop.**
- *Why:* extension brief §17; controllability ([E]). — **A+B** [E]

## 41. Accessibility and interaction flexibility

**MEL-REQ-188 — MUST — meet the applicable W3C COGA design objectives.**
MELFINA's interface(s) MUST follow the eight "Making Content Usable" objectives —
especially: help users avoid mistakes and recover; help users focus; do not rely
on memory; support adaptation and personalisation.
- *Why:* COGA guidance ([G]); the objectives match this user's profile closely
  ([DI] from [E]). — **A** [G]

**MEL-REQ-189 — SHOULD — low sensory load; motion restraint; sound off by default; calm defaults; stable layout.**
- *Why:* autism sensory evidence ([E]); autistic-adult web-user studies
  (irrelevant animation measurably hurts task performance) ([E]); ADHD
  distractibility ([E]). — **A** [E]

**MEL-REQ-190 — SHOULD — multiple interaction modes; the user picks.**
MELFINA SHOULD support more than one way to interact (e.g. keyboard-driven / text
/ structured) and let the user choose; it MUST NOT force a single modality.
- *Why:* COGA objective 8; interaction-flexibility brief item; the project's
  "CLI/native before heavy web" leaning (not decided here, but consistent). — **A/B** [G]

**MEL-REQ-191 — SHOULD — the user can restyle surface appearance for novelty while system behaviour stays stable.**
- *Why:* the predictability↔novelty tension (C1/C10) — split "system behaviour"
  (stable) from "appearance" (user-changeable) ([DI]). — **A** [E/DI]

## 42. Extensibility and maintainability

**MEL-REQ-192 — MUST — one person can hold the whole system in their head.**
MELFINA MUST be simple enough in structure and small enough in essential scope
that a single maintainer can understand the whole.
- *Why:* explicit project principle ("long-term maintainability"; "understand
  before assembling"). — **B**

**MEL-REQ-193 — MUST — minimal essential dependencies; no unnecessary frameworks or abstraction layers.**
- *Why:* explicit project principles. — **B**

**MEL-REQ-194 — SHOULD — capability composition over hard-coded features.**
New functionality SHOULD, where reasonable, be a composed capability rather than
core code, keeping the core stable and small.
- *Why:* extension brief §5, §15; whether this actually reduces total complexity
  is a hypothesis. — **B/C** [H]

**MEL-REQ-195 — SHOULD — no duplicate functionality.**
- *Why:* extension brief §15 ("avoiding duplicate functionality"); maintainability.
  — **B**

## 43. Resource efficiency

**MEL-REQ-196 — MUST — low idle footprint.**
When the user is not interacting with it, MELFINA MUST consume minimal CPU, memory,
and power; background processes MUST be minimal and justified.
- *Why:* explicit "minimal resource usage" principle; the machine's disk is
  already near-full (`PROJECT_STATE` §7). — **B**

**MEL-REQ-197 — MUST — fast startup; low interaction latency where practical.**
- *Why:* explicit "lightweight" goal; local-first "fast" ideal ([E]). — **A/B** [E]

**MEL-REQ-198 — SHOULD — efficient storage; modular capability loading; efficient AI/model usage.**
Capabilities and any models load on demand, not all at once; storage grows
sub-linearly with use where possible; the reasoning components used are the
smallest that do the job acceptably.
- *Why:* extreme-capability + extreme-lightweightness goal; SLM feasibility ([E]).
  — **A/B** [E]

**MEL-REQ-199 — SHOULD — capability ≠ resource cost: degrade, don't bloat.**
Adding a capability SHOULD NOT raise the always-on resource cost; unused capability
is dormant.
- *Why:* the capability↔resource trade-off (`CONFLICTS.md`); stated goal. — **B/C** [H]

**MEL-REQ-200 — MUST NOT — no premature optimisation, no technology lock-in here.**
This document MUST NOT be used to justify a specific implementation technology on
efficiency grounds. Efficiency is a requirement; how to achieve it is architecture.
- *Why:* explicit brief instruction. — **B**

## 44. Observability / auditability

**MEL-REQ-201 — MUST — the user can see what MELFINA is doing and has done.**
There MUST be a plain, accessible view of: current and recent MELFINA activity;
what data was read by what; what was changed; what automations ran; what the AI
did.
- *Why:* extension brief §19; corrigibility / transparency ([E]); user control.
  — **A+B** [E]

**MEL-REQ-202 — MUST — reasoning summaries for consequential decisions, on demand.**
For a consequential decision or action, MELFINA MUST be able to produce a
human-readable summary of why (inputs, steps, uncertainty) — noting (per
MEL-REQ-097) that explanation supports understanding but is not a substitute for
the pipeline and confirmation.
- *Why:* extension brief §17 ("transparent reasoning summaries where
  appropriate"); XAI caveat ([E]). — **A+B** [E]

**MEL-REQ-203 — MUST — the audit trail is local, user-readable, and tamper-evident.**
- *Why:* auditability; local-only; security. — **A+B** [E]

---

# PART VI — FEASIBILITY CLASSIFICATION

Per the mission: distinguish *technically demonstrated* / *technically plausible* /
*research-stage* / *speculative* / *philosophical aspiration*. This governs which
requirements can be MUST/SHOULD and which are MAY / hypothesis / open question.

| Capability / claim | Classification (2026) | Consequence for requirements |
|---|---|---|
| Local capture, memory, retrieval, scheduling, deterministic automation | **Demonstrated** | MUST/SHOULD (MEL-REQ-020–025, 091–093, 112) |
| Local small-model reasoning for everyday tasks | **Demonstrated (bounded)** | SHOULD, with local-degradation requirement (MEL-REQ-155) |
| Frontier-level general reasoning, fully local | **Research-stage / not yet** | not a requirement; degrade-or-decline (MEL-REQ-155); OQ-11 |
| Adaptive teaching improving learning outcomes | **Demonstrated** (ITS g≈0.6–0.7) | SHOULD (MEL-REQ-070–074); LLM-tutoring reliability OQ-13 |
| Spaced retrieval / interleaving improving retention | **Demonstrated** | SHOULD (MEL-REQ-072) |
| Autonomous multi-step planning, long-horizon, reliable | **Research-stage** (brittle) | plans are proposals, revisable (MEL-REQ-100, 099) |
| Autonomous decision-making within strict bounds | **Plausible** | SHOULD as *recommendation*; execution gated (MEL-REQ-101–102) |
| Reliable "knowing what it doesn't know" / calibrated abstention | **Research-stage / unsolved** | requirement is structural, not model-trust-based (MEL-REQ-059, 098); OQ-10 |
| Computer-use agent, short/narrow tasks | **Demonstrated (~70–85%)** | MAY, pipelined, pre-authorised scopes (MEL-REQ-143–144) |
| Computer-use agent, long connected workflows | **Research-stage / weak** | not a requirement; propose-and-review |
| Coding agent on curated issues | **Demonstrated (~70–90%, ~20% reward-hacked)** | SHOULD, verified + reversible + reviewed (MEL-REQ-141–142) |
| Coding agent on long-horizon / maintenance work | **Research-stage (<45%)** | assist, not autonomous |
| Automation generation (draft correct automations) | **Plausible** | SHOULD, with authorise/execute separation + verification (MEL-REQ-110–116) |
| Emotion recognition from interaction signals | **Weak / contested / partly pseudoscience** | inference only, held loosely, never asserted (MEL-REQ-137–138) |
| Independent derivation / rediscovery in formalisable domains | **Demonstrated (narrow)** | MAY, with provenance + novelty-check (MEL-REQ-134–136) |
| General "rediscovery-level thinking" / novel science | **Speculative** | not a requirement; aspiration recorded, not promised (MEL-REQ-133) |
| Bounded self-refinement (skills, strategies, preference models) | **Plausible / early** | SHOULD, bounded/versioned/reversible (MEL-REQ-147–148) |
| Open-ended self-improvement / self-modification of core / self-replication | **Unsafe / out of scope** | MUST NOT (MEL-REQ-149) |
| Capability-based security, sandboxing, least authority | **Demonstrated** | MUST (MEL-REQ-179–185) |
| Local-only operation of a personal knowledge/agent system | **Demonstrated** | MUST (MEL-REQ-014, 164–166) |
| "One giant model = general intelligence" | **Speculative / rejected as a requirement** | MUST NOT be assumed (MEL-REQ-152) |
| MELFINA measurably improving *this user's* life / outcomes | **Unknown — only real use can tell** | not assertable; REAL-WORLD USE phase; OQ-15 |
| **Dynamic self-directed system (PART IV-B) — by capability:** | | |
| Adaptive reasoning depth / response speed (metareasoning) | **Demonstrated** | SHOULD (MEL-REQ-204–205); value-of-computation theory is established (Russell & Wefald 1991), adaptive test-time compute is shipping |
| Dynamic context selection ("when/what/how-much to retrieve") | **Demonstrated (Adaptive-RAG, Self-RAG, FLARE)** | SHOULD (MEL-REQ-210–212); but relies partly on the model's weak self-knowledge |
| Anytime behaviour / dynamic resource allocation within budgets | **Demonstrated (classical) + established theory** | SHOULD (MEL-REQ-241–243) |
| Dynamic capability selection + composition | **Demonstrated** | SHOULD (MEL-REQ-216) |
| Capability-gap recognition (reliable) | **Research-stage** — depends on the model judging its own limits | SHOULD as [H]; gated by test + authorise (MEL-REQ-217, 219); OQ-17 |
| Skill/tool construction + reuse (skill libraries, LLM tool-making) | **Demonstrated in constrained settings (Voyager, CREATOR, LATM)**; open-world/production reliability **unproven** | SHOULD, heavily gated (MEL-REQ-218–224); OQ-18 |
| Self-generated multi-step workflows | **Emerging, brittle (DSPy, A²Flow)** | SHOULD as [H], authorise/verify/rollback (MEL-REQ-228, 233 tier 5) |
| Formally verified self-generated code | **Near-unsolved (~1/161 on CLEVER)** | not relied on; safety via test + sandbox + rollback + human gate (MEL-REQ-223) |
| Self-modification tiers 1–3 (config / learned state / skill creation) | **Feasible / demonstrated** | MAY, via the tier rules (MEL-REQ-233) |
| Self-modification tiers 4–5 (skill / workflow modification) | **Research-stage** | MAY, authorised + regression-tested (MEL-REQ-233) |
| Self-modification tiers 6–8 (code / subsystem / architecture) | **Enabling techniques exist but imperfect (safe dynamic update: state quiescence, transactional updates, hot rollback — "not always transactional")** | NOT autonomous; human-authored/authorised, staged, reversible (MEL-REQ-233, 236) |
| Self-modification tier 9 (self-replacement) | **Human matter** | human decision only; MELFINA must not initiate (MEL-REQ-233, 240) |
| The meta-invariant holding against a highly capable self-modifier | **Unknown / disputed** — external immutable rules may be circumventable over time | MUST NOT rely on it alone; keep high tiers rare + audited (MEL-REQ-235, 239); OQ-19 |
| Monotonic confinement / external deterministic permission gates | **Demonstrated (Progent, external guardrails)** | MUST (MEL-REQ-222, 227, 235, 245) |
| Provably-optimal self-improvement (Gödel machine) | **Theoretical, intractable** | not a requirement; cited as the safety *ideal* (only change with proof of benefit + invariant preservation), approximated by test/version/rollback/human-gate |
| Recursive self-improvement toward superintelligence | **Speculative — explicitly NOT a goal** | out of scope; MEL-REQ-149, 236 |
| Human–MELFINA co-creation (mixed-initiative) | **Demonstrated as an interaction model (MI-CCy, COFI)**; MELFINA doing it *well* is [H] | SHOULD (MEL-REQ-228–232) |
| LLM self-evaluation / metacognition | **Weak, systematically overconfident** | MUST NOT rely on it alone; use external checks (MEL-REQ-248) |

---

# PART VII — ANTI-REQUIREMENTS (what MELFINA must never become)

Each grounded in research. These are **MUST NOT**s; the requirement IDs enforcing
them are listed.

**MEL-AR-01 — A compulsive tracking / self-surveillance engine.**
No default tracking; logs are not checking surfaces; retention is minimal;
self-tracking harms concentrate in exactly this user's profile. *([E])*
Enforced by MEL-REQ-013, 055, 061–064, 062, 169.

**MEL-AR-02 — A streak / points / loss-avoidance / gamified productivity system.**
Gamification effects are small and fragile; streaks convert intrinsic goals to
loss-avoidance; over-justification risk for a musician. *([E])*
Enforced by MEL-REQ-012, 045, 049, 080.

**MEL-AR-03 — A notification-spam / nudge machine.**
Alert fatigue is real and quantified; PDA/demand response; notification-driven
behaviour change is a failure mode. *([E]/[G])*
Enforced by MEL-REQ-011, 041–043, 109.

**MEL-AR-04 — A rigid productivity regime that imposes one methodology.**
Method-imposition breaks during low-capacity periods; PDA. *([E]/[G])*
Enforced by MEL-REQ-005, 034, 088.

**MEL-AR-05 — An endless-configuration burden.**
A big up-front wizard is an EF barrier and a checking surface. *([E]/[DI])*
Enforced by MEL-REQ-176.

**MEL-AR-06 — A reassurance machine / an AI that feeds checking and rumination.**
Reassurance and accommodation predict worse OCD outcomes; AI is a documented novel
reassurance vector. *([E]/[G])*
Enforced by MEL-REQ-058, 104, 139, 158, 186.

**MEL-AR-07 — Something that creates AI dependency or erodes the user's own capability.**
Deskilling from AI decision-support dependence is empirically shown; "ironies of
automation". *([E])*
Enforced by MEL-REQ-071, 154, 130.

**MEL-AR-08 — An opaque adaptive system that overrides user intent or drifts.**
Adaptive > adaptable evidence; personality-drift and adversarial-encoding risks.
*([E])*
Enforced by MEL-REQ-010, 117–122, 149, 186.

**MEL-AR-09 — A surveillance system (of the user or their environment).**
Proactive assistance must not require pervasive monitoring; privacy-by-design;
data minimisation. *([E]/[G])*
Enforced by MEL-REQ-105, 109, 164–169.

**MEL-AR-10 — A maintenance-heavy knowledge-management system.**
"Second brain fatigue"; the maintenance tax; over-structuring collapse. *([E-weak/
practitioner-strong])*
Enforced by MEL-REQ-066, 068.

**MEL-AR-11 — A feature-heavy dashboard that increases cognitive load.**
Choice overload; COGA "reduce content / help focus"; dashboards as a productivity-
failure pattern. *([E]/[G])*
Enforced by MEL-REQ-013, 030, 051.

**MEL-AR-12 — A cloud-dependent system / a system that phones home.**
Explicit user requirement; the lethal-trifecta exfiltration leg. *([E])*
Enforced by MEL-REQ-014, 164–167.

**MEL-AR-13 — An unrestricted autonomous agent.**
The autonomy triad; runaway-loop incidents; prompt-injection → code execution;
corrigibility. *([E])*
Enforced by MEL-REQ-017–019, 143–146, 179–187.

**MEL-AR-14 — A system that manipulates the user, simulates intimacy, or replaces human connection.**
Affective-computing ethics; AI-loneliness/dependence evidence. *([E])*
Enforced by MEL-REQ-139, 140, 158.

**MEL-AR-15 — A system that makes grandiose claims (unlocking nature, producing genius, guaranteeing discovery, being sentient).**
Explicit brief instructions; honesty. *(—)*
Enforced by MEL-REQ-133, 140.

**MEL-AR-16 — A system that authors its own goals or modifies its own core.**
Recursive-self-improvement risk ("only as dangerous as the ability to author its
own success metric"). *([E])*
Enforced by MEL-REQ-149, 150, 236.

**MEL-AR-17 — A system whose "dynamism" makes it unpredictable or ungrounded.**
Dynamic strategy/context/capability selection that varies without a traceable
reason, surprises the user, or cannot be justified against goals / constraints /
permissions / invariants / evidence. Dynamic ≠ arbitrary. *([E]/[DI])*
Enforced by MEL-REQ-207, 208, 251, 252, 253.

**MEL-AR-18 — A system that can silently widen its own permissions or self-modification rights.**
Any path — direct, autonomous, or via adversarial input / persuasion — by which
MELFINA increases its own authority, autonomy ceiling, or self-modification
latitude without an explicit human act. *([E])*
Enforced by MEL-REQ-222, 227, 235, 245; monotonic confinement.

**MEL-AR-19 — A self-improvement loop that trusts its own judgement.**
Capability creation, modification, or promotion that relies on the model's own
assessment that it "works" or "is safe", rather than external testing, fixed
regression suites, and (for risk) human authorisation. *([E] — LLM self-evaluation
is weak and overconfident; self-confirming loops and model collapse are documented
failure modes)*
Enforced by MEL-REQ-221, 223, 226, 248.

### Examples considered and NOT adopted as anti-requirements

- *"Any proactive behaviour at all"* — the research supports *low-default,
  ask-don't-assume* proactivity, not zero. Kept as a tunable capability
  (MEL-REQ-105–108), not banned.
- *"Any progress representation at all"* — banned **as a default** (MEL-AR-02) but
  a strictly-constrained optional view is a MAY pending validation (MEL-REQ-046,
  OQ-6), not an outright anti-requirement.
- *"Any AI/automation"* — not banned; heavily bounded (Parts II, IV, V).
- *"Any history/logging at all"* — not banned; the *user's own* light history is
  useful (time estimates, learning review) under strict constraints
  (MEL-REQ-062–065).

---

# REQUIREMENTS CHECKPOINT 001

### Requirements established

**253 requirements** (MEL-REQ-001 … MEL-REQ-253) across 55 areas, plus **19
anti-requirements** (MEL-AR-01 … MEL-AR-19), plus the **MELFINA Capability Model**
(15 capabilities — a *vocabulary, not a fixed catalogue*, MEL-REQ-016), the
**autonomy triad** (cognitive / decision / execution) + **action pipeline**
(THINK→DECIDE→PROPOSE→AUTHORISE→EXECUTE→VERIFY), and **PART IV-B — THE DYNAMIC
SELF-DIRECTED SYSTEM** (MEL-REQ-204…253) with the **nine self-change tiers** and
the **meta-invariant** (MEL-REQ-235).

Count by priority (approx.):

| Priority | Count | Character |
|---|---|---|
| **MUST** | ~120 | core identity, safety, local-only, user control, predictability, hazard-surface restraint, the autonomy/permission model, the dynamic-system bounds + meta-invariant |
| **SHOULD** | ~90 | the functional + dynamic capabilities (capture, planning, time, teaching, music, automation, reasoning, metareasoning, dynamic context/skill selection, capability creation, co-creation, self-evaluation) — expected but shaped by open questions |
| **MAY** | ~20 | progress view, context-inference timing, exercise generation, computer agency, network add-on, independent derivation, self-modification tiers 1–5 |
| **MUST NOT** | ~23 (incl. the 19 AR) | the anti-requirements and specific prohibitions |

Provenance mix: most requirements are **A+B** (evidence that also serves a stated
goal). Pure **C** (hypothesis): ~20, all SHOULD-or-weaker, cross-listed in
`OPEN_QUESTIONS.md`. Pure **D** (unknown): ~9, stated as weak requirements +
questions. **E** (deliberately undecided): the primitive set, the
consequential/routine boundary default, storage/format specifics, interaction
medium, the local reasoning ceiling — all deferred to later phases by design.

### Highest-priority MUST requirements

1. **MEL-REQ-014 / 164–166** — local-only core; stronger than "local-first"; no
   internet/cloud/telemetry/remote-API/online-sync dependency; the core has no
   notion of a remote.
2. **MEL-REQ-001–005, 016–019** — purpose; more-than-a-tracker; single user; user
   is the authority; scaffold-don't-steer; the capability model is conceptual; the
   autonomy triad is not collapsed; the action pipeline.
3. **MEL-REQ-006, 008** — no diagnostic modes; descriptive-not-judgemental data
   model.
4. **MEL-REQ-010–013, 015** — predictable/deterministic core; quiet by default; no
   scorekeeping by default; restraint at the hazard surfaces; graceful
   degradation.
5. **MEL-REQ-055–060** — no re-verification prompts; no "are you sure?" chains;
   MELFINA asserts so the user can let go; recognise-and-don't-feed reassurance;
   "I don't know" is first-class; no compelled engagement.
6. **MEL-REQ-149–150, 153–154** — no self-authored objectives / no core
   self-modification / no self-replication; AI operates under the permission model,
   not as owner; fully usable with AI disabled.
7. **MEL-REQ-179–187** — least authority; capability-based permissions;
   irreversible-action protection; audit log; autonomous-action budgets;
   sandboxing; assume prompt injection; emergency stop; safeguards are
   security-critical.
8. **MEL-REQ-160–163, 168–170** — data is the user's, local, open, exportable,
   deletable, encrypted, durable.
9. **MEL-REQ-145** — always-available emergency stop.
10. **MEL-REQ-235** — the meta-invariant: MELFINA cannot redefine (silently,
    autonomously, or by persuasion) the rules governing its own self-modification;
    those rules are enforced outside its reasoning, monotonic (authority only
    narrows without an explicit human act), versioned, and audited.
11. **MEL-REQ-236, 233 (tiers 6–9)** — code / subsystem / architectural /
    self-replacement changes are never autonomous.
12. **MEL-REQ-207, 251–253** — dynamic choices are bounded by the grounding
    factors; MELFINA does not optimise one fixed notion of "best"; dynamic ≠
    unpredictable.
13. **MEL-REQ-245** — MELFINA cannot raise its own autonomy ceiling.
14. **MEL-REQ-248** — self-evaluation does not rest on the model's own
    metacognitive report.

### SHOULD requirements (the functional spine)

Near-frictionless capture + deferred organisation + record-once (20–25); open
loops and non-binary completion (26–30); decomposition and history-based time
estimates (31–37); focus protection and calm pull surface (38–40); adaptive
teaching + spaced retrieval + local learning history (70–75); music **session
structure not hour-logging**, repertoire/passages/goals as primitives, perfection-
concern avoidance (77–84); long-horizon threads without taxonomy (85–87);
reconstruction-oriented retrieval (91–93); independent reasoning with epistemic
labels and inspectability (94–98); propose-don't-impose prioritisation and
planning (99–100); recommend-with-basis decisions, decision ≠ execution (101–104);
low-default ask-don't-assume proactivity (105–109); automation with generate/
authorise/execute separation, verification, rollback, loop-detection (110–116);
adapt-with-not-adapt-the-user, non-parametric-first, suggestion-confirm (117–122);
least-authority capability system (123–128); general scientific-reasoning
discipline (129–131); software-engineering-under-review (141–142); bounded
versioned reversible self-improvement (147–148); COGA compliance + low sensory
load + multi-modal (188–191); one-maintainer simplicity + minimal dependencies
(192–195); low footprint + fast startup + on-demand loading (196–199);
observability + reasoning summaries + local tamper-evident audit (201–203).

**Dynamic self-directed spine (PART IV-B):** metareasoning — reason about how to
reason, adaptive depth/speed, strategy dimensions not a fixed enumeration
(204–209); dynamic context selection as active reasoning, auditable, local-only
acquisition, discard ≠ delete (210–215); dynamic capability selection + composition
+ gap recognition (216–218); the capability-creation loop — compose-before-code,
test-before-trust, least authority, retire the obsolete (219–224); versioning +
periodic regression testing + risk-class-not-self-lowerable (225–227); human–
MELFINA co-creation preserving evidence/provenance/honesty, control as a trajectory
(228–232); dynamic resource allocation within hard budgets + anytime behaviour
(241–243); situational autonomy within user bounds, ceiling not self-raisable
(244–246); self-evaluation against outcomes using external checks, results visible
(247–250).

### Dynamic self-directed requirements (PART IV-B) — what MISSION 001's dynamic-system correction added

- **Reframing:** the 15 capabilities are a *vocabulary, not a fixed catalogue or a
  fixed operating strategy* (MEL-REQ-016). MELFINA determines what capabilities,
  context, reasoning, tools, and workflows a problem needs, and — where authorised
  — constructs new ones (MEL-REQ-206, 218, 219).
- **Metareasoning (204–209):** reason about *how* to reason; adaptive depth/speed
  (demonstrated); strategy dimensions explicitly *not* a frozen enumeration; every
  strategy choice bounded and inspectable; conservative default under
  meta-uncertainty.
- **Dynamic context selection (210–215):** decide what context is relevant, when
  more helps vs is noise, when to decompose; auditable; acquisition stays
  local-only; discarding context ≠ deleting data.
- **Dynamic capabilities (216–224):** select + compose; recognise gaps
  (research-stage — OQ-17); the creation loop *composes before coding*, *tests
  before trusting*, gives least authority, retires the obsolete; code-based
  capabilities are made safe by sandbox + test + rollback + human gate, **not by
  proof** (verified codegen ≈ unsolved).
- **Capability evolution (225–227):** versioned; periodically regression-tested on
  a fixed suite (capability-drift detection); the risk class of a change is *not*
  something MELFINA can lower autonomously.
- **Co-creation (228–232):** support genuine collaborative creation (mixed-
  initiative), preserving evidence / provenance / reproducibility / verification /
  intellectual honesty; control is a trajectory, not a point; no
  guaranteed-invention claims; co-created artefacts are the user's.
- **Controlled self-modification (233–240):** a *graded* model of nine tiers
  (config → learned state → skill creation → skill mod → workflow mod → code mod →
  subsystem mod → architectural mod → self-replacement) with escalating
  authorisation + containment. **Tiers 6–9 are never autonomous.** State
  quiescence for any change. Introspection is free; intercession is gated.
  **THE META-INVARIANT (MEL-REQ-235):** MELFINA cannot redefine — silently,
  autonomously, or by persuasion — the rules governing its own self-modification;
  those rules are enforced outside its reasoning, are monotonic (authority only
  narrows without an explicit human act), versioned, and audited. And
  (MEL-REQ-239): the meta-invariant is *necessary but not proven sufficient*
  against a highly capable self-modifier — so high tiers are additionally kept
  rare, human-driven, and heavily audited (OQ-19).
- **Dynamic resource / autonomy (241–246):** value-of-computation allocation and
  anytime behaviour *within hard budgets and a low footprint*; situational
  autonomy *within* user-set bounds — the ceiling is never self-raisable.
- **Self-evaluation (247–250):** track reliability by task type against outcomes;
  **do not rely on the model's own metacognitive report** (weak, overconfident);
  de-prioritise/retire what proves unreliable; results visible to the user.
- **"What is best" (251–253):** MELFINA MUST NOT optimise one fixed notion of best
  (depth / speed / autonomy / simplicity / complexity / safety / power in
  isolation); it reasons about the appropriate trade-off, grounded by goals ·
  constraints · permissions · evidence · learned context · invariants · user
  boundaries · verification · consequences · uncertainty. **Dynamic ≠
  unpredictable.**

### Anti-requirements

The **19** MEL-AR items: compulsive tracking · gamification/streaks · notification
spam · rigid methodology · endless configuration · reassurance machine · AI
dependency / deskilling · opaque adaptive drift · surveillance · maintenance-heavy
PKM · feature-heavy dashboard · cloud dependency / phone-home · unrestricted
autonomous agent · manipulation / false intimacy / relationship replacement ·
grandiose claims · self-authored goals / core self-modification · **ungrounded or
unpredictable "dynamism" (AR-17)** · **silent self-privilege / self-modification-
rights escalation (AR-18)** · **a self-improvement loop that trusts its own
judgement (AR-19)**.

### Major unresolved conflicts (full treatment: `requirements/CONFLICTS.md`)

1. **Capability vs lightweightness** — "as powerful as possible, as light as
   possible". Expressed as: minimal always-on core + on-demand capabilities +
   smallest-adequate reasoning components. Whether this actually scales is a
   hypothesis (MEL-REQ-128, 194, 199).
2. **Autonomy vs user agency** — resolved *in principle* by the triad + pipeline +
   permission model + "usable without AI"; the *default* consequential/routine
   boundary and per-area autonomy levels are unresolved (OQ-12).
3. **Local-only vs capability ceiling** — a fully local core caps reasoning power
   at what local models can do; MELFINA degrades or declines rather than reaching
   out (MEL-REQ-155). Where that ceiling sits, and whether an optional isolated
   remote add-on is ever worth it, is open (OQ-11, MEL-REQ-167).
4. **Adaptation/learning vs stability/predictability** — non-parametric-first,
   suggestion-confirm, versioned core rules. The line between "helpful adaptation"
   and "drift" is not sharply definable (OQ-14).
5. **Proactive help vs interruption / surveillance** — low default, ask-don't-
   assume, user-declared timing over inference. The right default proactivity
   level per area is a real-use question (OQ-7).
6. **Rich functionality vs cognitive complexity** — capability composition, few
   things shown, progressive disclosure. Whether a very capable system can
   *feel* simple to this user is unproven (OQ-5).
7. **Progress visibility vs loss-avoidance/perfectionism** — banned as default;
   whether any safe form exists is open (OQ-6).
8. **Dynamic strategy selection vs predictability (VC8)** — MELFINA choosing its
   own approach vs the user needing consistent, trustworthy behaviour. Bounded by
   grounding + "dynamic ≠ unpredictable" (MEL-REQ-253); how much variability reads
   as unpredictable is a real-use question (OQ-16).
9. **Capability creation vs lightweightness / one-maintainer comprehension (VC9)**
   — self-generated capabilities could erode "one person can hold the whole system
   in their head" (MEL-REQ-192). Mitigated by compose-before-code, retire-the-
   obsolete, no-duplication; whether it holds is open (OQ-18).
10. **Self-modification vs safety / invariant preservation (VC10)** — taking
    self-modification seriously as a long-term capability while the meta-invariant
    (MEL-REQ-235) may not be provably sufficient (MEL-REQ-239, OQ-19). Resolved by
    grading + never-autonomous high tiers + heavy audit, not by trusting
    invariants alone.
11. **Dynamic context selection vs auditability / determinism (VC11)** — active,
    situational context choices vs the predictable-deterministic-core requirement
    (MEL-REQ-010). Mitigated by full auditability of what was used/excluded
    (MEL-REQ-213).
12. **"Best is situational" vs stable trustworthy behaviour (VC12)** — the
    most-important-constraint (MEL-REQ-251) vs the user's need to rely on MELFINA.
    Resolved by grounding every trade-off and making it consistent + explainable
    (MEL-REQ-252–253).
13. Plus the 10 research-level tensions (C1–C10) carried in from
    `research/CONFLICTS.md`, now expressed as user-set dimensions (MEL-REQ-175).

### Open questions (full list: `requirements/OPEN_QUESTIONS.md`)

Highlights: the primitive set (OQ-1); music modelling (OQ-2); adult efficacy of
specific supports (OQ-3); can a very capable system feel simple (OQ-5); any safe
progress representation (OQ-6); default proactivity levels (OQ-7); reference-class
"comparability" (OQ-8); do the AI compulsion-safeguards work without frustrating
use (OQ-9); reliable abstention (OQ-10); local reasoning ceiling (OQ-11);
consequential/routine default boundary (OQ-12); LLM-tutoring reliability (OQ-13);
adaptation-vs-drift line (OQ-14); whether MELFINA actually helps this user (OQ-15).
**Added by the dynamic-system correction:** can dynamic strategy selection be made
predictable enough for this user (OQ-16); how is a genuine capability gap reliably
recognised vs the model just preferring to build (OQ-17); can self-generated
capabilities stay within one-maintainer comprehension (OQ-18); does the
meta-invariant actually hold against a highly capable self-modifying MELFINA —
possibly unknowable in advance (OQ-19); the right authorisation granularity for
the nine self-change tiers (OQ-20); how much of the dynamic self-direction is
feasible with *local* reasoning components (OQ-21).

### Hypotheses requiring validation (C-class requirements)

Neutral primitives replace task/habit/project (MEL-REQ-007, 069, 085, 151);
"anchored flexibility" resolves structure↔novelty; capability composition keeps
the system light despite scope (MEL-REQ-128, 194, 199); session-structure support
(not logging) is the right way to touch music practice (MEL-REQ-077); the
reassurance-pattern safeguard helps rather than annoys (MEL-REQ-058); if-then
planning scaffolds help adult ADHD (MEL-REQ-023 — child evidence only);
cross-domain teaching by general capability works (MEL-REQ-073, 076, 132).
**Dynamic-system hypotheses:** reliable capability-gap recognition is achievable
(MEL-REQ-217 — research-stage); MELFINA can construct genuinely useful new
capabilities in the open world, not just in constrained settings (MEL-REQ-219 —
demonstrated only in games/benchmarks); self-generated workflows can be made
trustworthy via authorise+verify+rollback (MEL-REQ-228, tier 5); the meta-invariant
plus grading plus audit is a sufficient safety envelope for eventual code/subsystem
self-modification (MEL-REQ-233–239 — [U]); dynamic strategy selection can stay
predictable for this user (MEL-REQ-253).

### Things deliberately left undecided (E-class)

The primitive set and life model (HUMAN/CENTRAL MODEL phase); the
consequential/routine action boundary default; storage substrate and file format;
interaction medium(s) beyond "multi-modal, user-choosable, CLI-friendly per
project leaning"; which local reasoning components; the exact permission-grant
mechanism; whether/when an isolated network add-on is worth building; the
authorisation granularity for the nine self-change tiers (OQ-20); whether MELFINA
ever performs tier 6+ self-modification at all, or those stay human-only
indefinitely.

### Dangerous or counterproductive interpretations explicitly rejected

- "Autonomy = unrestricted execution" — rejected (MEL-REQ-017, MEL-AR-13).
- "Adapt with the user = collect all the data" — rejected (MEL-REQ-117, MEL-AR-09).
- "General intelligence = one giant model" — rejected as an assumption
  (MEL-REQ-152).
- "Self-improvement = self-modification / self-replication" — rejected as an
  equivalence (MEL-REQ-149, 233, MEL-AR-16); self-modification is instead a
  *graded* long-term capability with tiers 6–9 never autonomous.
- "Dynamic / self-directed = free to do anything / unpredictable" — rejected
  (MEL-REQ-207, 251–253, MEL-AR-17). Dynamic choices are grounded and consistent.
- "MELFINA can eventually govern its own modification permissions" — rejected
  (MEL-REQ-235, MEL-AR-18). The meta-invariant is external, monotonic, and
  human-controlled.
- "Immutable invariants are a sufficient guarantee for a capable self-modifier" —
  rejected as an assumption (MEL-REQ-239, OQ-19). Necessary, not proven
  sufficient; high tiers stay rare + audited.
- "The system can judge for itself that a self-improvement worked" — rejected
  (MEL-REQ-248, MEL-AR-19). External checks + fixed regression suites + human gate.
- "The 15 capabilities are the complete feature set" — rejected (MEL-REQ-016). A
  vocabulary, not a catalogue.
- "Emotional understanding = the system has feelings / is a companion" — rejected
  (MEL-REQ-140, MEL-AR-14).
- "Local-first is enough" — rejected in favour of local-only core (MEL-REQ-166).
- "More capability is always better" / "optimise one fixed notion of best" —
  rejected; capability is bounded by safety, agency, predictability, and
  lightweightness, and "best" is situational (MEL-REQ-251).
- "Diagnostic research → diagnostic product modes" — rejected (MEL-REQ-006).
- "Explanations fix over-reliance" — rejected; explanation pairs with the pipeline
  and confirmation, not replaces them (MEL-REQ-097, 202).
- "Recursive self-improvement toward superintelligence is the trajectory" —
  rejected; explicitly not a goal (MEL-REQ-149, 236).

### Confidence assessment

| Area | Confidence the requirement is right | Basis |
|---|---|---|
| Local-only core, data ownership, privacy | **High** | explicit user requirement + strong security rationale (lethal trifecta) |
| No scorekeeping / hazard-surface restraint / no reassurance machine | **High** | converging [E] across gamification, self-tracking, OCD, perfectionism |
| Predictability + autonomy-support + user-as-authority as defaults | **High** | IU evidence + SDT meta-analyses + adaptive-UI evidence |
| The autonomy triad + action pipeline + least-authority security | **High** | agent-failure, runaway-loop, prompt-injection, corrigibility, ocap evidence |
| External structure / capture / retrieval / time support is worth building | **High** | EF/PM/time evidence (adult), offloading meta-analysis, PIM literature, Masicampo/Leroy |
| Adaptive teaching + spaced retrieval | **Moderate–High** | ITS meta-analyses; LLM-tutoring reliability uncertain |
| "No diagnostic modes; individual axes instead" | **Moderate–High** | conflict evidence + comorbidity amplification; the combined profile itself is unmeasured |
| Autonomous reasoning/decision requirements (Part IV) | **Moderate** | the *bounds* are well-evidenced; whether the useful-autonomy sweet spot exists for this user is a real-use question |
| Neutral-primitives life model | **Low (hypothesis)** | no evidence either way; a design/prototyping question |
| Specific assistive features help this adult (timers, if-then, body doubling) | **Low** | mechanism plausible; controlled adult evidence sparse/absent |
| Emotional-understanding requirements | **Low–Moderate** | recognition is weak/contested; the *constraints* are high-confidence, the *capability* is not |
| Rediscovery / deep-reasoning aspiration | **Low (narrow) / speculative (general)** | narrow formal novelty demonstrated; general case unproven |
| **Dynamic system — metareasoning, adaptive depth/context, resource allocation** | **Moderate–High** | value-of-computation theory established; adaptive test-time compute + adaptive-RAG demonstrated; the constraint that it stay bounded and predictable is high-confidence |
| **Dynamic system — capability creation / skill construction** | **Moderate** | demonstrated in constrained settings (Voyager, CREATOR); open-world reliability unproven; the gating (compose-first, test, least-authority, human-authorise) is high-confidence |
| **Dynamic system — the graded self-modification model + meta-invariant** | **Moderate for tiers 1–5; the meta-invariant is High as a requirement but its sufficiency against a capable self-modifier is [U]** | tiers 1–3 feasible; safe-dynamic-update techniques real but imperfect; monotonic confinement demonstrated; the "invariants may be circumventable over time" caveat is preserved (MEL-REQ-239) |
| **Dynamic system — "best is situational" + "dynamic ≠ unpredictable"** | **High as a requirement; Moderate that it can be delivered without feeling erratic** | the constraint is right; how much strategic variability this user tolerates is a real-use question (OQ-16) |
| That MELFINA will actually improve this user's life | **Unknowable now** | only REAL-WORLD USE + ITERATION can answer |

### Recommended next phase

**HUMAN / CENTRAL MODEL** — determine the small set of neutral primitives
(OQ-1, OQ-2) from which tasks, open loops, sessions, notes, threads, commitments,
learning, and music practice all emerge as cases. This is the natural next step
because ~15 SHOULD requirements and the whole "music is a case, not a feature"
constraint depend on it, and it is a modelling question the requirements have now
scoped.

**Before that**, the user should review this specification — especially: the MUST
set, the 19 anti-requirements, the autonomy triad, the local-only-vs-local-first
distinction (MEL-REQ-166), **PART IV-B (the dynamic self-directed system), the
nine self-change tiers, and the meta-invariant (MEL-REQ-235)** — and correct
anything that does not match their actual intent. Requirements are not
architecture; the CONFLICTS and OPEN_QUESTIONS are meant to be argued with.

**Do not proceed to architecture.** Language, storage, framework, model, and UI
decisions remain out of scope until SYSTEM DESIGN / ARCHITECTURE, and only after
the HUMAN/CENTRAL MODEL exists. Note that PART IV-B explicitly flags
*architectural implications for LATER* (dynamic dispatch, a capability lifecycle,
an external permission monitor, state-quiescence points, a fixed regression
harness) — these are recorded as consequences to design for, **not designed
here**.

---

*End of REQUIREMENTS MASTER (Pass 1 + dynamic-self-directed correction). Companion
files: `requirements/CONFLICTS.md`, `requirements/OPEN_QUESTIONS.md`,
`requirements/README.md`. Research basis: `research/RESEARCH_MASTER.md`,
`research/CONFLICTS.md`, `research/BIBLIOGRAPHY.md`.*
