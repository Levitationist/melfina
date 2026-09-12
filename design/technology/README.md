# MELFINA — TECHNOLOGY SELECTION

**Phase:** TECHNOLOGY SELECTION MISSION 001. **Status:** first pass complete,
pending human review. **Baseline:** requirements `4f5e97c`, foundations
`19d0135`, architecture `8a40207`. **Rubric:**
`design/foundations/TECHNOLOGY_SELECTION_CRITERIA.md` (F12).

**This is still a research and recommendation phase.** No implementation,
no `src/`, no dependency installed. Every document here names, for each
technology category: the candidates evaluated, the MUST/SHOULD/MUST-NOT
scoring, the recommendation, the alternative, the rejected options and why,
the residual risk, and whether it requires explicit human sign-off before
CORE ENGINE proceeds.

**Start with `TECHNOLOGY_SELECTION.md`** — the cross-cutting synthesis:
overall stack, hard-gate results, F1–F10 compatibility, offline-core/AI-
boundary/self-modification-boundary tests, TCB analysis, and the explicit
human-decision boundary. Read it before the category documents below.

## Documents

| File | Covers |
|---|---|
| **`TECHNOLOGY_SELECTION.md`** | The synthesis — read this first |
| `LANGUAGE_EVALUATION.md` | Primary language, supporting languages, multi-language architecture |
| `CHRONICLE_EVALUATION.md` | Chronicle authoritative substrate, physical record format, current-state cache |
| `ISOLATION_EVALUATION.md` | Ring-3 isolation mechanism (default + escalated tiers) |
| `PROCESS_AND_IPC_EVALUATION.md` | Process topology, Reference Monitor's process boundary, IPC, concurrency model |
| `TERMINAL_GUI_EVALUATION.md` | Terminal/process execution, filesystem access, GUI control, resource limiting |
| `CRYPTO_GOVERNANCE_EVALUATION.md` | Governance integrity: signature scheme, hash function, key custody, rollback protection |
| `REASONING_COMPUTATION_EVALUATION.md` | Verifier execution, math/symbolic and numerical substrates, local AI runtime, capability packaging |
| `BUILD_AND_SUPPLY_CHAIN.md` | Build system, test/diagnostic tooling, packaging, supply chain, license notes, platform strategy, resource-efficiency recap |
| `MIGRATION_PORTABILITY.md` | What transfers across machines, what must not, the TPM/migration interaction |
| `TECHNOLOGY_ADVERSARIAL_REVIEW.md` | 36 named attacks against the chosen technologies specifically |
| `TECHNOLOGY_EXPERIMENT_PLAN.md` | The concrete, prioritised experiments CORE ENGINE should run before trusting each choice; formal-verification opportunities |
| `TECHNOLOGY_DECISION_LOG.md` | The auditable index of every recommendation: alternative, reversal cost, confidence, approval requirement |

## What this mission does NOT do

Choose a final, irrevocable stack unilaterally; implement anything; create
`src/`; install dependencies; resolve any open model or requirements
question; weaken any foundation contract (F1–F10) or requirement
(`MEL-REQ`/`MEL-AR`); silently redefine the architecture to fit a convenient
technology.

## Next step

Human review of the decisions flagged **YES** in
`TECHNOLOGY_SELECTION.md` §14, then CORE ENGINE — beginning with the
experiments in `TECHNOLOGY_EXPERIMENT_PLAN.md`, in the priority order given
there.
