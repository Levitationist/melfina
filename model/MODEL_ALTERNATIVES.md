# MELFINA — MODEL ALTERNATIVES

Companion to `model/HUMAN_CENTRAL_MODEL.md`. The candidate conceptual models
investigated, compared rigorously, with the reasons for selecting / rejecting
each.

**Mission rule (§21):** develop at least 3 serious candidates; compare rigorously;
explain what each handles well and poorly; then determine whether one is clearly
superior or whether a synthesis is necessary.

**Conclusion up front:** no single model A/B/C is sufficient against the full set
of quality criteria (mission §20). A **synthesis (E²CI)** is necessary and is the
leading model. Model D (aggressive three-primitive reduction) is documented and
rejected. Evidence tiers: **[E]** established · **[DI]** design inference ·
**[H]** hypothesis · **[U]** unknown.

---

## The evaluation criteria (mission §20)

Each model is scored against: conceptual simplicity · expressive power ·
neutrality · extensibility · temporal coherence · provenance · uncertainty ·
reversibility · user comprehensibility · dynamic-reasoning support ·
autonomy support · teaching support · scientific-reasoning support ·
software-engineering support · music support · personal-life support ·
long-term maintainability · resistance to semantic drift.

Scoring: ✅ strong · 🟡 partial · ❌ weak.

---

## Model A — Event-centric

**Idea.** The single primitive is the **Event** (a dated happening on one actual
timeline). Entities are "worms" of events (perdurantism, `[E]`); a *state* is the
stretch of timeline between an initiating and a terminating event (event calculus
fluents, `[E]`); knowledge is events of observing / being told / inferring;
identity is a thread of `same-as` links between event participants. Grounded in:
**event calculus** (Kowalski & Sergot), **perdurantism / 4D ontology**,
**bitemporal modelling / event sourcing**, **W3C PROV** (activities as the spine).

### Strengths

- **Temporal coherence ✅** — history *is* the model; nothing bolted on.
- **Provenance ✅** — every claim is an observing/inferring event with
  participants; PROV is native.
- **Reversibility ✅** — append-only by construction; "change the past" = append an
  earlier-valid-time event.
- **Change ✅** — the whole point; default persistence of fluents is elegant.
- **Scientific reasoning ✅** — the observation→experiment→measurement chain is
  literally a sequence of events.
- **Resistance to semantic drift ✅** — one primitive, hard to abuse.
- **Neutrality ✅** — an event is an event whether it is a practice session, a
  commit, or an experiment.

### Weaknesses

- **"What is true now" ❌** — must be *derived* by replaying/summing events every
  time; conceptually expensive; the personal-assistant "what needs attention"
  query (mission §9) becomes a computation over the whole timeline.
- **Entities that just *are* 🟡** — a concept, a place, a value: modelling them as
  "an event of coming-into-being" is unnatural and often fictional (when did the
  concept "counterpoint" begin for the user?).
- **States with no known cause ❌** — "I've always disliked crowds" has no
  initiating event; the model must invent one or special-case it.
- **User comprehensibility 🟡** — people narrate their lives in events *and* in
  standing facts ("my teacher is X"); an all-events model feels alien for the
  latter.
- **Uncertainty 🟡** — confidence attaches to the *claim that an event occurred*,
  which is awkward for standing beliefs.
- **Intention 🟡** — an intention is not an event; it must be modelled as "an
  event of forming an intention" plus a fluent, which is indirect.

### Failure cases

1. *"What's my teacher's name?"* — a standing relational fact; forcing it through
   an event ("the event of my teacher being named X") is contortion.
2. *"What do I currently believe about my sleep?"* — requires deriving a fluent
   from a long event history; no first-class answer.
3. *"I want to learn category theory."* — an open-ended intention with no plan and
   no event; the model has to represent it as "the event of forming this wish".

### Verdict: **rejected as the whole model, adopted as a layer.**
Event-centrism is *right about history, provenance, and change* and E²CI takes
its `Event` primitive and its bitemporal / initiate-terminate machinery wholesale.
But a life is not *only* a narrative; it also has standing referents and standing
beliefs and open wishes, which event-centrism serves poorly.

---

## Model B — Entity / relationship-centric

**Idea.** The primitives are **Entity** and **Relation** (typed, attributed,
possibly n-ary edges). Events are a *kind of entity* (or a relation over time);
states are attributes or time-stamped relations; the whole is a **personal
knowledge graph** (Balog & Kenter, `[E]`) with **PROV metadata** on edges.
Grounded in: **PKG research**, **property-graph / RDF-star thinking**,
**Bergman & Whittaker's PIM** (navigation over search, `[E]`).

### Strengths

- **User comprehensibility ✅** — matches how people name and connect things;
  "these are the people, places, pieces, projects, and how they relate".
- **Neutrality ✅** — an entity is domain-agnostic; relations are open verbs.
- **Extensibility ✅** — add a kind, add a relation type; no structural change.
- **Navigation-based retrieval ✅** — matches the PIM finding that people prefer
  browsing context to search for their *own* information (`[E]`, MEL-REQ-067).
- **Relationships first-class ✅** — directly answers mission §15.
- **Music ✅** — a score's part-whole structure is a natural graph.
- **Personal life ✅** — people, places, possessions, relationships.

### Weaknesses

- **Temporal coherence ❌** — time is bolted on (valid-time on edges, reification
  for "when did I believe this"); "what happened last Tuesday" is second-class.
- **Change / history 🟡** — versioning edges is possible but clunky; the *narrative*
  ("first I tried X, then Y, then decided Z") is not native.
- **Provenance 🟡** — metadata on edges, not structure; "how do I know this" is a
  property lookup, not a walkable derivation.
- **Epistemic status ❌** — belief vs observation vs hypothesis vs question has no
  natural home; everything tends toward an undifferentiated "fact in the graph"
  (the exact thing mission §14 forbids).
- **Process ❌** — a static graph represents *structure* well and *unfolding*
  poorly; the scientific-reasoning chain becomes a set of nodes with ordering
  edges, losing the sense of a process in motion.
- **Uncertainty 🟡** — a confidence property; conflict = two edges, but nothing
  says "these contradict and are unresolved".
- **Semantic drift 🟡** — "just add another entity kind / relation type" is easy,
  which over time yields a sprawling, inconsistent vocabulary (the PKM
  "maintenance tax", research `[E]`).

### Failure cases

1. *"Walk me through how I concluded the bug was in the parser."* — a reasoning
   process; a graph of claim-nodes with `supports` edges loses the sequence and
   the "I was uncertain here, then this observation shifted me".
2. *"What did I believe about this last month, and what changed my mind?"* —
   requires per-edge bitemporal versioning and a `supersedes` convention that the
   base model doesn't give you.
3. *"Is this a thing I observed or a thing I inferred?"* — no structural
   distinction; relies on a discipline the model doesn't enforce.

### Verdict: **rejected as the whole model, adopted as a layer.**
Entity/relationship-centrism is *right about referents and navigation* and E²CI
takes its `Entity` primitive and its first-class-relations stance (as
`Claim`s of relational content). But it is weak on time, process, provenance-as-
structure, and — critically — the epistemic distinctions the requirements demand.

---

## Model C — State / transition-centric

**Idea.** The primitive is the **State** (the world as a set of currently-holding
fluents); **Events / actions** are transitions between state-sets; **Entities**
are bundles of fluents that travel together; **intentions / goals / commitments /
permissions are simply desired or normative states**. Grounded in: **situation
calculus** (McCarthy & Hayes), **statecharts** (Harel), **STRIPS/PDDL-style
planning** (preconditions/effects).

### Strengths

- **"What's true now" ✅** — the model *is* the current state; the
  personal-assistant "what needs attention / what's unfinished" query is a direct
  read.
- **Intention / goal / commitment ✅** — "a goal is a desired state" is clean and
  unifying; no separate primitive needed; direction of fit is captured as
  "desired vs actual".
- **Planning ✅** — preconditions/effects are native; autonomous planning
  (MEL-REQ-100) fits naturally.
- **Autonomy support ✅** — "authorised" is a state; the triad maps to
  states MELFINA holds.
- **Conceptual simplicity 🟡** — one primitive, but the fluent vocabulary can
  balloon.

### Weaknesses

- **History ❌** — states are snapshots; the *narrative* is lost unless you also
  keep the transition log (at which point Event is back as a primitive).
- **Provenance ❌** — "how do I know this fluent holds" has no home; situation
  calculus assumes a knowing agent, not a fallible one.
- **Hypothetical vs actual time ❌** — situation calculus is tree-like
  (hypothetical action sequences); a real life is one actual line. Adapting it
  (as event calculus did) essentially converts C into A.
- **Entities-as-fluent-bundles ❌** — deeply unintuitive for users ("a person is
  a set of co-travelling properties").
- **Uncertainty 🟡** — a fluent either holds or doesn't; "I'm 60% sure the piece
  is ready" needs an extra layer.
- **Epistemic status ❌** — same problem as B: no belief/observation/hypothesis
  distinction.
- **User comprehensibility ❌** — "your life as a state machine" is the least
  human of the three framings.

### Failure cases

1. *"What was I working on last Tuesday and what did I decide?"* — narrative
   retrieval; a snapshot model has no Tuesday unless it also logged transitions.
2. *"This source says X but I observed not-X."* — conflicting epistemic inputs;
   a fluent can't be both true and false, so the conflict must be pushed into a
   meta-layer the model doesn't have.
3. *"How confident am I that I'll be ready?"* — degrees, not a boolean fluent.

### Verdict: **rejected as the whole model; its *insight* adopted.**
State/transition-centrism is *right that "what holds now" and "intentions are
desired states" deserve first-class treatment*. E²CI adopts the insight two ways:
(1) "what holds now" = the set of currently-valid descriptive `Claim`s (a
first-class *query*, not a stored blob); (2) intentions are a primitive
(`Intention`), and goals/commitments/permissions are cases of it or of
descriptive claims. What E²CI does **not** adopt is making `State` *the* primitive
— because history, provenance, and epistemic status then have nowhere to live.
**If prototyping shows the "currently-valid-claims" query is too central and too
awkward as a derivation, `State` returns as a fifth primitive** (OQ-M2).

---

## Model D — Three primitives (aggressive reduction)

**Idea.** `Entity`, `Event`, `Claim` only. Fold `Intention` into `Claim`: an
intention is "a `Claim` with a special modality (`intended`) about a future
condition."

### Strengths

- **Maximum simplicity** — three primitives; one-maintainer comprehension
  (MEL-REQ-192) is easiest.
- **Everything is a claim or a happening or a thing** — a clean slogan.

### Weaknesses / why rejected

- **Direction of fit is erased (`[E]`).** "I believe I'll finish" and "I intend to
  finish" become the same shape with a different tag. Failure of an intention is
  then a *falsified claim*, which imports "success/failure" semantics the
  requirements forbid (MEL-REQ-008, P-3) and which is exactly wrong for the user
  profile (perfectionism, research `[E]`).
- **The agency pipeline collapses.** USER-intends / MELFINA-proposes /
  USER-authorises (mission §13) all become "claims with modalities", losing the
  crisp separation the autonomy triad needs (MEL-REQ-017) — a proposed intention
  and an authorising decision are genuinely different acts, not different claim
  tags.
- **"What matters to the user" loses its anchor.** `Intention` is the model's main
  handle on what the user is trying to do; demoting it to a claim modality buries
  it.
- **Semantic drift risk rises**, not falls: `Claim` becomes an over-loaded
  catch-all ("claim with modality X" for every X), which is drift by another name
  (P-13).

### Verdict: **rejected.** Three primitives is *possible* but not *sufficient*
(mission §4: minimum sufficient, not minimum possible). The `[E]` distinction of
direction of fit, plus the explicit requirement to model agency without a task
primitive, make `Intention` earn its place.

---

## Model E²CI — the synthesis (LEADING)

**`Entity` · `Event` · `Claim` · `Intention`**, situated in bitemporal **`Time`**,
with **`Context`** derived. Full definition in `HUMAN_CENTRAL_MODEL.md`.

- **`Entity`** — from Model B (referents; open `kind`; thin entities are fine).
- **`Event`** — from Model A (single actual timeline; initiate/terminate; PROV
  participation & derivation; expected future events).
- **descriptive `Claim` = "what holds"** — Model C's insight, as a *query over
  claims* rather than a stored state blob.
- **`Claim` (the epistemic layer)** — from epistemic/doxastic logic + PROV + AGM
  (`[E]`): status (observed/reported/inferred/hypothesised/open/stipulated/
  decided/recalled), provenance, confidence, bitemporal, supersedes. **None of
  A/B/C carries this well** — it is the synthesis's main addition.
- **`Intention`** — kept as a primitive (direction of fit, `[E]`); goals /
  commitments / tasks / plans / routines / reminders are cases.
- **`Time`** — bitemporal dimension (from A); Allen relations for qualitative
  temporal talk.
- **`Context`** — derived per situation by a relevance trade-off (Sperber &
  Wilson, `[E]`); never stored (required by PART IV-B).

### Why a synthesis is necessary (not just preferable)

The mission's quality criteria are **jointly** required, and they pull toward
different base models:

| Criterion cluster | Favoured base | In E²CI from |
|---|---|---|
| temporal coherence · reversibility · change · history | A | `Event` + bitemporal `Time` |
| referents · navigation · relationships · user comprehensibility | B | `Entity` + relations-as-claims |
| "what holds now" · intentions/goals · planning · autonomy | C | descriptive-`Claim` query + `Intention` primitive |
| **provenance-as-structure · epistemic status · belief revision · "I don't know"** | **none** | the `Claim` epistemic layer (new) |

No two-of-four base model exists; the epistemic layer is uncovered by all three.
Hence synthesis.

### E²CI scored against all criteria

| Criterion | A | B | C | **E²CI** |
|---|---|---|---|---|
| conceptual simplicity | ✅ | 🟡 | 🟡 | 🟡 (4 primitives) |
| expressive power | 🟡 | 🟡 | 🟡 | ✅ |
| neutrality | ✅ | ✅ | ✅ | ✅ |
| extensibility | 🟡 | ✅ | 🟡 | ✅ (open `kind`, open relations) |
| temporal coherence | ✅ | ❌ | ❌ | ✅ |
| provenance | ✅ | 🟡 | ❌ | ✅ |
| uncertainty | 🟡 | 🟡 | 🟡 | ✅ (4 kinds, §9) |
| reversibility | ✅ | 🟡 | ❌ | ✅ (append-only + supersedes) |
| user comprehensibility | 🟡 | ✅ | ❌ | 🟡→✅ (entities+events+wishes match how people talk) |
| dynamic-reasoning support | 🟡 | 🟡 | ✅ | ✅ (context derived; strategies = claims) |
| autonomy support | 🟡 | ❌ | ✅ | ✅ (triad naturally separated, §10) |
| teaching support | 🟡 | 🟡 | 🟡 | ✅ (§13.2) |
| scientific-reasoning support | ✅ | 🟡 | 🟡 | ✅ (§13.4) |
| software-engineering support | 🟡 | 🟡 | 🟡 | ✅ (§13.3) |
| music support | 🟡 | ✅ | 🟡 | ✅ (§13.1, zero music primitives) |
| personal-life support | 🟡 | ✅ | 🟡 | ✅ |
| long-term maintainability | ✅ | 🟡 | 🟡 | 🟡 (4 primitives is more than 1, but each has one clear job) |
| resistance to semantic drift | ✅ | 🟡 | 🟡 | 🟡 (guarded by P-13: emergent concepts are *named patterns*, not new primitives) |

### E²CI's own weak spots (honest)

- **conceptual simplicity 🟡** — four primitives + a bitemporal dimension + a
  derived-context notion is more to hold than Model A's one. Mitigation: each
  primitive has exactly one job; the *emergent* concepts (§13) are where the
  richness lives, and they cost nothing structurally.
- **`State` folded, not primitive** — a real bet (§3.3); Model C's advocates would
  keep it. OQ-M2.
- **`Relation` folded** — Model B's advocates would keep it. OQ-M1.
- **user comprehensibility** — depends entirely on how it is *presented* (an
  INTERFACE-phase concern); the primitives themselves are close to ordinary
  language ("things, what happened, what I think, what I want") but "Claim with
  epistemic status" needs friendly surfacing.
- **not runnable** — like all four, E²CI can only be validated by prototypes and
  use (mission §26; `MODEL_OPEN_QUESTIONS.md`).

---

## Comparison summary

| | A event-centric | B entity/relation | C state/transition | D three-primitive | **E²CI (leading)** |
|---|---|---|---|---|---|
| primitives | 1 (`Event`) | 2 (`Entity`,`Relation`) | 1 (`State`) | 3 | **4** |
| history / change | ✅ | 🟡 | ❌ | ✅ | ✅ |
| referents / relations | ❌ | ✅ | ❌ | 🟡 | ✅ |
| "what holds now" | ❌ | 🟡 | ✅ | 🟡 | ✅ (as a query) |
| epistemic status / provenance | 🟡 | 🟡 | ❌ | ✅ | ✅ |
| agency without "task" | 🟡 | ❌ | ✅ | ❌ | ✅ |
| dynamic MELFINA, tech-neutral | 🟡 | 🟡 | ✅ | 🟡 | ✅ |
| user comprehensibility | 🟡 | ✅ | ❌ | 🟡 | 🟡→✅ |
| **overall sufficiency** | insufficient | insufficient | insufficient | insufficient | **sufficient (pending prototype)** |

**One clearly superior model? No.** **A synthesis is necessary.** E²CI is that
synthesis and is the leading model, subject to the open questions (chiefly:
is `State` a fifth primitive; is `Relation`; is four genuinely minimal-sufficient).
