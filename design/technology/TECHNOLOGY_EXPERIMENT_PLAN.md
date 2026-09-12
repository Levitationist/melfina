# MELFINA — TECHNOLOGY SELECTION: EXPERIMENT PLAN AND FORMAL VERIFICATION
OPPORTUNITIES

**Phase:** TECHNOLOGY SELECTION MISSION 001. First pass, pending human review.
Scope: mission §38 (experiments for uncertain choices) and §39 (formal
verification opportunities). **No experiment listed here has been run. No
implementation has been performed.** This document defines what CORE ENGINE
should test *before* trusting each technology choice with real data or real
authority, in priority order.

---

## 1. Experiments, in priority order

| # | Experiment | Why it matters | What would change the recommendation |
|---|---|---|---|
| 1 | **Crash/power-loss injection on the Chronicle's authoritative log** (kill the writing process mid-`fsync`, mid-frame, simulate a torn write at every possible byte offset) | The custom append-only log (`CHRONICLE_EVALUATION.md`) is the one piece of genuinely new, unfuzzed code in this whole recommendation set — this is explicitly named as the highest-priority open risk in that document | Frequent, unrecoverable corruption under this test would trigger the recorded fallback: LMDB as the authoritative substrate instead |
| 2 | **Fuzz the Chronicle frame parser, the structured-action parser (F2), and the grant-token parser (F1)** with `cargo fuzz` | These are the three parsers sitting directly on trust boundaries (F3 §7's "fail closed on malformed input" doctrine) | A parser that panics, hangs, or misparses under fuzzing instead of cleanly rejecting is a defect to fix before any further trust is placed in it |
| 3 | **Attempt a symlink/TOCTOU race against `openat2`-protected file access** | Directly tests `TERMINAL_GUI_EVALUATION.md` §1–2's core claim | A successful race would mean the `RESOLVE_NO_SYMLINKS` usage is wrong, not that the approach is wrong — fix the implementation, not the mechanism |
| 4 | **Attempt a network syscall from inside a default-tier sandbox** (`socket`, `connect`, `bind`, and less obvious paths — e.g. `sendmsg` on an already-open fd, `io_uring`-mediated network operations) | Directly tests F7 P3's "structurally withheld, not merely unconfigured" claim | A successful network call from inside the sandbox would mean the seccomp filter is incomplete — expand the filter, and treat this as an ongoing review item as new syscalls are added anywhere in the allow-list (`TECHNOLOGY_ADVERSARIAL_REVIEW.md` AD) |
| 5 | **Revoke a grant during an in-flight multi-step capability execution** (attack I) | Tests F4 §3's atomicity claim under real concurrency, not just on paper | Confirms whether the single-writer serialisation point genuinely closes the race, or whether a specific effect kind needs its own compensating logic |
| 6 | **Attempt a shell-metacharacter injection through an argv element** (attack K), and **a manipulated-`PATH` attack** (attack L) | Confirms `execve`-only invocation and the explicit env allow-list actually behave as claimed in the real implementation, not only in the design | Any success here indicates an implementation bug (e.g. an accidental `sh -c` somewhere), not a design flaw |
| 7 | **Stress-test the P0↔P1↔P2 IPC channel**: kill each process while the others are mid-operation; flood P0 with authorisation requests; attempt to open a connection to P0 from an unexpected peer | Validates the process-topology recommendation (`PROCESS_AND_IPC_EVALUATION.md`) and the peer-credential authentication claim | Peer-credential bypass or an unhandled crash cascade would require hardening the IPC layer before trusting the topology |
| 8 | **Attempt to fabricate or replay a capability grant** — both the `SCM_RIGHTS`-passed-descriptor class and the abstract-token class (attacks F, G) | The descriptor class is claimed `[SEN]`-strength; the token class is `[ID]` and less proven | A successful token forgery would mean the (not-yet-finalised) token scheme needs a concrete redesign, independent of the fd-passing mechanism, which is expected to hold |
| 9 | **Present an old, validly-signed governance version as current**, both with and without a TPM present (attack S) | Tests the rollback-protection claim in both the recommended and fallback configurations | A successful rollback in the TPM-present configuration would be a serious finding requiring re-evaluation of the TPM integration; a successful rollback in the fallback-only configuration is the already-disclosed, expected weaker case |
| 10 | **Directly edit a governance file on disk and attempt to start MELFINA** | Tests F9 §3's fail-closed behaviour end-to-end | Any successful start on tampered governance is a critical defect, not a residual risk — this must be closed before any further work, not merely noted |
| 11 | **Migrate a full Chronicle + governance chain to a different (virtual or physical) machine** and confirm the sequence in `MIGRATION_PORTABILITY.md` §4 | Validates the "zero live grants, fresh hardware detection, explicit TPM re-authorisation ceremony" design end-to-end, not just on paper | A grant or hardware capability found to be silently inherited would be a serious defect in the migration handling, requiring a fix before the design is trusted |
| 12 | **Measure idle resource footprint** (P0+P1+P2 running, no Ring-3 capability active, no model loaded) and **measure Ring-3 sandbox startup latency** (default tier) | The lightweightness claims throughout this mission (`BUILD_AND_SUPPLY_CHAIN.md` §8, F7 P14) are currently `[ETL]` — asserted from the properties of the chosen mechanisms, not measured | A surprisingly high idle footprint or sandbox-startup cost would not necessarily change the technology choice, but would need explaining before the lightweightness requirement is considered met |
| 13 | **Confirm no debug/introspection interface is reachable from a default build, and that reaching one from inside a Ring-3 sandbox is denied** (attack AJ, the one new gap this mission's adversarial review surfaced) | Newly identified, not previously covered by any prior foundation or technology document | Any reachable debug interface in a default build is a defect to close immediately, per `TECHNOLOGY_ADVERSARIAL_REVIEW.md`'s summary |

## 2. Explicitly out of scope for this mission

Per mission §45 and §33: none of the above experiments are to be run as
part of *this* mission. They are recorded as the concrete, falsifiable
validation plan CORE ENGINE should execute before real user data or real
authority is placed in any of these mechanisms. Running dangerous
experiments (power-loss simulation, fuzzing, sandbox-escape attempts) on the
user's actual machine, unsandboxed, is explicitly not authorised by this
document — each experiment should itself run inside appropriate isolation
(a disposable VM or container for anything destructive) when actually
executed.

## 3. Formal verification opportunities (mission §39)

| Candidate component | Why formal verification is valuable | Candidate method | Expected scope | Difficulty | Residual unverified surface |
|---|---|---|---|---|---|
| **The Reference Monitor's `AUTHORIZED(e,g,s,t)` predicate** (F3 §4) | This one predicate is the single point every effect in the system passes through — a proof that it correctly implements its own specification would be the highest-leverage formal-verification investment in the whole system | An SMT-based check (Z3, `REASONING_COMPUTATION_EVALUATION.md` §3) of the predicate's logic against a formalised version of F1/F3/F8's rules; a full functional-correctness proof (e.g. via a proof assistant) is a further, heavier step | The pure decision logic only, not its IPC/process-hosting code around it | MEDIUM (SMT check) to HIGH (full proof) | The formalisation itself could be wrong or incomplete relative to the intended contract — this is an inherent limit of any formal method, not specific to the tool chosen |
| **The governance integrity chain's verification logic** (F9 §3) | A small, security-critical, rarely-changing piece of code — a strong candidate for the highest verification-to-effort ratio in the system | Property-based testing first (`proptest` against a model of valid/invalid chains); a full proof is a `DEFER` | The chain-walk and signature-verification logic, not the cryptographic primitives themselves (which rely on `ed25519-dalek`'s own audit, not a from-scratch proof) | LOW (property-based) to HIGH (full proof) | Same general limit as above |
| **The structured-action parser/canonicaliser** (F2) | Directly on the containment-critical (RC-4) path | Property-based testing (already recommended, `BUILD_AND_SUPPLY_CHAIN.md` §2) as the practical near-term tool; formal verification of the canonicalisation function specifically (a pure, stateless function, relatively tractable to specify) is a plausible `DEFER` target | The canonicalisation function in isolation | LOW–MEDIUM | Formal verification of canonicalisation does not verify the *isolation mechanism's* enforcement of the result — the two must both hold |
| **The revocation state machine** (F4) | A state machine with well-defined states and transitions is a natural fit for model checking | A model checker (e.g. TLA+ or a similar tool) applied to the state-machine specification | The abstract state machine, not the concrete Rust implementation directly (a further step, `DEFER`) | MEDIUM | Model checking the specification does not prove the implementation matches it — a further verification gap, honestly acknowledged |
| **The Chronicle append state machine** (F6 §2, §7) | Single-writer serialisation and atomicity are exactly the kind of property model checking excels at | Same as above | The abstract append protocol | MEDIUM | Same limitation |
| **Aggregate-effect accounting** (F1 §11, F8 §2.7) | A numeric/logical accounting rule, well-suited to an SMT-style check for "can this set of individually-permitted effects ever be classified as if their union were still low-risk" | Z3 | The accounting rule in isolation from the rest of the Monitor | MEDIUM | Same general limit — verifies the *rule*, not that P0's actual code implements the rule correctly, unless the rule's implementation is generated from or checked against the same formal model |

**None of the above is required before CORE ENGINE begins.** They are
recorded as a prioritised menu, not a gate — per mission §39's own
instruction not to require formal verification everywhere. The Reference
Monitor's predicate and the governance chain's verification logic are judged
the two highest-value candidates given their position as the system's most
security-critical, least-frequently-changing components.

## 4. Human review required

Per mission §46, not itself a top-tier sign-off decision, but the priority
ordering in §1 (crash-injection on the Chronicle log ranked highest) should
be confirmed or reordered by the human before CORE ENGINE begins, since it
determines the sequencing of the very first real engineering work.
