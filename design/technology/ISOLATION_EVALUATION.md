# MELFINA — TECHNOLOGY SELECTION: RING-3 ISOLATION EVALUATION

**Phase:** TECHNOLOGY SELECTION MISSION 001. First pass, pending human review.
Scope: **F. Ring-3 isolation mechanism** (this is mission §8's highest-risk
decision). Rubric: `design/foundations/TECHNOLOGY_SELECTION_CRITERIA.md` §3;
logical contract: `design/foundations/ISOLATION_CONTRACT.md` (F7, properties
P1–P14).

---

## 1. Candidates evaluated

| Candidate | Category | Why considered |
|---|---|---|
| **Landlock + seccomp-bpf + user/mount/network namespaces + cgroups v2** (composed, unprivileged) | OS process isolation | The current (2025–2026) Linux-native answer to "unprivileged, self-applied, no root, no container runtime" sandboxing; each mechanism is independently mature and they compose as defence-in-depth. |
| **Containers (e.g. a container runtime used purely for its namespace/cgroup machinery, no orchestration)** | OS process isolation, packaged | The common industry default; evaluated to be explicitly rejected as a *default*, not ignored. |
| **gVisor** (user-space kernel intercepting syscalls) | Syscall-interception isolation | Stronger isolation than raw namespaces without a full VM's overhead; mature, widely deployed for untrusted-workload isolation. |
| **Firecracker-class microVM** | Hardware-virtualised isolation | The strongest isolation category available on commodity hardware; evaluated for the *high-risk capability tier*, not the default. |
| **WebAssembly / WASI (component model, Preview 2/3)** | Language-level capability sandbox | A sandbox with **no ambient authority by default at the language-runtime level**, structurally different from composing OS primitives; strong fit for CPU-bound, non-effect-causing capabilities. |
| **seL4 / a dedicated microkernel** | Formally verified isolation | The strongest *proof-backed* isolation category in existence; evaluated and rejected for this project's shape, reasons below. |

## 2. Mapping candidates onto F7's fourteen required properties

`H` = the property is satisfied by this candidate at the mechanism level;
`P` = partially, needs composition with another mechanism; `–` = not
addressed by this candidate alone.

| F7 property | Landlock+seccomp+ns+cgroups | Containers (bare) | gVisor | Firecracker microVM | WASI (Preview 2/3) | seL4 |
|---|---|---|---|---|---|---|
| P1 no ambient authority | **H** — default-deny is the explicit design of Landlock ("any process… can securely restrict its own access rights") `[E]` | P — depends entirely on the image/runtime configuration, easy to misconfigure permissively `[DI]` | H | H | **H, and structurally by construction** — components declare their WIT world; unlisted imports are unreachable, not merely denied `[E]` | H (proof-backed) |
| P2 explicit capability access only | H — grants map onto Landlock rule-sets + seccomp filters per invocation | P | H | H | **H** — this is WASI's whole design point | H |
| P3 zero network by default, primitive withheld not unconfigured | **H** — seccomp can deny `socket()`/`connect()`/`bind()` outright (not merely restrict), which is the "withheld" property F7 P3 requires, not Landlock's network-*port* restriction alone `[DI]` | P | H (syscall interception denies it) | H (no NIC attached) | **H** — no socket import in the component's WIT world, nothing to call `[E]` | H |
| P4 filesystem confinement per grant | **H** — Landlock's core purpose `[E]` | P | H | H | H — WASI filesystem capabilities are explicitly granted directories only `[E]` | H |
| P5 process authority per grant, no inherited authority | H — namespaces + seccomp on `execve`/`clone` | P | H | H | P — WASI has no native process-spawn model in the core component model as of Preview 2/3; would need a host-mediated bridge | H |
| P6 hard resource bounds | H — cgroups v2 + `rlimit` | P | P — resource limits layered on, not native | H — VM-level memory/CPU caps | P — engine-level fuel/memory limits (e.g. Wasmtime's), not OS-enforced | H |
| P7 terminable | H — signal/cgroup-freezer kill | P | H | H | H — engine can abort execution | H |
| P8 failure containment | H — process boundary + Supervisor | P | H | **H, strongest of the process-level options** — a crashed guest kernel cannot touch the host kernel | H — a WASM trap does not corrupt the host process | H (proof-backed) |
| P9 no Ring-0 access | H — simply never bind-mount/map the Ring-0 path into the sandboxed mount namespace | P | H | H | H — not in the WIT world | H |
| P10 cannot forge a grant | ID — depends on the chosen grant representation (F1 §19), not the isolation mechanism itself | ID | ID | ID | ID | ID |
| P11 cannot impersonate the Monitor | ID — depends on IPC authentication (§ below), not isolation alone | ID | ID | ID | ID | ID |
| P12 reasoning↔effect boundary real even if co-located | Structural (design-level), not mechanism-level | Structural | Structural | Structural | Structural | Structural |
| P13 no persistent inter-action state | H — fresh namespace/process per invocation | P | H | H | H — a fresh component instance per invocation | H |
| P14 lightweight (dormant capabilities cheap) | **H — process launch is ~1–5 ms class, cgroup/namespace setup is cheap** `[DI]` | P — image pull/layer overhead unless pre-warmed | P — 10–30% overhead on I/O-heavy workloads `[E]` | **P — ~125 ms startup, ~5 MB per instance `[E]`; too heavy for routine, frequent, low-risk invocations** | **H — sub-millisecond-class instantiation for a small component** `[DI]` | H in principle, but the whole system would need to be built around it |

## 3. Why not just pick the single "best" one

No single mechanism dominates on all fourteen properties at acceptable cost.
Landlock+seccomp+namespaces+cgroups is the strongest **default-tier** choice
(cheap, mature, unprivileged, 2026-production-ready per Landlock's own
maturity trajectory); Firecracker-class microVMs are the strongest
**isolation-tier** choice but too expensive to use for every routine
capability invocation given AP-12/F7-P14's lightweightness requirement; WASI
is structurally the cleanest match to "no ambient authority" but has real
gaps for process-spawning and terminal-effect capabilities specifically. This
motions directly toward a **tiered design**, not a single winner.

## 4. Recommendation — a tiered isolation architecture

> **DEFAULT TIER (the vast majority of Ring-3 capability invocations):
> composed Landlock + seccomp-bpf + Linux namespaces (mount, user, network,
> PID) + cgroups v2, unprivileged, one fresh sandbox per invocation.**
> **Confidence: HIGH.**
>
> **ESCALATED TIER (reserved for capability classes the governance object
> (F8) marks high-risk, or genuinely untrusted/unreviewed code): a
> Firecracker-class microVM.** **Confidence: MEDIUM** — the mechanism is
> mature and well-evidenced; *which* capability classes warrant it is a
> governance-content decision (F8 §2.4), not a technology decision, and is
> explicitly deferred.
>
> **COMPLEMENTARY, DOMAIN-SPECIFIC TIER: WASI (component model) for
> CPU-bound, non-effect-causing capabilities** — symbolic/numerical
> computation, a pure verifier that only reads and computes (F10), or a
> reasoning-strategy component (M3) that genuinely needs no OS-level effect
> at all. **Confidence: MEDIUM** — evaluated further in
> `REASONING_COMPUTATION_EVALUATION.md`; not recommended as the *general*
> Ring-3 mechanism because of its process-spawn/terminal-effect gaps (P5
> above).

**Why the default tier wins for the common case:**

1. **Cost matches F7 P14 directly.** A microVM's ~125 ms / ~5 MB per instance
   is real, well-measured overhead `[E]` that would make "capabilities are
   dormant and cheap when unused" (AP-12) false in practice if applied to
   every invocation of, say, a routine file-read capability. Process-level
   isolation with Landlock+seccomp+namespaces is order-of-magnitude cheaper
   and lets the system genuinely support "a large capability surface, flat
   resident footprint" (F7 P14's acceptance criterion).
2. **Landlock's specific 2026 maturity claim is load-bearing here.**
   Landlock was historically the weakest link in "unprivileged, no root, no
   container runtime" sandboxing on Linux; by the 2025–2026 kernel cycle it
   is independently reported as reaching production-ready maturity for
   exactly this use (filesystem **and** network restriction, self-applied,
   no privileged setup) `[E]`. This is a *recent* fact, not an old one — a
   TECHNOLOGY SELECTION mission run even a year or two earlier would have
   had to lean more heavily on containers or a heavier mechanism.
3. **Composition, not a single silver bullet, delivers F7's properties.**
   No single one of {Landlock, seccomp, namespaces, cgroups} alone satisfies
   P1–P9; **together** they do, and existing Rust crates already compose
   them without requiring root — directly reducing the amount of
   isolation-policy code MELFINA itself must write and audit (smaller TCB,
   §26).
4. **Structural, not merely configured, network absence (P3).** The
   distinction F7 P3 draws — "the socket primitive is withheld, not
   unconfigured" — is realised concretely by having seccomp **deny the
   `socket`/`connect`/`bind` syscalls outright** for a sandboxed capability
   process, not merely by omitting a network namespace's interfaces (which a
   determined process could still attempt to use, and would only be stopped
   by a *policy*, not an *absent primitive*). This is the correct technical
   reading of L4/RC-6's "cannot, not does not."

**Why the escalated tier exists rather than "just use microVMs everywhere":**

Firecracker/gVisor-class isolation gives genuinely stronger containment
(dedicated or emulated kernel, P8's strongest form) — appropriate for the
rare high-risk action (F8 §2.4's risk floors already identify these: shell
interposition, coordinate-fallback GUI actions, file-delete/truncate/rename,
and any capability the governance object has not yet built trust in). Using
it *only* there keeps the cost proportionate to the risk, matching the
mission's own hierarchy (§3: safety first, but performance/resource cost is
still a real, later-ranked criterion, not one to ignore).

**Rejected:**
- **Bare containers as the default mechanism** — a container *runtime*
  (Docker-class) adds image-management, daemon, and layered-filesystem
  complexity that is not needed here and that the mission's "don't
  overengineer" test (§33) explicitly warns against; its actual isolation is
  provided by the *same* underlying namespaces/cgroups/seccomp primitives
  MELFINA can use directly, so the container runtime itself is pure added
  surface with no isolation benefit over using the primitives natively.
  **Confidence in rejection: HIGH.**
- **seL4 / a dedicated microkernel as the whole system's foundation** — the
  strongest possible isolation category (formally proven), but it would mean
  building MELFINA as a seL4 userland from the ground up, which is a
  multi-year, specialist undertaking wildly disproportionate to a
  single-user personal system, and would sacrifice virtually all of the
  mature Linux desktop ecosystem (GUI toolkits, AT-SPI, D-Bus, the terminal
  itself) that Ring-3 capabilities need to interoperate with. **Rejected as
  the platform; not rejected as an inspiration** — F7's property list (§4 of
  the foundation contract) is explicitly modelled on seL4's
  confidentiality/integrity/availability framing, so its *ideas* are already
  incorporated even though its *implementation* is not adopted.
  **Confidence in rejection: HIGH.**

## 5. What happens if isolation fails (F7 §5) — technology-specific notes

- A **default-tier escape** (a Landlock/seccomp/namespace bypass) is, per the
  current public record, rare and typically requires a kernel-level bug
  distinct from a misconfiguration of the sandboxing policy itself — this is
  an argument for keeping the *policy* minimal and generated mechanically
  from the grant (F1), not hand-tuned per capability, to keep the
  configuration-error surface small.
- An **escalated-tier escape** (a microVM/gVisor breakout) would be a
  significant, rare event; F7 §5's stop-condition response (revoke the
  grant family, retire the capability, treat the mechanism as suspect) holds
  regardless of which tier failed.
- Both tiers' failure responses route through the same Supervisor and
  emergency-stop machinery (F8 §2.8) — the technology choice does not change
  F7's failure-response contract, only its likelihood and blast radius.

## 6. Empirically testable vs. structural — per F7 §6, updated with concrete
technology

| Property | Guaranteed by | How this technology choice is tested |
|---|---|---|
| default-deny (P1) | the mechanism (Landlock/seccomp policy applied before any capability code runs) | load a capability with no grants, attempt every effect class, expect all denied — `TECHNOLOGY_EXPERIMENT_PLAN.md` |
| network primitive withheld (P3) | seccomp syscall denial, not namespace configuration alone | attempt `socket()`/`connect()` from inside a sandboxed capability; expect an immediate syscall-level failure, not a routing failure |
| filesystem/process confinement (P4–P5) | Landlock + namespaces | an adversarial capability corpus attempting path traversal, symlink escape, and unauthorised `execve` |
| resource bounds (P6) | cgroups v2 + `rlimit` | a capability that loops/allocates/forks; expect cgroup-enforced termination at the ceiling |
| escalated-tier containment (P8, strongest form) | Firecracker/gVisor | fault-injection inside a microVM guest; assert host untouched |

## 7. Interaction notes (§25 of the mission)

- **Language × isolation:** Rust's `nix`/Landlock/seccomp crates (found
  actively maintained and composing without root, §1) make the default tier
  natively reachable from the recommended primary language with no FFI
  detour — a direct, positive interaction, reinforcing the language
  recommendation.
- **Isolation × terminal (§ below):** the default tier's `execve` confinement
  is exactly the mechanism `TERMINAL_GUI_EVALUATION.md` relies on to make
  F2's "structured action, never a shell" invariant enforceable at the OS
  level, not only at the application level.
- **Isolation × verifier (F10):** a verifier's "minimal/no ambient
  authority, cannot cause effects" requirement is naturally satisfied by
  running it in the **default tier with an empty or read-only grant set** —
  no special verifier-specific isolation technology is needed.
- **Isolation × AI runtime:** see `REASONING_COMPUTATION_EVALUATION.md` for
  whether a local-model inference process needs its own isolation tier
  (recommended: yes, default tier, treated as any other Ring-3 process).

## 8. Human review required

Per mission §46, the isolation mechanism is one of the decisions requiring
explicit sign-off. **Recommendation: the tiered design above.** The specific
governance-content question of *which* capability classes escalate to the
microVM tier is deferred to the genesis governance authoring (F8 §8), not
decided here.

## 9. Sources

[Landlock: kernel documentation](https://docs.kernel.org/security/landlock.html) ·
[Landlock: Unprivileged Sandboxing](https://landlock.io/) ·
[sandbox-rs / sandlock-core (Landlock+seccomp composition in Rust)](https://oneuptime.com/blog/post/2026-01-07-rust-sandboxing-seccomp-landlock/view) ·
[WASI and the WebAssembly Component Model: Current Status](https://eunomia.dev/blog/2025/02/16/wasi-and-the-webassembly-component-model-current-status/) ·
[WASI Security Roadmap](https://www.systemshardening.com/articles/wasm/wasip3-security-roadmap/) ·
[How to sandbox AI agents in 2026: MicroVMs, gVisor & isolation strategies](https://northflank.com/blog/how-to-sandbox-ai-agents) ·
[Your Container Is Not a Sandbox: The State of MicroVM Isolation in 2026](https://emirb.github.io/blog/microvm-2026/) ·
`design/foundations/ISOLATION_CONTRACT.md` (F7, carried forward, not re-derived).
