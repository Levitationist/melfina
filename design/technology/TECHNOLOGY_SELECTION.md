# MELFINA — TECHNOLOGY SELECTION MISSION 001 — SYNTHESIS

**Phase:** TECHNOLOGY SELECTION MISSION 001. **Status:** first pass complete,
pending human review. **Baseline:** requirements `4f5e97c`, foundations
`19d0135`, architecture `8a40207`.

**What this document is.** The cross-cutting synthesis that ties together
the twelve category-specific evaluation documents in this directory: the
overall proposed stack, the hierarchy used to weigh decisions, the
compatibility check against every foundation contract (F1–F10) and the
requirements/anti-requirements (`MEL-REQ`/`MEL-AR`), the hard-gate results,
the TCB analysis, the offline-core and AI-boundary tests, and the explicit
human-decision boundary. **What it is not:** a restatement of each
category's full reasoning — that lives in the linked document.

**A note on this mission's resume history.** This mission was interrupted
mid-way and resumed. The resume instructions referenced a claim —
`"VC1–VC18: CLEAN, NO GAPS, NO DUPLICATES"` — as if it were this mission's
own validation checkpoint. **It is not.** `VC1`–`VC18` are conflict entries
in `requirements/CONFLICTS.md`, validated and committed as part of the
**prior** mission, REQUIREMENTS EXPANSION MISSION 002 (`4f5e97c`) — that
validation was real, is independently re-confirmed in §7 below, and remains
correct, but it is a **requirements-phase** artifact, not a
**technology-selection-phase** one. This mission has no VC-numbered items of
its own; its own validation is the F1–F10/MUST-gate compatibility matrix in
§4–§6. This distinction is stated plainly here rather than silently
perpetuated, per this project's standing discipline against reporting a
validation that was not actually performed for the artifact at hand.

---

## 1. Decision hierarchy applied (mission §3)

Every recommendation across all twelve documents was weighed in this order:
safety invariants → authority boundaries → local-only → correctness/
determinism → isolation → failure/recovery → data integrity → capability
model → dynamic evolution → performance/resource → maintainability →
ecosystem convenience. No recommendation in this mission was made on
popularity, benchmark performance alone, or developer convenience where a
higher-ranked criterion pointed elsewhere — see each document's own
"Rejected" sections for the specific cases this mattered (e.g. Rust over
Go/C++ despite the latter's larger existing ecosystems for some bindings;
a custom Chronicle log over the more "convenient" single-database answer).

## 2. Overall proposed technology stack

| Layer | Recommendation | Confidence | Human approval required |
|---|---|---|---|
| Primary language | **Rust** | HIGH | **YES** |
| Multi-language architecture | Single-language core; isolated capabilities may use other runtimes | MEDIUM | YES (bundled) |
| Chronicle authoritative substrate | **Custom append-only framed log** (JSON payload, length+CRC framing) | MEDIUM, pending crash-injection experiment | **YES** |
| Chronicle cache/query engine | **SQLite**, WAL, `synchronous=FULL` | HIGH | bundled |
| Ring-3 default isolation | **Landlock + seccomp-bpf + namespaces + cgroups v2** | HIGH | **YES** |
| Ring-3 escalated isolation | **Firecracker-class microVM** (rare, governance-flagged classes) | MEDIUM | bundled |
| Complementary sandbox | **WASI component model** (pure-computation capabilities) | MEDIUM | no |
| Reference Monitor boundary | **Its own OS process (P0)** | HIGH | **YES** |
| Process topology | **P0 (governance) / P1 (core) / P2 (reasoning) / P3ₙ (ephemeral capabilities)** | HIGH | bundled |
| IPC | **Unix domain sockets + `SCM_RIGHTS`** | HIGH | no |
| Concurrency | **Synchronous threads; single-writer Chronicle** | HIGH | no |
| Terminal execution | **`execve` via `Command`, `openat2`** | HIGH | bundled (containment-critical) |
| Filesystem access | **Landlock + `openat2`** | HIGH | bundled |
| GUI control | **AT-SPI2 + portal-based Wayland fallback + XTEST** | MEDIUM–HIGH | recommended |
| Governance signature scheme | **Ed25519 (`ed25519-dalek`)** | HIGH | **YES** |
| Governance hash | **SHA-256** | HIGH | bundled |
| Signing-key custody | **Offline key + standalone tool** | HIGH (baseline); `[OPEN]` (hardware upgrade) | **YES** |
| Current-head/rollback protection | **TPM 2.0 monotonic counter**, disclosed fallback otherwise | HIGH/MEDIUM | **YES** |
| Verifier execution | **Ring-3 sandboxed process, deliberately diverse implementations** | HIGH | no |
| Math/symbolic substrate | **SymPy, sandboxed** | MEDIUM | no |
| Numerical substrate | **Rust-native + SciPy fallback; Z3 for formal checks** | MEDIUM–HIGH | no |
| Local AI substrate | **`llama.cpp`/GGUF**, model deferred | HIGH (engine); `[OPEN]` (model) | **YES** |
| Capability packaging | **Two-lane: WASI / native sandboxed subprocess** | MEDIUM–HIGH | no |
| Build system | **Cargo** + pinned Python env | HIGH | no |
| Platform | **Linux-first, x86-64 primary, ARM64 secondary** | HIGH | no |
| Migration policy | **Portable state / non-portable authority, strictly separated** | HIGH | recommended |

Full reasoning for every row: `LANGUAGE_EVALUATION.md`,
`CHRONICLE_EVALUATION.md`, `ISOLATION_EVALUATION.md`,
`PROCESS_AND_IPC_EVALUATION.md`, `TERMINAL_GUI_EVALUATION.md`,
`CRYPTO_GOVERNANCE_EVALUATION.md`, `REASONING_COMPUTATION_EVALUATION.md`,
`BUILD_AND_SUPPLY_CHAIN.md`, `MIGRATION_PORTABILITY.md`. The full,
row-by-row decision record (alternatives, reversal cost) is
`TECHNOLOGY_DECISION_LOG.md`.

## 3. Hard-gate results (mission §6, §23)

No candidate recommended above **fails** a MUST-HAVE or trips a
MUST-NOT-HAVE from `design/foundations/TECHNOLOGY_SELECTION_CRITERIA.md`.
The candidates that **were** rejected specifically because of a hard-gate
failure (not merely a lower SHOULD-score):

| Rejected candidate | Hard gate failed |
|---|---|
| C / C++ as the **primary** language | MUST: "memory- and type-safety by default" (§1) — `LANGUAGE_EVALUATION.md` §2 |
| PostgreSQL as the Chronicle substrate | MUST-NOT: "a server process... as the only access path" (§2) — `CHRONICLE_EVALUATION.md` §4 |
| RocksDB as the Chronicle substrate | MUST: "full history retained... no compaction discards a unit" (§2) — its LSM design structurally fights this |
| Bare containers as the default isolation | Not a hard-gate failure per se, but fails the "boring/structural-guarantee over convenience" ranking (§13) once its isolation is shown to derive from the same primitives MELFINA can use directly, at extra cost |
| A remote transparency-log service for the current-head marker | MUST-NOT: implicit network dependency, conflicting with INV-1/local-only |
| A heavier AI serving stack (vLLM/SGLang) | Not a hard-gate failure, fails the proportionality/lightweightness ranking for a single-user system |

**No SHOULD-tier advantage was allowed to rescue a MUST-tier failure
anywhere in this mission** — consistent with mission §6's explicit
instruction.

## 4. Compatibility against F1–F10 (mission §5, §49)

| Contract | Compatibility check | Result |
|---|---|---|
| **F1 Capability Grant Model** | Does any technology choice let authority be inferred rather than granted, or let a grant be forged? | ✓ — `SCM_RIGHTS` fd-passing gives OS-resource-shaped grants a `[SEN]`-strength unforgeability property (`PROCESS_AND_IPC_EVALUATION.md` §4); the abstract-token class remains `[ID]`, unchanged, not weakened |
| **F2 Structured Action Model** | Does terminal/GUI technology preserve "structured action, never a shell string, never re-splittable argv"? | ✓ — `execve`-only invocation, no shell stage anywhere (`TERMINAL_GUI_EVALUATION.md` §1) |
| **F3 Reference Monitor** | Can any technology bypass complete mediation or make authorisation non-deterministic? | ✓ — P0 is a separate, single-threaded process; every effect-causing technology (terminal, GUI, capability packages) routes through the same IPC channel to P0, no side path (`PROCESS_AND_IPC_EVALUATION.md` §2–3) |
| **F4 Revocation** | Can the chosen concurrency/storage model realise atomic, ordered mid-execution revocation? | ✓ — the Chronicle's single-writer serialisation point gives revocation-id marking a natural, total-ordering realisation (`PROCESS_AND_IPC_EVALUATION.md` §5; `TECHNOLOGY_ADVERSARIAL_REVIEW.md` row I) |
| **F5 Chronicle Logical Format** | Does the physical format distort the logical E²CI model (accidental new "types", lost provenance)? | ✓ — the JSON payload maps E²CI units directly with no relational-impedance mismatch (`CHRONICLE_EVALUATION.md` §5); no technology choice introduces a Task/Goal/Project-shaped table anywhere |
| **F6 Chronicle Contract** | Durable-before-return, atomic append, corruption detection without silent repair, single-writer? | ✓ — realised by the framing + fsync design (`CHRONICLE_EVALUATION.md` §4), pending the crash-injection experiment (§8 below is honest about this being MEDIUM, not HIGH, confidence) |
| **F7 Isolation Contract** | Does the isolation technology deliver P1–P14, and is the network primitive *withheld*, not merely unconfigured? | ✓ — seccomp denies `socket`/`connect`/`bind` outright, not just via namespace omission (`ISOLATION_EVALUATION.md` §4); P10/P11 (forgery/impersonation) map onto the IPC/grant design in F1/§4 above |
| **F8 Governance Format** | Does any technology choice give MELFINA a path to write, or a Ring-3 process a path to reach, the Ring-0 region? | ✓ — Landlock confinement + the absence of any effect class targeting Ring 0 (F1 §3) means no capability's isolation policy ever includes the governance path (`ISOLATION_EVALUATION.md` §7; `TECHNOLOGY_ADVERSARIAL_REVIEW.md` row Z) |
| **F9 Governance Integrity** | Does the crypto/key-custody choice keep the signing key off the running system, and is rollback resisted? | ✓ — offline key + standalone signer (T1/T4); TPM monotonic counter for C7, with an honestly-disclosed weaker fallback (`CRYPTO_GOVERNANCE_EVALUATION.md` §3, §5) |
| **F10 Verifier Contract** | Does any technology choice implicitly claim statistical independence between verifiers, or let a verifier hold ambient authority? | ✓ — verifiers are ordinary Ring-3 sandboxed processes with empty/read-only grants; independence is realised as genuinely different evidence sources/mechanisms/lineages, and the "architecturally ≠ statistically independent" caveat is repeated, not silently dropped (`REASONING_COMPUTATION_EVALUATION.md` §1) |

**F11** (`FOUNDATION_CROSS_CONTRACT_ANALYSIS.md`) is not a contract to satisfy
but a method this synthesis reuses: §4 above and `TECHNOLOGY_ADVERSARIAL_
REVIEW.md` together are this mission's version of F11's nine-path trace,
applied to concrete technology rather than abstract contracts. **F12**
(`TECHNOLOGY_SELECTION_CRITERIA.md`) is the rubric every category document
scored against — see each document's own MUST/SHOULD/MUST-NOT tables.

## 5. Compatibility against requirements and anti-requirements

No individual `MEL-REQ`/`MEL-AR` cross-check was performed line-by-line
against all 364/27 items (that would be disproportionate — most requirements
are technology-independent by design, per `REQUIREMENTS_MASTER.md` §0). The
check that **is** relevant and was performed: does any technology choice
create a path around the specific requirements PART IV-C (§74–76) exists to
close (knowledge ≠ authority, intelligence-must-not-bypass)?

| PART IV-C concern | Technology-level check |
|---|---|
| `MEL-REQ-347–348` (knowledge ≠ authority) | P2 (reasoning) cannot send a grant-shaped message over the P1 IPC channel — this is a property of the **message protocol's type**, not a policy P2 could talk its way around (`TECHNOLOGY_ADVERSARIAL_REVIEW.md` row AC) |
| `MEL-REQ-351–353` (math/experiment/decomposition cannot bypass risk classification) | P0 computes risk classification from the canonical action alone, never accepts a classification suggested by P2 (`TECHNOLOGY_ADVERSARIAL_REVIEW.md` rows AA–AB, X–Y) |
| `MEL-REQ-356`, `361–364` (understanding governance ≠ governing) | No technology gives any process other than the offline signing tool a path to author a governance version; P0 only ever *verifies* (`CRYPTO_GOVERNANCE_EVALUATION.md`, `TECHNOLOGY_ADVERSARIAL_REVIEW.md` row T) |
| `MEL-AR-13` (unrestricted autonomous agent) | The process topology structurally caps P2's reach at "propose," regardless of what reasoning technology (which local model, which strategy) runs inside it |

**Result: no technology recommendation in this mission narrows, reinterprets,
or creates a bypass around any PART IV-C requirement.**

## 6. Offline-core test (mission §9, §34)

| Check | Result |
|---|---|
| Network cable unplugged / Wi-Fi disabled | P0, P1, P2 and the default Ring-3 tier have no code path requiring network reachability; `llama.cpp` performs pure local inference | ✓ |
| DNS/proxy/cloud unavailable | No technology choice in this mission resolves a hostname or calls a cloud API at runtime | ✓ |
| Telemetry | No telemetry mechanism was selected or is implied by any chosen technology (Rust's toolchain, Cargo, SQLite, `llama.cpp` do not phone home by default) | ✓ |
| Online license checks | None of the selected technologies (Rust — MIT/Apache dual-licensed toolchain; SQLite — public domain; Landlock/seccomp — kernel features; Ed25519/SHA-256 — open algorithms; `llama.cpp` — MIT) require online licensing | ✓ |
| Online package resolution **at runtime** | None — package resolution (Cargo/PyPI) is a **build-time** concern, addressed separately in `BUILD_AND_SUPPLY_CHAIN.md` §5's vendoring recommendation for **offline builds**, distinct from runtime | ✓, with the build-time/runtime distinction made explicit per mission §9's own instruction |
| Remote authentication | Not applicable — no remote-authentication mechanism exists anywhere in this stack | ✓ |
| Background network services | The chosen process topology (P0/P1/P2) opens no listening network socket; the IPC layer is Unix-domain-socket-only | ✓ |

**Verdict: the proposed stack passes the offline-core test as specified,
with build-time package resolution correctly separated from runtime
behaviour and addressed by the vendoring recommendation.**

## 7. AI-dependency and AI-boundary test (mission §10, §35)

| Condition | Ring 1 behaviour under this stack |
|---|---|
| AI enabled | P2 runs, `llama.cpp` loaded on demand; proposals flow through the ordinary pipeline |
| AI unavailable/crashed | P1 continues fully (INV-9); the Supervisor restarts P2 independently; no core function depends on P2 being up (`PROCESS_AND_IPC_EVALUATION.md` §6) |
| AI corrupted/hallucinating/malicious | Holds zero ambient authority regardless of output — the containment is architectural (P1 of F7), not model-behaviour-dependent (`REASONING_COMPUTATION_EVALUATION.md` §4) |
| AI too slow / out of memory | Bounded by the same cgroup/`rlimit` ceilings as any other Ring-3-hosted process; a timeout is a contained failure, not a core failure |
| AI cannot become source of truth / authority / self-verifying / network gateway | Structural: P2 sends proposals only, never grants (§5 above); P2 has no network primitive (F7 P3) regardless of what the model requests; self-evaluation explicitly uses external checks, never the model's own confidence (F10, `MEL-REQ-330`) |

**Verdict: passes.** No technology choice makes AI a prerequisite for
deterministic core operation, and none gives AI a path to authority.

## 8. Dynamic-capability / self-modification boundary check (mission §11)

| Concern | Technology-level status |
|---|---|
| Capability creation cannot self-authorise more power than the minter held | Enforced at P0 (F1 §9, M8) regardless of a capability's packaging lane (WASI or native) — `REASONING_COMPUTATION_EVALUATION.md` §5 |
| Versioning, regression testing | `BUILD_AND_SUPPLY_CHAIN.md` §2's property-based/fuzz testing infrastructure applies equally to capability packages, not only the core |
| Least authority, creator authority | Unaffected by technology choice — a manifest-declared authority ceiling is checked by P0 the same way regardless of implementation language |
| Human-only tiers 6–9 | No technology in this mission grants any process write access to P0's or P1's own binaries (Landlock confinement, `TECHNOLOGY_ADVERSARIAL_REVIEW.md` row Z) — **more computational power (a bigger local model, a faster language, a stronger prover) never translates into a new effect class or a new writable path, because the effect vocabulary and the filesystem confinement are independent of compute power entirely** |
| The governance meta-invariant | Unweakened; F9's technology choices (§4 of this document) are additive integrity mechanisms around an unchanged rule, not a reinterpretation of it |

**Verdict: no technology recommendation in this mission weakens any of
these. This is the direct, concrete answer to mission §11's closing
principle ("more computational power must never become more authority") —
it holds here because authority is gated by grant-possession (a kernel-
enforced fact, §4's F1 row) and effect-class membership (a governance-data
fact, F8), neither of which is a function of how much compute or how
sophisticated the requesting technology is.**

## 9. TCB analysis (mission §8, §26)

| Component | What must be trusted | Approximate scope | Notes |
|---|---|---|---|
| P0 (Governance/Monitor) | Its own Rust code, `ed25519-dalek`, a SHA-256 implementation, the trusted loader | **Smallest** — deliberately minimised (AP-5); a distinct, smaller dependency tree than the rest of the system (`CRYPTO_GOVERNANCE_EVALUATION.md` §6) | The single most security-critical TCB member |
| P1 (Core) | Rust std, the Chronicle log's framing code (new, unfuzzed — flagged), SQLite (large but extraordinarily mature), the chosen IPC crate | **Largest of the long-lived processes** | SQLite's size is accepted specifically because its role (cache) makes its failure recoverable, not catastrophic (`CHRONICLE_EVALUATION.md` §4) |
| P2 (Reasoning) | Rust std, `llama.cpp` (via a sandboxed child, not linked in-process), the reasoning-strategy code | Excluded from the authority TCB entirely — it holds no grant, so its correctness affects proposal *quality*, never system *authority* | This is the architectural point of separating it into its own process |
| P3ₙ (capabilities) | Whatever each capability's own dependencies are — explicitly **not** part of the trusted core's TCB, contained by isolation regardless of internal trustworthiness | Varies per capability | The entire justification for the isolation-tier investment |
| Isolation mechanism (kernel) | Linux kernel's Landlock/seccomp/namespace/cgroup implementations | Outside MELFINA's own code, but load-bearing | Acknowledged residual (`TECHNOLOGY_ADVERSARIAL_REVIEW.md` rows O–P) |
| Build toolchain | `rustc`, Cargo, and their own supply chain | Outside runtime TCB but part of the *build-time* trust chain | `BUILD_AND_SUPPLY_CHAIN.md` §5 |

**No dependency is casually classified as trusted merely because its
technology category is "considered secure."** Each row above states
specifically what is trusted and why (or, for SQLite, why a *large* trusted
component was accepted anyway — its failure mode is recoverable).

## 10. Interaction analysis — summary (full detail in each document's own
§ "Interaction notes")

All 17 interactions named in mission §7/§25 were addressed in the document
where the *stronger* of the two technologies' constraints binds:
Language×Isolation, Language×Chronicle, Language×IPC, Language×GUI,
Language×Terminal (all in `LANGUAGE_EVALUATION.md` and the relevant
category document); Storage×Integrity, Storage×Backup/Recovery
(`CHRONICLE_EVALUATION.md` §7, `MIGRATION_PORTABILITY.md` §6);
Process×IPC (`PROCESS_AND_IPC_EVALUATION.md` §7); Isolation×Terminal,
Isolation×GUI, Isolation×Verifier (`TERMINAL_GUI_EVALUATION.md` §5,
`REASONING_COMPUTATION_EVALUATION.md` §6); Reasoning×Capability, AI×
Isolation, AI×Resource (`REASONING_COMPUTATION_EVALUATION.md` §6); Build×
Reproducibility (`BUILD_AND_SUPPLY_CHAIN.md`); Multi-language×FFI
(`REASONING_COMPUTATION_EVALUATION.md` §6, `TECHNOLOGY_ADVERSARIAL_REVIEW.md`
row AI); Governance×OS security (`CRYPTO_GOVERNANCE_EVALUATION.md` §7,
`MIGRATION_PORTABILITY.md` §6). **No emergent failure was found that does
not already appear as a named risk in the relevant document** — the
adversarial review (§11 below) is where any such failure would surface, and
its one genuinely new finding (attack AJ, debug interfaces) is an
addition, not evidence of a missed interaction among the seventeen listed.

## 11. Adversarial review — summary

Full 36-attack review: `TECHNOLOGY_ADVERSARIAL_REVIEW.md`. **Verdict
restated:** no attack defeats a foundation-contract invariant. Two
permanent, disclosed residuals (sandbox escape / kernel compromise;
verifier common-mode failure) are inherited from F7/F10 and not claimed
solved. **One genuinely new gap was surfaced and closed with a concrete
recommendation**: debug/introspection interfaces must be gated by the same
capability-grant model as any other effect and disabled by default in
non-development builds (attack AJ) — folded into
`BUILD_AND_SUPPLY_CHAIN.md` practice and `TECHNOLOGY_EXPERIMENT_PLAN.md`
item 13.

## 12. Residual risks (consolidated, ranked)

1. Sandbox escape / kernel compromise — permanent, disclosed, mitigated by
   defence-in-depth and the escalated tier, not eliminated.
2. Verifier common-mode failure — permanent, disclosed, inherited from F10.
3. The Chronicle's custom append-only log is new, unfuzzed code — the
   highest-priority item in `TECHNOLOGY_EXPERIMENT_PLAN.md`.
4. Signing-key physical compromise — a human operational-security question,
   outside technology scope.
5. GUI automation's Wayland coordinate-fallback gap — genuine, disclosed,
   inherent to Wayland's own security model, not a MELFINA-specific defect.
6. Supply-chain risk across two package ecosystems (crates.io + PyPI) — an
   ongoing discipline, not a one-time gate.
7. TPM availability is hardware-contingent (`[ID]`) — the fallback is
   honestly weaker, not hidden.

## 13. Open questions carried by this mission

- Which specific governance-content risk classes escalate to the microVM
  tier (F8 content-authoring, not decided here).
- Whether the human adopts the hardware-key-custody upgrade (§4 of
  `CRYPTO_GOVERNANCE_EVALUATION.md`).
- Which local model(s) to run under the `llama.cpp` engine (explicitly
  deferred per mission §17).
- Whether the custom Chronicle log survives the crash-injection experiment
  or falls back to LMDB.
- The exact WASI-to-OS-effect bridging detail for capabilities that need
  both WASI's sandbox *and* an OS effect (flagged, not designed, in
  `TERMINAL_GUI_EVALUATION.md` §5).

## 14. Confidence and the human-decision boundary (mission §19, §46)

This document and its eleven companions **recommend**; they do not declare
a final, irrevocable stack. Per mission §46 and §19, the following remain
explicit **human decisions**, listed with their status:

| Decision | Status |
|---|---|
| Primary language (Rust) | **RECOMMENDED**, high confidence |
| Chronicle substrate (custom log + SQLite cache) | **CONDITIONALLY RECOMMENDED** — pending the crash-injection experiment |
| Isolation mechanism (Landlock/seccomp/ns/cgroups default; microVM escalated) | **RECOMMENDED**, high confidence |
| Reference Monitor as its own process | **RECOMMENDED**, high confidence |
| Governance integrity (Ed25519/SHA-256, offline key, TPM counter) | **RECOMMENDED** for the algorithms; **REQUIRES HUMAN DECISION** on the hardware-key upgrade and on confirming TPM availability |
| AI runtime boundary (`llama.cpp`) | **RECOMMENDED**, engine only; **UNRESOLVED** on model selection (deliberately, per mission scope) |
| GUI mechanism (AT-SPI2 + portal fallback) | **RECOMMENDED**, with the Wayland gap disclosed for awareness |
| Everything else in §2's table | **RECOMMENDED**, offered for review, not blocking |

**No implementation has been performed. No dependency has been installed.
No `src/` file has been created or modified.** This mission ends at the
recommendation boundary described above.

## 15. Human review boundary statement

Consistent with mission §46: *this technology stack is a recommendation,
not a mandate.* Each blocking decision in §14 names its alternative,
confidence, and residual risk in the linked category document, so the
human's review has a concrete basis rather than a bare assertion.
