# MELFINA — TECHNOLOGY SELECTION: DECISION LOG

**Phase:** TECHNOLOGY SELECTION MISSION 001. First pass, pending human review.
Scope: mission §44 — an auditable record of every recommendation made in
this mission. **Format note:** the full Context/Criteria/Evidence/Reasoning
for each decision is written out in its own evaluation document (linked in
the "Detail" column) — this log is the compact, auditable index across all
of them, not a duplicate of the reasoning. Repeating the full argument for
~25 decisions here would violate this mission's own "smallest clear
structure" instruction (§43); the index format below still satisfies §44's
purpose (every recommendation traceable, alternatives named, confidence and
reversal cost stated, approval requirement explicit).

---

| # | Decision | Chosen | Alternative recorded | Reversal cost | Confidence | Human approval required? | Detail |
|---|---|---|---|---|---|---|---|
| 1 | Primary implementation language | **Rust** | Zig (if it reaches 1.0) | **HIGH** — a language choice is expensive to reverse once code exists, but no code exists yet | HIGH | **YES** (mission §46) | `LANGUAGE_EVALUATION.md` §4 |
| 2 | Multi-language architecture | **Single-language for now** (option A); option D (isolated components in other runtimes) already used for capabilities, not the core | OCaml for the Monitor's decision logic specifically, if a concrete pain point emerges | LOW — adding a second language later for one narrow component is a contained change | MEDIUM | YES (bundled with #1) | `LANGUAGE_EVALUATION.md` §5 |
| 3 | Chronicle authoritative substrate | **A custom append-only framed log** | LMDB (named fallback if experiment #1 in `TECHNOLOGY_EXPERIMENT_PLAN.md` finds problems) | **HIGH** — the authoritative data format is the hardest thing to migrate once real data exists | MEDIUM (pending crash-injection experiment) | **YES** (mission §46) | `CHRONICLE_EVALUATION.md` §4 |
| 4 | Chronicle physical record format | **Length+type+CRC32C frame wrapping a JSON payload** | A binary payload format (CBOR), deferred | MEDIUM — payload encoding can change without changing the log's framing/authoritative semantics | HIGH | Bundled with #3 | `CHRONICLE_EVALUATION.md` §5 |
| 5 | Current-state projection/cache | **SQLite, WAL mode, `synchronous=FULL`, version ≥ 3.51.3 (or backport-patched)** | LMDB, if the SQL-parser attack surface is judged too large on later audit | **LOW** — disposable, rebuildable by design (F6 §6) | HIGH | No (a rebuildable cache is inherently low-stakes) | `CHRONICLE_EVALUATION.md` §6 |
| 6 | Ring-3 default isolation tier | **Composed Landlock + seccomp-bpf + Linux namespaces + cgroups v2, unprivileged** | Containers (rejected as default), gVisor (a close alternative for the same tier) | **HIGH** — the isolation mechanism shapes the whole capability-invocation path | HIGH | **YES** (mission §46) | `ISOLATION_EVALUATION.md` §4 |
| 7 | Ring-3 escalated isolation tier | **Firecracker-class microVM**, reserved for governance-flagged high-risk capability classes | gVisor (a lighter alternative if microVM overhead proves unnecessary for the escalated tier's actual use cases) | MEDIUM — an additive tier, can be introduced or adjusted without disturbing the default tier | MEDIUM | Bundled with #6 | `ISOLATION_EVALUATION.md` §4 |
| 8 | Complementary sandbox for pure-computation capabilities | **WASI component model (Preview 2/3)** | None — this is an additive, narrow-scope choice | LOW — applies only to capabilities that opt into this packaging lane | MEDIUM | No | `ISOLATION_EVALUATION.md` §4; `REASONING_COMPUTATION_EVALUATION.md` §5 |
| 9 | Reference Monitor implementation boundary | **Its own OS process (P0)**, separate from Ring 1 | An in-process module boundary (rejected — weaker guarantee) | HIGH — the process topology is foundational to the IPC design around it | HIGH | **YES** (mission §46) | `PROCESS_AND_IPC_EVALUATION.md` §3 |
| 10 | Overall process topology | **4 process classes: P0 (governance), P1 (core), P2 (reasoning), P3ₙ (ephemeral capabilities)** | Splitting Ring 1's own responsibilities into further processes (rejected — no must-be-real boundary among them) | HIGH | HIGH | Bundled with #9 | `PROCESS_AND_IPC_EVALUATION.md` §2 |
| 11 | IPC mechanism | **Unix domain sockets, `SCM_RIGHTS` fd-passing for OS-resource-shaped grants, framed messages otherwise** | A full RPC framework (gRPC, Cap'n Proto) — rejected as unnecessary weight | MEDIUM | HIGH | No (not separately listed in mission §46, bundled with #9's review) | `PROCESS_AND_IPC_EVALUATION.md` §4 |
| 12 | Concurrency model | **Synchronous, thread-based; single-writer discipline for the Chronicle append path; no async runtime in P0/P1** | Tokio/async-everywhere (deferred, not rejected — revisit only on a measured bottleneck) | LOW — the simplest model is also the easiest to later layer async onto for a specific, justified hot path | HIGH | No | `PROCESS_AND_IPC_EVALUATION.md` §5 |
| 13 | Terminal/process execution mechanism | **Direct `execve` via `std::process::Command`, `openat2` with `RESOLVE_NO_SYMLINKS` for path resolution** | N/A — no serious alternative to non-shell execution exists given F2's invariant | LOW — a mechanism-level choice with a single clearly-superior option | HIGH | Bundled with containment-critical review | `TERMINAL_GUI_EVALUATION.md` §1 |
| 14 | Filesystem access mechanism | **Landlock (coarse) + `openat2` (fine-grained, atomic)** | Canonicalise-then-check alone (rejected — leaves a TOCTOU window) | LOW | HIGH | Bundled with #6 | `TERMINAL_GUI_EVALUATION.md` §2 |
| 15 | GUI control mechanism | **AT-SPI2 (structured), `xdg-desktop-portal` RemoteDesktop/ScreenCast (Wayland coordinate fallback), XTEST (X11 fallback)** | Pure computer-vision/screenshot targeting — rejected as the default, not ruled out as a last resort | MEDIUM — GUI-automation code is generally more brittle to platform changes than the other choices in this log | MEDIUM–HIGH | Not explicitly listed in mission §46, but recommended for review given the genuine Wayland-gap caveat | `TERMINAL_GUI_EVALUATION.md` §3 |
| 16 | Resource limiting/watchdog | **cgroups v2 + `rlimit` + a wall-clock deadline watchdog in P1's Supervisor** | N/A — this composes mechanisms already selected elsewhere | LOW | HIGH | No | `TERMINAL_GUI_EVALUATION.md` §4 |
| 17 | Signature scheme | **Ed25519 (`ed25519-dalek`)** | RSA-PSS (deferred, no current reason to prefer), a threshold scheme (deferred, single-signer for now) | MEDIUM — governance versions are rare so a scheme change is a contained, low-frequency migration | HIGH | **YES** (bundled with governance integrity, mission §46) | `CRYPTO_GOVERNANCE_EVALUATION.md` §1 |
| 18 | Hash function | **SHA-256** | BLAKE3 (deferred, only if a measured performance need arises) | LOW | HIGH | Bundled with #17 | `CRYPTO_GOVERNANCE_EVALUATION.md` §2 |
| 19 | Signing-key custody | **Offline key, standalone minimal signing tool** | A hardware token (FIDO2/TPM-backed key) — explicitly left `[OPEN]` for the human's own risk decision | LOW — custody procedure can be upgraded without changing the signature scheme itself | HIGH for the baseline; the hardware upgrade is the human's own decision | **YES** — the human decides whether to adopt the hardware upgrade | `CRYPTO_GOVERNANCE_EVALUATION.md` §3–4 |
| 20 | Current-head marker / rollback protection | **TPM 2.0 monotonic counter where available; signed-marker-plus-human-vigilance fallback otherwise** | A remote transparency-log service — rejected (requires network) | MEDIUM — depends on target hardware's actual TPM presence, `[ID]` pending a concrete check | HIGH if TPM present; MEDIUM (honestly disclosed) if not | **YES** (bundled with #17, mission §46) | `CRYPTO_GOVERNANCE_EVALUATION.md` §5 |
| 21 | Verifier execution mechanism | **Ordinary Ring-3 sandboxed process, read-only/empty grant, deliberately-diverse implementations per independent verifier** | N/A — this follows directly from the isolation-tier choice, no separate mechanism needed | LOW | HIGH | No | `REASONING_COMPUTATION_EVALUATION.md` §1 |
| 22 | Mathematical/symbolic substrate | **SymPy via a sandboxed Python subprocess** | A native Rust CAS crate (preferred where it already suffices for simple cases) | LOW — an isolated capability's internal implementation is swappable without touching the core | MEDIUM | No | `REASONING_COMPUTATION_EVALUATION.md` §2 |
| 23 | Numerical/scientific substrate | **Rust-native crates first (`nalgebra`, `ndarray`, `statrs`), sandboxed Python (NumPy/SciPy) fallback; Z3 for formal/constraint checks** | Full proof assistants (Lean 4, Coq) — deferred | LOW | MEDIUM–HIGH | No | `REASONING_COMPUTATION_EVALUATION.md` §3 |
| 24 | Local reasoning/AI substrate | **`llama.cpp` / GGUF**, model selection deferred | A heavier serving stack (vLLM/SGLang) — rejected as disproportionate to a single-user system | LOW — the engine is swappable behind the same sandboxed-process boundary | HIGH for the engine; `[OPEN]` for the model | **YES** (mission §46 explicitly lists the AI runtime boundary) | `REASONING_COMPUTATION_EVALUATION.md` §4 |
| 25 | Capability packaging mechanism | **Two-lane: WASI components for pure computation, native-subprocess bundles for OS-effect capabilities** | A single, WASI-only packaging model — rejected (no native process-spawn/terminal-effect story in WASI as of Preview 2/3) | MEDIUM — affects the capability-lifecycle tooling built around it | MEDIUM–HIGH | No | `REASONING_COMPUTATION_EVALUATION.md` §5 |
| 26 | Build system | **Cargo**, with a separately hash-pinned Python environment for sandboxed capabilities | A separate build orchestrator (CMake/Bazel) on top — rejected as unjustified for a single-primary-language core | LOW | HIGH | No | `BUILD_AND_SUPPLY_CHAIN.md` §1 |
| 27 | Platform strategy | **Linux-first, x86-64 primary, ARM64 secondary; no Windows/macOS** | A cross-platform-from-day-one approach — rejected, would weaken the strongest available isolation mechanisms (§6 of this log) | HIGH — a full cross-platform port would require re-evaluating #6, #9, and #15 from scratch | HIGH | No (the trade-off is explicit and mission-sanctioned, §42) | `BUILD_AND_SUPPLY_CHAIN.md` §7 |
| 28 | Migration policy | **Portable state (Chronicle, governance content) vs. non-portable authority (live grants, hardware detection, TPM counter) strictly separated; zero live grants after migration by default** | An "assume prior state's capabilities still apply" approach — rejected outright, this is the mission's own explicit warning | LOW — a policy decision enforced at the loader/Supervisor level, not a data-format commitment | HIGH | Not separately listed, but flagged given the TPM interaction | `MIGRATION_PORTABILITY.md` §2–4 |

## Decisions explicitly requiring human sign-off (mission §46, consolidated)

Per mission §46's own list ("Especially require human review for: primary
language, isolation mechanism, Chronicle substrate, governance integrity
mechanism, multi-language boundary, AI runtime boundary"), the following
rows are the ones this mission flags as **blocking** on explicit human
approval before CORE ENGINE proceeds to build against them: **#1–2
(language), #3–4 (Chronicle substrate), #6–8 (isolation), #9–12 (process/
IPC, since the Monitor's boundary is named explicitly), #17–20 (governance
integrity), #24 (AI runtime boundary).**

Everything else in this log is a **recommendation offered for review**, not
a blocking gate — consistent with mission §46's instruction that Claude
"must NOT frame the recommendation as the only correct choice" outside those
explicitly named categories.

## Confidence distribution (self-assessment, not a score to game)

- **HIGH-confidence, low-controversy:** #2, #4–6, #9–10, #12–14, #16–18,
  #21, #23 (partial), #26–28 — mostly places where one candidate clearly
  dominated on the mission's own MUST-tier criteria, or where the decision
  simply composes already-chosen mechanisms.
- **MEDIUM-confidence, genuinely close calls or pending validation:** #1
  (Rust vs. the Zig-at-1.0 contingency), #3 (pending the crash-injection
  experiment), #7–8, #15 (the Wayland gap), #19–20 (contingent on actual
  hardware), #22, #25.
- **Explicitly `[OPEN]`, left to the human:** the hardware-key-custody
  upgrade (#19), and — not a technology decision at all but worth restating
  here — *which* governance-content risk classes escalate to the microVM
  tier (#7), which is F8 content-authoring, not a technology choice.
