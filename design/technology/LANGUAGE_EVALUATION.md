# MELFINA — TECHNOLOGY SELECTION: LANGUAGE EVALUATION

**Phase:** TECHNOLOGY SELECTION MISSION 001. First pass, pending human review.
**Baseline:** requirements `4f5e97c`, foundations `19d0135`, architecture
`8a40207`. Rubric: `design/foundations/TECHNOLOGY_SELECTION_CRITERIA.md` §1.

Scope: **A. Primary implementation language**, **B. Supporting languages**,
and the **multi-language architecture question** (mission §5–6).

---

## 0. What the language must do (recap of the MUST-HAVEs it is scored against)

From `TECHNOLOGY_SELECTION_CRITERIA.md` §1: memory- and type-safety by default
for the TCB (Reference Monitor, trusted loader, Chronicle append path,
isolation supervisor); deterministic execution on the safety-critical paths
(authorisation, canonicalisation, governance verification, Chronicle
ordering); a credible isolation story; straightforward FFI/process control for
terminal and GUI; AOT/self-contained deployment; long-term toolchain
stability. MUST-NOT: mandatory large managed runtime dominating the
footprint; pervasive UB in the safe subset; a GC with unbounded pauses on the
hard-real-time (emergency-stop/revocation) paths; single-vendor lock-in.

---

## 1. Candidates evaluated

| Candidate | Why considered |
|---|---|
| **Rust** | Memory+type safety without a GC; capability-security-friendly type system; the language the current Linux-kernel and Android hardening efforts are standardising on for exactly this reason. |
| **C++** | Mature, fast, huge ecosystem, direct OS/GUI bindings; the traditional systems-language default. |
| **C** | Minimal, auditable, the language of the OS interfaces themselves (and of SQLite, seccomp/Landlock headers, most GUI toolkits' C ABI). |
| **Zig** | A "boring, explicit" systems language positioned as a safer C; comptime; no hidden control flow. |
| **Go** | Memory-safe, simple, excellent concurrency primitives, fast builds. |
| **OCaml** | Best-in-class algebraic types and exhaustiveness checking for encoding closed vocabularies and state machines; decades of use in security/compiler contexts. |
| **Haskell** | Purity + a monadic effect system could, in principle, encode "Ring 2 has no effect capability" *in the type system itself*. |
| **Swift** | Memory-safe (ARC, no stop-the-world GC), good sum types; evaluated because of its enum ergonomics, despite an obviously weaker Linux fit. |

---

## 2. Evaluation matrix

Scale: **PASS/FAIL/UNKNOWN** for MUST-HAVE and MUST-NOT-HAVE; **HIGH/MEDIUM/
LOW** for SHOULD-HAVE dimensions, with the specific reason stated (per mission
§29, no fake numeric precision).

| Dimension | Rust | C++ | C | Zig | Go | OCaml | Haskell | Swift |
|---|---|---|---|---|---|---|---|---|
| **Memory safety by default (MUST)** | **PASS** — ownership/borrow checker rejects use-after-free, double-free, most data races at compile time `[E]` | **FAIL** — raw pointers, manual lifetime, UB by default; sanitizers/RAII mitigate, don't structurally prevent `[E]` | **FAIL** — same, more manual `[E]` | **PARTIAL** — bounds checks + explicit allocators in safe build modes, no ownership/borrow checking; safer than C, weaker than Rust `[E]` | **PASS** — GC-backed, no manual memory `[E]` | **PASS** — GC-backed `[E]` | **PASS** — GC-backed `[E]` | **PASS** — ARC `[E]` |
| **Type safety / closed-vocabulary exhaustiveness (SHOULD, high-leverage here)** | **HIGH** — `enum`+exhaustive `match`, no fallthrough by default; directly encodes F1 §3 / F2 §2 / F8 §2.3's closed vocabularies as compiler-checked types `[E]` | **LOW** — enums are weakly-typed by default (`enum class` helps but no exhaustiveness enforcement) `[E]` | **LOW** — `enum` is just an int `[E]` | **MEDIUM** — tagged unions + exhaustive `switch`, close to Rust but younger tooling `[E]` | **LOW** — no sum types until recently, no exhaustiveness checking on interfaces `[E]` | **HIGH** — the traditional strength of ML-family languages; arguably the best fit of any candidate `[E]` | **HIGH** — same family, plus can push effect separation into the type system `[E]` | **MEDIUM** — enums with associated values are good, ecosystem/tooling for exhaustiveness is decent but less battle-tested at scale `[E]` |
| **Deterministic execution on hard-real-time paths (MUST)** | **PASS** — no GC; allocator/panic behaviour is controllable `[E]` | **PASS** | **PASS** | **PASS** | **CONDITIONAL PASS** — Go's GC has had sub-millisecond STW pauses since 1.8; likely acceptable for the emergency-stop/revocation path but not proven for this workload `[ID]` | **CONDITIONAL PASS** — OCaml's GC is generally low-pause for typical workloads; less battle-tested under this specific latency profile `[ID]` | **CONDITIONAL FAIL** — lazy evaluation makes worst-case latency and space usage hard to reason about (space leaks are a documented, recurring pain point) `[E]` | **PASS** |
| **Credible isolation story (F7)** | **HIGH** — mature `seccomp`/Landlock crates exist and compose (§below); native `nix`/`libc` bindings for namespaces, `cgroups`, `execve` `[E]` | **HIGH** — same OS primitives, more manual, larger unsafe surface per line | **HIGH** — same primitives, most auditable per line but most manual | **MEDIUM** — same C ABI access, smaller ecosystem of ready-made sandboxing crates | **MEDIUM** — OS primitives reachable but typically through `cgo`, which itself widens the trust boundary | **LOW** — usable via C FFI, no ecosystem of ready-made sandboxing bindings | **LOW** — same gap, worse | **LOW** — Linux systems-programming ecosystem for Landlock/seccomp is essentially absent |
| **FFI / process control (terminal, GUI)** | **HIGH** — `std::process`, `nix`, mature AT-SPI/D-Bus crates (`zbus`, `atspi`) `[E]` | **HIGH** — native | **HIGH** — native, the reference ABI everything else binds to | **HIGH** — first-class C interop, no header generation needed | **MEDIUM** — `os/exec` is fine; `cgo` for D-Bus/AT-SPI is friction-heavy and reintroduces unsafety at exactly the boundary Go's safety model is supposed to protect `[E]` | **MEDIUM** — usable but smaller ecosystem | **LOW** — smaller ecosystem, FFI is more awkward under purity | **LOW** — weak Linux D-Bus/AT-SPI ecosystem |
| **AOT / self-contained deployment (MUST)** | **PASS** — single static-ish native binary | **PASS** | **PASS** | **PASS** | **PASS** — static binaries by default | **PASS** — native compilation | **PASS** — native compilation (GHC) | **PARTIAL** — Swift-on-Linux runtime deployment is less turnkey than the others |
| **Long-term toolchain stability (MUST)** | **HIGH** — stable since 1.0 (2015), edition mechanism preserves compatibility, no known plan to break the language `[E]` | **HIGH** — decades-stable, standards-committee governed | **HIGH** — the most stable language in existence | **LOW–MEDIUM** — pre-1.0 as of this evaluation; the language itself has changed release to release; a real risk for a decades-long solo-maintained project `[E]` | **HIGH** — stable since 1.0 (2012), strong compatibility promise | **HIGH** — decades-stable | **MEDIUM** — GHC extensions churn faster than the Haskell Report | **MEDIUM** — evolves quickly, Linux support is the less-invested platform |
| **No mandatory large managed runtime (MUST-NOT)** | **PASS** | **PASS** | **PASS** | **PASS** | **PASS** — runtime is small and statically linked, not a JVM/CLR-class runtime `[DI]` | **PASS** | **PASS** — GHC RTS is moderate, not JVM-class | **PASS** |
| **One-maintainer comprehensibility (`MEL-REQ-192`)** | **HIGH** — large community, extensive documentation, AI-coding-assistant familiarity is very high (relevant given the project's own working style) | **MEDIUM** — huge language surface, many idioms/eras to know | **HIGH** — small language, but the safety burden shifts entirely onto the maintainer's own discipline | **MEDIUM** — small language, but sparse prior art/answers relative to Rust/C/C++ | **HIGH** — deliberately small language | **LOW–MEDIUM** — much smaller talent pool/prior art; a genuine solo-maintainer risk despite the language's technical merits | **LOW** — steepest learning curve, hardest to debug for a non-specialist (space leaks, strictness analysis) | **LOW** — smallest Linux-systems prior art of the set |
| **Ecosystem support for storage/crypto/GUI/AI bindings** | **HIGH** — mature crates for SQLite/LMDB (`rusqlite`, `heed`), Ed25519 (`ed25519-dalek`, audited), D-Bus/AT-SPI (`zbus`, `atspi`), and `llama.cpp` bindings all exist and are actively maintained `[E]` | **HIGH** — native to most of these libraries (SQLite, llama.cpp, BLAS/LAPACK are C/C++) | **HIGH** — same, most native of all | **MEDIUM** — growing, smaller than Rust's | **MEDIUM** — decent but `cgo` friction recurs | **LOW–MEDIUM** — usable via FFI, thinner ecosystem | **LOW** | **LOW** |

## 3. Supply-chain note (applies to all GC-free, crate/package-ecosystem
languages)

Rust's crates.io, like any large package ecosystem, is a genuine supply-chain
surface (`MEL-REQ-193`, F12 §1). This is a **known, manageable** cost, not a
disqualifier: it is addressed by minimal-dependency discipline, vendoring, and
`cargo audit`/`cargo vet`-style review at BUILD_AND_SUPPLY_CHAIN.md, not by
picking a language with no ecosystem at all (C's "ecosystem" is header files
and manual vetting of every dependency, which is not actually less
supply-chain risk — it is *less visible* risk).

## 4. Primary-language recommendation

> **PRIMARY: Rust.** **Confidence: HIGH.**

**Why it wins the hierarchy (per `TECHNOLOGY_SELECTION_CRITERIA.md` §0's
cross-cutting MUSTs and the mission's §3 ordering — safety invariants first):**

1. It is the only candidate that is simultaneously **memory-safe by default**
   (matches C/C++'s only real weakness against F7/F9's TCB requirement) *and*
   has a **type system strong enough to make F1/F2/F5/F8's closed
   vocabularies compiler-checked** (`match` exhaustiveness on `enum`s — the
   property that made OCaml/Haskell attractive is already substantially
   present in Rust). This combination is unique among the systems-capable
   candidates.
2. Its isolation story is concretely ahead of the alternatives *for this
   project specifically*: mature, actively-maintained crates combine Landlock
   (filesystem/network confinement) and seccomp-bpf (syscall filtering)
   without requiring root or containers — directly matching F7's P1–P6
   (default-deny, filesystem/process confinement, resource bounds) with a
   **2026-era production-ready primitive** (Landlock reached this maturity
   level specifically in the 2025–2026 kernel cycle `[E]`), not a
   speculative one.
3. `ed25519-dalek` (F9's signing scheme candidate) is independently audited
   (Quarkslab 2019), constant-time, and zeroizes key material on drop by
   default — a direct match for F9 T1/T4 (key material must not linger
   reachable in a compromised running process).
4. It carries the **lowest long-term risk of the memory-safe options**: Go and
   OCaml are both reasonable second choices but lose on FFI ergonomics (Go's
   `cgo`) or ecosystem breadth (OCaml) respectively; Haswell-class purity
   (Haskell) is rejected as **too clever relative to the "boring technology"
   test** (mission §31) for a project one person must maintain for decades.
5. Zig is **the most interesting near-miss** — its "no hidden control flow"
   philosophy is philosophically aligned with `AP-5`'s "economy of mechanism"
   — but its pre-1.0 status is a concrete, admitted risk (`[E]`, stated by the
   language's own release history) that the mission's "boring over
   interesting" test (§31) weighs against for a decades-long solo project.
   **Revisit if/when Zig reaches a stable 1.0** — recorded as a `DEFER`, not a
   rejection.

**What would change this recommendation:** a documented, reproducible finding
that Rust's `unsafe`/FFI boundary (the same boundary implicated in 2025's
first-ever Rust CVE in the Linux kernel, a race condition in the Android
Binder driver's Rust rewrite `[E]`) is the dominant source of defects in a
built prototype — in which case OCaml's narrower FFI surface for the specific
decision-logic component (§5) would be re-evaluated.

**Alternative:** none of the other candidates scores close enough on the
MUST-tier criteria to be a genuine co-primary. If Rust proves unworkable in
practice, **Zig** (once stable) is the fallback, not C/C++.

**Rejected as primary, with reasons:**
- **C, C++** — fail the memory-safety-by-default MUST for the TCB. Both
  remain acceptable **as libraries consumed through a narrow, audited FFI
  boundary** (SQLite is C; BLAS/LAPACK are C/Fortran; `llama.cpp` is C/C++) —
  this is "small trusted core + isolated components" (mission §6, option D),
  not "choosing C/C++ as an implementation language."
- **Go** — memory-safe, but its weak sum-type/exhaustiveness support
  undermines exactly the "make invalid states difficult to represent"
  property the closed vocabularies most need, and `cgo` reintroduces an
  unsafe FFI boundary at the terminal/GUI/AI-binding surface where Rust does
  not need to. Kept as a **DEFER** option for isolated peripheral tooling,
  not needed now.
- **Haskell** — the most elegant theoretical fit for encoding effect
  separation in the type system, rejected on practical solo-maintainer
  grounds (space-leak debugging difficulty, smaller talent pool, `MEL-REQ-192`).
- **Swift** — rejected on Linux-systems-ecosystem grounds alone (weak
  Landlock/seccomp/AT-SPI/D-Bus prior art); its language properties are
  otherwise reasonable.

## 5. Supporting-language / multi-language architecture question (mission §6)

**Recommendation: single-language (option A) for now — DEFER option D.**

Rust's own `enum`+exhaustive-`match` already delivers most of the benefit that
would have motivated adding OCaml as a second language for the Reference
Monitor's `AUTHORIZED()` predicate or the governance parser (F3 §4, F8). The
mission's own criteria (`TECHNOLOGY_SELECTION_CRITERIA.md` §1: "one primary
language for the whole core… a second language only with a clear reason")
sets a high bar this project does not yet have evidence to clear.

- **Option A (single-language)** is the starting recommendation: lower FFI
  attack surface (§25's interaction-analysis concern "Multi-language × FFI"),
  lower build complexity, lower supply-chain surface, and a smaller TCB to
  reason about (`design/technology/TECHNOLOGY_ADVERSARIAL_REVIEW.md` §TCB).
- **Option D (small trusted core + isolated specialised components)** is the
  likely eventual shape once Ring-3 capabilities exist — but "isolated
  component" there means a **process boundary with a capability-mediated IPC
  channel** (already required by F7), not a second *implementation language
  for the trusted core*. A Ring-3 capability MAY be written in a different
  language (even a different sandboxed runtime, e.g. a WASI component) — that
  is a *capability-packaging* decision (`REASONING_COMPUTATION_EVALUATION.md`),
  not a primary-language decision, and does not enlarge the TCB.
- **`[OPEN]`:** whether a second language is ever justified for the
  Monitor/governance decision logic specifically. Recorded as a concrete,
  falsifiable trigger for revisiting: *if* implementing F3's `AUTHORIZED()`
  predicate or F8's governance parser in Rust is found, empirically, to
  require unsafe workarounds or produces classes of bugs Rust's type system
  was supposed to prevent, re-evaluate OCaml for that one component only.
  This is a CORE ENGINE-phase experiment (`TECHNOLOGY_EXPERIMENT_PLAN.md`),
  not a decision made here.

## 6. Human review required

Per mission §46, the primary language is one of the decisions requiring
explicit human sign-off. **Recommendation: Rust, HIGH confidence, with the
Zig-at-1.0 and OCaml-for-Monitor-logic paths recorded as concrete,
falsifiable reconsideration triggers rather than closed questions.**

## 7. Sources

[Rust in Android: move fast and fix things (Google Security Blog, 2025)](https://security.googleblog.com/2025/11/rust-in-android-move-fast-fix-things.html) ·
[CVE-2025-68260 analysis, first Rust Linux-kernel CVE](https://www.penligent.ai/hackinglabs/rusts-first-breach-cve-2025-68260-analysis-the-reality-of-kernel-safety/) ·
[Landlock LSM kernel documentation](https://docs.kernel.org/security/landlock.html) ·
[Landlock: Unprivileged Sandboxing](https://landlock.io/) ·
[sandbox-rs / sandlock-core crates (Landlock+seccomp composition)](https://oneuptime.com/blog/post/2026-01-07-rust-sandboxing-seccomp-landlock/view) ·
[ed25519-dalek security audit (Quarkslab 2019)](https://blog.quarkslab.com/resources/2019-08-26-audit-dalek-libraries/19-06-594-REP.pdf) ·
[ed25519-dalek documentation (zeroize-on-drop, constant-time)](https://docs.rs/ed25519-dalek).
