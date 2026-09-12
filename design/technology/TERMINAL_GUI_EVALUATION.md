# MELFINA — TECHNOLOGY SELECTION: TERMINAL, FILESYSTEM, AND GUI EVALUATION

**Phase:** TECHNOLOGY SELECTION MISSION 001. First pass, pending human review.
Scope: **M. Terminal/process execution**, **N. Filesystem access mechanism**,
**O. GUI control mechanism**, **P. Resource limiting/watchdog**. Rubric:
`design/foundations/TECHNOLOGY_SELECTION_CRITERIA.md` §8 (terminal), §7 (GUI);
logical contract: `design/foundations/STRUCTURED_ACTION_MODEL.md` (F2).

**CONTAINMENT-CRITICAL** (RC-4). Every recommendation here is scored first
against F2 §5's core invariant: a grant for `program X + argv A` must never
accidentally authorise `X + argv B`, `shell → X`, or added composition.

---

## 1. Terminal / process execution mechanism (M)

> **Recommendation: direct `execve`-family invocation via the language's
> native process API (Rust's `std::process::Command`, which uses
> `posix_spawn`/`fork`+`execve`, never a shell), with an explicit
> environment map, explicit `argv` vector never re-split, one new process
> group per invocation, and `openat2(2)` for TOCTOU-resistant path
> resolution where the kernel supports it (Linux ≥ 5.6, universal on a
> 2026-era system). Confidence: HIGH.**

| Requirement (F2 §2.1, §4, §5) | Technology realisation |
|---|---|
| Resolved absolute program path | `std::fs::canonicalize` at grant-issue time; **re-resolved via `openat2` with `RESOLVE_NO_SYMLINKS`** immediately before `execve`, and the two resolutions compared — a difference is a deny, not a best-effort proceed (F2 §3's TOCTOU requirement) |
| `argv` a vector, never re-split | Native to `std::process::Command::args()` — there is no shell-string stage to re-split at any point |
| Explicit env allow-list | `Command::env_clear()` followed by explicitly setting only the allow-listed variables — never `Command::envs(std::env::vars())` (which would inherit MELFINA's own environment, a documented, easy-to-make mistake this recommendation exists specifically to forbid) |
| Explicit `cwd` | `Command::current_dir()`, itself canonicalised and Landlock-confined |
| No shell interpretation | `Command` never invokes `/bin/sh -c`; `shell-exec` (F2 §2.2) is a **structurally distinct** action type, spawning `/bin/sh` (or the user's chosen shell) as an explicit, named program with its own high-risk grant class — never an implicit fallback of `process-exec` |
| Child process authority = nothing by default (F2 §2.1) | The child is `execve`d **inside** the already-Landlock/seccomp/namespace-confined sandbox process (`ISOLATION_EVALUATION.md` §4's default tier) — it inherits the sandbox's confinement, not MELFINA's own authority |
| Terminable, contained (F7 P7–P8) | Each invocation runs in its own process group (`setsid`/`setpgid` at spawn); the Supervisor can signal the whole group atomically; combined with the sandbox's cgroup, a runaway child tree is killable as one unit |
| Resource bounds (P) | `rlimit` for simple per-process ceilings (`RLIMIT_NOFILE`, `RLIMIT_NPROC`, `RLIMIT_CPU`) set before `execve`; cgroups v2 (already selected, `ISOLATION_EVALUATION.md`) for memory/CPU ceilings enforced from outside the process; a wall-clock watchdog in P1's Supervisor (a deadline check against the invocation's authorised lifetime, F1 §5) kills via cgroup freezer + `SIGKILL` to the process group on timeout |

**Why `openat2` specifically, not just "canonicalise and check":** a
canonicalise-then-open sequence still has a race window between the check and
the actual open (classic TOCTOU) — a symlink could be swapped in between the
two steps. `openat2`'s `RESOLVE_NO_SYMLINKS`/`RESOLVE_BENEATH` flags make
path resolution **atomic with the open itself** at the kernel level, closing
that window structurally rather than by re-checking faster. This is a
concrete, `[SEN]`-strength realisation of F2 §3's "resolve, then re-check at
every guarded operation, deny on any difference" acceptance criterion,
available on any kernel a 2026-era Linux system will be running.

## 2. Filesystem access mechanism (N)

> **Recommendation: two-layer defence — Landlock (already selected,
> process-wide, coarse-grained, set once at sandbox creation) as the outer
> boundary, `openat2` (per-open, fine-grained, atomic) as the inner
> TOCTOU defence within that boundary. Confidence: HIGH.**

Landlock answers "can this process reach this path *at all*" (F7 P4);
`openat2` answers "is *this specific open, right now*, still resolving to
the path it was granted for" (F2 §3's TOCTOU requirement). Neither alone is
sufficient: Landlock's ruleset is set once per sandbox and does not protect
against a symlink swap *within* an already-permitted directory; `openat2`
alone would not stop a process from simply trying a different, unauthorised
path. Together they give both coarse containment and fine-grained
correctness — the same "compose several focused mechanisms" pattern already
used for the isolation tier itself.

## 3. GUI control mechanism (O)

> **Recommendation: AT-SPI2 (the Linux accessibility API, over D-Bus) as
> the primary, structured targeting mechanism; the `xdg-desktop-portal`
> `RemoteDesktop` + `ScreenCast` portals as the explicit, higher-risk
> coordinate-fallback mechanism on Wayland; XTEST as the equivalent fallback
> under X11/XWayland. Confidence: MEDIUM–HIGH for AT-SPI2; MEDIUM for the
> fallback path, given genuine, documented Wayland gaps.**

| Requirement (F2 §2.3) | Technology realisation |
|---|---|
| Structured targeting (role/label/hierarchy, not raw coordinates) | AT-SPI2 exposes an **accessibility tree** — the same interface screen readers use — giving exactly the structured element addressing F2 §2.3 requires; Rust bindings (`atspi`, `zbus` for the underlying D-Bus transport) are actively maintained `[E]` |
| Closed operation set (activate/set-text/select/invoke/read-state/scroll/key) | AT-SPI2's action/text/value interfaces map directly onto this closed vocabulary — no operation outside the accessibility protocol's own defined interfaces is expressible, which is a *structural* narrowing in MELFINA's favour |
| Coordinate fallback explicit and higher-risk | Realised as a **distinct action type and grant** using `xdg-desktop-portal`'s `RemoteDesktop`/`ScreenCast` portals under Wayland (which additionally require the **compositor's own user-consent prompt** before granting screen-capture/input-injection access — a genuine, independent, OS-level authorisation gate on top of MELFINA's own grant, i.e. real defence in depth) or the `XTEST` extension under X11/XWayland |
| Observability for VERIFY (F10 V4) | AT-SPI's read-state interface gives a verifier a read-only way to confirm the resulting UI state without needing the coordinate-fallback path at all — reinforcing that the structured path should be preferred whenever available |

**A genuine, documented gap, stated honestly:** Wayland's security model
deliberately does **not** expose absolute desktop coordinates to arbitrary
clients (a *feature* of Wayland's isolation-by-default design, not a bug) —
meaning coordinate-based automation on Wayland needs a helper (the pattern
used by accessibility-automation tooling like `gnome-ponytail-daemon`) that
goes through the compositor's own remote-desktop/screencast portal APIs
rather than reading coordinates directly `[E]`. This is **more friction, not
less**, for the coordinate-fallback path specifically — which is a
**structural feature, not a defect**, given F2 §2.3 already requires that
path to be the explicit, higher-risk, harder one. **`[ID]`**: the exact
integration approach (whether MELFINA runs its own portal-based helper or
relies on an existing tool's approach) is deferred to CORE ENGINE.

**The user's actual environment** (Ubuntu 24.04, GNOME, Wayland-by-default
per `PROJECT_STATE.md` §7) means this gap is not theoretical — it will be
encountered on day one of building the GUI capability, and the fallback path
should be budgeted for accordingly in any future GUI-capability estimate.

**Rejected as the primary mechanism:** pure computer-vision/screenshot-based
targeting. It is not rejected as *never useful* (it may be the only option
for an application with no accessibility-tree support at all), but it is
explicitly the **least structured, hardest-to-verify** option and is
therefore never the default — consistent with F2 §2.3's own framing
("reject rather than guess" applies here: if an element cannot be found via
AT-SPI and the coordinate fallback is not explicitly authorised, the action
should fail closed, not silently degrade to guessing from a screenshot).

## 4. Resource limiting / watchdog mechanism (P)

> **Recommendation: cgroups v2 (memory, CPU) + POSIX `rlimit` (file
> descriptors, process count, CPU time as a backstop) + a wall-clock
> deadline watchdog implemented as a plain supervising thread in P1.
> Confidence: HIGH.**

No new technology beyond what `ISOLATION_EVALUATION.md` already selected —
this section exists only to confirm that the *terminal-specific* resource
concerns (a runaway child process tree, an unbounded-output command) are
fully covered by the general Ring-3 resource-bounding mechanism, with no
terminal-specific gap: output capture is bounded by piping the child's
`stdout`/`stderr` through a size-capped reader (a plain, well-understood
pattern — read up to a ceiling, then treat further output as a bound
violation, not a silent truncation the user is unaware of).

## 5. Interaction notes (§25 of the mission)

- **Isolation × terminal:** the default isolation tier's seccomp filter
  (`ISOLATION_EVALUATION.md` §4) should explicitly permit exactly the
  syscalls `execve`, `openat2`, `wait4`, and the small set process spawning
  genuinely needs — not a generic "allow most syscalls" filter. This keeps
  the terminal capability's own attack surface as narrow as the isolation
  layer it runs inside.
- **Isolation × GUI:** the AT-SPI/D-Bus connection is itself a form of IPC
  and must be reachable from inside a Ring-3 sandbox — meaning the default
  isolation tier's namespace configuration needs a narrow exception for the
  D-Bus session socket, scoped exactly as tightly as the filesystem/network
  exceptions already are. This is a concrete design detail for CORE ENGINE,
  flagged here so it is not discovered late.
- **GUI × language:** the `zbus`/`atspi` Rust crates give this entirely
  within the primary language recommendation, with no FFI detour needed.

## 6. Human review required

Per mission §46 is not explicit about terminal/GUI specifically, but given
RC-4's CONTAINMENT-CRITICAL tag, this is treated as requiring the same level
of sign-off. **Recommendation: `execve`-only terminal execution with
`openat2` TOCTOU defence; AT-SPI2 primary GUI targeting with a portal-based
Wayland coordinate fallback, explicitly higher-risk.**

## 7. Sources

[Inside Linux computer-use: AT-SPI, XTEST, and background agents](https://cua.ai/blog/inside-linux-computer-use) ·
[Automation through Accessibility (Fedora Magazine)](https://fedoramagazine.org/automation-through-accessibility/) ·
[wayland-accessibility-notes](https://github.com/splondike/wayland-accessibility-notes) ·
[AT-SPI2 Wiki](https://wiki.linuxfoundation.org/accessibility/atk/at-spi/start) ·
`design/foundations/STRUCTURED_ACTION_MODEL.md` (F2, carried forward).
