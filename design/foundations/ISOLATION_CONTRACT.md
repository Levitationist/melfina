# MELFINA — FOUNDATION 7: RING-3 ISOLATION CONTRACT

**Phase:** LOW-LEVEL FOUNDATIONS — MISSION 001. First pass, pending review.
**Baseline:** `8a40207`. Read F1–F3 first.

**This document does not choose an isolation mechanism.** Not process isolation,
not a VM, not WebAssembly, not containers, not `seccomp`/namespaces/Landlock/
Capsicum, not a language sandbox. It fixes the **properties any mechanism must
provide** for a Ring-3 capability instance, which are structural vs
implementation-dependent vs empirically-testable, and what happens if isolation
fails.

Derives from: **AP-7, AP-8**; **RC-6, RC-7**; **L4**; **INV-8, INV-9**;
**MEL-REQ-124, 125, 126, 127, 184, 185, 196, 199**; seL4 (proven spatial
isolation of confidentiality/integrity/availability; timing channels *not*
covered), the WASI capability model (no ambient authority by default, explicit
wiring, no privilege escalation), object-capability confinement `[E]`.

---

## 0. Dependencies discovered

- **On F3:** "non-bypassable" and "complete mediation" for the Monitor **depend
  on** this contract — the isolation mechanism is what guarantees there is no
  unmediated path from a Ring-3 capability to an effect.
- **On F1:** a capability instance receives authority **only** as
  Monitor-issued grants presented at guarded-effect boundaries; the isolation
  mechanism is where those boundaries physically are.
- **On F2:** each structured effect crosses the isolation boundary in its
  canonical form so the boundary can gate it.
- **On F4:** "stop the running execution where possible" requires a
  **terminable** and **containable** sandbox.
- **On F10:** Verifiers are also Ring-3 components and run under this contract
  (minimal authority, read-only, no ambient authority).
- **Discovered constraint (RC-7):** this contract governs the **one boundary that
  must be a real, enforced security boundary between Ring 1 and the sandboxed
  outer region**. The other ~24 subsystem boundaries may be co-located. But
  **this** boundary, plus the Ring-0 boundary and the Ring-2↔Ring-3
  (reasoning↔effect) boundary, must be real. `[SEN]`.

---

## 1. What is isolated

A **Ring-3 capability instance** — one loaded capability component executing one
authorised action, plus any child processes/effects it causes. It is created on
demand, given exactly the grants for this action, run, and **torn down after**
(`CAPABILITY_MODEL.md` §2). "Ring 3" also includes: skills, tools, workflows,
terminal control, GUI control, file access, and **Verifiers**.

**Ring 2 (reasoning) is also outside Ring 1's trust boundary**, but Ring 2 is
isolated *differently*: it gets **Query (read) + the Reasoner Interface** and
**no capability grant, no Chronicle write, no effect path** (the reasoning↔effect
boundary, §7). This document is about **Ring 3** (effect-bearing); §7 covers the
Ring-2 side.

---

## 2. The required properties

Any mechanism chosen at LOW-LEVEL FOUNDATIONS must provide **all** of the
following. Each row: the property, its acceptance criterion, and its class.

| # | Property | Acceptance criterion | Class |
|---|---|---|---|
| P1 | **No ambient authority** | a freshly loaded capability instance can do **nothing** — no filesystem, no process spawn, no IPC, no environment, no clock, no network — until a grant is wired in for this specific action (WASI model, `[E]`). | **[ID]** (mechanism must default-deny) — this is the single most load-bearing property |
| P2 | **Explicit capability access only** | every effect the instance causes is mediated by presenting a Monitor-issued grant at the boundary (F1, F3); an operation with no matching grant fails at the boundary, not inside. | **[ID]** |
| P3 | **Zero network capability by default** (L4) | a socket **cannot be opened** from within the sandbox — the syscall/host-function is absent or denied, not merely unconfigured. A capability wanting network is a Ring-4 concern, not grantable here. | **[SEN]** as a rule (no `network-*` effect class exists, F1 §3) + **[ID]** (the mechanism must actually withhold the primitive) |
| P4 | **Filesystem access strictly per grant** | the instance sees **only** the resolved, canonicalised path set of its `file-*` grants (F2 §3); it cannot enumerate, traverse to, or `open()` outside it; the Ring-0 region is **never** in any grant (RC-1). | **[ID]** |
| P5 | **Process authority strictly per grant** | the instance can spawn a child **only** via a `process-exec` grant; the child inherits **no** MELFINA authority (F2 §2.1); child count, depth, and resource use are bounded. | **[ID]** |
| P6 | **Resource bounds — hard** | wall-clock, CPU, memory, output size, child count, and any metered resource have **hard ceilings** enforced as cut-offs, not alerts (`MEL-REQ-183`); a loop / no-progress detector runs. | **[ID]** |
| P7 | **Terminable** | the sandbox can be **stopped** on a revocation trigger (F4) — signalled to halt, and hard-terminated if it does not. A running instance cannot make itself un-stoppable. | **[ID]** |
| P8 | **Failure-contained** | a crash, hang, exploit, or resource exhaustion **inside** an instance does not reach Ring 1, the Chronicle, Ring 0, or another instance. The Supervisor restarts or abandons the subtree (INV-11, "let it crash", `[E]`). | **[ID]** + **[ETL]** (containment is a red-team target) |
| P9 | **Cannot access Ring 0** | no path — read or write — from a Ring-3 instance to the Ring-0 governance region or the Monitor's internal state. The Monitor is *called* across the boundary with a request; it is not *entered*. | **[SEN]** (the effect vocabulary has no Ring-0 effect) + **[ID]** (spatial isolation) |
| P10 | **Cannot forge a grant** | a grant is unforgeable (F1); the instance cannot manufacture, guess, or replay one; it can only present grants handed to it by the Monitor for this action. | **[ID]** (representation — a MAC'd token / an OS handle the sandbox cannot mint / a resource the host owns) + **[ETL]** |
| P11 | **Cannot impersonate the Monitor** | the instance cannot present itself to another component *as* the Reference Monitor, and cannot answer a decision request. The Monitor is a distinct authority, not a role a capability can assume. | **[SEN]** (rule) + **[ID]** |
| P12 | **Cannot bypass the reasoning↔effect boundary** | Ring 2 cannot invoke a Ring-3 capability directly; a Ring-3 capability cannot be *driven* by Ring 2 except by an authorised Intention flowing through the pipeline (§7). | **[SEN]** (rule: Ring 2 holds no grant) + **[ID]** |
| P13 | **No persistent instance state between actions** | teardown is real; an instance does not carry state to its next invocation unless it was explicitly granted a scoped store (`CAPABILITY_MODEL.md` §2). | **[ID]** |
| P14 | **Lightweight** | isolation cost (load, teardown, boundary crossing) is low enough that a large capability *surface* does not raise the resident footprint (AP-12, `MEL-REQ-196`, `199`) — capabilities are dormant when unused. | **[ID]** + **[ETL]** (measured) |

---

## 3. What the contract does NOT require

To avoid over-constraining the mechanism:

- **Not** a formally verified isolation kernel (seL4-level proof would *help* —
  AU-5 — but is not mandated). `[OPEN]` whether formal verification is pursued.
- **Not** covering **timing / covert channels.** seL4's proven confidentiality
  "does not cover timing channels" (`[E]`); neither does this contract, as a
  *guaranteed* property. Timing-channel leakage between a capability and the
  outside is a **documented residual limit** (`AUTHORITY_AND_SECURITY_MODEL.md`
  §9 territory), mitigated by: the core has no outbound channel (INV-1), so a
  timing channel has nowhere to exfiltrate *to*; and least authority limits what
  a channel could carry. `[OPEN]` / `[U]`.
- **Not** a specific number of protection domains, or a specific supervisor
  topology (O4).
- **Not** that reasoning and capability code run in **different OS processes** —
  they MAY be co-located for lightness, **provided the authority distinction is a
  real, enforced check** (O2 rejected, §7). `[SEN]`.

---

## 4. Spatial-isolation properties (the seL4 framing)

Borrowing the three properties seL4 proves for an isolated subsystem (`[E]`), the
contract requires the mechanism to enforce, between a Ring-3 instance and
everything outside it:

- **Confidentiality** — the instance reads nothing it was not granted read
  access to (P4, P2). (Timing channels excepted — §3.)
- **Integrity** — the instance writes / causes effects on nothing it was not
  granted (P2, P4, P5, P9).
- **Availability** — the instance cannot deny service to Ring 1 or another
  instance by consuming shared resources (P6, P8).

**An isolated instance is one where none of these can be violated toward another
subsystem whose authority is not already present in the instance's grant set**
(the seL4 "isolated subsystem" definition, `[E]`). `[ID]` / `[ETL]`.

---

## 5. What happens if isolation fails

Isolation *will* be imperfect — assume adversarial content sometimes wins (INV-8).
The contract's response to a breach:

| Breach | Response |
|---|---|
| an instance escapes its resource bounds | the external watchdog / Supervisor terminates it (P6, P7); an Event records the failure; contained (P8) |
| an instance attempts an ungranted effect | the boundary denies it (P2); repeated attempts ⇒ the Reference Monitor / lifecycle flags the capability; it is de-prioritised then **retired** (`MEL-REQ-224`, `249`) |
| an instance is found to have caused an effect it was not granted (a real isolation bug) | **stop-condition**: the affected action's grant family is revoked; the capability is retired; the incident is recorded in full; the isolation mechanism itself is treated as suspect and re-evaluated. This is the kind of event that halts autonomous operation until reviewed (`MEL-REQ-186` territory) |
| the isolation mechanism itself crashes | the Supervisor cannot safely run Ring-3 capabilities ⇒ Ring 3 is unavailable; **Ring 1 continues fully** (INV-9) — capture, memory, retrieval, time support, user-made plans, deterministic automations, search all work; MELFINA reports "capabilities unavailable" |
| a suspected but unconfirmed breach | conservative: treat as confirmed for the affected action (revoke, contain), investigate, do not resume that capability until cleared |

**The core is never at risk from a Ring-3 failure** (INV-11) — the Chronicle,
Ring 1, and Ring 0 are untouched. `[SEN]` for the containment intent; `[ETL]` for
the mechanism actually delivering it.

---

## 6. Empirically testable vs structurally guaranteed

Per the mission's requirement to distinguish these:

| Property | Guaranteed by | Confidence route |
|---|---|---|
| no `network-*` effect class in the core vocabulary (RC-6, L4) | the **shape** of F1 §3 | structural — code audit of the vocabulary |
| the Monitor has no "widen authority" / "edit Ring 0" effect to grant | the **shape** of F1 §3 | structural |
| Ring 2 holds no capability grant (reasoning↔effect) | the **shape** of the pipeline (F3, AD-6) | structural — Ring 2's only interfaces are Query + Reasoner Interface |
| a fresh instance can do nothing (P1) | the **mechanism** (default-deny) | test: load a capability with no grants, attempt every effect class, expect all denied |
| filesystem / process / network confinement (P3–P5) | the **mechanism** | test: an adversarial capability that *tries* to escape each; red-team |
| resource bounds are hard cut-offs (P6) | the **mechanism** + external watchdog | test: a capability that loops / allocates / spawns; expect termination at the ceiling |
| terminability (P7) | the **mechanism** | test: revoke mid-action a capability designed to resist stopping |
| failure containment (P8) | the **mechanism** + Supervisor | fault-injection: crash / hang / OOM an instance; assert Ring 1 + Chronicle unaffected |
| grant unforgeability (P10) | the **representation** | red-team: attempt to mint / replay / tamper a grant from inside |
| lightweight (P14) | the **mechanism** | measure: load/teardown/crossing cost; resident footprint with 0 / N capabilities |
| no timing-channel leakage | — | **not guaranteed** — documented residual limit (`[U]`) |

---

## 7. The reasoning↔effect boundary (Ring 2 ↔ Ring 3) — O2 rejected

This is one of the three must-be-real boundaries (RC-7). The contract:

- **Ring 2 emits only proposals + claims.** It holds **Query (read)** and the
  **Reasoner Interface**, and **nothing else** — no Chronicle write, no capability
  invocation, no grant, no network, no Ring-0 access. `[SEN]` (the shape of Ring
  2's interfaces).
- **Ring 2 cannot invoke a Ring-3 capability.** A capability runs only when an
  **authorised** Intention reaches EXECUTE through the pipeline (F3 §6). A Ring-2
  proposal is an Intention with `status = proposed`; it becomes `authorised` only
  via a recorded AUTHORISE. `[SEN]`.
- **Implementations MAY co-locate Ring-2 and Ring-3 code in one OS process for
  lightness** (O2's cost saving) — **but the authority distinction must remain a
  real, enforced check, not a code convention** (O2 rejected). Concretely: even
  co-located, the component acting as "Ring 2" must not be able to present a
  grant, and the component acting as "Ring 3" must receive its grants from the
  Monitor for an authorised Intention. `[SEN]` (the rule) + **[ID]** (how a
  co-located implementation enforces it — e.g. distinct in-process authority
  contexts).
- A **Reasoning Contributor that MELFINA itself constructs** (a novel reasoning
  strategy) is a **Ring-3 lifecycle capability** under the full pipeline
  (created / tested / risk-classified / authorised / retired), run **sandboxed**
  — so reasoning-strategy dynamism is genuine but bounded (M3, SYSTEM_ARCHITECTURE
  §2). `[SEN]`.

---

## 8. Contract Q&A

| Question | Answer |
|---|---|
| **Guarantees** | a Ring-3 instance starts with zero authority; every effect it causes is a Monitor-mediated grant use; it has no network primitive, no Ring-0 path, no way to forge a grant or impersonate the Monitor; it is terminable, resource-bounded, and failure-contained; teardown is real; Ring 1 is unaffected by any Ring-3 failure |
| **Requires** | a default-deny isolation mechanism (P1); an unforgeable grant representation (F1); a Supervisor + external watchdog; the Monitor's boundary checks (F3); canonical structured effects (F2) |
| **Trusts** | the isolation mechanism's spatial-isolation enforcement (once built + tested); the Supervisor's containment; the external watchdog's cut-offs |
| **Distrusts** | the capability code itself (assume it can be compromised — INV-8); a capability's self-report of what it did (VERIFY re-reads — F10); a capability's claim to have "cleaned up"; timing behaviour as leak-free |
| **Enters** | a capability component + the Monitor's grants for one authorised action + the canonical structured effects it proposes |
| **Leaves** | the effects it actually caused (observed by VERIFY), a result handed to VERIFY, and Events recording what happened; on failure, a contained crash + a failure Event |
| **Malformed input** | a capability component that does not load, or declares an authority the classifier cannot map, is rejected by the lifecycle (untrusted until tested); it never runs consequentially |
| **Failure** | contained to the instance's subtree; Supervisor restarts or abandons; Ring 1 + Chronicle + Ring 0 untouched; if the mechanism itself fails, Ring 3 is unavailable and Ring 1 continues (INV-9) |
| **Authoritative** | VERIFY's observation of the actual post-state is authoritative for what a capability did; the Monitor is authoritative for what it may do |
| **Independently verifiable** | yes — the structural properties by code audit of the effect vocabulary and Ring-2 interfaces; the mechanism properties (P1, P3–P8, P10, P13, P14) by an adversarial capability corpus + fault injection + red-team + measurement (§6) |

---

## 9. Enforceability summary

| Invariant | Class |
|---|---|
| no ambient authority; explicit-grant-only (P1, P2) | **[ID]** (default-deny mechanism) |
| zero network capability by default; socket cannot be opened (P3, L4) | **[SEN]** (no effect class) + **[ID]** (primitive withheld) |
| filesystem / process confinement per grant; Ring-0 region never granted (P4, P5, RC-1) | **[ID]** + **[SEN]** (exclusion rule) |
| hard resource bounds as cut-offs (P6) | **[ID]** + external watchdog |
| terminable; cannot make itself un-stoppable (P7) | **[ID]** |
| failure contained to the instance; Ring 1 / Chronicle / Ring 0 untouched (P8) | **[ID]** + **[ETL]** |
| no Ring-0 path; cannot forge a grant; cannot impersonate the Monitor (P9–P11) | **[SEN]** (rules) + **[ID]** (spatial isolation, representation) |
| reasoning↔effect boundary real even if code co-located (O2 rejected) | **[SEN]** (rule) + **[ID]** (co-located enforcement) |
| Ring 1 fully usable with Ring 3 unavailable (INV-9) | **[SEN]** |
| lightweight; large surface, flat resident footprint (P14) | **[ID]** + **[ETL]** (measured) |
| timing / covert channels | **[OPEN]** / **[U]** — not guaranteed; documented residual limit |
| formal verification of the isolation mechanism | **[OPEN]** (AU-5) |

---

## 10. Deferred (LOW-LEVEL FOUNDATIONS build)

- The isolation mechanism (OS process + `seccomp`/namespaces/Landlock/pledge /
  a lightweight VM / a Wasm component runtime / a language sandbox / a
  capability microkernel — none chosen; evaluated against P1–P14 + AP-12).
- The grant representation (F1 §19) — the P10 "unforgeable" property depends on it.
- The Supervisor topology (O4).
- The adversarial capability corpus + fault-injection harness (built here,
  maintained).
- Whether the mechanism is formally verified (AU-5) — a `[OPEN]` decision.
- The timing-channel posture beyond "no outbound channel to exfiltrate to"
  (`[U]`).

## 11. Traceability

| Element | Source |
|---|---|
| sandboxing / privilege separation proportionate to power | `MEL-REQ-184`; AP-8 |
| capability isolation, versioning, discovery, auditing | `MEL-REQ-125` |
| least-authority capabilities; no ambient authority | `MEL-REQ-124`, `179`; AP-7 |
| untrusted capabilities cannot form the lethal trifecta | `MEL-REQ-126`; L4, L5 |
| sub-agents under the same permission model; delegation ≠ escape | `MEL-REQ-127`; F1 §7 |
| assume prompt injection wins; contain the blast radius | `MEL-REQ-185`; INV-8; AP-8 |
| fully usable with AI disabled; failure does not cascade | `MEL-REQ-154`, `172`; INV-9, INV-11 |
| lightweight; capability ≠ resource cost | `MEL-REQ-128`, `196`, `199`; AP-12 |
| responsibilities not components; three must-be-real boundaries | RC-7; `ARCHITECTURAL_PRINCIPLES.md` AP-12; `ARCHITECTURAL_ALTERNATIVES.md` AA-1 |
| do not collapse Ring 2 / Ring 3 | O2 rejected; SYSTEM_ARCHITECTURE §26 |
| reasoner locality structural (no network capability in Ring 1/2) | RC-6 |
| proven spatial isolation; timing channels not covered | seL4 `[E]` |
| no ambient authority by default; explicit wiring; no privilege escalation | WASI capability model `[E]` |
