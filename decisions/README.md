# decisions/

Lightweight decision records. One file per decision:
`NNNN-short-title.md` (e.g. `0001-use-plain-text-storage.md`).

Suggested structure per file:

```
# NNNN — Title

Date: YYYY-MM-DD
Status: proposed | accepted | superseded by NNNN

## Context
What forced the decision. Which principles are in tension.

## Decision
What was chosen.

## Consequences
What this makes easy, what it makes hard, what it rules out.
```

Keep them short. A decision that violates a principle in `PROJECT_STATE.md`
must say so explicitly here.
