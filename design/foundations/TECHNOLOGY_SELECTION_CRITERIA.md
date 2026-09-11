# MELFINA — FOUNDATION 12: TECHNOLOGY-SELECTION CRITERIA (for the NEXT phase)

**Phase:** LOW-LEVEL FOUNDATIONS — MISSION 001. First pass, pending review.
**Baseline:** `8a40207`. Read F1–F11 first.

**This document chooses NOTHING.** It defines the criteria — **MUST HAVE /
SHOULD HAVE / MUST NOT HAVE** — against which the *next* phase (TECHNOLOGY
SELECTION) will evaluate candidates for each area. Every criterion traces to a
foundation contract or a requirement, so a candidate can be scored, not argued.

Derives from: F1–F11; **AP-5, AP-12**; **MEL-REQ-155, 174, 179, 184, 192, 193,
196, 197, 198, 199, 200**; the deliberately-undecided list in
`requirements/OPEN_QUESTIONS.md` (items 2, 3, 5, 6, 13, 16).

**Scoring note.** "MUST HAVE" failures disqualify a candidate. "SHOULD HAVE"
failures are recorded as risk and weighed. "MUST NOT HAVE" presence disqualifies.
Where two areas' choices interact (language ↔ isolation, storage ↔ crypto), the
next phase evaluates the **combination**, not each in isolation.

---

## 0. Cross-cutting criteria (apply to every area)

**MUST HAVE**

- **Runs fully offline / locally**, no phone-home, no mandatory network service
  (`MEL-REQ-155`, INV-1, F7 §7, F11 Path H).
- **Single-maintainer sustainable** — one person can understand, operate,
  upgrade, and debug it (`MEL-REQ-192`, AP-12).
- **Minimal dependency surface** — few transitive dependencies, each auditable
  (`MEL-REQ-193`, AP-5).
- **Inspectable / debuggable** with ordinary local tooling.
- **Stable enough** to still build and run in years, not months (`MEL-REQ-197`).
- **Permissively licensed** for a personal single-user system, no per-seat or
  usage-metered licensing.

**SHOULD HAVE**

- Mature, boring, widely deployed; large enough community that problems are
  already documented.
- Good failure diagnostics (clear errors, not silent corruption).
- Reproducible builds.

**MUST NOT HAVE**

- A mandatory cloud account, telemetry that cannot be disabled, or a licence
  server.
- A track record of silent data loss or undebuggable failure modes.
- A dependency tree so large it cannot be reviewed by one person.

---

## 1. Implementation language(s)

*(OPEN_QUESTIONS deliberately-undecided item 3.)*

**MUST HAVE**

- **Memory- and type-safety by default** for the trusted computing base (the
  Reference Monitor, the trusted loader, the Chronicle append path, the
  isolation supervisor). Unsafe escape hatches allowed only where audited
  (AP-5, F9 §0, F3 §2).
- **Deterministic execution** available for the enumerated safety-critical paths
  — authorisation decision, canonicalisation, governance verification, chronicle
  ordering (`MEL-REQ-174`, F2 §4, F3 §4).
- **A credible isolation story** — either first-class OS-primitive access
  (spawn, drop privileges, restrict syscalls) or a capability-safe execution
  model, so F7's properties are reachable without fighting the language.
- **Straightforward FFI / process control** to drive terminal and GUI without a
  heavyweight runtime bridge.
- **Ahead-of-time or self-contained deployment** — the running system does not
  depend on a package manager being online.
- **Long-term toolchain stability**, active security maintenance.

**SHOULD HAVE**

- A strong static type system (helps encode the closed vocabularies of F1/F2/F5
  as types the compiler checks).
- Low idle resource footprint (AP-12, `MEL-REQ-196`).
- Good concurrency primitives for the single-writer Chronicle + supervised
  workers (F6 §7).
- One primary language for the whole core (economy of mechanism, AP-5); a second
  language only with a clear reason (e.g. a verified component).
- Ecosystem support for the storage and crypto choices below.

**MUST NOT HAVE**

- A mandatory large managed runtime / VM that dominates the resident footprint.
- Pervasive undefined behaviour in the safe subset.
- A garbage collector with unbounded pause behaviour **on the hard-real-time
  paths** — soft real-time is fine; the emergency-stop and revocation paths must
  stay responsive (F4 §3, F8 §2.8).
- Reliance on a single-vendor closed toolchain.

---

## 2. Chronicle storage substrate + format

*(OPEN_QUESTIONS deliberately-undecided item 2.)*

**MUST HAVE**

- **Durable append** — a unit is not acknowledged until it survives power loss
  (fsync / durability barrier); "durable before APPEND returns" (F6 §1, §2).
- **Total order preserved** — a single monotonic transaction-time sequence
  (F5 §2, F6 §1).
- **Atomic unit append** — a unit is fully present or absent, never half
  (F6 §2); torn-write detection on recovery (F6 §8).
- **Corruption detection** — structural verification finds bit-rot / truncation
  without silent repair (F6 §5, `MEL-REQ-170`).
- **Crash recovery to a known-good prefix** — after a crash, the log is readable
  up to the last complete durable unit (F6 §8, `MEL-REQ-173`).
- **Full history retained** — no compaction that discards superseded units;
  redaction is an explicit append, not a delete (F5 §7, §8, F6 §6).
- **Rebuildable projections** — every derived view can be reconstructed
  deterministically from the log (F6 §4, §6).
- **Local file(s) on the user's disk**, portable, backup-able by copying
  (`MEL-REQ-160, 171`).
- **Bitemporal query support** — either native or cleanly implementable over the
  substrate: as-of-valid-time × as-of-transaction-time (F5 §5, F6 §3).

**SHOULD HAVE**

- Single-writer friendly (F6 §7 — the Chronicle is the serialisation point; no
  distributed consensus needed).
- Efficient range scans and provenance-link traversal for projection rebuild.
- A stable, documented on-disk format (so a future tool can read it even if the
  engine changes) — favours an open format over an opaque one.
- Reasonable storage efficiency for a decade of one person's data (not a
  concern at small scale, but not pathological).
- Independent verifiability of the log (a second tool can validate structure +
  provenance — F6 §5, F10).

**MUST NOT HAVE**

- A server process that must be running and reachable over a network socket as
  the only access path (conflicts with local-first, single-maintainer, INV-1).
  *(A purely-local embedded engine is fine; a local-only loopback service is a
  weigh-the-risk SHOULD-NOT.)*
- Silent lossy behaviour under disk-full / power-loss / concurrent-access.
- In-place mutation as the only update model (the Chronicle is append-only —
  F5 §2).
- A format that cannot be inspected or salvaged without the original engine
  version.
- A schema migration story that requires rewriting history.

**Note.** The choice must not turn the Chronicle into anything other than "a
totally-ordered append-only sequence of E²CI units" (F5 §1). A relational or
document store *may* be used as the substrate, but the **logical model stays
E²CI** — the store is an implementation detail, and Context/State/Relation/Task/
Goal/etc. **do not become tables or collections** (F5 §1, mission constraint).

---

## 3. Ring-3 isolation mechanism

*(OPEN_QUESTIONS deliberately-undecided item 6 is adjacent — external permission
monitor design.)*

**MUST HAVE** (each maps to an F7 property)

- **Default-deny** — a loaded component has zero authority until grants are
  wired (F7 P1). This is the single most important criterion.
- **Withhold the network primitive entirely** — a socket cannot be opened from
  inside, structurally (F7 P3, L4).
- **Filesystem confinement to an explicit path set** (F7 P4), with the Ring-0
  region excludable (RC-1).
- **Process-spawn control** — child creation only via an explicit grant, child
  inherits no authority (F7 P5).
- **Hard resource limits** — CPU, memory, wall-clock, child count, output size,
  as enforced cut-offs (F7 P6, `MEL-REQ-183`).
- **Forcible termination** — the host can kill the instance and its subtree; the
  instance cannot make itself unkillable (F7 P7).
- **Fault containment** — a crash/hang/exploit inside does not reach the host
  (F7 P8, INV-11).
- **No path to the host's privileged state** — the Monitor is *called*, not
  *entered* (F7 P9, P11).
- **Low per-instance cost** — create + tear down cheaply enough that a large
  capability surface stays dormant-cheap (F7 P14, AP-12, `MEL-REQ-196, 199`).

**SHOULD HAVE**

- A small, auditable trusted base for the mechanism itself (AP-5).
- Fast boundary crossing (grants presented per effect — F1, F3).
- Introspectable — the host can observe what an instance attempts (for the audit
  record, F11 Paths E/F).
- Composability with the language choice (§1) without a fragile bridge.
- A credible red-team track record for the confinement claims (F7 §6 `[ETL]`).
- Optionally, a path toward formal verification of the mechanism (AU-5,
  `[OPEN]`).

**MUST NOT HAVE**

- Ambient authority by default (disqualifying — F7 P1).
- A design where "no network" is a configuration flag rather than an absent
  primitive (F7 P3).
- Shared mutable state between instances that breaks confidentiality/integrity
  (F7 §4).
- Escape vectors that are known-and-unfixed.
- A footprint per instance that makes AP-12's "capabilities are dormant when
  unused" false.

**Explicitly NOT prejudged:** OS process + syscall filtering, a lightweight VM,
a Wasm component runtime, a language-level capability sandbox, a
capability-microkernel host — all evaluated against the above. Co-location of
Ring-2 and Ring-3 code is permitted **iff** the authority distinction remains a
real enforced check (F7 §7, O2 rejected).

---

## 4. Cryptographic integrity (governance chain — F9)

*(OPEN_QUESTIONS deliberately-undecided item 5 is adjacent — permission-grant
representation; item 6 — external monitor.)*

**MUST HAVE**

- **Asymmetric signature** over `hash(content) ‖ parent-version-id`, verifiable
  with public material MELFINA may hold, signable only with a key MELFINA does
  **not** possess in usable form (F9 T1, T2, §6 authenticity).
- **Collision-resistant hash** for the chain links (F9 C2, §6 integrity).
- **Key custody off the running system** — at minimum an offline key on the
  maintainer's separate machine; hardware custody is an allowed upgrade, not a
  requirement (F9 §1, AU-6).
- **A protected current-head marker** mechanism — a second signed pointer, a
  monotonic counter, or an OS-protected file — resisting rollback/downgrade
  (F9 C7, §6).
- **Anchor-integrity** — the verification anchor MELFINA holds is human-installed
  and checkable at startup; MELFINA cannot silently substitute it (F9 T3).
- **Deterministic verification** — same chain, same verdict, every startup
  (F9 §3, `MEL-REQ-174`).
- **Small enough to sit in the trusted loader** with minimal dependencies
  (F9 §0, AP-5).
- **Standard, well-reviewed primitives** — no bespoke cryptography.

**SHOULD HAVE**

- A widely-implemented scheme (so a second tool can independently re-verify the
  chain — F9 §7).
- A threshold / multi-signature option (TUF lesson — F9 §6) for future
  robustness, even if single-signer at first.
- Forward-compatible agility (the ability to rotate to a stronger primitive via
  a new governance version).
- Hardware-token support available if the maintainer later wants it (AU-6).

**MUST NOT HAVE**

- Any design where the signing key must be present on the running system
  (disqualifying — F9 T1, T4).
- Home-grown or unreviewed cryptographic constructions.
- A primitive with known practical weaknesses or a deprecated status.
- A verification step that depends on a network service (CRL/OCSP-style online
  checks) — offline verification only (INV-1).
- Dependence on the Chronicle for integrity (must be independent — F9 §6).

---

## 5. Process / concurrency model

**MUST HAVE**

- **A single-writer path to the Chronicle** — appends serialise through one
  point; no multi-writer race on the log (F6 §7).
- **Supervised workers** — Ring-3 instances run under a supervisor that can
  start, stop, restart, and abandon a subtree (F7 P7, P8; INV-11 "let it
  crash").
- **An out-of-band control path** for emergency stop that works even if Ring 1 /
  Ring 2 are unhealthy (F8 §2.8, F9 §1).
- **Bounded, observable concurrency** — the number of concurrent Ring-3
  instances is capped and visible (F7 P6; `MEL-REQ-183`).
- **Deterministic ordering where required** — the authorisation decision and
  chronicle append are not subject to scheduling nondeterminism in their outcome
  (`MEL-REQ-174`).
- **Graceful shutdown** — from any state, shutdown leaves a recoverable state
  (F6 §8, `MEL-REQ-187`).

**SHOULD HAVE**

- Cheap concurrency (so supervision + isolation don't dominate the footprint,
  AP-12).
- Backpressure on the append queue rather than unbounded buffering (F6 §7).
- Crash-only design friendliness (recover by restart, not by cleanup).
- Clear liveness/health signals per worker for the Supervisor and Self-Evaluation
  (`MEL-REQ-248`).

**MUST NOT HAVE**

- Shared-mutable-state concurrency without enforced discipline in the TCB.
- A model where a stuck worker can wedge the whole system (must be
  timeoutable/killable — F7 P7).
- Hidden global background threads in the TCB that are hard to reason about.
- An event loop that the emergency-stop path can be starved on.

---

## 6. Inter-process / inter-component communication (IPC)

**MUST HAVE**

- **Authenticated endpoints** — a component can tell *which* component it is
  talking to; a Ring-3 instance cannot impersonate the Monitor or Ring 1 (F7
  P11, F11 Path F).
- **Explicit, typed messages** — the boundary carries canonical structured
  actions and grant references, not free-form strings (F2, F3 §3).
- **No ambient reachability** — a Ring-3 instance can reach only the endpoints
  its grants imply; it cannot enumerate or connect to arbitrary components
  (F7 P1, P2).
- **Local-only transport** — in-process channels, pipes, or unix domain sockets;
  **no TCP/IP even on loopback** as a required transport (INV-1, F7 §7).
- **Bounded messages** — size and rate limits, so IPC is not a resource-exhaustion
  vector (F7 P6).
- **Fail-closed** — a malformed or unauthenticated message is rejected, not
  best-effort-parsed (F3 §7, F2 §4).

**SHOULD HAVE**

- A schema/contract for each message type that both sides check (encodes F1/F2/F5
  vocabularies).
- Low latency on the AUTHORISE round-trip (it is on the critical path of every
  effect).
- Observability — the host can log boundary crossings for the audit record
  (F11 Paths B–F).
- Backpressure rather than drop-or-buffer-unbounded.

**MUST NOT HAVE**

- A network-exposed IPC surface (disqualifying — INV-1).
- Unauthenticated shared memory or a shared bus any component can post to as any
  identity.
- Implicit broadcast that leaks one component's messages to another beyond its
  grant.
- A serialization format with known deserialization-RCE history used at a trust
  boundary.

---

## 7. GUI control

*(Terminal + GUI are separate capabilities — `MEL-REQ-143`, AU-2/AU-3.)*

**MUST HAVE**

- **Structured targeting** — address UI elements by a structured descriptor
  (role, label, hierarchy), not only by screen coordinates (F2 §2.3).
- **A closed operation set** — activate / set-text / select / invoke / read-state
  / scroll / key — each a distinct grantable operation (F2 §2.3).
- **Coordinate fallback is explicit and higher-risk** — when structural targeting
  is impossible, the coordinate action is recorded as such and classified higher
  (F2 §2.3, F8 §2.3).
- **Observability for VERIFY** — the resulting UI state can be read back by a
  verifier (F10 V4, F11 Path B).
- **Runs under isolation** — the GUI-control capability is a Ring-3 instance with
  only its granted scope (F7).
- **Local, on-device automation only** — no remote-control protocol to another
  machine (INV-1).

**SHOULD HAVE**

- Works with the user's actual desktop environment without replacing it.
- Degrades predictably when a target element is not found (fail-closed, not
  wild clicking).
- Low overhead when idle (AP-12).
- An accessibility-API basis (these give structured element trees) rather than
  pure pixel automation.

**MUST NOT HAVE**

- Coordinate-only automation as the *primary* model (F2 §2.3 — brittle,
  unverifiable, bypasses structured scope).
- A cloud relay or account requirement.
- An approach that requires disabling OS security features wholesale.
- Silent, unlogged input injection.

---

## 8. Terminal control

*(CONTAINMENT-CRITICAL — RC-4, AU-2. Terminal authority = structured actions,
NOT raw shell.)*

**MUST HAVE**

- **`execve`-style structured invocation** — program (resolved absolute path) +
  `argv` vector + explicit env allow-list + cwd, with **no shell interpretation**
  (F2 §1, §2.1, §5).
- **`argv` is never re-split or expanded** by MELFINA (F2 §4).
- **Path canonicalisation** — resolve symlinks, collapse `.`/`..`, bind to a
  fixed root, re-canonicalise at every check, deny on any difference (F2 §3;
  TOCTOU defence).
- **`shell-exec` is a distinct capability** with its own grant and a **high risk
  floor** — not reachable from a `local-exec` grant (RC-4, F8 §2.3, F11 Path E).
- **Child process authority = nothing by default** (F2 §2.1, F7 P5).
- **Resource + time bounds + forcible termination** on every invocation (F7 P6,
  P7).
- **Output captured, bounded, and available to VERIFY** (F10, F11 Path B).
- **The grant binds to exactly `program X + argv A`** — the core invariant (F2
  §5): not `X + argv B`, not `shell → X`, not an added pipe/redirect, unless
  separately authorised.

**SHOULD HAVE**

- A way to express common safe operations (spawn, wait, signal, read output)
  without ever constructing a shell string.
- Deterministic environment construction (fixed map, F2 §4).
- Clear separation between "observe" (read-only, e.g. `which`, `ls`) and
  "execute" (F2 §6).
- Pseudo-terminal support where a program genuinely needs it, still under the
  structured model.

**MUST NOT HAVE**

- A raw shell string as the invocation primitive (disqualifying — F2 §1, RC-4).
- Implicit shell wrapping ("just prepend `sh -c`") anywhere in the path.
- Environment inheritance from MELFINA's own process by default.
- Unbounded output buffering or un-killable child processes.
- Any path where a `local-exec` grant can be escalated to shell composition
  (F2 §5, F11 Path E attempt 3).

---

## 9. How the next phase uses this document

1. For each area, enumerate 2–4 concrete candidates.
2. Score each against MUST HAVE (pass/fail), SHOULD HAVE (weighted), MUST NOT
   HAVE (pass/fail).
3. Evaluate the **interacting pairs** — (language × isolation),
   (storage × crypto), (process model × IPC), (isolation × terminal/GUI).
4. For every criterion a candidate cannot meet, record which **foundation
   contract property** becomes `[ID]`→at-risk or `[ETL]`, and design the test
   that will settle it.
5. Produce a recommendation per area **with the residual risks named**, for
   human review — technology selection is not an autonomous decision.

---

## 10. Traceability

| Area | Primary sources |
|---|---|
| cross-cutting | `MEL-REQ-155, 192, 193, 196, 197`; AP-5, AP-12; INV-1 |
| language | F2, F3, F9; `MEL-REQ-174, 179`; AP-5; OQ item 3 |
| storage | F5, F6; `MEL-REQ-160, 170, 171, 173`; OQ item 2 |
| isolation | F7 (P1–P14); RC-1, RC-6, L4; `MEL-REQ-184`; AP-8, AP-12 |
| crypto integrity | F9 (T1–T4, C1–C7, §6); RC-1; AU-6; OQ item 5 |
| process model | F4, F6 §7, F7, F8 §2.8; `MEL-REQ-174, 183, 187` |
| IPC | F2, F3, F7 P11; INV-1; F11 Paths B–F |
| GUI control | F2 §2.3, F7, F10; `MEL-REQ-143`; AU-3 |
| terminal control | F2 (§1, §2.1, §3, §4, §5, §6); RC-4; F8 §2.3; AU-2; F11 Path E |
| "human reviews the selection" | `MEL-REQ-157`; the autonomy triad (INV-2) applied to the project's own decisions |
