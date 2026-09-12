# MELFINA — TECHNOLOGY SELECTION: PROCESS MODEL, CONCURRENCY, AND IPC

**Phase:** TECHNOLOGY SELECTION MISSION 001. First pass, pending human review.
Scope: **G. Process model**, **H. Concurrency model**, **I. IPC mechanism**,
**J. Reference Monitor implementation boundary**. Rubric:
`design/foundations/TECHNOLOGY_SELECTION_CRITERIA.md` §5–6; logical contracts:
F3 (Reference Monitor), F6 §7 (Chronicle concurrency), F7 §0/§7 (the three
must-be-real boundaries), RC-7.

---

## 1. The load-bearing constraint: RC-7's three real boundaries

RC-7 (`design/ARCHITECTURE_ADVERSARIAL_REVIEW.md`, carried into F7 §0) states
that MELFINA's ~25 named subsystems are *responsibilities*, not necessarily
separate processes — **except three boundaries, which must be real,
OS-enforced boundaries, not code convention:**

1. **Ring 0 ↔ everything else** (governance/the Reference Monitor is never
   entered, only called).
2. **Ring 1 ↔ the sandbox** (the core mechanism and a running Ring-3
   capability are separated).
3. **Reasoning ↔ effect** (Ring 2 holds no capability grant, cannot invoke
   Ring 3 directly).

This section's entire recommendation is organised around realising exactly
these three boundaries as **process** boundaries — the strongest, most
auditable, least convention-dependent mechanism available — while keeping
everything *not* named above grouped into as few processes as possible, per
RC-7's explicit permission to do so and the mission's "don't overengineer"
test (§33).

## 2. Recommended process topology

| Process | Hosts | Lifetime | Why separate / why grouped |
|---|---|---|---|
| **P0 — Governance** | Ring 0: the trusted loader (F9 startup verification), the Reference Monitor (F3), the governance object (F8) in memory, read-only | Long-lived, started first, supervises nothing else | **Separate by RC-7 rule 1.** The smallest, most privileged, most audited process in the system — its TCB is deliberately minimised (AP-5). It never itself touches the filesystem beyond its own governance file and never spawns a Ring-3 capability directly — it only *authorises*. |
| **P1 — Core** | Ring 1: Chronicle (append + authoritative read), Projection Engine, Query, Entity Registry, Pipeline State Machine, Audit, Capability Registry, Policy Store, Notification Gateway, Supervisor | Long-lived | **Grouped per RC-7** — none of these individually needs a hard boundary from the others; grouping keeps process count, IPC hops, and build/debug complexity down (mission §33). P1 is the only process that spawns and supervises Ring-3 sandboxes. |
| **P2 — Reasoning** | Ring 2: Metareasoning Controller, Context Constructor, Reasoning Contributors, Self-Evaluation, the Reasoner Interface | Long-lived, but restartable independently of P1 (INV-9: the core survives Ring 2's absence) | **Separate by RC-7 rule 3.** P2 connects to P1 **only** through a narrow, read-oriented channel (§4) and can only *emit* proposals back — it is never handed a capability grant, and the OS process boundary makes "Ring 2 cannot invoke a capability directly" true by construction, not by code discipline alone. |
| **P3ₙ — Capability instances** (one per invocation) | Ring 3: one loaded capability, one authorised action | Ephemeral — created for one action, torn down after (`CAPABILITY_MODEL.md` §2) | **Separate by RC-7 rule 2**, realised via `ISOLATION_EVALUATION.md`'s default/escalated tiers. Spawned only by P1's Supervisor, only after P0 has issued a live grant. |

This is a **4-process-class topology** (3 long-lived + 1 ephemeral class),
deliberately minimal. It does **not** split Ring 1's nine responsibilities
into separate processes — RC-7 explicitly does not require that, and doing so
would multiply IPC hops and TCB-auditing surface for no boundary the
architecture actually calls "must be real."

## 3. Reference Monitor implementation boundary (J)

> **Recommendation: P0, its own OS process, communicating with P1 over the
> IPC mechanism in §4. Confidence: HIGH.**

This is more expensive (one more process, one more IPC hop per authorisation
decision) than an in-process module boundary would be — but RC-7 names this
the *first* of the three boundaries that must be real, and a process boundary
is the mechanism that makes "the Monitor is called, not entered" (F7 P9,
P11) true regardless of a bug elsewhere in P1's much larger codebase. A
logic error or a future dependency's memory-safety issue in P1 (Chronicle
parsing, projection rebuilding, notification handling — all larger, more
frequently-changed code than P0) cannot reach P0's authorisation logic or its
loaded governance object, because there is no shared address space to
corrupt. This is the single highest-leverage structural decision in this
document for the meta-invariant's practical strength (though it does not
change `OQ-19`'s theoretical status).

## 4. IPC mechanism (I)

> **Recommendation: Unix domain sockets, `SCM_RIGHTS` file-descriptor
> passing where the payload is an OS-level resource handle, length-prefixed
> framed messages otherwise. Confidence: HIGH.**

| Candidate | Evaluation |
|---|---|
| **Unix domain sockets** | **Selected.** Local-only by construction (no TCP/IP stack involved even on loopback — directly satisfying `TECHNOLOGY_SELECTION_CRITERIA.md` §6's "no TCP/IP even on loopback" MUST-NOT); kernel-enforced peer credentials (`SO_PEERCRED`) give **free, unforgeable process identity** for P0/P1/P2/P3ₙ to authenticate each other without inventing an application-level protocol; supports `SCM_RIGHTS` ancillary messages to pass open file descriptors between processes. |
| **Pipes** | Simpler, but no built-in framing, no peer-credential authentication, no fd-passing — would need to be built on top, at which point a Unix domain socket already does more for the same cost. Not selected as the primary channel; may still appear as a child process's stdio, which is a different, already-covered concern (`TERMINAL_GUI_EVALUATION.md`). |
| **Shared memory** | Rejected as the primary IPC — shared mutable state is exactly the kind of ambient, unmediated channel F7 P1/P2 exist to prevent between Ring 2 and Ring 3, and it would need its own authentication/locking protocol built from scratch. Not ruled out as a narrow, explicitly-granted optimisation later (e.g. for bulk data transfer under an already-issued grant) — **DEFER**. |
| **A full RPC framework (gRPC, Cap'n Proto RPC, etc.)** | Rejected as unnecessary weight for a single-machine, four-process-class system — pulls in a serialization/schema toolchain and, for gRPC, an HTTP/2 stack that has nothing to do with MELFINA's actual needs (mission §33, "don't overengineer"). A hand-rolled, small, versioned message format on top of Unix domain sockets is simpler to audit and has a far smaller TCB. |

**Why file-descriptor passing matters beyond convenience — a genuine finding
for F1's deferred grant representation (F1 §19):**

A grant bound to an OS-level resource (an open file with `O_PATH`-restricted
flags, an already-`execve`-ready process handle, a bound socket) can be
represented as **a file descriptor passed once, over `SCM_RIGHTS`, from P0/P1
to a P3ₙ sandbox**. A process that was never handed that descriptor
**structurally cannot forge, guess, or replay it** — the kernel itself
enforces this, independent of any userspace representation MELFINA invents.
This gives F1 §19 / F7 P10 ("cannot forge a grant") a **`[SEN]`-strength**
realisation for exactly the class of grants that are OS-resource-shaped
(file access, a bound listening capability, a prepared child-process
handle), rather than the `[ID]` status the foundation contract had to assume
before a mechanism was chosen. For grants that are **not** OS-resource-shaped
(e.g. "you may append a Claim of this shape to the Chronicle"), a small
signed/MAC'd token passed over the same socket remains the fallback
representation — this part of F1 §19 stays `[ID]`, unchanged by this
finding.

## 5. Concurrency model (H)

> **Recommendation: synchronous, thread-based concurrency with explicit
> single-writer discipline where the contracts require it — no async
> runtime in P0 or P1's Chronicle-append path. Confidence: HIGH for P0/the
> Chronicle writer; MEDIUM for whether P1's Supervisor or P2 ever adopt
> async later.**

| Model | Where it fits | Why / why not |
|---|---|---|
| **P0 (Monitor): single-threaded, one authorisation request processed at a time** | The whole of P0 | F3 §2 requires deterministic authorisation and F1 §11's aggregate-budget accounting is exactly the kind of shared mutable state that a single-threaded event loop makes trivially race-free. Throughput is not a concern at this project's scale (one user's action rate). **This is the simplest model that satisfies the requirement (mission §14's stated preference) — no justification exists for anything more complex here.** |
| **P1 Chronicle append path: single-writer, explicitly serialised** | The append operation only | Restates F6 §7 directly: the Chronicle is *the* serialisation point; a dedicated single-writer thread (or an actor-style mailbox with one consumer) realises this without needing a database-level lock manager. |
| **P1 Chronicle reads / projection queries: concurrent** | Read paths | F6 §7 explicitly allows concurrent readers; the SQLite cache (`CHRONICLE_EVALUATION.md` §6) is already safe for multiple readers under WAL mode. Ordinary OS threads, no async runtime needed. |
| **P1 Supervisor: one thread (or task) per supervised Ring-3 child process** | Process supervision | Supervising a handful of concurrently-running Ring-3 sandboxes is fundamentally "wait on a small number of OS processes," which `waitpid`/`signalfd`-style blocking or a small thread pool handles cleanly, without pulling in an async runtime purely to multiplex a handful of child-process lifetimes. |
| **An async runtime (e.g. Tokio) anywhere in P0/P1** | **Not recommended at this phase** | Adds a large, actively-evolving dependency and a cooperative-scheduling model to reason about, for a workload (one user, a handful of concurrent actions) that does not need it. Revisit **only** if a measured I/O-concurrency bottleneck appears — a concrete, falsifiable trigger, not a stylistic preference. |
| **P2 (Reasoning): synchronous by default; concurrency only if a specific reasoning strategy genuinely benefits from parallel exploration** | Reasoning strategies (§45 of REQUIREMENTS_MASTER.md) | Kept open — reasoning workloads are the least well-understood at this phase (OQ-21, OQ-31). Starting synchronous avoids premature complexity; nothing here blocks introducing structured concurrency (e.g. scoped threads for parallel candidate-plan evaluation, `MEL-REQ-260`) later if a concrete case emerges. |

**Rejected as a default:** actor frameworks and full async-everywhere
architectures. Both are legitimate patterns in general, but neither is
justified by this project's actual concurrency shape (a handful of
long-lived processes, a handful of concurrent Ring-3 children, one user) —
adopting one by default would be exactly the kind of complexity mission §33
warns must "earn its place."

## 6. Failure and crash-recovery notes (mission §27, process-level)

| Failure | Behaviour under this topology |
|---|---|
| P3ₙ crashes | Contained to that process; P1's Supervisor observes it (via `waitpid`) and follows F4/F7 §5's failure response; P0, P1's other responsibilities, and P2 are unaffected |
| P2 crashes | P1 continues fully (INV-9); P0 is unaffected; P2 is restarted by P1's Supervisor, restoring reasoning once healthy |
| P1 crashes | The most consequential failure in this topology: no new Chronicle appends, no new authorisation requests can be *served* by the whole system in practice — but the Chronicle **file itself is durable** (`CHRONICLE_EVALUATION.md` §4) and P0's governance state is independent, so recovery is "restart P1, replay/rebuild the projection cache" (F6 §4), not data loss |
| P0 crashes | The most privileged process failing; per F9's fail-closed doctrine, MELFINA does not continue authorising anything — P1 must detect P0's absence and refuse to treat any pending or new proposal as authorised until P0 restarts and re-verifies governance from scratch (F9 §3) |
| IPC channel between P1↔P2 or P0↔P1 breaks | Fail-closed: no authorisation without P0 reachable; no new reasoning-driven proposals without P2 reachable, but P1's deterministic, pre-authorised routine operations continue |

## 7. Interaction notes (§25 of the mission)

- **Process × IPC:** the `SCM_RIGHTS` mechanism (§4) only works over Unix
  domain sockets between processes on the same host — reinforcing, at the
  IPC layer, that this design has no path to a remote/networked variant
  without a deliberate, separate redesign (consistent with INV-1).
- **Isolation × IPC:** a P3ₙ sandbox's only channel back to P1 is the one
  Unix domain socket connection it was launched with; the default isolation
  tier's namespace/Landlock policy denies it any other IPC surface (no
  arbitrary abstract-socket or shared-memory access), so the capability
  cannot open a side channel to another P3ₘ instance or forge a connection
  to P0 directly.
- **Governance × OS security:** P0's isolation should itself use the same
  default tier as Ring-3 sandboxes, *inverted* — instead of confining what
  P0 can reach, the goal is confining what can reach *into* P0 (a smaller,
  simpler policy: P0 accepts connections only from P1's known socket path,
  with peer-credential checks on every connection).

## 8. Human review required

Per mission §46 (the Monitor's implementation boundary is explicitly listed).
**Recommendation: the 4-process-class topology (P0/P1/P2/P3ₙ) with Unix
domain sockets + `SCM_RIGHTS` as the IPC mechanism, and a synchronous,
single-writer-disciplined concurrency model with no async runtime in the
trusted core.**

## 9. Sources

Reasoning here is primarily an application of already-cited foundation
contracts (RC-7, F3, F6 §7, F7) to concrete OS mechanisms; no new external
research claims are made beyond standard, well-established Unix IPC
semantics (`SCM_RIGHTS` file-descriptor passing, `SO_PEERCRED`), which are
part of the POSIX/Linux ABI rather than a contested or fast-moving area
requiring citation.
