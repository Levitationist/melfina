# research/

Deep-research findings for MELFINA. Raw material for later phases — **not**
requirements, design, or architecture.

## Status: RESEARCH MISSION 002 complete (second, deeper pass), pending review

- **Mission 001** — first scoping synthesis. Accepted as a checkpoint.
- **Mission 002** — deeper pass targeting Mission 001's evidence gaps: adult-specific
  evidence, the three-way overlap, musician/pianist context, CSCW/PIM, PKM &
  cognitive-offloading trade-offs, controlled adult assistive-tech evidence,
  notification/reminder evidence, AI compulsion risk, self-tracking, academic prior
  art, and an evidence-quality audit of Mission 001. Lives in
  `RESEARCH_MASTER.md` **§§20–32 + `RESEARCH CHECKPOINT 002`**.

Two Mission-001 claims were **corrected** in Mission 002 (notification batching;
alert-fatigue magnitude); several refined; one partly downgraded to `[U]`
(implementation intentions for adults with ADHD). None disproven. Details in
`RESEARCH_MASTER.md` §31.

## RESEARCH MISSION 001 — summary

**Question:** How should a personal computing system be designed to reduce
cognitive friction and support daily functioning for a person with lived
experience of ADHD, autism, and OCD, while avoiding designs that reinforce harmful
patterns?

Not a medical project. Does not diagnose. Does not prescribe treatment. Does not
describe the user's individual condition.

### Files

| File | What it is |
|------|-----------|
| **`RESEARCH_MASTER.md`** | The report. §§1–19: Mission 001 scoping synthesis (exec summary, methodology, ADHD / autism / OCD / overlap, cognitive science, HCI, assistive tech, productivity failure modes, gamification, privacy/local-first, AI, prior art, personalisation, consolidated design implications, open questions, preliminary principles, and the `RESEARCH FINDING → POSSIBLE DESIGN IMPLICATION → [STOP]` boundary). §§20–32 + `RESEARCH CHECKPOINT 002`: Mission 002 deeper pass + evidence-quality audit. |
| **`CONFLICTS.md`** | Working doc expanding MASTER §6.3 — the 10 catalogued tensions where a feature helpful for one need harms another, with evidence on both sides and candidate (un-chosen) resolution approaches. Plus a **Second-Pass Evidence Updates** section. This is the main input to the requirements phase. |
| **`BIBLIOGRAPHY.md`** | References with provenance tags — *(verified)* / *(abstract/summary)* / *(background)* / *(primary-verified)* / *(search-summary)* — plus a **Second Pass Additions** section and remaining gaps. |

### How to read

1. `RESEARCH_MASTER.md` §1 (executive summary) and §19 (the boundary) first.
2. `RESEARCH CHECKPOINT 002` (end of the file) for what the second pass changed —
   strengthened, corrected, contradictions, gaps, confidence levels.
3. `CONFLICTS.md` for the decisions the next phase has to make.
4. `RESEARCH_MASTER.md` §16 and §18 for the consolidated implications and
   provisional principles; §31 for the audit of Mission 001's claims.
5. Dive into §§3–§15 and §§20–32 for the evidence behind any specific claim;
   follow the `Author year` refs into `BIBLIOGRAPHY.md`.

### Evidence tiers used throughout

**[E]** evidence · **[G]** clinical/professional guidance · **[DI]** design
inference · **[H]** hypothesis · **[U]** unknown / insufficient evidence (added in
Mission 002). Conflicting evidence and evidence gaps are called out inline;
contradictions are preserved, not resolved.

### Status / caveats

- Scoping synthesis, not a systematic review. Web search, Sept 2026.
- Some key papers still reached only as abstracts (paywalls) — flagged in the bib.
- Two Mission-001 primary sources retrieved in full in Mission 002 (Demetriou 2018;
  Macnamara & Maitra 2019).
- Largest remaining gap: **no direct evidence on the ADHD + autism + OCD
  combination** on any design-relevant variable, and little controlled
  assistive-tech evidence in adults with these conditions specifically.
- Nothing here is a requirement, a feature, or an architectural commitment.
