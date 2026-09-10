# MELFINA — HUMAN / CENTRAL MODEL

**Phase:** HUMAN / CENTRAL MODEL.
**Mission:** HUMAN / CENTRAL MODEL MISSION 001.
**Status:** first pass, pending user review.
**Date:** 2026-09-10.
**Built on:** `requirements/` (all four files, checkpoint 001, 253 requirements +
19 anti-requirements) and `research/` (missions 001 + 002).
**Companion files:** `model/MODEL_ALTERNATIVES.md`, `model/MODEL_OPEN_QUESTIONS.md`,
`model/README.md`.

---

## 0. What this document is — and is not

This is a **conceptual model**: the smallest coherent set of primitives from which
MELFINA's many capabilities and the user's whole life can be described. It is
**not**:

- an architecture, a schema, a data structure, or a file format;
- a database choice (relational / graph / document / triple-store / hypergraph —
  none is chosen or implied);
- a programming language, framework, library, model provider, or inference-engine
  choice;
- a UI or interaction design.

Where a formalism (event-calculus predicates, Allen relations, direction-of-fit,
bitemporal coordinates, AGM operations) is used, it is to **clarify meaning**, not
to prescribe implementation. **A conceptual model must survive changes in
implementation technology** (`requirements` MISSION 001 §23). If any statement
below reads as a technology decision, it is a defect — report it.

The requirements are treated as **the current specification to analyse, not as
automatically correct** (mission §1). Where this modelling exercise suggests a
requirement is mis-stated, it is flagged in `MODEL_OPEN_QUESTIONS.md`.

**Evidence tiers** (carried from research/requirements): **[E]** established ·
**[G]** professional guidance · **[DI]** design inference · **[H]** hypothesis ·
**[U]** unknown / speculative. This is a *design* artefact, so most of it is
`[DI]`/`[H]`; grounding claims from the literature are `[E]`.

---

## 1. Purpose

MELFINA must hold "the meaningful state, activity, knowledge, intention,
experience, and history of a human life" (mission §2) in a form that:

- represents the user's life **without being organised around application
  categories** (tasks, habits, notes, calendars, projects, goals, reminders,
  journals, dashboards, music practice) — those must be able to **emerge** as
  views (mission §2, §4; `requirements` MEL-REQ-007);
- is **descriptive, not judgemental** — it never encodes "compulsion vs routine",
  "obligation vs choice", or "success vs failure" that the user did not enter
  (`requirements` MEL-REQ-008; research §6.2, `[E]`);
- keeps **provenance and epistemic status** for everything MELFINA knows —
  "I observed this" / "this source says this" / "I inferred this" /
  "I hypothesise this" / "I am uncertain" are **not** collapsed into a generic
  "fact" (mission §14; `requirements` MEL-REQ-093, 096, 131);
- supports **reality changing** — revised conclusions, obsolete knowledge,
  changed goals, "historical truth vs current truth" (mission §16);
- supports **MELFINA reasoning, proposing, and (within permission) acting**, and
  **MELFINA's own state**, in the *same* model rather than an ad-hoc structure per
  feature (mission §9, §13, §17);
- supports the **dynamic self-directed** MELFINA (`requirements` PART IV-B)
  **without freezing today's agent technology into the model** (mission §18);
- supports **genuine novelty** — user + MELFINA creating systems that did not
  previously exist (mission §19).

The objective is **reduction**: the *minimum sufficient* model, not the minimum
possible (mission §4).

---

## 2. Design principles

| # | Principle | Source |
|---|---|---|
| P-1 | **Minimum sufficient, not minimum possible.** Reduce aggressively; stop when a further merge would lose real semantics. Preserve a distinction only with a stated reason. | mission §4 |
| P-2 | **Neutral across domains.** No primitive is defined in terms of music, code, science, or "productivity". Domains are *cases*, not categories. | mission §2, §5 |
| P-3 | **Descriptive, not prescriptive or evaluative.** The model records what is, what happened, what is intended, and who claims what — never a verdict the user did not give. | MEL-REQ-008 |
| P-4 | **Everything MELFINA knows is a claim, with an origin and a status.** Storage ≠ memory. There is no unqualified "fact". | mission §12, §14; epistemic/doxastic logic (Hintikka), `[E]` |
| P-5 | **A single actual timeline; the past is appended to, not overwritten.** History is immutable as *record*; the *world it describes* can be re-described. Bitemporal: what-was-true vs when-we-recorded-it. | mission §10, §16; bitemporal modelling, `[E]` |
| P-6 | **Change is carried by occurrents; identity is carried by continuants.** Things persist and change; happenings unfold and are done. | BFO continuant/occurrent, `[E]` |
| P-7 | **Direction of fit separates describing the world from trying to change it.** A belief adjusts to the world; an intention tries to adjust the world. This is not a "task vs not-task" line. | Anscombe / Searle, `[E]` |
| P-8 | **Relevance is derived, not stored.** Context is what a situation makes relevant, computed from a cost/benefit trade-off — never a fixed attribute. | relevance theory (Sperber & Wilson), `[E]` |
| P-9 | **Relationships carry their own time, provenance, and confidence.** An assertion that two things are connected is itself something the model can talk about. | PROV, RDF reification, `[E]` |
| P-10 | **MELFINA is modelled by the same primitives as the user's world**, distinguished only by *who* authored or owns each element. | mission §17; `[DI]` |
| P-11 | **Kinds are open attributes, not primitives.** "Person", "place", "concept", "capability", "workflow" are *kinds of entity*, extensible without changing the model — this is what lets novelty (mission §19) and future capabilities fit. | `[DI]` |
| P-12 | **The model states meaning, not mechanism.** How relevance is computed, how claims are stored, how time is indexed — all out of scope. | mission §23 |
| P-13 | **Resist semantic drift.** Each primitive has one clear job; emergent concepts are named as *patterns*, not smuggled in as new primitives. | mission §20 |

---

## 3. Candidate primitives — the reduction

### 3.1 The long list, tested

The mission (§3) offers ~25 candidate concepts. Each was run through the
seven-question test (§4.7 of the mission: what does it represent · why fundamental
· can another primitive represent it · what is lost if removed · what emerges from
it · does it add unnecessary complexity · can it stay neutral). Summary:

| Candidate | Verdict | Represented as |
|---|---|---|
| **entity** | **PRIMITIVE** | `Entity` |
| person, place, thing, resource | derived | `Entity` with `kind` = person / place / thing / resource |
| knowledge, concept, question, hypothesis, topic, subject | derived | `Entity` (kind = concept/body-of-knowledge) **and/or** `Claim` (a question is a `Claim` with status *open*; a hypothesis is a `Claim` with status *hypothesised*) |
| **event** | **PRIMITIVE** | `Event` |
| action, process, observation, experience, decision, outcome | derived | `Event` (kinds: action / observation / session / decision / communication / …); an *outcome* is an `Event` or the `Claim`/`State` an `Event` brought about |
| **state** | folded into `Claim` | a `Claim` whose content is "condition C holds over interval I" (a *descriptive claim*) — see §3.3 for why it is not a separate primitive, and the dissent in `MODEL_ALTERNATIVES.md` |
| **intention** | **PRIMITIVE** | `Intention` (distinct from `Claim` by direction of fit — P-7) |
| commitment | derived | an `Intention` with a *creditor* (a relational, social intention) — Castelfranchi, `[E]` |
| goal | derived | an `Intention` with a *target condition* and (optionally) sub-`Intention`s |
| constraint | derived | a `Claim` (descriptive: "X must not exceed Y") **or** an `Intention` (to keep a condition true) |
| authorization, permission | derived | a `Claim` "agent A is authorised re: action-class K" (a descriptive claim about a normative state) — its *enforcement* is not in this model (see §12) |
| relationship | folded into `Claim` | a `Claim` of relational content, "X ρ Y" (P-9); flagged as a possible primitive in `MODEL_OPEN_QUESTIONS.md` OQ-M1 |
| context | **DERIVED, NOT STORED** | a relevance-weighted selection over the other primitives (§7) |
| time | **DIMENSION, NOT A PRIMITIVE** | a bitemporal coordinate system every `Event`, `Intention`, and `Claim` is situated in (§6) |
| uncertainty | **ATTRIBUTE** | `confidence` on every `Claim`; status *hypothesised* / *open*; conflicting `Claim`s left in place |
| history | **DERIVED** | the append-only sequence of `Event`s and `Claim`s ordered by transaction-time |
| relationships between people / places / concepts / events | derived | `Claim`s of relational content, with a small structural relation vocabulary (§5) |

### 3.2 The result: four primitives

> **ENTITY · EVENT · CLAIM · INTENTION**
> — situated in **TIME** (a bitemporal dimension), with **CONTEXT** derived.

This is the leading model, named **E²CI** ("Entities, Events, Claims, Intentions").
Three serious alternatives, and the dissent on folding `State`, are in
`MODEL_ALTERNATIVES.md`.

### 3.3 Why not fewer, why not more

**Why State is not a fifth primitive.** In event-calculus terms a state (a
*fluent*) is derivable: it *holds at* a time if an event *initiated* it and no
event has *terminated* it, and it *persists by default* (`[E]`). The
design-relevant "state" content — "the bug is open", "the piece is
performance-ready", "I am learning topic T", "A is authorised" — is well
represented as a **descriptive `Claim`** with a valid-time interval. Making State
a separate primitive would duplicate the machinery `Claim` already needs
(valid-time, provenance, confidence, supersession). **What is lost:** a crisp,
first-class "current world state" object; and states with *no* known initiating
event sit a little awkwardly as claims-without-an-event. This is a real cost — see
the state/transition alternative (Model C) and OQ-M2. The leading model accepts
it because "what holds now" is then simply *the set of currently-valid descriptive
claims MELFINA holds with sufficient confidence*, which is a query, not a stored
thing — and that keeps the model honest about the fact that MELFINA only ever
knows the world through claims (P-4).

**Why Intention is not folded into Claim.** Direction of fit (P-7, `[E]`): a
belief has *mind-to-world* fit (I revise the belief to match reality); an
intention has *world-to-mind* fit (I act to make reality match the intention).
Collapsing them would erase the difference between "I believe I will finish the
sonata" (a prediction, revisable) and "I intend to finish the sonata" (a
commitment of the self, which failure does not *falsify* — it frustrates). The
mission (§13) explicitly requires representing agency "without assuming everything
is a task", and this is the primitive that does it. **What is lost if removed:**
the entire user↔MELFINA agency pipeline (§10) becomes unrepresentable except as
"task" records — the exact anti-pattern.

**Why Entity and Event are irreducible.** `Entity` is the referent everything
else is *about*; nothing can represent it (an entity-less model has nothing to
attach claims or events to). `Event` carries *participation-with-roles* and
*derivation/causal* structure (PROV `used` / `wasGeneratedBy` / `wasInformedBy`,
`[E]`) that neither a claim nor a state carries, and *expected future events* are
not state-changes yet. Merging Event into Claim ("a claim that X happened") loses
the participant/role/derivation structure and the single-timeline discipline
(P-5).

**Can it be three?** Only by folding `Intention` into `Claim` (rejected above) or
`Event` into `Claim` (rejected above). Three is *possible* but not *sufficient*
(P-1). See `MODEL_ALTERNATIVES.md` Model D.

---

## 4. Primitive definitions

Notation: attributes are `lowercase`; a primitive's *content* may reference other
primitives.

### 4.1 ENTITY

**An entity is anything that persists through time, can be referred to, and can
bear properties and stand in relations.** (A *continuant*, BFO, `[E]`.)

- `id` — a stable internal handle (opaque; not a name).
- `kind` — an **open** classifier: `person` · `place` · `thing` / `resource` ·
  `work` / `artifact` (a piece of music, a document, a repository, a recording) ·
  `concept` / `topic` · `body-of-knowledge` · `collective` (a group, an
  ensemble, an organisation) · `capability` (a skill/tool MELFINA can invoke) ·
  `workflow` / `system` (a way of doing something, possibly co-created) ·
  `self` (the user) · `melfina` · `source` (a book, a website, a person *as an
  information source*) · … extensible (P-11).
- `names` / `aliases` — how the user and others refer to it, over time.
- Entities may be **parts of** other entities (a movement part-of a work; a
  passage part-of a movement; a file part-of a repository; a concept part-of a
  subject). Part-of is a structural relation (§5).
- Entities have **no truth value and no epistemic status** — they are hooks.
  What is *claimed about* them carries the epistemic load (P-4).

**Emerges:** repertoire, subjects, people, places, tools, projects-as-things,
capabilities, co-created systems.

### 4.2 EVENT

**An event is something that occurs at a time-point or over a time-interval,
possibly with participants in roles, possibly bringing about or ending
conditions.** (An *occurrent*, BFO; a *narrative* happening, event calculus,
`[E]`.)

- `id`.
- `kind` — **open**: `observation` (the user or MELFINA perceived something) ·
  `action` (an agent did something) · `session` / `activity` (a bounded stretch
  of engaged doing, directed at an *object* — activity theory, `[E]`: practice
  session, study session, coding session, writing session, an experiment run) ·
  `communication` · `decision` · `state-change` (a change noticed but not
  attributed to a specific action) · `performance` · `milestone` · … extensible.
- `time` — a point or an interval on the single actual timeline (§6). May be
  **expected** (a future event: an appointment, a planned session, a predicted
  result) rather than **occurred**.
- `participants` — Entities in **roles**: `agent` (who did it), `object` (what it
  was done to / directed at — activity theory's *object*), `instrument` (tools
  used), `beneficiary`, `witness`, `location` (a place-Entity). PROV
  `wasAssociatedWith` / `used` / `wasAttributedTo`, `[E]`.
- `on-behalf-of` — for actions an agent performed *for* another (PROV
  `actedOnBehalfOf`) — central to MELFINA acting for the user (§10).
- `brings-about` / `ends` — the descriptive `Claim`s (conditions) this event
  initiates or terminates (event-calculus `Initiates` / `Terminates`, `[E]`).
- `derived-from` / `informed-by` — other Events or Claims this Event used or
  responded to (PROV `wasInformedBy` / `wasDerivedFrom`).

**Emerges:** actions, observations, practice/learning/coding sessions,
experiments, decisions, performances, appointments (expected events), the raw
material of history.

### 4.3 CLAIM

**A claim is a statement held by an agent, about the world (including about
entities, events, other claims, and intentions), with an origin and an epistemic
status.** *All of MELFINA's knowledge is claims* (P-4).

- `id`.
- `content` — what is asserted. May be:
  - **descriptive** — "condition C holds" (over a valid-time interval); this is
    the *state* content (§3.3). E.g. "passage P is difficult", "bug B is open",
    "the user is learning topic T", "agent A is authorised re: K".
  - **relational** — "X ρ Y" for a relation ρ (§5). E.g. "movement M part-of work
    W", "observation O is evidence for hypothesis H", "concept C1 is prerequisite
    of C2", "decision D superseded decision D0".
  - **about a claim** — "claim K1 contradicts claim K2", "claim K1 is superseded
    by claim K2" (P-9; PROV bundles).
  - **about an entity** — "entity E has name N", "entity E has kind person".
- `holder` — whose claim it is: `self` (the user asserted / accepts it) ·
  `melfina` · a `source`-Entity ("this book says…") · another person.
- `status` — the epistemic mode (**not collapsed**, mission §14; `[E]`):
  - `observed` — the holder directly perceived it (grounded in an `observation`
    Event).
  - `reported` — a source states it (grounded in a `source` and a `communication`
    Event).
  - `inferred` — the holder derived it from other claims (grounded in the claims
    used — reproducible chain, MEL-REQ-131).
  - `hypothesised` — proposed, not established; a candidate.
  - `open` — a question; content is an interrogative, not yet answered.
  - `stipulated` — the user simply declares it so (a preference, a definition, a
    note, an interpretation choice).
  - `decided` — the outcome of a `decision` Event (a resolved choice).
  - `recalled` — the user or MELFINA is retrieving it from memory with the normal
    fallibility that implies.
- `confidence` — a coarse degree of belief (**not** a false-precision number by
  default). Conflicting claims are **left in place** (P-4, MEL-REQ-008); the model
  does not force resolution.
- `provenance` — `derived-from` (claims/events used), `generated-by` (the
  reasoning or observation Event), `attributed-to` (holder). Full PROV-style.
- `valid-time` — the interval over which the content is claimed to be true *in the
  world* (§6).
- `transaction-time` — when this claim was *recorded* in MELFINA. **Immutable and
  append-only** (§6, P-5).
- `supersedes` — a prior claim this one revises or replaces (AGM revision, `[E]`);
  the prior claim is **retained** (queryable by transaction-time).

**A "fact" is not a primitive.** The closest thing is *a descriptive claim held by
`self`, status `observed` or `stipulated`, high confidence, not superseded, not
contradicted* — and even that is a claim.

**Emerges:** knowledge, beliefs, observations, questions (`open`), hypotheses
(`hypothesised`), notes and journal entries (`stipulated`/`observed` claims
authored by the user), interpretations, decisions, the epistemic layer of
scientific reasoning, MELFINA's beliefs and uncertainty, the relation graph.

### 4.4 INTENTION

**An intention is an agent's directedness toward a future condition or action —
the world-to-mind stance (P-7, `[E]`).**

- `id`.
- `owner` — whose intention: `self`, `melfina`, or another agent (as the user's
  claim about them).
- `target` — what is intended: a **condition** to bring about (a descriptive
  content, like a Claim's), and/or a specific **Event** to perform / see happen.
  Goals are intentions with a condition target; tasks are intentions with an
  Event target and often a plan.
- `creditor` — if present, the agent to whom the owner is **committed** (social
  commitment — Castelfranchi, `[E]`). Absent ⇒ a purely internal intention.
  Present ⇒ the creditor holds an *expectation* and the owner *knows* it does.
- `status` — `active` · `proposed` (put forward, not yet adopted — how MELFINA
  suggests) · `authorised` (the owner or a permitting agent has cleared it to
  proceed) · `suspended` · `fulfilled` · `abandoned` · `blocked` (a claim records
  *what* blocks it).
- `plan` — an *optional* ordered set of sub-`Intention`s and/or expected `Event`s
  with temporal/`dependency` relations among them (§5). Its absence is normal (an
  open loop is an intention with no plan — MEL-REQ-026).
- `standing` — `one-off` · `recurring` (a standing intention to do something
  repeatedly — this is where "habit"/"routine" lives, *without* the word or any
  streak — MEL-REQ-012, MEL-AR-02) · `maintenance` (keep a condition true).
- `provenance` / `valid-time` / `transaction-time` / `supersedes` — as for Claim.
  Changed goals and abandoned plans are **superseded, not deleted** (P-5, §11).

**Failure of an intention does not falsify anything** — it moves `status` to
`abandoned`/`blocked`. There is no "success/failure" verdict in the model (P-3,
MEL-REQ-008); "fulfilled" is descriptive, "abandoned" is neutral.

**Emerges:** goals, sub-goals, tasks, open loops, plans, commitments to others,
routines, MELFINA's proposals, deadlines (an intention with a time-bounded
condition target), reminders (a MELFINA intention to surface something at a
trigger).

---

## 5. Relationships

Relationships are **`Claim`s of relational content** (P-9): each carries its own
`holder`, `status`, `confidence`, `provenance`, `valid-time`, `transaction-time`.
This is why "my teacher is X" (relational claim, valid 2019–now, status
`stipulated`) and "I once thought my teacher was Y" (superseded) both live.

A small **structural relation vocabulary** is proposed (closed-ish, because these
carry model semantics), plus **open user/MELFINA-defined relations** (any verb
phrase — neutrality, P-2):

| Relation | Connects | Meaning |
|---|---|---|
| `part-of` | Entity → Entity | mereology (movement part-of work; file part-of repo; concept part-of subject) |
| `participates-in` (+ `role`) | Entity → Event | with a role (agent / object / instrument / …) |
| `about` | Claim / Event / Intention → Entity / Claim / Event | topicality |
| `attributed-to` | Event / Claim / Intention → Entity(agent) | responsibility / authorship (PROV) |
| `on-behalf-of` | Event → Entity(agent) | delegated action (PROV) |
| `derived-from` | Claim / Event / Entity → Claim / Event | provenance / causal precedence (PROV) |
| `brings-about` / `ends` | Event → Claim(descriptive) | event-calculus initiate / terminate |
| `evidence-for` / `evidence-against` | Claim(observed) → Claim(hypothesised) | scientific support |
| `prerequisite-of` | Entity(concept) / Claim → Entity(concept) / Intention | learning / dependency ordering |
| `contributes-to` | Event / Intention / Claim → Intention | this advances that goal |
| `depends-on` | Intention / Event → Intention / Event / Claim | planning dependency |
| `supersedes` | Claim / Intention → Claim / Intention | revision (AGM) |
| `contradicts` | Claim → Claim | unresolved conflict (kept, not auto-resolved) |
| `located-at` | Entity / Event → Entity(place) | spatial |
| `expects` | Entity(agent) → Event(expected) / Claim | the creditor's expectation in a commitment |

**Are relationships more fundamental than objects?** (mission §15) — Partly.
Many *entities* only matter through their relations (a "passage" is
interesting *because* it is `part-of` a work the user is preparing and *because*
sessions `participate-in` working on it). But relations still need *relata* to
connect, so `Entity`/`Event`/`Claim`/`Intention` cannot be dissolved into
pure relations. The model's stance: **relations are first-class (they are claims
with full metadata), and some entities are thin (little more than a name + their
relations) — that is fine.** Whether `Relation` deserves to be a fifth primitive
rather than a claim-subtype is `MODEL_OPEN_QUESTIONS.md` OQ-M1.

**No graph database is implied.** "First-class relations with metadata" is a
*semantic* requirement; it can be met by many representations (mission §23).

---

## 6. Time semantics

**Time is a dimension, not a primitive.** Nothing in the model *is* time; every
`Event`, `Claim` (valid-time), and `Intention` is *situated in* it. A "calendar"
is a **view** of Events (occurred and expected) and time-bounded Intention targets
on a time axis — not a thing in the model (mission §10).

### 6.1 Two time axes (bitemporal, `[E]`)

- **Valid-time** — the interval over which a claim's content is asserted true *in
  the world* (or an event occurred, or an intention's target window is). Can be
  past, present, or future; can be corrected.
- **Transaction-time** — when MELFINA *recorded* the claim/event/intention.
  **Monotonic, immutable, append-only** (P-5).

This is how "historical truth vs current truth" (mission §16) is represented:

- *"What did I believe about X last March?"* → query at `transaction-time =
  last March`.
- *"What do I now believe was true last March?"* → query at `valid-time = last
  March, transaction-time = now`.
- *"Change the past"* → append a new claim with an earlier `valid-time` and a
  `now` `transaction-time`, `supersedes` the old; the old stays.

### 6.2 Points, intervals, and their relations

- Events are points or intervals; Claim valid-times and Intention target-windows
  are intervals (possibly half-open, possibly with fuzzy endpoints).
- Qualitative temporal relations use **Allen's interval algebra** (`[E]`): the 13
  relations `before / after`, `meets / met-by`, `overlaps / overlapped-by`,
  `during / contains`, `starts / started-by`, `finishes / finished-by`, `equals`.
  This lets the model say "the practice session was *during* the week before the
  performance" without committing to exact timestamps.

### 6.3 Expectation, recurrence, deadlines, duration

- **Expected future event** — an `Event` with `time` in the future and a status of
  *expected*; when it occurs, it becomes *occurred* (possibly with a corrected
  time). An appointment is exactly this.
- **Recurrence** — **not** a primitive. A *recurring* `Intention` (`standing =
  recurring`) is a generative pattern that produces expected `Event`s; each
  occurrence is its own `Event`. Whether the recurring intention is *one* thing or
  a series is OQ-M3.
- **Deadline** — an `Intention` whose `target` condition has a `valid-time` upper
  bound. Missing it changes the intention's `status`, not a "fail" flag (P-3).
- **Duration** — an interval length; an *estimate* is a `Claim` (status
  `inferred`, provenance = prior similar Events — this is the reference-class
  mechanism, MEL-REQ-033); the *actual* is read off the occurred Event's interval.
- **Temporal uncertainty** — fuzzy endpoints on intervals; `confidence` on the
  Claim that an Event occurred / will occur when it is said to.

### 6.4 Sequence and process

A **process** is not a primitive: it is a **connected set of `Event`s** (linked
by `derived-from` / `depends-on` / Allen relations) plus the `Claim`s and
`Intention`s about them. "observation → question → hypothesis → prediction →
experiment → measurement → evidence → analysis → conclusion → revised hypothesis"
(mission §8) is a *process pattern* — a shape a set of the four primitives takes,
not a new type (§13.4).

---

## 7. Context semantics

**Context is derived, never stored** (P-8; mission §11). The conceptual question —
"what makes some information relevant to a situation while other information is
irrelevant?" — is answered by **relevance theory** (Sperber & Wilson, `[E]`):

> An item of information is **relevant to a situation** to the extent that
> combining it with what MELFINA already holds produces **worthwhile changes to
> MELFINA's model of that situation** (confirming, correcting, or extending it)
> **for low processing effort**. The most relevant items are those with the best
> effect-to-effort ratio *relative to the current focus*.

A **situation** (for which context is derived) is characterised by:

- a **focus** — the `Entity` / `Event` / `Intention` currently in play (the piece
  being practised; the bug being fixed; the question being asked);
- the **present moment** (a valid-time and transaction-time coordinate);
- the **active `Intention`(s)** — what the user (or MELFINA, on their behalf) is
  trying to bring about (Conway's *goal-sensitive working self*, `[E]`);
- optionally a **place** and **other present agents**.

**Context** is then the relevance-ranked selection of `Claim`s, `Event`s,
`State`-content, and `Intention`s connected (directly or through short relation
paths) to the focus, that would most improve MELFINA's grip on *this* situation.

Design consequences (`[DI]`):

- Context is a **query with a ranking**, re-computed per situation — exactly what
  `requirements` PART IV-B (`MEL-REQ-210–213`) needs: MELFINA decides what to pull
  in and what to leave out, and that decision is **auditable** (what was included,
  what excluded, why) precisely because it is a derivation, not a lookup.
- "Discard context" means *drop from the working selection*, never *delete the
  claim* (MEL-REQ-215).
- The model does **not** say how relevance/effort are scored — that is the dynamic
  engine's job (mission §23; OQ-M6).

---

## 8. Information / provenance semantics

Every `Claim` (and every `Event` and `Intention`) records:

- **holder / owner** — whose it is (`self`, `melfina`, a `source`, another agent).
- **status** — the epistemic mode (§4.3): `observed` · `reported` · `inferred` ·
  `hypothesised` · `open` · `stipulated` · `decided` · `recalled`.
- **provenance** — PROV-style (`[E]`): `attributed-to` (holder), `generated-by`
  (the observation / communication / reasoning `Event` that produced it),
  `derived-from` (the `Claim`s and `Event`s it used).
- **confidence** — coarse degree of belief.
- **bitemporal coordinates** — valid-time, transaction-time (§6).

MELFINA can therefore always answer *"how do I know this?"* by walking
`generated-by` / `derived-from` to an `observation` Event, a `source`, or a
reasoning chain (MEL-REQ-097, 131, 093). **Storage is not memory** (mission §12):
a raw file the user dropped in is an `Entity` (kind `source` or `artifact`); what
MELFINA *knows from it* is a set of `Claim`s with status `reported` and provenance
pointing at that Entity and the `observation` Event of ingesting it.

**Obsolete information** is a `Claim` whose `valid-time` has ended, or which has
been `supersede`d, or whose confidence has decayed — it is **retained**, not
deleted (P-5), and simply not surfaced by default (relevance, §7).

**MELFINA-generated vs user-provided vs inferred** is the `holder` + `status`
pair. This is the structural basis for MEL-REQ-093 (retrieval never invents:
MELFINA-authored `inferred` claims are visibly distinct from `self`/`observed`
ones) and MEL-REQ-096 (epistemic labelling).

---

## 9. Uncertainty semantics

The model represents uncertainty in **four** distinct ways — deliberately not
collapsed:

1. **Degree of belief** — `confidence` on a `Claim`.
2. **Epistemic mode** — `status` `hypothesised` (a candidate) / `open` (a question)
   is *different* from a low-confidence `observed` claim. "I don't know" is a
   first-class output (MEL-REQ-059): it is *the absence of a sufficiently
   confident, non-contradicted `Claim`*, which the model can detect and report.
3. **Conflict** — two `Claim`s connected by `contradicts`, both retained. The
   model does **not** force resolution (P-4); resolving is a `decision` Event that
   `supersedes` one.
4. **Temporal / identity uncertainty** — fuzzy interval endpoints; uncertainty
   about whether two `Entity` references are the same thing (a `Claim` "E1 sameAs
   E2", status `hypothesised`).

**Belief change** follows AGM (`[E]`): *expansion* (add a consistent `Claim`),
*revision* (add a `Claim`, `supersede` conflicting ones to stay coherent),
*contraction* (retire a `Claim` and mark those that depended on it). All three
leave the prior state queryable by transaction-time.

---

## 10. Agency semantics

Agency is represented **without a "task" primitive** (mission §13) using
`Entity`(agents), `Event`(actions), `Claim`(descriptive: authorisations,
outcomes), and `Intention`.

The canonical pipeline (mission §13; `requirements` MEL-REQ-018 THINK → DECIDE →
PROPOSE → AUTHORISE → EXECUTE → VERIFY) in model terms:

| Step | Model representation |
|---|---|
| USER **intends** | an `Intention`, `owner = self`, `status active` |
| MELFINA **reasons** | `Claim`s, `holder = melfina`, `status inferred`, provenance = the claims/events used; a *reasoning* `Event` (kind `action`, agent `melfina`) `generated-by` |
| MELFINA **proposes** | an `Intention`, `owner = melfina`, `status proposed`, `target` = an `Event` to perform; `contributes-to` the user's `Intention` |
| USER **authorises** | a `decision` `Event` (agent `self`) that `brings-about` a `Claim` "melfina is authorised re: <this Event / this action-class>", and moves the proposed `Intention` to `status authorised` |
| MELFINA **acts** | an `Event` (kind `action`), `attributed-to melfina`, `on-behalf-of self` (PROV), within the authorised scope |
| WORLD **changes** | the action `Event` `brings-about` / `ends` descriptive `Claim`s; new `Event`s occur |
| MELFINA **observes** | `observation` `Event`s → `Claim`s, `status observed` |
| MELFINA **updates model** | AGM revision: new `Claim`s `supersede` outdated ones; the `Intention` moves to `fulfilled` / `blocked` |

- **Responsibility** = `attributed-to`; **delegation** = `on-behalf-of`;
  **authorisation/permission** = a descriptive `Claim` about a normative state.
- **The autonomy triad** (`requirements` MEL-REQ-017): *cognitive autonomy* =
  MELFINA freely creating `Claim`s and `Intention`s of `status proposed`;
  *decision autonomy* = MELFINA reaching a preferred `Intention` internally;
  *execution authority* = the presence of an "authorised" `Claim` covering the
  `Event`. These are **naturally separated** by the model — reaching a proposed
  `Intention` never itself produces an "authorised" `Claim`.
- **The model does not enforce anything.** Whether MELFINA *may* perform an
  `Event` is checked by a mechanism *outside* this model against the authorisation
  `Claim`s and the (external, immutable) permission rules (MEL-REQ-235). The model
  *represents* permission; it does not *guard* it (§12, §17.3).

---

## 11. Change / history semantics

**History is the append-only sequence of `Event`s and `Claim`s ordered by
transaction-time** (P-5). Nothing is overwritten or erased by normal operation
(deletion is a separate, explicit user act — MEL-REQ-162 — modelled as a
`redaction` Event that tombstones the target).

The mission (§16) asks whether change is state / events / versions / observations.
**The model uses all four, layered:**

- **Events** are the *drivers* of change (something happened).
- **Descriptive Claims with valid-time** are the *what-holds* layer (the state),
  event-grounded by `brings-about` / `ends` where the initiating event is known.
- **`supersedes` chains** are the *versioning* — a claim or intention revised over
  transaction-time; every version retained.
- **Observations** are how MELFINA *learns of* change it did not cause.

"Changing goals / preferences / plans" = `supersede` the old `Intention` /
`stipulated` `Claim` with a new one. "Changing identity/context" = the user's
`self`-Entity accrues new `stipulated` claims about themselves; old ones are
superseded, not deleted. **The model can always reconstruct "who the user was" at
any past transaction-time** — which matters for MEL-REQ-121 (don't treat a
temporary state as a stable preference: a `stipulated` claim with a short
`valid-time` or low `confidence` is visibly not a standing preference).

---

## 12. MELFINA state semantics

**MELFINA's own state is modelled by the same four primitives** (P-10; mission
§17), distinguished by `holder`/`owner`/`attributed-to` = `melfina`:

| MELFINA's… | Model representation |
|---|---|
| beliefs | `Claim`s, `holder = melfina` |
| uncertainty | `confidence`; `status hypothesised`/`open`; `contradicts` links |
| goals / plans / intentions | `Intention`s, `owner = melfina` |
| proposals | `Intention`s, `status proposed` |
| capabilities / skills | `Entity`(kind `capability`) + `Claim`s about what each does, its authority, its version, its reliability-by-task-type |
| a newly created capability | an `Entity` `generated-by` a *capability-creation* `Event` (PROV) |
| a retired capability | a `redaction`/`retirement` `Event` that `ends` the "active" `Claim`; the Entity and its history stay |
| active processes | `Event`s in progress (interval open, no end yet) |
| decisions | `decision` `Event`s + `decided` `Claim`s + the option `Claim`s considered |
| results / outcomes | `Event`s and the `Claim`s they `brought-about` |
| what it has done | the `Event`s `attributed-to melfina` (this is the audit trail's content — MEL-REQ-182, though the *tamper-evidence* is a mechanism, not model) |
| what it is allowed to do | descriptive `Claim`s "melfina is authorised re: K" |
| learned adaptation (preferences it has picked up) | `Claim`s, `holder = melfina`, `status inferred`, `about` the user — **explicit, inspectable, supersedable** (MEL-REQ-118) |
| self-evaluation | `Claim`s "melfina's capability X is (un)reliable for task-type Y", `status inferred`, provenance = outcome `Event`s (MEL-REQ-247) |

### 12.1 What is NOT in this model (deliberately)

- **The meta-invariant and the permission-enforcement mechanism** (MEL-REQ-235,
  the external monotonic gate). The model *represents* authorisations as `Claim`s;
  it does **not** contain the rules governing whether MELFINA may change itself,
  and it must not — those are external to MELFINA's reasoning *by requirement*
  (MEL-REQ-235, MEL-AR-18). If they were `Claim`s in this model, MELFINA could
  reason about editing them.
- **Tamper-evidence, encryption, sandboxing, budgets, emergency stop** — all
  mechanisms (`requirements` §40), not concepts.
- **How relevance / strategy / reasoning-depth are chosen** — the dynamic engine
  (§13.6; OQ-M6).

The model gives the dynamic engine *something to reason over*; it does not *be*
the engine.

---

## 13. Emergent higher-level concepts

Every application category is a **view or pattern** over the four primitives —
none is a primitive (mission §2; MEL-REQ-007).

| Concept | Pattern |
|---|---|
| **Open loop / "on my mind"** | an `Intention` with no `plan`, low elaboration, `owner = self` |
| **Task** | an `Intention` with an `Event` `target` and usually a `plan` step or a time |
| **Goal** | an `Intention` with a condition `target`; sub-goals = `Intention`s `contributes-to` it |
| **Project / endeavour / "thread"** | an `Entity`(kind `work`/`system`) + the `Event`s `about`/`attributed-to` it + `Intention`s `contributing-to` it + `Claim`s about its state, over time |
| **Habit / routine** | a `recurring` `Intention` + the series of `session`/`action` `Event`s realising it — **no streak, no count surfaced** (MEL-AR-02) |
| **Appointment** | an `expected` `Event` with a fixed `time` and `participants` |
| **Deadline** | an `Intention` whose condition `target` has a `valid-time` upper bound |
| **Reminder** | a `melfina`-owned `Intention` to surface a `Claim`/`Event`/`Intention` to the user at a trigger (`depends-on` an `Event` or time) |
| **Note** | a `Claim` (or cluster), `holder = self`, `status stipulated`/`observed`, `about` an `Entity`/`Event` |
| **Journal entry** | `Claim`s authored by `self` `about` a period or `Event`, often `status observed`/`recalled`, plus how the user felt (a `Claim` "self felt X") |
| **Calendar** | a **view**: `Event`s (occurred + expected) and time-bounded `Intention` targets on a time axis |
| **Dashboard** | a view — **and one the requirements largely forbid by default** (MEL-AR-11); the model does not privilege it |
| **Decision** | a `decision` `Event` terminating a "deliberating" `Claim`, initiating a `decided` `Claim`; option `Claim`s + rationale `Claim`s attached |
| **Question** | a `Claim`, `status open` |
| **Hypothesis** | a `Claim`, `status hypothesised`; **prediction** = a `Claim` about an expected `Event`/condition, `derived-from` it; **evidence** = `observed` `Claim`s `evidence-for`/`-against` it |
| **Experiment** | a `session` `Event` with `object` = a hypothesis-`Claim`, producing `observation` `Event`s → `Claim`s |
| **Learning trajectory** | `Intention`(s) targeting "understands concept C" conditions, ordered by `prerequisite-of`, realised by `session` `Event`s, tracked by `Claim`s about current understanding |
| **Repertoire piece** | an `Entity`(kind `work`); **movement/section/passage** = `Entity`s `part-of` it; **technical problem** = a descriptive `Claim` "passage P has difficulty D"; **interpretation** = `stipulated` `Claim`s about how to play; **performance** = an `Event`; **preparation** = `Intention`s + `session` `Event`s |
| **A capability / skill / tool** | an `Entity`(kind `capability`) + `Claim`s (what it does, authority, version, reliability) |
| **A co-created workflow / system** | an `Entity`(kind `workflow`/`system`) `generated-by` a collaborative `Event`, provenance = joint authorship |
| **Automation** | a `melfina`-owned `recurring` or trigger-conditioned `Intention` + the `Event`s it performs + verification `Claim`s (MEL-REQ-110–116) |
| **Strategy / reasoning approach** | **not modelled as a thing** — a `Claim` "melfina will approach problem P via approach A" (`status decided`), so the model does not freeze today's agent tech (mission §18; MEL-REQ-206) |

### 13.1 Test case — MUSIC (mission §5)

Requirement: represent repertoire, work, movement, section, passage, practice
session, practice intention, technical problem, interpretation, performance,
preparation, reflection, historical progression, uncertainty, observations —
**without a music subsystem**.

- **Repertoire** = the set of `Entity`(kind `work`) the user has a standing
  relation to (`part-of` a "current repertoire" collective-Entity, or simply
  `Intention`s `about` them).
- **Work / movement / section / passage** = `Entity`s linked by `part-of`. Kind
  can stay `work` at every level or gain a sub-kind; **no music-specific
  primitive**.
- **Practice session** = an `Event`, kind `session`, `object` = a passage-Entity
  or a technical-problem-`Claim`, `instrument` = the piano-Entity, agent `self`.
- **Practice intention** = an `Intention`, `target` = "passage P is fluent at
  tempo T" (a condition), `owner = self`; **preparation** = an `Intention`
  targeting "work W is performance-ready", with sub-`Intention`s per passage.
- **Technical problem** = a descriptive `Claim` "passage P: difficulty = <the
  user's own description>" (P-3: the user names it, MELFINA does not judge it a
  "weakness"). Sessions aim to `end` it; progress = superseding `Claim`s with
  changed descriptions.
- **Interpretation** = `stipulated` `Claim`s "in passage P, phrase toward bar N" —
  the user's artistic choices, never overwritten by MELFINA.
- **Performance** = an `Event`, kind `performance`; **reflection** = `Claim`s
  (`status observed`/`recalled`) authored after, `about` the performance Event.
- **Historical progression** = the `supersedes` chain of difficulty-`Claim`s +
  the series of session `Event`s, queryable over time — *without* a "practice
  streak" or "hours" headline (MEL-REQ-077, 080, 081; MEL-AR-02).
- **Uncertainty** = `confidence` on "work W is performance-ready"; `status
  hypothesised` on "this fingering will work".
- **Observations** = `observation` `Event`s ("in today's run-through, bar 40
  rushed") → `Claim`s.

**Genuinely music-specific semantics found:** *none at the conceptual level.* The
part-whole structure of a work, the notion of a "passage", and "performance-ready"
are all instances of general primitives. What *is* music-specific is **vocabulary**
(tempo, fingering, phrasing, dynamics) — and that lives in the *content* of
`Claim`s and the `kind`/attributes of `Entity`s, which are open (P-11). Documented
for the record: a music domain will want a **shared vocabulary** (a set of
agreed terms and relations) layered on top — but that is a *domain profile*, not a
subsystem, and the same is true of every domain (§13.5).

### 13.2 Test case — LEARNING (mission §6)

- **Subject / concept** = `Entity`(kind `concept`/`body-of-knowledge`);
  **prerequisites** = `prerequisite-of` `Claim`s between concepts.
- **Question** = `Claim` `status open`; **learning attempt / exercise** = a
  `session` `Event` with `object` = a concept-Entity or a question-`Claim`.
- **Mistake** = an `observation` `Event` + a `Claim` "in attempt A, self applied
  rule R incorrectly" (`status observed`, neutral wording — P-3).
- **Understanding** = a descriptive `Claim` "self understands concept C at depth
  D", superseded upward over time; **uncertainty** = its `confidence`.
- **Discovery** = a `Claim` the user reached themselves (`status inferred`,
  `holder = self`) — MELFINA marks it as the user's, not its own (MEL-REQ-071).
- **Explanation** = `Claim`s `holder = melfina` `about` a concept; **resource** =
  an `Entity`(kind `source`).
- **Long-term trajectory** = an ordered set of `Intention`s targeting
  "understands C" conditions, plus a spaced-review schedule = `melfina`-owned
  `recurring` `Intention`s to resurface concepts (MEL-REQ-072).

**Learning-specific semantics found:** none. "Prerequisite" is a general
dependency relation; "understanding at depth D" is a descriptive claim.

### 13.3 Test case — SOFTWARE ENGINEERING (mission §7)

- **Repository / file / code** = `Entity`s (kinds `work`/`artifact`), `part-of`
  chained; **release** = a `milestone` `Event`.
- **Bug** = a descriptive `Claim` "in repo R, behaviour B is wrong" (`status
  observed`); **reproduction** = a `session` `Event` producing `observation`
  `Event`s.
- **Hypothesis about a bug** = `Claim` `status hypothesised` ("the null comes from
  function F"); **experiment** = a `session` `Event` testing it; **evidence** =
  `observed` `Claim`s.
- **Implementation / fix** = `action` `Event`s `attributed-to` an agent that
  `brings-about` "behaviour B is correct"; **test** = a `session`/`action` `Event`
  producing a verification `Claim`; **failure** = an `observation` `Event`.
- **Decision** (e.g. an architecture choice) = a `decision` `Event` + option and
  rationale `Claim`s; **requirement** = a `stipulated`/`decided` `Claim`;
  **dependency** = `depends-on` `Claim`s.
- **Project / workflow / automation** = as in §13 (an `Entity` + its Events +
  Intentions; automation = a `melfina`-owned trigger-conditioned `Intention`).

**SWE-specific semantics found:** none at the conceptual level. This is
unsurprising — the mission's own §7 list (hypotheses, experiments, decisions,
failures) is the *scientific-reasoning* pattern (§13.4) applied to code.

### 13.4 Test case — SCIENTIFIC REASONING (mission §8)

The chain *observation → question → hypothesis → prediction → experiment →
measurement → evidence → analysis → conclusion → uncertainty → revised
hypothesis* maps to:

| Step | Primitive(s) |
|---|---|
| observation | `observation` `Event` → `Claim` `status observed` |
| question | `Claim` `status open` |
| hypothesis | `Claim` `status hypothesised` |
| prediction | `Claim` `derived-from` the hypothesis, `about` an expected `Event`/condition |
| experiment | `session` `Event`, `object` = the hypothesis-`Claim` |
| measurement | `observation` `Event` → `Claim` `status observed` |
| evidence | `evidence-for` / `evidence-against` `Claim`s linking measurements to the hypothesis |
| analysis | `action` `Event` (reasoning) → `Claim`s `status inferred` |
| conclusion | `Claim` `status inferred` or `decided`, high `confidence`, provenance = the evidence chain |
| uncertainty | `confidence`; unresolved `contradicts` links |
| revised hypothesis | a new `Claim` `supersedes` the old (AGM revision); both retained |

**Which parts are fundamental human-world concepts and which are scientific
interpretations?** (mission §8) — *Fundamental:* observing, asking, believing with
a degree of confidence, doing a bounded activity toward an object, learning of an
outcome, changing one's mind. *Scientific-interpretation:* the specific *labels*
"hypothesis / prediction / evidence / experiment" and the *discipline* of
demanding provenance and falsification attempts. The model provides the former as
primitives and the latter as the **scientific-reasoning discipline**
(`requirements` MEL-REQ-129–131) operating over them — a *way of using* the model,
not an extension of it.

### 13.5 Test case — PERSONAL ASSISTANT (mission §9)

| "Can the model represent…" | Yes, as |
|---|---|
| what matters to the user | `Intention`s (`owner = self`) + relevance weighting (§7) |
| what the user intends | `Intention`s |
| what has been promised | `Intention`s with a `creditor` (commitments) |
| what remains unfinished | `Intention`s with `status active`/`blocked` and unmet targets |
| what is happening | `Event`s in progress |
| what happened | occurred `Event`s + the `Claim`s they brought about |
| what may happen | `expected` `Event`s; `hypothesised` `Claim`s about the future |
| what requires attention | derived: `Intention`s whose targets are near a `valid-time` bound, or `blocked`, or high-relevance to the present focus — **surfaced neutrally, on a pull, not pushed** (MEL-REQ-040, 047) |
| what context matters | the derived context for the situation (§7) |
| what MELFINA believes | `Claim`s `holder = melfina` |
| what MELFINA is uncertain about | low-`confidence` / `hypothesised` / `contradicts` `Claim`s of MELFINA's |
| what MELFINA proposes | `Intention`s `status proposed` |
| what MELFINA has done | `Event`s `attributed-to melfina` |
| what MELFINA is allowed to do | authorisation `Claim`s (represented, not enforced — §12.1) |

**No ad-hoc structure per feature is needed** (mission §9) — every assistant
behaviour reads or writes the four primitives.

### 13.6 Test case — DYNAMIC SELF-DIRECTED SYSTEM (mission §18; `requirements` PART IV-B)

The model must support the dynamic MELFINA **without encoding today's AI
architecture** (mission §18).

| Dynamic capability (`requirements`) | Model support |
|---|---|
| dynamic context selection (MEL-REQ-210) | context is *already* a per-situation derivation (§7), not a stored structure |
| dynamic reasoning depth / strategy (MEL-REQ-204–206) | a strategy is a `Claim` "melfina will approach P via A" (`status decided`) + a reasoning `Event`; strategies are **not** a primitive or an enum, so future strategies need no model change |
| dynamic skill selection (MEL-REQ-216) | capabilities are `Entity`s; selecting is choosing which to make an `instrument` of an `Event` |
| capability-gap recognition (MEL-REQ-217) | a `Claim` "no available capability covers task-type T" (`status inferred`) |
| capability creation (MEL-REQ-219) | a capability-creation `Event` `generates` a new `Entity`; provenance, versioning (`supersedes`), retirement (`redaction` Event) all use existing machinery |
| capability evolution / versioning (MEL-REQ-225) | `supersedes` chains on capability `Entity`s and their describing `Claim`s |
| autonomous planning / task generation (MEL-REQ-99–100, 110) | `Intention`s `owner = melfina`, `status proposed`, with `plan`s |
| adaptation (MEL-REQ-117–122) | `inferred` `Claim`s `holder = melfina` `about` the user — explicit, supersedable, forgettable |
| self-evaluation (MEL-REQ-247) | `Claim`s about capability reliability by task-type, provenance = outcome `Event`s |
| controlled self-modification (MEL-REQ-233) | a self-change is an `Event` `attributed-to` an agent (`melfina` for low tiers, a human for tiers 6–9), `on-behalf-of` where relevant, that `generates`/`supersedes` capability or (for high tiers) other `Entity`s, with authorisation `Claim`s and verification `Claim`s — the *tiers and gates* are mechanism (§12.1), the *record* is model |

**Why this does not freeze today's tech:** the model never names "LLM", "prompt",
"context window", "agent", "tool call", "fine-tune". It has entities that *do*
things (capabilities), events of *doing* and of *making*, and claims about *how
well* things work. A completely different future implementation of MELFINA's
reasoning would produce the same kinds of `Event`s and `Claim`s.

### 13.7 Test case — HUMAN + MELFINA CO-CREATION (mission §19)

*user + MELFINA discover a problem → formulate a new concept → create a new
workflow → experiment → observe → revise → establish a new system.*

- **discover a problem** — a `Claim` `status observed`/`inferred` (`holder` =
  both, via two `Claim`s or a joint one) "current approach to X is inadequate".
- **formulate a new concept** — a new `Entity`(kind `concept`), `generated-by` a
  collaborative `Event`; its meaning accrues as `stipulated`/`inferred` `Claim`s.
- **create a new workflow** — a new `Entity`(kind `workflow`), `generated-by` a
  collaborative `Event`, provenance = both agents.
- **experiment / observe / revise** — `session` and `observation` `Event`s;
  `supersedes` on the workflow-Entity as it changes.
- **establish a new system** — the workflow-Entity gains a `Claim` "in active use"
  (`status decided`), and `melfina`-owned `Intention`s begin to reference it.

**Novelty needs no predefinition** (mission §19) because `kind` is open (P-11) and
the model never enumerates the entities, events, or relations that may exist. The
constraint the model *does* impose on novelty is `requirements`-derived:
provenance, verification, and honesty are preserved because every new thing is
`generated-by` a recorded `Event` with `attributed-to` agents and (for anything
risky) authorisation `Claim`s (MEL-REQ-229).

---

## 14. Model limitations

1. **Single-perspective.** The model represents the user's and MELFINA's claims;
   it represents *other* agents' minds only as the user's/MELFINA's `Claim`s
   *about* them ("I believe my teacher expects…"). It is not a full multi-agent
   epistemic model. `[DI]` — adequate for a personal system; flagged.
2. **`confidence` is a coarse scalar.** Real uncertainty is structured
   (imprecision, ambiguity, conflict, ignorance). The model separates *conflict*
   and *epistemic mode* from *degree*, but degree itself is flattened. OQ-M4.
3. **Direction of fit is binary.** Hope, fear, worry, ambivalence are mixed
   states poorly served by "Claim vs Intention". An OCD-relevant point:
   an intrusive worry is *not* a well-formed `Intention` and *not* a confident
   `Claim` — it may need its own light treatment. OQ-M5, and note MEL-REQ-026
   (open loops), MEL-REQ-058 (reassurance patterns).
4. **State folded into Claim.** The leading model has no first-class "current
   world state" object (§3.3). If prototyping shows "what holds now" queries are
   central and expensive-to-express, `State` may need to be reinstated as a fifth
   primitive (Model C direction). OQ-M2.
5. **Relation-as-Claim may be too thin.** Heavily relational domains (a musical
   score's structure; a dependency graph) may strain "relations are claims".
   OQ-M1.
6. **Emotion / affect is under-modelled.** Only "self felt X" `Claim`s. Given the
   user profile (research: emotion dysregulation `[E]`), this may be insufficient.
   OQ-M5.
7. **The model says nothing about mechanism** — relevance scoring, strategy
   choice, storage, indexing, the permission gate. By design (P-12), but it means
   the model alone cannot be validated by running it; prototypes are needed
   (`MODEL_OPEN_QUESTIONS.md`).
8. **Recurrence identity is unresolved** — is a routine one `Intention` or a
   series? OQ-M3.
9. **No account of *importance* independent of intention.** Something can matter
   without the user having formed an intention about it (a relationship, a value).
   The model leans on `Intention` + relevance; whether that covers "what matters"
   fully is OQ-M7.
10. **Continuant/occurrent is applied pragmatically**, not with full 4D rigour;
    edge cases (is "the user's ADHD" a continuant property, an ongoing occurrent,
    or neither?) are not resolved and, per P-3, the model deliberately does **not**
    reify diagnostic categories at all.

---

## 15. Confidence

| Element | Confidence it is right | Basis |
|---|---|---|
| Four primitives are the right *kinds* of thing (referents / happenings / claims / directedness) | **Moderate–High** | converges across BFO (continuant/occurrent), event calculus (events+fluents), epistemic logic (belief), Searle (direction of fit), PROV (entity/activity/agent) — all `[E]` |
| Claims carry provenance + epistemic status + bitemporal coordinates | **High** | directly required (MEL-REQ-093, 096, 131); well-established (PROV, bitemporal, doxastic logic) `[E]` |
| Context is derived, not stored | **High** | relevance theory `[E]`; required by PART IV-B `[E]` |
| Time is a dimension, not a primitive; "calendar" dissolves | **High** | event calculus + Allen `[E]`; mission §10 |
| Application categories emerge as views | **Moderate–High** | demonstrated in §13 for 7 domains; music/learning/SWE/science all mapped with *no* domain primitive |
| `State` folded into `Claim` (not a 5th primitive) | **Moderate** | defensible (event calculus derives fluents) but has real costs (§3.3); the state/transition alternative is serious |
| `Relation` folded into `Claim` (not a 5th primitive) | **Moderate** | P-9 preserves the semantics; but heavily relational domains may strain it |
| Exactly *four* (not three, not five/six) | **Moderate** | three is insufficient (loses agency or events); five/six is not clearly *necessary* — but "sufficient" is a prototyping question, not settled here |
| The model does not leak implementation | **High** | no DB, language, framework, or engine named; formalisms are semantic only |
| The model supports the dynamic self-directed MELFINA without freezing today's tech | **Moderate–High** | §13.6 — strategies/skills are entities+claims, never enums; but "does it *really* not constrain future architecture" is only provable later |
| The model can hold *this user's* life well | **Unknown** | only real use tells (mission §27; OQ-M-final) |

---

## 16. Deliberately undecided questions

Recorded here so no later phase treats them as settled. Full list with
dependencies in `MODEL_OPEN_QUESTIONS.md`.

- Whether `State` is a fifth primitive (OQ-M2) and whether `Relation` is
  (OQ-M1).
- Whether the primitive count is genuinely minimal-sufficient at four (OQ-M8).
- Recurrence identity — one intention or a series (OQ-M3).
- How `confidence` should be structured (OQ-M4).
- How worries / intrusive thoughts / affect are represented (OQ-M5).
- Whether "what matters" needs representation beyond `Intention` + relevance
  (OQ-M7).
- The exact structural relation vocabulary (§5) — the list is provisional.
- Whether `self` and `melfina` should be ordinary `Entity`s or have a special
  status.
- Everything about *mechanism* (relevance scoring, strategy selection, storage,
  the permission gate) — out of scope by P-12, deferred to SYSTEM DESIGN /
  ARCHITECTURE.
- Language, storage substrate, format, graph-vs-relational-vs-other — **not
  decided, not implied** (mission §23).

---

## MODEL CHECKPOINT 001

### Candidate models investigated

Four, detailed in `MODEL_ALTERNATIVES.md`:

- **A — Event-centric** (event calculus + perdurantism + bitemporal + PROV):
  the primitive is the `Event`; entities and states are derived.
- **B — Entity/relationship-centric** (personal knowledge graph + property-graph
  thinking + PROV-as-metadata): primitives are `Entity` and `Relation`.
- **C — State/transition-centric** (situation calculus + statecharts + fluents):
  the primitive is `State`; events are transitions; entities are fluent bundles.
- **D — Three-primitive** (`Entity`, `Event`, `Claim`; intention folded into
  claim): the aggressive reduction.
- **E²CI — the synthesis (leading):** `Entity` · `Event` · `Claim` · `Intention`,
  in bitemporal `Time`, with `Context` derived.

### Leading model

**E²CI — four primitives: ENTITY, EVENT, CLAIM, INTENTION.**
Time is a bitemporal dimension (valid-time / transaction-time). Context is a
per-situation relevance derivation, never stored. Relationships are Claims of
relational content, carrying their own time/provenance/confidence. "State" (what
holds now) is the set of currently-valid descriptive Claims.

Chosen because no single alternative (A/B/C) satisfies the full set of model
quality criteria (mission §20) — temporal coherence *and* provenance *and*
uncertainty *and* user-comprehensibility *and* support for scientific reasoning
*and* support for the personal-assistant role *and* neutrality across music /
learning / software / life. E²CI takes the referent layer from B, the
history/change layer from A, the "what-holds-now" discipline from C (as claims,
not a separate primitive), and adds the epistemic layer (claim status +
provenance + AGM revision) that none of A/B/C carries well.

### Primitive count

**4** primitives (`Entity`, `Event`, `Claim`, `Intention`)
\+ **1** dimension (`Time`, bitemporal)
\+ **0** stored context (derived).

Possible expansions under active question: `State` (→ 5), `Relation` (→ 5 or 6).
Possible contraction: fold `Intention` (→ 3, **rejected** — loses agency).

### Derived concepts (not primitives)

state · relationship · context · history · process · task · goal · sub-goal ·
open loop · project / thread · habit / routine · appointment · deadline ·
reminder · note · journal entry · calendar · dashboard · decision · question ·
hypothesis · prediction · evidence · experiment · learning trajectory ·
repertoire / work / movement / section / passage · technical problem ·
interpretation · performance · preparation · reflection · capability / skill /
tool · co-created workflow / system · automation · strategy / reasoning approach ·
authorization / permission (as a described condition) · duration estimate ·
recurrence.

### Strongest test-case results

- **Music (§13.1):** *zero* music-specific primitives needed. Work/movement/
  section/passage = `part-of`-linked `Entity`s; practice session = a `session`
  `Event`; technical problem = a user-worded descriptive `Claim`; interpretation =
  `stipulated` `Claim`s MELFINA never overwrites; progression = a `supersedes`
  chain — with **no streak or hours headline** (satisfies MEL-REQ-077, 080, 081;
  MEL-AR-02).
- **Scientific reasoning (§13.4):** the full observation→…→revised-hypothesis
  chain maps cleanly; "hypothesis/evidence/experiment" are *labels and a
  discipline* over the primitives, not extensions of the model.
- **Personal assistant (§13.5):** all 14 "can the model represent…" items from
  mission §9 map to reads/writes of the four primitives — **no ad-hoc per-feature
  structure**.
- **Agency pipeline (§10):** USER intends → MELFINA reasons → proposes →
  USER authorises → MELFINA acts → world changes → MELFINA observes → updates —
  maps exactly, and the **autonomy triad is naturally separated** (reaching a
  `proposed` Intention never produces an "authorised" Claim).
- **Dynamic self-directed MELFINA (§13.6):** strategies and skills are
  `Claim`s/`Entity`s, never enums — the model does not freeze today's agent tech.

### Unresolved semantic questions (top)

1. Is `State` a fifth primitive? (OQ-M2)
2. Is `Relation` a fifth/sixth primitive, or is claim-of-relational-content
   enough? (OQ-M1)
3. Is four genuinely *minimum sufficient*? (OQ-M8 — a prototyping question)
4. Recurrence: one `Intention` or a series of them? (OQ-M3)
5. Structure of `confidence` / uncertainty. (OQ-M4)
6. Worries, intrusive thoughts, ambivalence, affect — how represented? (OQ-M5)
7. "What matters" beyond `Intention` + relevance. (OQ-M7)

### Deliberately undecided

The provisional structural relation vocabulary (§5); whether `self`/`melfina` are
ordinary entities; all mechanism (relevance scoring, strategy selection, storage,
the permission gate); and — untouched, as required — **every implementation
technology** (language, storage, graph-vs-relational, format, engine, model
provider).

### What this model does NOT contain (by requirement)

- The self-modification meta-invariant rules (MEL-REQ-235) — external to MELFINA's
  reasoning *by design*; if they were `Claim`s here, MELFINA could reason about
  editing them.
- Any enforcement mechanism (permissions are *represented* as `Claim`s, not
  *guarded*).
- Any evaluative verdict the user did not enter (P-3; MEL-REQ-008).
- Any diagnostic category (P-3) — ADHD/autism/OCD are **not** reified.

### Files created / changed

- `model/HUMAN_CENTRAL_MODEL.md` (this file) — new.
- `model/MODEL_ALTERNATIVES.md` — new.
- `model/MODEL_OPEN_QUESTIONS.md` — new.
- `model/README.md` — new.
- `PROJECT_STATE.md` — updated (phase, checkpoint).
- `src/` — **untouched.** `research/` — **untouched.** `requirements/` —
  **untouched** (issues with requirements are logged as open questions, not
  edited).

### Recommended next step

**User review of this model** — especially the four primitives, the
`State`-folded-into-`Claim` decision (§3.3), the `Relation`-as-`Claim` decision
(§5), and the seven top open questions. The model is meant to be argued with.

**After acceptance:** SYSTEM DESIGN / ARCHITECTURE — but **not before** the
human/central model is reviewed and explicitly accepted (mission §27). No
architecture, no technology, no implementation begins here or next without that
acceptance.

---

*End of HUMAN / CENTRAL MODEL (Pass 1). Companion: `MODEL_ALTERNATIVES.md`,
`MODEL_OPEN_QUESTIONS.md`, `README.md`. Basis: `requirements/`, `research/`.*
