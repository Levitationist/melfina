# requirements/

The user's real requirements for MELFINA — what it must, should, may, and must not
do. Derived from research (`research/`), the user's stated priorities, and the
PERSONAL REQUIREMENTS MISSION 001 briefs.

**Not** architecture. **Not** implementation. **No** technology choices (language,
storage, framework, model, UI). Those come later, and only after the HUMAN /
CENTRAL MODEL phase.

## Status: PERSONAL REQUIREMENTS MISSION 001 complete (first pass), pending user review

Mission = base brief (30 requirement areas, MUST/SHOULD/MAY/MUST NOT, IDs,
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

### Files

| File | What it is |
|---|---|
| **`REQUIREMENTS_MASTER.md`** | The spec. Part I foundations · Part II the **MELFINA Capability Model** (15 capabilities — a *vocabulary, not a fixed catalogue*) + the **autonomy triad** (cognitive/decision/execution) + **action pipeline** (THINK→DECIDE→PROPOSE→AUTHORISE→EXECUTE→VERIFY) · Parts III–IV functional + autonomous-intelligence requirements · **PART IV-B the DYNAMIC SELF-DIRECTED SYSTEM** (metareasoning, dynamic context/skill selection, capability creation/evolution, co-creation, the nine self-change tiers + the meta-invariant, dynamic resource/autonomy, self-evaluation, "best is situational") · Part V cross-cutting · Part VI feasibility classification · Part VII **anti-requirements** · `REQUIREMENTS CHECKPOINT 001`. |
| **`CONFLICTS.md`** | The tensions (10 research-level RC1–RC10 + 7 vision-level VC1–VC7 + 5 dynamic-system VC8–VC12) expressed as configurable dimensions / constraints / safeguards / open questions — not resolved by fiat. |
| **`OPEN_QUESTIONS.md`** | OQ-1 … OQ-21 — what the requirements phase cannot answer, deferred to HUMAN/CENTRAL MODEL, ARCHITECTURE, or REAL-WORLD USE. Plus the deliberately-undecided list. |

### Requirement IDs

`MEL-REQ-001` … `MEL-REQ-253` (functional + cross-cutting + dynamic self-directed).
`MEL-AR-01` … `MEL-AR-19` (anti-requirements).
IDs are unique and stable; gaps are not reused.

### Priority discipline

MUST (~120) · SHOULD (~90) · MAY (~20) · MUST NOT (~23 incl. anti-requirements).
Most requirements are SHOULD or MAY. MUST is reserved for core identity, safety,
local-only operation, user control, predictability, the hazard-surface
restraints, the autonomy/permission model, and the dynamic-system bounds
(grounding factors + the meta-invariant + "dynamic ≠ unpredictable").

### Evidence discipline

Tiers **[E]** evidence · **[G]** guidance · **[DI]** design inference · **[H]**
hypothesis · **[U]** unknown. A `[U]` or `[H]` finding never becomes a MUST/SHOULD
on its own — it becomes a MAY, a hypothesis to validate, or an open question.
Provenance classes **A** evidence · **B** user-goal · **C** hypothesis · **D**
unknown · **E** deliberately-undecided.

### How to read

1. `REQUIREMENTS_MASTER.md` §0 (what this is / is not, tiers, format), then the
   MUST set in `REQUIREMENTS CHECKPOINT 001`.
2. Part II — the capability model (a vocabulary, not a catalogue) and the autonomy
   triad.
3. **Part IV-B — the dynamic self-directed system** (the conceptual correction:
   MELFINA determines what capabilities/context/reasoning/tools a problem needs
   and, where authorised, builds new ones) and the checkpoint's dynamic-system
   subsection.
4. Part VII — the 19 anti-requirements.
5. `CONFLICTS.md` and `OPEN_QUESTIONS.md` — argue with these.
6. Dive into Parts III–V for any specific requirement; each traces to research or
   a stated goal.

### Next phase

**HUMAN / CENTRAL MODEL** — determine the neutral primitive set (OQ-1, OQ-2). Not
started until the user authorises it. Architecture and technology choices remain
out of scope until SYSTEM DESIGN / ARCHITECTURE.
