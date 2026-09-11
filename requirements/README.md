# requirements/

The user's real requirements for MELFINA — what it must, should, may, and must not
do. Derived from research (`research/`), the user's stated priorities, and the
PERSONAL REQUIREMENTS MISSION 001 + REQUIREMENTS EXPANSION MISSION 002 briefs.

**Not** architecture. **Not** implementation. **No** technology choices (language,
storage, framework, model, UI). Those come later, and only after the HUMAN /
CENTRAL MODEL phase.

## Status: MISSION 001 + MISSION 002 complete (first pass), pending user review

**MISSION 001** = base brief (30 requirement areas, MUST/SHOULD/MAY/MUST NOT, IDs,
conflicts, anti-requirements, musician requirements, AI requirements) +
**autonomous-reasoning addendum** (cognitive / decision / execution autonomy) +
**local-only / network-isolation addendum** + **autonomous-personal-intelligence
extension** (personal assistant, autonomous automation, adaptation, skills,
scientific thinking, teaching, world understanding, deep reasoning, emotional
understanding, software engineering, computer agency, controlled self-improvement,
general-capability, extreme-capability-+-lightweightness, local-only foundation,
safety/agency boundary, capability model) + **dynamic-self-directed correction**
(PART IV-B: metareasoning / dynamic strategy, context, and skill selection;
capability-gap recognition and capability creation; human–MELFINA co-creation; a
graded nine-tier self-modification model with the meta-invariant; dynamic resource
allocation and dynamic autonomy; self-evaluation; "best is situational, dynamic ≠
unpredictable").

**MISSION 002** (2026-09-11) = **REQUIREMENTS EXPANSION MISSION 002** — adds
**PART IV-C** (§56–76): genuine internal decision ownership/rationale/
reconsideration (strengthens the autonomy triad without expanding execution
authority); cross-domain foundational knowledge within the existing E²CI ontology;
a closed analogy/correlation/causation vocabulary; "wisdom" as a bounded,
falsifiable, non-mystical capability target; a generalised adversarial-systems-
thinking lens; first-principles decomposition; mathematical abstraction; natural-
language↔math↔code translation; philosophical programming; programming as a
universal modelling tool (still fully gated by the action pipeline); extended
scientific reasoning, independent-derivation/novelty discipline, and teaching;
knowledge acquisition that stays inside the local-only core; self-evaluation via
external verification; structured (not scored) ethics/law/civics reasoning;
art/music/martial-arts/embodied-skill and physiology/biology/cognitive-science
modelling; and — the two load-bearing safety sections — **knowledge ≠ authority**
and a dedicated **intelligence-must-not-become-a-security-bypass** hardening set
mapped to the mission's full attack list.

### Files

| File | What it is |
|---|---|
| **`REQUIREMENTS_MASTER.md`** | The spec. Part I foundations · Part II the **MELFINA Capability Model** (15 capabilities — a *vocabulary, not a fixed catalogue*) + the **autonomy triad** (cognitive/decision/execution) + **action pipeline** (THINK→DECIDE→PROPOSE→AUTHORISE→EXECUTE→VERIFY) · Parts III–IV functional + autonomous-intelligence requirements · **PART IV-B the DYNAMIC SELF-DIRECTED SYSTEM** (metareasoning, dynamic context/skill selection, capability creation/evolution, co-creation, the nine self-change tiers + the meta-invariant, dynamic resource/autonomy, self-evaluation, "best is situational") · **PART IV-C GENERAL REASONING, KNOWLEDGE, AND WISDOM** (§56–76 — decision ownership, cross-domain knowledge, analogy/causation discipline, wisdom, adversarial-systems lens, first-principles reasoning, mathematical abstraction, NL↔math↔code, philosophical programming, programming-as-modelling, extended scientific reasoning/derivation/teaching, knowledge acquisition, external self-evaluation, ethics/law/civics, art/music/martial-arts/embodied skill, physiology/biology/cognition, **knowledge ≠ authority**, and the **anti-bypass hardening set**) · Part V cross-cutting · Part VI feasibility classification · Part VII **anti-requirements** · `REQUIREMENTS CHECKPOINT 001` + `REQUIREMENTS CHECKPOINT 002`. |
| **`KNOWLEDGE_AND_REASONING_MODEL.md`** | Companion to PART IV-C: the operational definition of "wisdom" + its critique, the analogy/correlation/causation taxonomy (grounded in Gentner's structure-mapping theory and Pearl's causal ladder), the first-principles decomposition pipeline, the architecture/model/foundation compatibility test, a dedicated adversarial review (attacks A–T), and the research grounding for mission areas A–Z with evidence tags. |
| **`CONFLICTS.md`** | The tensions (10 research-level RC1–RC10 + 7 vision-level VC1–VC7 + 5 dynamic-system VC8–VC12 + 6 knowledge/wisdom VC13–VC18) expressed as configurable dimensions / constraints / safeguards / open questions — not resolved by fiat. |
| **`OPEN_QUESTIONS.md`** | OQ-1 … OQ-32 — what the requirements phase cannot answer, deferred to HUMAN/CENTRAL MODEL, ARCHITECTURE, CORE ENGINE, or REAL-WORLD USE. Plus the deliberately-undecided list. |

### Requirement IDs

`MEL-REQ-001` … `MEL-REQ-364` (functional + cross-cutting + dynamic self-directed
+ general reasoning/knowledge/wisdom).
`MEL-AR-01` … `MEL-AR-27` (anti-requirements).
IDs are unique and stable; gaps are not reused.

### Priority discipline

MUST (186) · SHOULD (116) · MAY (8) · MUST NOT (54) + 27 anti-requirements.
Most requirements are SHOULD or MAY. MUST is reserved for core identity, safety,
local-only operation, user control, predictability, the hazard-surface
restraints, the autonomy/permission model, the dynamic-system bounds
(grounding factors + the meta-invariant + "dynamic ≠ unpredictable"), and — as of
MISSION 002 — the **knowledge ≠ authority** separation and the anti-bypass set
(§74–76), which are exactly as absolute as the safety MUSTs they restate.

### Evidence discipline

Tiers **[E]** evidence · **[G]** guidance · **[DI]** design inference · **[H]**
hypothesis · **[U]** unknown. A `[U]` or `[H]` finding never becomes a MUST/SHOULD
on its own — it becomes a MAY, a hypothesis to validate, or an open question.
Provenance classes **A** evidence · **B** user-goal · **C** hypothesis · **D**
unknown · **E** deliberately-undecided.

### How to read

1. `REQUIREMENTS_MASTER.md` §0 (what this is / is not, tiers, format), then the
   MUST set in `REQUIREMENTS CHECKPOINT 001` and `REQUIREMENTS CHECKPOINT 002`.
2. Part II — the capability model (a vocabulary, not a catalogue) and the autonomy
   triad.
3. **Part IV-B — the dynamic self-directed system** (the conceptual correction:
   MELFINA determines what capabilities/context/reasoning/tools a problem needs
   and, where authorised, builds new ones).
4. **Part IV-C — general reasoning, knowledge, and wisdom** — read §74
   ("knowledge ≠ authority") and §75–76 (the anti-bypass set) first; they are the
   requirements that keep everything else in this Part from becoming an
   authority-expansion path.
5. Part VII — the 27 anti-requirements.
6. `CONFLICTS.md` and `OPEN_QUESTIONS.md` — argue with these.
7. Dive into Parts III–V for any specific requirement; each traces to research or
   a stated goal.

### Next phase

**HUMAN / CENTRAL MODEL, ARCHITECTURE MISSION 001, and LOW-LEVEL FOUNDATIONS
MISSION 001 are already complete** (see `PROJECT_STATE.md`). MISSION 002 was
checked for compatibility against all of them (`KNOWLEDGE_AND_REASONING_MODEL.md`
§4) and found no contradiction requiring a change to any of them. The user should
review MISSION 002 — especially §74–76 and `CONFLICTS.md` VC13–VC18 — before it
folds into the next TECHNOLOGY SELECTION or CORE ENGINE mission's scope.
