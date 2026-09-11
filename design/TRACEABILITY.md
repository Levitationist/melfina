# MELFINA — ARCHITECTURE TRACEABILITY

**Phase:** SYSTEM DESIGN / ARCHITECTURE — MISSION 001. First pass, pending review.
**Revision 1 applied** — §2.1 (Revision 1 corrections → sources) added; AD-8/AD-12/
AD-13 updated; AD-14/AD-15 added; §3–§6 rows updated for RC-1…RC-7 + M/L items.
Baseline `f3093fc`.

Every major architectural decision traces back to a **requirement**, a **model
concept**, a **research finding**, or an **explicitly documented architectural
inference** (`[DI]`). No unsupported invention.

Legend: **R** = `requirements/REQUIREMENTS_MASTER.md` (`MEL-REQ-` / `MEL-AR-` /
`VC`) · **M** = `model/HUMAN_CENTRAL_MODEL.md` · **Res** = `research/` finding ·
**Pat** = an established architectural pattern (`[E]`) · **DI** = architectural
inference (stated).

---

## 1. Invariants → sources

| Invariant | Requirement | Model | Research | Pattern |
|---|---|---|---|---|
| INV-1 local-only core | MEL-REQ-014, 164–166; local-only addendum | — | lethal trifecta (Res §—, MEL-REQ-126) | local-first `[E]` |
| INV-2 autonomy triad never collapsed | MEL-REQ-004, 016–019 | M §10 | corrigibility, adjustable autonomy `[E]` | HITL/HOTL/HOOTL `[E]` |
| INV-3 predictable deterministic core | MEL-REQ-010 | M P-3, P-12 | intolerance of uncertainty `[E]`; adaptive-UI `[E]` | — |
| INV-4 everything is a Claim w/ provenance+status | MEL-REQ-093, 096, 131 | M §4.3, §8 | epistemic/doxastic logic `[E]`; deskilling/XAI `[E]` | W3C PROV `[E]` |
| INV-5 append-only history | MEL-REQ-160–162, 170 | M P-5, §11 | — | bitemporal modelling, event sourcing `[E]` |
| INV-6 self-modification meta-invariant | MEL-REQ-235, 239; MEL-AR-18 | M §12.1 | monotonic confinement, external guardrails `[E]` | reference monitor `[E]` |
| INV-7 least authority, unforgeable scoped grants | MEL-REQ-124, 179–180, 222 | — | capability-based security `[E]` | object-capability model, WASI `[E]` |
| INV-8 assume adversarial content wins | MEL-REQ-185, 186; MEL-AR | prompt-injection unsolved `[E]` | — | defence-in-depth `[E]` |
| INV-9 usable with AI disabled | MEL-REQ-154; MEL-AR-07 | M §12 | deskilling, "ironies of automation" `[E]` | — |
| INV-10 restraint at hazard surfaces; quiet default; no scorekeeping | MEL-REQ-010–013, 041–047; MEL-AR-02, 03, 11 | M §13 | gamification, self-tracking, OCD, perfectionism `[E]`; Fitz 2019 `[E]` | — |
| INV-11 failure contained, not cascading | MEL-REQ-015, 172, 173 | — | personal-informatics abandonment `[E]` | Erlang/OTP supervision `[E]` |
| INV-12 lightweight; low idle; on-demand | MEL-REQ-192–199; VC1 | — | — | microkernel size trade-off `[E]` |
| INV-13 dynamic, not arbitrary | MEL-REQ-204–253; VC8–VC12 | M §7, §13.6 | rational metareasoning, adaptive test-time compute, anytime algorithms `[E]` | metareasoning controller + contributors (blackboard `[E]` = one candidate impl, O3) |
| INV-14 traceability | mission "TRACEABILITY" | — | — | — |

## 2. Architectural decisions → sources

| AD | Decision | Traces to |
|---|---|---|
| AD-1 | five inward-only rings | INV-2, 6, 7, 9; mission §8; `[DI]`: rings make the separations structural rather than conventional |
| AD-2 | Ring 0 read-only to all outer rings; changed only by human file edit | INV-6; MEL-REQ-235; reference monitor `[E]`; monotonic confinement `[E]` |
| AD-3 | Chronicle (append-only Events/Claims/Intentions) = single source of truth; rest = rebuildable projections | M §4, §11, P-5; INV-5; event sourcing `[E]`; `[DI]`: E²CI's shape *is* an event log |
| AD-4 | hybrid persistence (Chronicle authoritative + materialised current-state cache) | AA-2; event-sourcing slow-read trade-off `[E]`; MEL-REQ-197; `[DI]` |
| AD-5 | reasoning = a metareasoning control loop, not an agent loop (blackboard = one candidate coordination pattern, O3) | mission ("not a generic agent loop"); MEL-REQ-204–213, 251–253; VC8; blackboard `[E]`; rational metareasoning `[E]` |
| AD-6 | Ring 2 emits only proposals + claims; no Chronicle write, no capability invocation | INV-2; AP-2; MEL-REQ-017, 102 |
| AD-7 | capabilities = sandboxed components, no ambient authority, per-action typed grants | MEL-REQ-124, 179, 184, 185; INV-7, 8; object-capability + WASI `[E]` |
| AD-8 | risk class set by a deterministic Ring-1 classifier from the UNION of authority + scope + aggregate effect; Ring 2 cannot lower it; composition never launders risk | MEL-REQ-227; RC-2; `[DI]`: the classifier's inputs are the capability's own declared authority/scope/aggregate, not a reasoning claim |
| AD-9 | self-modification tiers 6–9 are not MELFINA operations (a human path) | MEL-REQ-233, 236, 239; recursive-self-improvement risk `[E]`; the research caveat about invariants not generalising `[U]` |
| AD-10 | one memory (the Chronicle); no separate long-term/vector/episodic store as a primitive | M §12; AP-5; MEL-REQ-093; `[DI]` |
| AD-11 | single Notification Gateway; everything else pull | MEL-REQ-011, 041–047; Fitz 2019 `[E]`; Mehrotra TOCHI `[E]`; MEL-AR-03; `[DI]`: one choke point enforces the restraint requirements once |
| AD-12 | supervision-based failure isolation (topology deferred — O4); Chronicle = recovery ground truth | MEL-REQ-015, 172, 173; Erlang/OTP `[E]`; event-sourcing replay recovery `[E]` |
| AD-13 | technology deferred (language, storage engine, isolation impl, reasoner, format, Ring-0 signing scheme, action parser) | project pipeline; `ARCHITECTURAL_PRINCIPLES.md` §3; mission ("do not choose technologies merely because familiar/convenient") |
| AD-14 | the meta-invariant is enforced at rest (Ring-0 signing) as well as at runtime; verifier trust is specified; grants are revocable mid-execution | RC-1, RC-3, RC-5; adversarial review §1–3, H1/H3/H5; MEL-REQ-235, 239, 221, 248, 181 |
| AD-15 | aggregate-effect governance — routine actions carry a per-activity-chain aggregate budget that escalates to consequential on crossing | RC-2; MEL-REQ-019, 181, 220, 227; adversarial review H2 |

### 2.1 Revision 1 corrections → sources

| RC / item | Correction | Traces to | Integrated in |
|---|---|---|---|
| **RC-1** | Ring 0 at-rest integrity: cryptographic signature chain rooted in a human-held key MELFINA never possesses; verified every startup; invalid ⇒ refuse to run; Ring 0 path hard-excluded from every File Access grant | MEL-REQ-235, 239; INV-6; MEL-AR-18; reference monitor `[E]`; adversarial review §1, H1 | `ARCHITECTURAL_PRINCIPLES.md` AP-6, §3; `SYSTEM_ARCHITECTURE.md` §2, §5 (startup), §17, §20; `AUTHORITY_AND_SECURITY_MODEL.md` §5, §6; `DATA_AND_STATE_MODEL.md` §2; `RUNTIME_MODEL.md` §2; `CAPABILITY_MODEL.md` §7; `ARCHITECTURAL_ALTERNATIVES.md` AA-4 |
| **RC-2** | aggregate-effect governance; composition classified by UNION of authority + scope + aggregate; no laundering high-risk into low-risk steps; "practically irreversible in aggregate" is a risk input | MEL-REQ-019, 181, 220, 227; INV-13; MEL-AR-13, 19; adversarial review §4, H2 | `SYSTEM_ARCHITECTURE.md` §2 (Budget Authority), §6.3, §12, §16, §21, §25 (AD-8, AD-15); `AUTHORITY_AND_SECURITY_MODEL.md` §2, §4; `CAPABILITY_MODEL.md` §2, §3, §4, §5, §6; `RUNTIME_MODEL.md` §4 |
| **RC-3** | verifier trust model: deterministic where practical, minimal/ no ambient authority, read-only unless narrowly justified, cannot cause effects, independently testable against fixtures, not a second agent; ≥2 architecturally-independent verifiers for high-risk; explicit fallback boundary | MEL-REQ-113, 142, 221, 248; INV-8; adversarial review §5, H5 | `ARCHITECTURAL_PRINCIPLES.md` §3; `SYSTEM_ARCHITECTURE.md` §2 (Verifiers), §6.6; `AUTHORITY_AND_SECURITY_MODEL.md` §4.1; `CAPABILITY_MODEL.md` §4 |
| **RC-4** | AU-2 CONTAINMENT-CRITICAL: terminal authority = structured actions not raw shell strings; grants matched against parsed argv/structured invocation; shell interposition a distinct higher-risk capability; GUI structured where practical | MEL-REQ-141–146, 179, 184; INV-7, 8; object-capability `[E]`; adversarial review §5, H4 | `ARCHITECTURAL_PRINCIPLES.md` §3; `SYSTEM_ARCHITECTURE.md` §2 (Terminal/GUI/File Access), §6.5, §12, §26, §27 (AU-2), §29, §30; `AUTHORITY_AND_SECURITY_MODEL.md` §3; `CAPABILITY_MODEL.md` §2, §5, §8; `ARCHITECTURAL_ALTERNATIVES.md` AA-6 |
| **RC-5** | capability grants are live revocable handles; Reference Monitor invalidates mid-execution; no new effects after revocation; stop running execution where possible; roll back reversible; record irreversible/partial as partial failure; user-visible outcome vocabulary completed / partially completed / rolled back / failed / interrupted | MEL-REQ-145, 181, 187; INV-2, 11; adversarial review §8, H6 | `SYSTEM_ARCHITECTURE.md` §2 (Capability Host), §6.5, §12; `AUTHORITY_AND_SECURITY_MODEL.md` §3; `CAPABILITY_MODEL.md` §2; `RUNTIME_MODEL.md` §4, §6; `FAILURE_AND_RECOVERY.md` §2, §4, §7, §8 |
| **RC-6** | reasoner locality structurally enforced: Ring 1 and Ring 2 hold no network capability; the Reasoner Interface cannot open a socket; a remote endpoint is unreachable through the core capability model — "cannot", not "does not" | MEL-REQ-014, 153–155, 159, 164–166; INV-1; local-only addendum; adversarial review §6 | `ARCHITECTURAL_PRINCIPLES.md` AP-9; `SYSTEM_ARCHITECTURE.md` §2 (Reasoner Interface), §22, §23; `AUTHORITY_AND_SECURITY_MODEL.md` §7; `RUNTIME_MODEL.md` §2; `ARCHITECTURAL_ALTERNATIVES.md` AA-5 |
| **RC-7** | the ~25 named subsystems are RESPONSIBILITIES, not necessarily separate processes/services/binaries/IPC boundaries; implementation may group into a few cohesive components; only Ring 0 ↔ all, Ring 1 ↔ sandbox, and reasoning ↔ effect must be real enforced boundaries | MEL-REQ-192–195; VC1, VC9; AP-12; adversarial review §3, §9, §15 | `ARCHITECTURAL_PRINCIPLES.md` AP-12; `SYSTEM_ARCHITECTURE.md` §2 (header), §20, §26; `ARCHITECTURAL_ALTERNATIVES.md` AA-1; `FAILURE_AND_RECOVERY.md` §2 |
| **M1** | context constructed on demand, not a continuously-maintained authoritative projection | M §7; INV-3; adversarial review M1 | `ARCHITECTURAL_PRINCIPLES.md` §2; `SYSTEM_ARCHITECTURE.md` §2, §8; `DATA_AND_STATE_MODEL.md` §1, §3; `RUNTIME_MODEL.md` §3, §5 |
| **M2** | DECIDE / Reference Monitor read authoritative/current state through the Chronicle contract where consequential correctness matters | AA-2; MEL-REQ-010; adversarial review M2 | `SYSTEM_ARCHITECTURE.md` §6.3, §28; `DATA_AND_STATE_MODEL.md` §3; `FAILURE_AND_RECOVERY.md` §3 |
| **M3** | lifecycle/trust status of Reasoning Contributors clarified (core = trusted Ring-2 code; novel strategies = Ring-3 lifecycle capabilities) | MEL-REQ-94–103, 129–136; adversarial review M3 | `SYSTEM_ARCHITECTURE.md` §2 (Reasoning Contributors) |
| **M5** | silent current-state-cache divergence audit noted as a future implementation concern (with O6) | adversarial review M5 | `SYSTEM_ARCHITECTURE.md` §28; `FAILURE_AND_RECOVERY.md` §3, §9 |
| **M6** | automation-trigger graph checked for cycles of any length, not just direct self-triggering | MEL-REQ-115; adversarial review M6 | `SYSTEM_ARCHITECTURE.md` §11; `CAPABILITY_MODEL.md` §6; `FAILURE_AND_RECOVERY.md` §4 |
| **M8** | capability creation/modification passes an explicit Reference Monitor check that creator authority ≥ declared authority | MEL-REQ-222; INV-7; adversarial review M8 | `SYSTEM_ARCHITECTURE.md` §16; `AUTHORITY_AND_SECURITY_MODEL.md` §3, §5; `CAPABILITY_MODEL.md` §4 |
| **M9** | conflict detection is conservative around declared-scope overlap unless stronger semantics available | MEL-REQ-010; adversarial review M9 | `SYSTEM_ARCHITECTURE.md` §6.3; `RUNTIME_MODEL.md` §6 |
| **M10** | proposals pass structural validation before authorization | INV-2; MEL-REQ-017; adversarial review M10 | `SYSTEM_ARCHITECTURE.md` §2 (Pipeline SM), §6.3 |
| **L4** | Ring 3 sandbox has zero network capability by default | INV-1; MEL-REQ-126, 167; adversarial review L4 | `ARCHITECTURAL_PRINCIPLES.md` §3; `SYSTEM_ARCHITECTURE.md` §2, §6.5; `AUTHORITY_AND_SECURITY_MODEL.md` §8; `RUNTIME_MODEL.md` §2; `CAPABILITY_MODEL.md` §2 |
| **L5** | the lethal-trifecta prohibition (no component holds all of private-data read + untrusted-content ingestion + outbound channel) is explicitly a Ring 0 governance invariant | MEL-REQ-126; INV-1, 8; adversarial review L5 | `SYSTEM_ARCHITECTURE.md` §2, §20, §23; `AUTHORITY_AND_SECURITY_MODEL.md` §7, §8 |

**Optional changes:** O1 adopted (one Chronicle + one current-state cache + on-demand views); O2 **rejected** (do not collapse Ring 2 / Ring 3 — the reasoning↔effect boundary is must-be-real); O3 adopted (blackboard = one candidate implementation, not a requirement); O4 adopted (supervisor topology not specified now); O5 preserved as an explicit design question (routine/consequential policy placement — AU-6, not moved into Ring 0 now); O6 retained as a future implementation concern (cache consistency audit), not an architecture requirement.

## 3. Subsystems → requirements coverage

| Subsystem | Primary requirements served |
|---|---|
| Reference Monitor (R0) | MEL-REQ-018, 179–185, 235; MEL-AR-13, 18 |
| Tier Policy (R0) | MEL-REQ-233–236, 227; MEL-AR-16 |
| Budget Authority (R0) | MEL-REQ-115, 183; MEL-AR-13, 19; RC-2 (aggregate budget, external watchdog) |
| Emergency Stop (R0) | MEL-REQ-145, 187 |
| Governance Store (R0) | MEL-REQ-235; INV-6; RC-1 (at-rest signing, File Access exclusion) |
| Chronicle (R1) | M §4, §11; MEL-REQ-020, 021, 160–162, 170; AP-4, AP-5 |
| Entity Registry (R1) | M §4.1 |
| Projection Engine (R1) | M §3.3, §7, §13; MEL-REQ-091–093; AP-5, AP-10 |
| Query (R1) | M §6.1; MEL-REQ-091–093 |
| Pipeline State Machine (R1) | MEL-REQ-018, 019, 144; INV-2; M10 (structural proposal validation), M2 (Chronicle-contract reads) |
| Capability Registry (R1) | MEL-REQ-123–128, 218–227 |
| Policy Store (R1) | MEL-REQ-175–178; requirements CONFLICTS RC1–RC10 |
| Audit (R1) | MEL-REQ-156, 182, 201–203 |
| Notification Gateway (R1) | MEL-REQ-011, 041–047; MEL-AR-03 |
| Supervisor (R1) | MEL-REQ-015, 172, 173 |
| Metareasoning Controller (R2) | MEL-REQ-204–213, 241–246, 251–253; VC8, VC11, VC12; M §7, §13.6 |
| Context Constructor (R2) | M §7; MEL-REQ-210–215 |
| Reasoning Contributors (R2) | MEL-REQ-94–103, 129–136, 70–76; M3 (core = trusted Ring-2 code; novel strategies = Ring-3 lifecycle capabilities) |
| Self-Evaluation (R2) | MEL-REQ-247–250; MEL-AR-19 |
| Reasoner Interface (R2) | mission §8; MEL-REQ-153–155, 159; AP-8, AP-9; RC-6 (no network capability — cannot bind a remote reasoner) |
| Capability Host (R3) | MEL-REQ-124–128, 184; AP-7, AP-8; RC-5 (live revocable handles), L4 (zero network) |
| Capability Lifecycle Manager (R3) | MEL-REQ-217–227; VC9; RC-2 (union risk class), M8 (≤creator authority check) |
| Terminal Control (R3) | MEL-REQ-141–146; RC-4 (structured actions, not raw shell; shell interposition distinct) |
| GUI Control (R3) | MEL-REQ-143–146; RC-4 (structured representations where practical) |
| File Access (R3) | MEL-REQ-164, 179; RC-1 (Ring 0 path hard-excluded) |
| Verifiers (R3) | MEL-REQ-113, 142, 221, 248; RC-3 (verifier trust model; ≥2 independent for high-risk) |
| Ring 4 (future) | MEL-REQ-126, 167; MEL-AR-12 |

## 4. Model concepts → architecture

| E²CI element | Architecture |
|---|---|
| Entity | Entity Registry (R1) |
| Event | Chronicle unit (R1) |
| Claim | Chronicle unit (R1); the *shape* of everything MELFINA knows (AP-4) |
| Intention | Chronicle unit (R1); proposals = Intentions `status=proposed` from R2 |
| Time (bitemporal) | valid-time + transaction-time on every unit; the query forms in `DATA_AND_STATE_MODEL.md` §4 |
| Context (derived) | a transient R2 construction, built on demand (M1); the Context Constructor; never stored |
| State ("what holds now") | the one materialised current-state cache (O1); `ARCHITECTURAL_ALTERNATIVES.md` AA-8 — compatible with model OQ-M2 either way |
| Relation (a Claim of relational content) | a Claim with relational content; model OQ-M1 unresolved, architecture unaffected |
| the meta-invariant NOT being in the model | Ring 0 governance files, not Chronicle Claims (AD-2) |
| "MELFINA's state = same primitives, `holder=melfina`" | Chronicle units with `holder`/`owner`/`attributed-to = melfina` (§`SYSTEM_ARCHITECTURE` §2) |

## 5. Requirements coverage check (against `REQUIREMENTS CHECKPOINT 001`)

| Requirement cluster | Covered by | Gaps / notes |
|---|---|---|
| Local-only core (014, 164–166) | INV-1, AD-1, AD-13; the ring model; the network boundary (Diagram 6) | covered structurally |
| User is the authority / scaffold not steer (001–005) | AD-6, AD-11; pipeline; Notification Gateway; Policy Store | "scaffold not steer" is also an INTERFACE concern (deferred) |
| No diagnostic modes / descriptive data model (006, 008) | M §4, §13; the data model carries no verdicts; no diagnostic-keyed code path | covered; INTERFACE must not reintroduce |
| Predictable / quiet / no scorekeeping / restraint / degrade (010–015) | INV-3, 10, 11; AD-3, AD-11, AD-12; no dashboard/metrics/streak subsystem | covered structurally |
| OCD checking / reassurance safeguards / "I don't know" (055–060) | "I don't know" = absence of a confident non-contradicted Claim (`DATA_AND_STATE_MODEL` §—); reassurance-pattern handling is a Metareasoning Controller behaviour | the *behaviour* is a CORE ENGINE design; architecture provides the substrate + the single Notification Gateway |
| AI under permission model / usable without AI / no self-authored objectives (149–154) | AD-6, AD-9; INV-6, 9; Reasoner Interface | covered structurally |
| Least authority / capability perms / audit / budgets / prompt-injection / e-stop (179–187) | INV-7, 8; AD-2, AD-7; Reference Monitor; Capability Host; Budget Authority; Audit; Emergency Stop | covered structurally |
| Data local / open / exportable / deletable / encrypted / durable (160–170) | `DATA_AND_STATE_MODEL` §6–§7; append-only + open format + redaction + backup + encryption-at-rest | mechanism deferred to STORAGE; properties committed |
| Dynamic self-directed (204–253) | INV-13; AD-5; the metareasoning controller (blackboard = one candidate impl); capability lifecycle; §13.6 of the model as test case | the controller *algorithm* and coordination mechanism deferred to CORE ENGINE (Risk 3) |
| The meta-invariant (235) | AD-2, AD-9, AD-14; `AUTHORITY_AND_SECURITY_MODEL` §5–§6, §9; RC-1 at-rest signing | necessary-not-sufficient limits documented; Revision 1 (RC-1/2/3/5) narrows several gaps, does not close them |
| Emergency stop (145) | Ring 0 Emergency Stop; `FAILURE_AND_RECOVERY` §7 | covered |
| Extensibility / one-maintainer / minimal deps (192–195) | AP-12; small Ring 0 + Ring 1; capabilities on-demand | risk: capability sprawl (VC9, Risk 6) — mitigations noted, not proven |
| Resource efficiency (196–200) | AP-12; `RUNTIME_MODEL` §4; on-demand loading; bounded background | concrete budgets deferred to LOW-LEVEL FOUNDATIONS |
| Observability / audit / reasoning summaries (201–203) | Audit (records reads too); provenance-as-structure; the "what MELFINA is doing" view | tamper-evidence mechanism deferred |
| Accessibility / multi-modal / low sensory (188–191) | INTERFACE phase | architecture only requires: one push path, hazard-surface restraint, multi-modal-capable |
| Musician requirements (077–084) | served by the general primitives (model §13.1) — no music subsystem | covered by the data model |
| Teaching / learning / scientific reasoning (070–076, 129–136) | `SYSTEM_ARCHITECTURE` §13–§14; a use of Ring 2 over the Chronicle, not a subsystem | the *pedagogy/discipline* is CORE ENGINE + AI LAYER |

**No requirement is unaddressed at the architectural level.** Behaviours that are
genuinely design-later (the relevance algorithm, the pedagogy, the tamper-evidence
mechanism, the isolation primitive, the interaction UI) are explicitly deferred to
named phases with the architectural constraints stated.

## 6. Open questions this architecture depends on (preserved, not resolved)

| Open question | Where it bites | Minimum commitment made |
|---|---|---|
| model OQ-M2 (`State` a 5th primitive?) | current-state projection | works either way; a primitive `State` ⇒ a checkpointed projection, not a new subsystem (AU-4) |
| model OQ-M1 (`Relation` a primitive?) | the data model | works either way; relations are Claims regardless |
| model OQ-M5 (worries / affect) | a possible new Chronicle unit type | if it becomes a primitive, another unit type — not a new ring (AU-7) |
| model OQ-M6 (relevance computation) | Context Constructor, Metareasoning Controller | committed: context is derived and transient; the algorithm is CORE ENGINE |
| requirements OQ-11 (local reasoner ceiling) | the Reasoner Interface; how much MELFINA can do | committed: local-or-decline; the ceiling is AI LAYER (Risk 5) |
| requirements OQ-12 (routine/consequential boundary) | the deterministic classifier + Policy Store | committed: conservative default (err to HITL); the list is Policy data (AU-6) |
| requirements OQ-17 (real gap vs "the model wants to build") | capability-gap recognition | committed: gated by compose-before-code + test + risk-scaled AUTHORISE regardless; the heuristic is CORE ENGINE |
| requirements OQ-19 (does the meta-invariant hold?) | the whole self-mod governance | committed: structural (no write path) + at-rest signing (RC-1) + T6–9 human-only + documented residual limits (`AUTHORITY_AND_SECURITY_MODEL` §9) |
| requirements OQ-12 (routine/consequential boundary) — placement | whether routine/consequential policy should move into Ring 0 (O5) | **preserved as an explicit design question** — not moved into Ring 0 now; the boundary list stays Policy Store data with a conservative default (AU-6). Aggregate-budget *values* (RC-2) are Ring-0 data; the boundary *definition* is not, yet. |
| how independent can verification realistically be? (RC-3) | high-risk actions where a second observation method may not exist | committed: ≥2 architecturally-independent verifiers where feasible; an explicit, enumerated, justified single-verifier fallback with raised governance otherwise (`AUTHORITY_AND_SECURITY_MODEL` §4.1) |
