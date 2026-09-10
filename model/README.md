# model/

The human / central life model for MELFINA: the smallest coherent set of
primitives from which the user's whole life — and MELFINA's many capabilities —
can be described.

**Not** architecture. **Not** a schema, data structure, or file format. **No**
database, language, framework, or engine choice — and none implied. A conceptual
model must survive any implementation technology.

## Status: HUMAN / CENTRAL MODEL MISSION 001 complete (first pass), pending user review

### Files

| File | What it is |
|---|---|
| **`HUMAN_CENTRAL_MODEL.md`** | The model. 18 sections: purpose · design principles · the reduction from ~25 candidates to 4 primitives · primitive definitions · relationships · time · context · provenance · uncertainty · agency · change/history · MELFINA's own state · emergent higher-level concepts · 7 test cases (music, learning, software engineering, scientific reasoning, personal assistant, dynamic self-directed system, human+MELFINA co-creation) · limitations · confidence · deliberately-undecided · `MODEL CHECKPOINT 001`. |
| **`MODEL_ALTERNATIVES.md`** | Four candidate models compared rigorously — **A** event-centric, **B** entity/relationship-centric, **C** state/transition-centric, **D** three-primitive — with strengths, weaknesses, failure cases, and why each was rejected as a whole. Concludes that a **synthesis (E²CI)** is necessary. |
| **`MODEL_OPEN_QUESTIONS.md`** | `OQ-M1 … OQ-M12`: unresolved semantic questions, concepts that may be primitives but aren't proven, concepts intentionally left derived (with the risk), and what needs prototypes / real-world use / future research. |

### The leading model — E²CI

Four primitives:

| Primitive | Is | Grounded in |
|---|---|---|
| **ENTITY** | anything that persists and can be referred to (person, place, thing, work, concept, capability, workflow, source, `self`, `melfina` — `kind` is an open attribute) | BFO continuant; personal knowledge graphs |
| **EVENT** | something that occurs at/over time, with participants in roles, possibly bringing about or ending conditions (includes observations, actions, sessions, decisions, and *expected* future events) | BFO occurrent; event calculus; W3C PROV |
| **CLAIM** | a statement held by an agent, with an origin and an epistemic **status** (observed / reported / inferred / hypothesised / open / stipulated / decided / recalled), a **confidence**, full **provenance**, and **bitemporal** coordinates (valid-time / transaction-time). *All of MELFINA's knowledge is claims.* Relationships and "what holds now" (state) are claims. | epistemic/doxastic logic; W3C PROV; AGM belief revision; bitemporal modelling |
| **INTENTION** | an agent's directedness toward a future condition or action — the world-to-mind stance (distinct from a belief by *direction of fit*). Goals, tasks, plans, commitments (an intention with a creditor), routines, and reminders are cases. | Anscombe / Searle direction of fit; BDI; Castelfranchi social commitment |

Plus: **TIME** is a bitemporal *dimension* (not a primitive; "calendar" dissolves
into a view of events on a time axis). **CONTEXT** is *derived per situation* by a
relevance trade-off (Sperber & Wilson) — never stored.

Every application category — task, habit, note, calendar, project, goal,
reminder, journal, dashboard, practice session, experiment, decision, hypothesis,
repertoire piece, capability, automation, strategy — is a **view or pattern** over
the four primitives, not a primitive itself.

### Key decisions (argue with these)

- **`State` is folded into `Claim`** (a descriptive claim about a holding
  condition), not a fifth primitive — `MODEL_OPEN_QUESTIONS.md` OQ-M2.
- **`Relation` is folded into `Claim`** (a claim of relational content, carrying
  its own time/provenance/confidence) — OQ-M1.
- **`Intention` is kept** (direction of fit) — folding it (Model D) is rejected.
- **Four, not three** (insufficient) and **not obviously five/six** (not clearly
  necessary) — OQ-M8, a prototyping question.

### What the model deliberately does NOT contain

- The self-modification meta-invariant rules (`requirements` MEL-REQ-235) —
  external to MELFINA's reasoning by design; if they were claims here, MELFINA
  could reason about editing them.
- Any enforcement mechanism (permissions are *represented*, not *guarded*).
- Any evaluative verdict the user did not enter.
- Any diagnostic category — ADHD / autism / OCD are **not** reified.
- Any implementation choice.

### How to read

1. `HUMAN_CENTRAL_MODEL.md` §1–§4 (purpose, principles, the reduction, the four
   primitive definitions).
2. `MODEL_ALTERNATIVES.md` — why not just one of A/B/C.
3. `HUMAN_CENTRAL_MODEL.md` §13 — the test cases (music first).
4. `MODEL_OPEN_QUESTIONS.md` — what's unsettled.
5. `HUMAN_CENTRAL_MODEL.md` `MODEL CHECKPOINT 001` — the summary.

### Next phase

**SYSTEM DESIGN / ARCHITECTURE** — but **only after** this model has been reviewed
and explicitly accepted. No architecture, technology, or implementation begins
before that.
