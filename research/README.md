# research/

Deep-research findings for MELFINA. Raw material for later phases — **not**
requirements, design, or architecture.

## RESEARCH MISSION 001 — complete (first pass, pending review)

**Question:** How should a personal computing system be designed to reduce
cognitive friction and support daily functioning for a person with lived
experience of ADHD, autism, and OCD, while avoiding designs that reinforce harmful
patterns?

Not a medical project. Does not diagnose. Does not prescribe treatment. Does not
describe the user's individual condition.

### Files

| File | What it is |
|------|-----------|
| **`RESEARCH_MASTER.md`** | The report. 19 sections: exec summary, methodology, ADHD / autism / OCD / overlap findings, cognitive science, HCI, assistive tech, productivity failure modes, gamification, privacy/local-first, AI, prior art, personalisation, consolidated design implications, open questions, preliminary principles, and the explicit `RESEARCH FINDING → POSSIBLE DESIGN IMPLICATION → [STOP]` boundary. |
| **`CONFLICTS.md`** | Working doc expanding MASTER §6.3 — the 10 catalogued tensions where a feature helpful for one need harms another, with evidence on both sides and candidate (un-chosen) resolution approaches. This is the main input to the requirements phase. |
| **`BIBLIOGRAPHY.md`** | References, each marked *(verified)* / *(abstract/summary)* / *(background)* for provenance, plus known gaps for a second pass. |

### How to read

1. `RESEARCH_MASTER.md` §1 (executive summary) and §19 (the boundary) first.
2. `CONFLICTS.md` for the decisions the next phase has to make.
3. `RESEARCH_MASTER.md` §16 and §18 for the consolidated implications and
   provisional principles.
4. Dive into §3–§15 for the evidence behind any specific claim; follow the
   `Author year` refs into `BIBLIOGRAPHY.md`.

### Evidence tiers used throughout

**[E]** evidence · **[G]** clinical/professional guidance · **[DI]** design
inference · **[H]** hypothesis. Conflicting evidence and evidence gaps are called
out inline.

### Status / caveats

- Scoping synthesis, not a systematic review. Web search, Sept 2026.
- Some key papers reached only as abstracts (paywalls) — flagged in the bib.
- A second pass with library-database access is recommended before requirements
  (see `RESEARCH_MASTER.md` §17 and the bib's "known gaps").
- Nothing here is a requirement, a feature, or an architectural commitment.
