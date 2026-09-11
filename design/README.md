# design/

System design and architecture for MELFINA — the structure that realises the
research, requirements, and human/central model.

**Not** implementation. **No** language, storage engine, framework, model, or UI
chosen. Architecture defines **boundaries, flows, and invariants**; the later
pipeline phases fill them with mechanism.

## Status: SYSTEM DESIGN / ARCHITECTURE MISSION 001 + Adversarial Review 001 + Revision 1 (first pass), pending review

Baseline: commit `f3093fc`. `src/`, `research/`, `requirements/`, `model/` — all
**untouched** (no inconsistency requiring an upstream change was found).

**Revision 1** applied the targeted corrections from the adversarial review
(RC-1…RC-7 + M1/M2/M3/M5/M6/M8/M9/M10 + L4/L5; optional O1/O3/O4 adopted, O2
rejected, O5/O6 preserved). It is a **focused revision, not a redesign** — no
technology chosen, no `src/`. The mapping from each finding to where it now lives
is in `ARCHITECTURE_ADVERSARIAL_REVIEW.md`, section "Revision 1 — resolution
status".

### Files (read in this order)

| # | File | What it is |
|---|---|---|
| 1 | **`ARCHITECTURAL_PRINCIPLES.md`** | The 14 invariants (INV-1…14) from requirements + model, the principle each forces (AP-1…14), the classification vocabulary (mechanism / policy / data / capability / governance / derived view), and what is deliberately deferred to later phases. |
| 2 | **`SYSTEM_ARCHITECTURE.md`** | The architecture. The five-ring model, the subsystem map, dependency directions, data ownership, the runtime lifecycle, and every flow (event / reasoning / decision / authorisation / execution / verification / context / memory / capability / automation / terminal / GUI / learning / scientific reasoning / self-evaluation / self-modification governance / audit / failure / security / resource / local-AI / network / human interaction). Diagrams 1–3, 6. The most important decisions (AD-1…15), alternatives rejected, uncertainties, risks, what must NOT be implemented yet. |
| 3 | **`ARCHITECTURAL_ALTERNATIVES.md`** | Nine significant choices (AA-1…9) — subsystem structure, persistence, reasoning coordination, self-mod governance placement, the AI boundary, capability isolation, notification posture, "current state" location, capability↔reasoning coupling — each with candidates, advantages/disadvantages, failure modes, recommendation, confidence. |
| 4 | **`RUNTIME_MODEL.md`** | Lifecycle states, startup sequence, event flow detail, idle behaviour + bounded background work, reasoning-flow detail, concurrency posture, resource management (AB). |
| 5 | **`DATA_AND_STATE_MODEL.md`** | E²CI → the Chronicle (the logical data model); the stores; projections; bitemporal semantics; memory architecture (O); data ownership (E); backup / retention / encryption; what STORAGE must honour. |
| 6 | **`AUTHORITY_AND_SECURITY_MODEL.md`** | The authority model; the autonomy triad enforced; the authorisation flow (J); capability grants; the Reference Monitor; the nine self-modification tiers and the meta-invariant (X); the threat model; the future network isolation; **the documented residual limits — necessary, not sufficient**. |
| 7 | **`CAPABILITY_MODEL.md`** | Ring 3 in detail — what a capability is architecturally; the Capability Host; selection and composition; the full lifecycle (W); deterministic risk classification; workflow / automation architecture (Q); what is *not* a capability. |
| 8 | **`FAILURE_AND_RECOVERY.md`** | The failure philosophy; supervision (topology deferred — O4); current-state-cache / capability / core / reasoning failure paths; mid-execution revocation (RC-5); the emergency stop; recovery guarantees. Diagram 7. |
| 9 | **`TRACEABILITY.md`** | Every invariant, architectural decision, and subsystem traced to a requirement, a model concept, a research finding, or a stated architectural inference. §2.1: Revision 1 corrections → sources. Requirements-coverage check. Open questions this architecture depends on (preserved, not resolved). |
| 10 | **`ARCHITECTURE_ADVERSARIAL_REVIEW.md`** | The adversarial review of Mission 001 (0 CRITICAL, 6 HIGH, 11 MEDIUM, 6 LOW, 15 attacks resisted) **plus** the "Revision 1 — resolution status" section mapping each finding to the correction applied. The findings themselves are unchanged. |

### The architecture in one paragraph

**A small governed core over an append-only claim/event substrate, with reasoning
and capabilities as optional, isolated, permissioned outer rings, coordinated by a
metareasoning controller that is a control loop, not an agent loop.** Five
concentric rings — **Governance · Core Mechanism · Reasoning · Capabilities ·
(future) Network** — with dependencies pointing strictly inward and nothing outer
able to modify anything inner. E²CI maps directly onto the substrate: `Event`,
`Claim`, `Intention` are the append-only units; `Entity` is a registry; "state"
is the one rebuildable current-state cache; `Context` is constructed on demand.
THINK→DECIDE→PROPOSE→AUTHORISE→EXECUTE→VERIFY is a state machine, never collapsed;
reasoning emits only proposals; execution needs an explicit scoped grant — a
live, revocable handle — that passes a non-bypassable Reference Monitor. The
self-modification meta-invariant is structural at rest *and* at runtime: Ring 0
is read-only to everything, cryptographically signed with a human-held key, and
tiers 6–9 (code / subsystem / architecture / self-replacement) are not MELFINA
operations at all. The ~25 named subsystems are **responsibilities**, not
processes — only Ring 0 ↔ all, Ring 1 ↔ the sandbox, and reasoning ↔ effect must
be real enforced boundaries.

### Most important decisions

Five inward-only rings (AD-1) · Ring 0 read-only, no write path, changed only by
human file edit, **signed at rest with a human-held key** (AD-2, AD-14) · the
Chronicle is the single append-only source of truth, everything else a rebuildable
view — one current-state cache + on-demand views (AD-3) · hybrid persistence
(AD-4) · reasoning is a metareasoning control loop, not an agent loop (AD-5) ·
Ring 2 emits only proposals (AD-6) · capabilities sandboxed, no ambient authority,
per-action grants that are **live revocable handles** (AD-7, AD-14) ·
deterministic risk classification from the **union** of authority + scope +
aggregate effect, not lowerable by reasoning, not launderable by composition
(AD-8, AD-15) · self-modification tiers 6–9 are a human path, not a MELFINA
operation (AD-9) · one memory, the Chronicle (AD-10) · a single Notification
Gateway (AD-11) · supervision-based failure isolation, topology deferred (AD-12) ·
all technology deferred (AD-13).

### Biggest risks

The meta-invariant is necessary, not sufficient (documented) · projection
consistency window · metareasoning-controller predictability · lightweightness
vs ring overhead · local reasoner ceiling · capability sprawl vs one-maintainer
comprehension. Full treatment: `SYSTEM_ARCHITECTURE.md` §28.

### What must NOT be implemented yet

Any `src/` code · a storage engine / format / schema · a language or project
skeleton · the capability isolation mechanism · the terminal/GUI action parser ·
the Ring 0 at-rest signing scheme · the relevance / metareasoning algorithm · any
local model integration · Ring 4 (network) in any form · an interaction UI ·
tiers 6–9 self-modification tooling. **Revision 1 fixed invariants, not
mechanisms** — the crypto, the parser, and the redundancy mechanism are all still
LOW-LEVEL FOUNDATIONS choices.

### Recommended next phase

**LOW-LEVEL FOUNDATIONS** — in order: (1) **the capability grant representation +
the Reference Monitor contract, including the structured (parsed-argv) terminal/
GUI action model — CONTAINMENT-CRITICAL (RC-4 / AU-2), and revocable
mid-execution (RC-5)**; (2) the Chronicle's logical format + the append/query
contract; (3) the Ring 3 isolation primitive (zero network by default — L4);
(4) the Ring 0 governance file format + version-chain verification + the at-rest
signing scheme (RC-1); (5) the verifier trust contract (RC-3); (6) only then a
language and project skeleton chosen to serve 1–5 and AP-12. **Do not start
`src/`** until LOW-LEVEL FOUNDATIONS has a reviewed, accepted design and the user
authorises it.
