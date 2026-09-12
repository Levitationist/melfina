# MELFINA — TECHNOLOGY SELECTION: BUILD, SUPPLY CHAIN, PLATFORM, AND
RESOURCE STRATEGY

**Phase:** TECHNOLOGY SELECTION MISSION 001. First pass, pending human review.
Scope: **V. Build system**, **W. Test framework**, **X. Profiling/
diagnostics**, **Y. Packaging/distribution**, plus the mission's §19–21,
§40–42 (supply chain, license, platform, resource efficiency), consolidated
into one document per the mission's own "smallest clear structure"
instruction (§43) — these are related, lower-individual-risk decisions that
do not each need a separate file.

---

## 1. Build system (V)

> **Recommendation: Cargo (Rust's native build tool), with `Cargo.lock`
> committed, `cargo vendor` for fully offline/reproducible builds, and a
> pinned, hash-locked Python environment (via `uv` or `pip-tools`) for the
> sandboxed Python capabilities (`REASONING_COMPUTATION_EVALUATION.md` §2–3).
> Confidence: HIGH.**

Cargo is mature, supports reproducible builds via lockfiles, offline builds
via vendoring, and cross-compilation out of the box — no alternative build
system was seriously considered given the primary-language decision already
made (`LANGUAGE_EVALUATION.md`); introducing a separate build orchestrator
(CMake, Bazel, Make) on top of Cargo would be exactly the kind of
unjustified complexity mission §33 warns against for a single-language
core.

## 2. Test framework (W)

> **Recommendation: `cargo test` (built-in) for unit/integration tests;
> `proptest` for property-based testing of parsers and state machines;
> `cargo fuzz` (libFuzzer-based) for fuzzing the action parser, the grant
> parser, and the Chronicle frame parser specifically. Confidence: HIGH.**

Property-based testing is particularly well-suited to exactly the components
this project cares most about getting right: F2's structured-action
canonicalisation (an ideal target for "does resolving path P always produce
the same canonical form regardless of how it's constructed"-style
properties), F1's grant-scope containment (`AUTHORIZED(e,g,s,t)` as a
predicate is naturally expressed as a property to fuzz against), and F6's
Chronicle frame parser (exactly the component `TECHNOLOGY_EXPERIMENT_PLAN.md`
flags as highest-priority to fuzz before trusting it with real data).

## 3. Static analysis, sanitizers, and diagnostics (X)

| Tool | Role |
|---|---|
| `cargo clippy` | Routine lint pass, catches a wide class of common mistakes before they become bugs |
| `cargo audit` | Checks dependencies against the RustSec advisory database — an actively maintained feed (e.g. `RUSTSEC-2025-0028`, a real 2025 advisory found during this mission's research, demonstrating the feed is live and used) `[E]` |
| Miri | Undefined-behaviour detector for `unsafe` code — run specifically over the isolation-policy-application and IPC-framing code, the two places `unsafe`/FFI-adjacent code is most likely to appear given the OS-syscall-heavy nature of Landlock/seccomp/`openat2` bindings |
| AddressSanitizer / ThreadSanitizer (nightly Rust sanitizer support) | Run over the same `unsafe`-heavy components during the experiment-plan's fuzzing/crash-injection work, not as a routine CI gate (nightly-only tooling is a known Rust ergonomics gap, acceptable for periodic deep checks) |
| `perf`/`strace`/standard Linux profiling tools | No MELFINA-specific profiling framework is needed at this phase — ordinary Linux tooling suffices for a native, non-VM-hosted binary; a dedicated profiling strategy is **DEFERRED** until a concrete performance question exists (`TECHNOLOGY_SELECTION.md` §5's performance model) |

## 4. Packaging / distribution strategy (Y)

> **Recommendation: a single, mostly-static native binary per long-lived
> process (P0, P1, P2), plus a capability-package directory format
> (`REASONING_COMPUTATION_EVALUATION.md` §5), distributed as a local
> filesystem layout — no installer service, no package-manager dependency
> for the core, no auto-update mechanism that reaches the network by
> default. Confidence: HIGH.**

This directly satisfies the offline-first test (mission §34): nothing about
installing or running MELFINA should require network reachability. A future
convenience packaging (a `.deb`, a distribution-specific package) is a
**DEFER**, not a rejection — it is a distribution-format question, not an
architecture question, and does not change any of the above.

## 5. Supply-chain analysis (§40)

**Principle (`MEL-REQ-193`, restated as a build-time discipline):** MELFINA's
core should not become "a thin wrapper around hundreds of uncontrolled
dependencies." Concretely:

- **Dependency count and depth are tracked, not just assumed acceptable.**
  A dependency audit (`cargo tree`, transitive-depth report) is a required
  CORE ENGINE deliverable before the first real build, not an afterthought.
- **`cargo vet`/`cargo-crev`-style review** for any dependency in the P0/P1
  trusted core specifically (a higher bar than for a sandboxed Ring-3
  capability's own dependencies, which are already contained by isolation
  regardless of their own supply-chain hygiene — a direct, concrete
  application of "small trusted core + isolated components" to supply-chain
  risk itself, not only to runtime isolation).
- **Two package ecosystems exist** (crates.io for the Rust core, PyPI for
  the sandboxed math/science capabilities) — both get the same pinning
  discipline (`Cargo.lock` / hash-locked Python requirements), not a lighter
  standard for the "less important" one.
- **Vendoring for reproducibility and offline builds:** `cargo vendor`
  (Rust) and a local wheel cache (Python) so that a build does not depend on
  package-registry reachability at build time, consistent with the
  offline-first test (mission §34) extending to *building*, not only
  *running*, MELFINA.
- **`llama.cpp` and any GGUF model weights are external artefacts** with
  their own provenance and update cadence, tracked the same way any other
  acquired knowledge is tracked (`MEL-REQ-269`, `327`) — a model file is not
  a "dependency" in the crates.io sense, but it is exactly the kind of
  externally-acquired artefact PART IV-C's knowledge-provenance discipline
  already covers.

## 6. License / legal notes (§41) — flagged for human/legal review, not
resolved here

- Rust itself and its standard toolchain are dual MIT/Apache-2.0
  (permissive) — no issue.
- Core dependencies should be vetted for permissive licenses (MIT/Apache-2.0/
  BSD); any copyleft (GPL-family) dependency in the P0/P1 trusted core is
  **flagged for legal review**, not automatically excluded — the practical
  implication depends on how (or whether) MELFINA is ever distributed beyond
  the current single-user personal use, which is outside this mission's
  scope to decide.
- **Model weights (deferred choice, `REASONING_COMPUTATION_EVALUATION.md`
  §4) carry their own licenses** independent of the software — commercial-
  use and redistribution terms vary significantly across model families and
  must be checked as part of that later, separate decision, not assumed
  clear by default.
- This document does **not** provide legal certainty on any of the above —
  per mission §41, it identifies issues requiring review, nothing more.

## 7. Platform strategy (§42)

> **Recommendation: Linux-first, x86-64 primary, ARM64 as a supported
> secondary target; no Windows/macOS support planned. Confidence: HIGH.**

The strongest available isolation mechanisms evaluated in this mission
(Landlock, seccomp-bpf, Linux namespaces, cgroups v2) are **Linux-specific**
— this is a deliberate, accepted trade-off, explicitly sanctioned by the
mission itself ("do not sacrifice the strongest Linux security architecture
merely to achieve premature universal portability," §42). The user's actual
development target (Ubuntu 24.04, per `PROJECT_STATE.md` §7) is the primary
platform. Rust's mature cross-compilation support means ARM64 (relevant for
future hardware migration, `MIGRATION_PORTABILITY.md`) is a low-cost
secondary target, not a design change. A future Windows/macOS port, if ever
pursued, would require re-evaluating `ISOLATION_EVALUATION.md` and
`TERMINAL_GUI_EVALUATION.md` from scratch for those platforms' own
sandboxing and accessibility primitives — this is recorded as a large,
**DEFERRED** undertaking, not a near-term goal.

## 8. Resource-efficiency recap (§21)

No new technology decision here — this section confirms the cumulative
resource shape of the choices already made elsewhere in this mission:

| Component | Idle cost |
|---|---|
| P0 (Governance) | A small, mostly-idle process; wakes only to answer authorisation requests |
| P1 (Core) | The Chronicle append path is idle between writes; the SQLite cache serves reads without a background service; the Supervisor is idle when no Ring-3 capability is running |
| P2 (Reasoning) | Idle when not reasoning; the local-inference engine (§4 of `REASONING_COMPUTATION_EVALUATION.md`) is unloaded from memory when not in active use, not held resident |
| Ring-3 capabilities | Zero cost when not invoked — no daemon, no pre-warmed pool, one process per invocation, torn down after (F7 P13–P14) |

No process in this design requires a persistent GC heap, a JIT warm-up, or a
managed-runtime baseline — the cumulative idle footprint is expected to be
small relative to a typical desktop application, consistent with AP-12's
lightweightness goal. **`[ETL]`**: an actual measured number is deferred to
CORE ENGINE + the experiment plan; no number is asserted here without a
build to measure.

## 9. Human review required

Per mission §46, build/supply-chain/platform decisions are lower-stakes than
language/isolation/storage/governance and do not require the same
escalated sign-off — reported for completeness, not flagged as a blocking
decision point.

## 10. Sources

[RUSTSEC-2025-0028 advisory](https://rustsec.org/advisories/RUSTSEC-2025-0028.html)
(cited to demonstrate the RustSec feed is live and actively used, not as a
finding about a MELFINA dependency specifically — no dependency has been
chosen yet to check against it).
