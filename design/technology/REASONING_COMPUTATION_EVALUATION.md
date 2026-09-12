# MELFINA — TECHNOLOGY SELECTION: VERIFIERS, MATH/SCIENCE COMPUTATION,
LOCAL AI, AND CAPABILITY PACKAGING

**Phase:** TECHNOLOGY SELECTION MISSION 001. First pass, pending human review.
Scope: **Q. Verifier execution**, **R. Mathematical/symbolic substrate**,
**S. Numerical/scientific substrate**, **T. Local reasoning/AI integration
boundary**, **U. Skill/plugin/capability packaging**. Rubric:
`design/foundations/TECHNOLOGY_SELECTION_CRITERIA.md` §3, §8; logical
contracts: F7 (isolation), F10 (verifier), `requirements/
KNOWLEDGE_AND_REASONING_MODEL.md` §1/§6 (first-principles pipeline, PART IV-C
research grounding).

---

## 1. Verifier execution mechanism (Q)

> **Recommendation: every verifier runs as an ordinary Ring-3 sandboxed
> process (default isolation tier, `ISOLATION_EVALUATION.md` §4) with a
> read-only or empty grant set; "architecturally independent" verifiers are
> realised by deliberately using genuinely different libraries, algorithms,
> or evidence sources per verifier, not merely two invocations of similar
> logic. Confidence: HIGH.**

No new isolation mechanism is needed — F10's requirements (minimal/no
ambient authority, cannot cause effects, bounded) are exactly what the
default isolation tier already provides for **any** Ring-3 process; a
verifier is simply a Ring-3 process whose grant set happens to be read-only
or empty. The genuinely load-bearing technology decision here is **how to
realise "architecturally independent"** (F10 §3) concretely, since F10 itself
warns that architecturally-independent does not mean statistically
independent:

| Independence axis (F10 §3) | Technology realisation |
|---|---|
| Different evidence source | e.g. one verifier reads the Chronicle's own recorded Event; a second reads the filesystem/OS state directly — genuinely different data paths, not two reads of the same record |
| Different checking mechanism | e.g. exact byte/hash comparison vs. a structural property check (a parser re-validating shape) vs. an SMT-solver constraint check (§3 below) — deliberately drawn from different technique families |
| Different implementation lineage | e.g. a hand-written Rust checker vs. an established third-party library (SQLite's own `PRAGMA integrity_check` for Chronicle-cache verification, or Z3 for a formal constraint) — not two components built from the same code template |
| Determinism class | a deterministic structural check is never *replaced* by a non-deterministic one (e.g. a local-model-based plausibility check); if a non-deterministic verifier is used at all, it is paired with, never substituted for, a deterministic one |

## 2. Mathematical / symbolic computation substrate (R)

> **Recommendation: no CAS in the trusted core. Symbolic-computation needs
> are met by a dedicated, sandboxed capability wrapping an established
> computer-algebra system (SymPy, via a sandboxed Python subprocess) —
> accepting a non-Rust runtime *inside that one isolated capability*, since
> isolation, not language uniformity, is what the architecture actually
> requires. Confidence: MEDIUM — a concrete, "boring over novel" choice, but
> genuinely a different runtime from the core, evaluated honestly below.**

| Candidate | Evaluation |
|---|---|
| **SymPy (Python), sandboxed** | **Selected.** Decades of maturity, an extremely broad symbolic-algebra/calculus feature set, and — critically — this is exactly the "small trusted core + isolated specialised components" pattern (mission §6 option D/E) applied concretely: the capability runs inside the same default isolation tier as any other Ring-3 capability (no ambient authority, no network, filesystem-confined), so accepting Python as its internal runtime does not enlarge the TCB or weaken any F7 property — it only means this *one, replaceable* capability is written differently from the core. |
| **A native Rust CAS crate** | Considered; the Rust symbolic-computation ecosystem is real but far less mature than SymPy's decades of development and validation. **Preferred where it suffices** for simple symbolic tasks (avoids spawning a Python sandbox at all for trivial cases) but not relied upon as the general-purpose substrate — `[DI]`, revisit as the Rust ecosystem matures. |
| **Shelling out to Mathematica/Maple** | Rejected — proprietary, licensing-restricted, and not installable/verifiable in a fully offline, reproducible way consistent with `TECHNOLOGY_SELECTION_CRITERIA.md` §0's cross-cutting MUSTs. |

## 3. Numerical / scientific computation substrate (S) and formal methods

> **Recommendation: native Rust numerical crates (`nalgebra`/`ndarray` for
> linear algebra, `statrs` for statistics) for capabilities that fit them;
> a sandboxed Python (NumPy/SciPy/statsmodels) capability as the fallback
> for advanced statistical/scientific needs; Z3 (via the `z3` Rust crate)
> as the SMT/constraint-solving substrate wherever a formal check is
> warranted. Confidence: MEDIUM-HIGH.**

- **Rust-native first, Python fallback:** keeps the common case (basic
  linear algebra, descriptive statistics, simple numerical routines) inside
  the primary language with no sandboxed-subprocess overhead, while still
  giving access to NumPy/SciPy's much deeper, more battle-tested numerical
  methods when a capability genuinely needs them — the same "isolated
  specialised component" pattern as §2.
- **Z3 for formal/constraint checks:** an actively-maintained,
  Microsoft-developed, MIT-licensed SMT solver with mature Rust bindings.
  Recommended as the default tool wherever `MEL-REQ-297` (uncertainty/
  constraint propagation) or a §39-style formal-verification opportunity
  (e.g. checking that a structured action's `argv`/scope constraints are
  satisfiable, or that a governance risk-floor rule set has no contradictory
  entries) calls for a genuine formal check rather than an ad hoc one.
  **Not** proposed for proving the Reference Monitor's own `AUTHORIZED()`
  predicate correct in full — that is a heavier, `[OPEN]` aspiration (§39
  below), not a near-term deliverable.
- **Full proof assistants (Lean 4, Coq)** — **DEFER.** Recorded as the
  eventual tool of choice *if* a future formal-verification effort on the
  Reference Monitor or the structured-action parser is undertaken (§39), but
  not adopted now; the tooling and expertise investment is disproportionate
  to this phase.

## 4. Local reasoning / AI integration boundary (T)

> **Recommendation: `llama.cpp` (GGUF model format) as the local-inference
> engine, run as its own Ring-3-sandboxed process (default isolation tier,
> loaded on demand, unloaded when idle), with model selection/backend
> (CPU/CUDA/Vulkan) detected at install time. Confidence: HIGH for the
> engine choice; `[OPEN]` for which specific model(s) — deliberately not
> decided here (mission §17 explicitly excludes model-weight selection).**

| Requirement | Realisation |
|---|---|
| Works with network fully disabled (`MEL-REQ-014`, `155`) | `llama.cpp` performs pure local inference with no network dependency by design; the sandbox additionally denies the socket syscalls outright regardless (`ISOLATION_EVALUATION.md` §4), so this holds even if a future model runtime tried to phone home |
| AI is optional to Ring 1 (INV-9) | The inference process is a Ring-3 capability like any other — Ring 1 is fully defined and functional with it absent, stopped, or crashed |
| Loaded/unloaded on demand, multiple specialised models possible | GGUF's memory-mapped loading and `llama.cpp`'s model-swap support make this a natural fit; a specific model is a versioned, provenance-tracked artefact (`MEL-REQ-269`, `327`), not a hard-coded dependency |
| AI must never become the source of truth, the authority, or the sole verifier (mission §49) | **Unchanged by this technology choice** — this is enforced by F3/F10/§74–76 of `REQUIREMENTS_MASTER.md`, not by anything about `llama.cpp` itself. The isolation choice's contribution is narrower but real: **a hallucinating or adversarially-manipulated local model still holds zero ambient authority**, because it runs in a sandbox with no capability grant by default (P1) — the containment does not depend on the model behaving correctly |
| A documented 2026 risk, noted honestly | Local inference removes centralised, provider-side abuse-monitoring guardrails by construction — "offline model runners strip every provider-side guardrail at once" `[E]`. MELFINA's mitigation is architectural, not a monitoring service: the reasoning process has no effect-causing authority regardless of what it outputs (§74–76 of `REQUIREMENTS_MASTER.md`), so the absence of a cloud-side guardrail is compensated by the presence of a *stronger*, local, structural one |

**Rejected as the primary local-inference engine:** a heavier serving
stack (vLLM, SGLang) — these are optimised for multi-request, server-scale
throughput, which is not this project's shape (one user, one machine);
`llama.cpp`'s simplicity, maturity, and broad GGUF ecosystem support are the
better fit (mission §33, don't overengineer).

## 5. Skill / plugin / capability packaging mechanism (U)

> **Recommendation: a two-lane packaging model, matching
> `ISOLATION_EVALUATION.md`'s two-lane isolation model. Lane 1 (preferred
> where it fits): WASI component-model packages for pure-computation
> capabilities with no OS-effect need. Lane 2 (the general case): a
> directory-bundle package (manifest + entry point + provenance/version
> metadata) run as a native subprocess under the default or escalated
> isolation tier. Confidence: MEDIUM-HIGH.**

| Requirement (`MEL-REQ-218–227`, F1 §9) | Realisation |
|---|---|
| Least authority, explicit grants | The manifest declares exactly the effect classes (F1 §3) the capability needs; the Monitor issues only those grants at invocation |
| Provenance, versioning | Every package carries a version, a human-readable changelog note (`MEL-REQ-225`), and its own creation/promotion history as Chronicle-recorded Events |
| Isolation | Lane 1 gets WASI's structural, deny-by-default sandbox "for free"; Lane 2 gets the composed Landlock/seccomp/namespace/cgroup sandbox |
| No authority laundering (`MEL-REQ-222`) | A capability's declared authority is checked against the minting context's own authority at creation time (F1 §9, M8) — a packaging-format field, not a trust decision left to the package itself |
| Runtime-agnostic at the boundary | Because invocation is always "spawn a sandboxed process/component and communicate over the fixed IPC channel" (`PROCESS_AND_IPC_EVALUATION.md`), a capability's internal language/runtime (Rust, Python, a WASI component, `llama.cpp`) is invisible to the Monitor and the Supervisor — this is what licenses §2–4's per-capability runtime choices without weakening the architecture |

**Why WASI gets its own lane rather than being the only mechanism:** as
found in `ISOLATION_EVALUATION.md` §2, WASI's component model has no native
process-spawn or terminal-effect story as of Preview 2/3 — it is the
*better* sandbox for a capability that only computes (math, a pure verifier,
a reasoning-strategy component with no OS effect), and the *wrong* one for a
capability whose entire point is to invoke `execve` or talk to AT-SPI/D-Bus.
Forcing every capability through WASI would either fail to cover the
terminal/GUI capabilities at all, or require building a large,
security-relevant WASI-to-OS bridge — more new, unaudited code than simply
using the native OS sandbox directly for that lane.

## 6. Interaction notes (§25 of the mission)

- **Reasoning × capability system:** the reasoning process (P2) never itself
  holds a capability grant (`PROCESS_AND_IPC_EVALUATION.md` §2) — a
  reasoning-generated *proposal* to invoke a math/AI/verifier capability
  still goes through the full THINK→DECIDE→PROPOSE→AUTHORISE pipeline; no
  technology choice in this document shortens that path.
- **AI runtime × isolation:** `llama.cpp` running under the default tier
  means a model-weight-loading bug or an adversarially-crafted GGUF file
  (a documented general risk class for any format-parsing code) is
  contained exactly as any other Ring-3 capability failure would be (F7 §5).
- **AI runtime × resource management:** GGUF's memory-mapping behaviour
  interacts with the cgroup memory ceiling (`ISOLATION_EVALUATION.md` §4) —
  a large model's mapped-but-not-resident pages should not be counted
  against the same ceiling as genuinely resident memory; this is a concrete
  tuning detail for CORE ENGINE, flagged here.
- **Build system × reproducibility:** a sandboxed Python capability (SymPy,
  NumPy/SciPy) introduces a second package ecosystem (PyPI) alongside
  Rust's crates.io — addressed in `BUILD_AND_SUPPLY_CHAIN.md` with the same
  pinning/vendoring discipline applied to both, not waived for the smaller
  one.

## 7. Human review required

Per mission §46 (the AI runtime boundary is explicitly listed).
**Recommendation: `llama.cpp`/GGUF for local inference (model selection
deferred); SymPy-via-sandboxed-Python for symbolic computation; Rust-native
numerical crates with a Python/SciPy fallback; Z3 for formal/constraint
checks; a two-lane (WASI / native-sandboxed-subprocess) capability packaging
model.**

## 8. Sources

[The Complete Guide to Local LLM Inference Tools, July 2026](https://dev.to/sreeraj-sreenivasan/the-complete-guide-to-local-llm-inference-tools-in-july-2026-llamacpp-ollama-vllm-sglang-and-4mh1) ·
[llama.cpp 2026 Guide](https://weavai.app/blog/en/2026/04/24/llama-cpp-2026-guide-local-ai-inference-setup/) ·
[What Is llama.cpp? Run GGUF Models Locally](https://explainx.ai/blog/what-is-llama-cpp-run-models-locally-2026) ·
`design/foundations/VERIFIER_CONTRACT.md` (F10, carried forward) ·
`design/foundations/ISOLATION_CONTRACT.md` (F7, carried forward, WASI
component-model citation already there).
