# PROJECT_STATE

Single source of truth for where this project is and the rules it operates under.
Update this file whenever the phase changes or a principle is added, removed, or revised.

Last updated: 2026-09-12 (TECHNOLOGY SELECTION MISSION 001 — first pass; REQUIREMENTS EXPANSION MISSION 002 committed + pushed as `4f5e97c`; LOW-LEVEL FOUNDATIONS MISSION 001 committed + pushed as `19d0135`; architecture checkpoint committed + pushed as `8a40207`)

---

## 1. What this is

**MELFINA** — a long-term effort to build a **lightweight, local-first personal
life operating system** designed around the actual cognitive, behavioral,
environmental, and practical needs of one user, rather than around a generic
productivity methodology.

MELFINA is the official project name (set 2026-09-09). Earlier notes may say
"Personal OS"; that was a placeholder.

## 2. Current phase

**Phase: LOW-LEVEL FOUNDATIONS — MISSION 001 and REQUIREMENTS EXPANSION
MISSION 002 both complete and committed. TECHNOLOGY SELECTION — MISSION 001
(first pass) complete, NOT yet committed, awaiting user review. Still
design/research — no `src/`, no implementation, no dependency installed;
technology is *recommended*, not yet adopted.**

The SYSTEM DESIGN / ARCHITECTURE checkpoint (MISSION 001 + ADVERSARIAL REVIEW 001
+ REVISION 1) was committed and pushed as **`8a40207`** ("design: ARCHITECTURE
MISSION 001 + adversarial review 001 + revision 1") — 11 `design/*.md` files +
this file. The LOW-LEVEL FOUNDATIONS MISSION 001 checkpoint (README.md + 12
technology-independent contracts in `design/foundations/`) was committed and
pushed as **`19d0135`** ("design: LOW-LEVEL FOUNDATIONS MISSION 001"). The
REQUIREMENTS EXPANSION MISSION 002 checkpoint (PART IV-C + companion file +
conflicts/open-questions updates) was committed and pushed as **`4f5e97c`**
("requirements: REQUIREMENTS EXPANSION MISSION 002"). `origin/main` is at
`4f5e97c`.

Research Missions 001 + 002, PERSONAL REQUIREMENTS MISSION 001 (+ correction), and
HUMAN / CENTRAL MODEL MISSION 001 are done and pushed (commit `f3093fc`).

**TECHNOLOGY SELECTION MISSION 001 (2026-09-11/12, NOT yet committed):**
determined which concrete technologies can satisfy the already-committed
requirements, architecture, and foundation contracts — this mission chooses
**candidates**, not new invariants, and was explicitly forbidden from
implementing anything. Output: `design/technology/` — `README.md` +
`TECHNOLOGY_SELECTION.md` (the cross-cutting synthesis) + 10 category
evaluations (`LANGUAGE_EVALUATION.md`, `CHRONICLE_EVALUATION.md`,
`ISOLATION_EVALUATION.md`, `PROCESS_AND_IPC_EVALUATION.md`,
`TERMINAL_GUI_EVALUATION.md`, `CRYPTO_GOVERNANCE_EVALUATION.md`,
`REASONING_COMPUTATION_EVALUATION.md`, `BUILD_AND_SUPPLY_CHAIN.md`,
`MIGRATION_PORTABILITY.md`) + `TECHNOLOGY_ADVERSARIAL_REVIEW.md` (36 named
attacks against the specific technologies chosen) + `TECHNOLOGY_EXPERIMENT_
PLAN.md` (prioritised pre-trust experiments + formal-verification
opportunities) + `TECHNOLOGY_DECISION_LOG.md` (an auditable index of every
recommendation, its alternative, reversal cost, and confidence).

**Proposed stack (full detail + confidence levels: `design/technology/
TECHNOLOGY_SELECTION.md` §2, §14):** Rust as the primary language (single-
language core; isolated capabilities may use another runtime); a two-tier
Chronicle (a purpose-built append-only framed log as the authoritative
store, SQLite/WAL as the disposable, rebuildable current-state cache); a
two-tier Ring-3 isolation model (Landlock + seccomp-bpf + namespaces +
cgroups v2 as the default, unprivileged tier; a Firecracker-class microVM
reserved for governance-flagged high-risk capability classes); the Reference
Monitor as its own OS process, separate from the rest of Ring 1, with Ring 2
(reasoning) also its own process holding no capability grant — realising
RC-7's three must-be-real boundaries as actual process boundaries; Unix
domain sockets + `SCM_RIGHTS` file-descriptor passing for IPC (which gives
OS-resource-shaped capability grants a kernel-enforced, `[SEN]`-strength
unforgeability property — a concrete finding for F1 §19's previously
`[ID]`-only grant-representation question); `execve`-only terminal
invocation with `openat2` TOCTOU defence; AT-SPI2 for structured GUI
targeting with a disclosed Wayland coordinate-fallback gap; Ed25519 +
SHA-256 for governance integrity, an offline signing key, and a TPM 2.0
monotonic counter for rollback protection where available; `llama.cpp`/GGUF
for local inference (model selection explicitly deferred). **Twelve
decisions are flagged as requiring explicit human sign-off before CORE
ENGINE proceeds** (language, Chronicle substrate, isolation, the Monitor's
process boundary, governance integrity, and the AI runtime boundary — the
exact set the mission's own instructions named); everything else is offered
as a reviewable, non-blocking recommendation.

**Verified in this mission:** compatibility against all ten LOW-LEVEL
FOUNDATIONS contracts (F1–F10) with no invariant weakened or bypassed; the
offline-core test (no technology choice requires network reachability at
runtime, with build-time package resolution correctly distinguished from
runtime behaviour); the AI-boundary test (the reasoning process holds zero
ambient authority regardless of model behaviour, and Ring 1 remains fully
functional with it absent); the dynamic-capability/self-modification
boundary (more computational power never becomes more authority, because
authority is gated by grant-possession and effect-class membership, neither
a function of compute power); a 36-attack adversarial review specific to the
chosen technologies, surfacing exactly one new gap (debug/introspection
interfaces must be gated like any other effect and disabled by default in
non-development builds) which is closed with a concrete recommendation, not
left open. **`src/` untouched, no dependency installed, no implementation
performed.** A note of record: this mission's resume instructions
momentarily conflated a validation claim ("VC1–VC18 clean") from the
*prior*, already-committed REQUIREMENTS EXPANSION MISSION 002 with this
mission's own scope; independently re-verified as accurate for that prior
mission and explicitly not this mission's own artifact
(`design/technology/TECHNOLOGY_SELECTION.md`, opening note).
retroactively expands the already-committed requirements phase with **PART
IV-C — GENERAL REASONING, KNOWLEDGE, AND WISDOM**
(`requirements/REQUIREMENTS_MASTER.md` §56–76, `MEL-REQ-254`…`MEL-REQ-364`, 111
new requirements) + 8 new anti-requirements (`MEL-AR-20`…`27`) + a new companion
file `requirements/KNOWLEDGE_AND_REASONING_MODEL.md` (the "wisdom" operational
definition + critique, the analogy/causation taxonomy grounded in Gentner and
Pearl, the first-principles decomposition pipeline, an architecture/foundation
compatibility test, and a dedicated adversarial review of 20 attack scenarios) +
6 new conflicts (`requirements/CONFLICTS.md` VC13–VC18) + 11 new open questions
(`requirements/OPEN_QUESTIONS.md` OQ-22–32). **Totals: 364 requirements + 27
anti-requirements.** The mission's two load-bearing new sections — **§74
knowledge ≠ authority** and **§75–76 the anti-bypass hardening set** — restate
existing F1/F3/F7/F8/F9/F10 foundation-contract invariants against 12+
specific reasoning-derived bypass attempts (self-authorising via knowledge,
capability-creation, mathematical reclassification, experiment/simulation
framing, aggregate-effect decomposition, GUI/terminal ambiguity, the lethal
trifecta, governance reinterpretation, autonomy-ceiling arguments, post-hoc
rationalisation, "wisdom"/ethics-based paternalism, and self-modification-tier
arguments) — **no new authority mechanism was invented; every closure routes
back to an existing contract.** **No architecture, model, or foundation
document was modified** — a full 20-item compatibility check found no
contradiction requiring one. One residual, unresolved tension is recorded as
`OQ-32`: whether explicitly building MELFINA toward first-principles reasoning
about its own governance changes the answer to `OQ-19` (meta-invariant
sufficiency) rather than only making the question more urgent — reported, not
resolved. `src/`, `research/`, `model/`, and all committed `design/` and
`design/foundations/` documents untouched. **Not committed, not pushed.**

**LOW-LEVEL FOUNDATIONS MISSION 001 (2026-09-11):** defined the smallest,
precise, technology-independent contracts every future implementation must obey.
Output: `design/foundations/` — `README.md` + 12 numbered contracts:
1. `CAPABILITY_GRANT_MODEL.md` — what a grant *is* (unforgeable, scoped,
   time-bounded, revocable, Monitor-issued, evaluable to exactly one of {this
   effect may / may not occur}); envelope fields; attenuation (narrow-only);
   delegation (downward, through the Monitor); composition (one grant per
   authorised Intention, union risk class); creator-authority ≤ check (M8);
   aggregate-effect budget (RC-2); declared-scope-overlap conflict ⇒ serialise
   (M9); the 9-conjunct minimum to answer "may this exact effect occur?".
2. `STRUCTURED_ACTION_MODEL.md` — CONTAINMENT-CRITICAL (RC-4). Terminal actions
   are structured (`process-exec`: resolved abs path + argv vector + env
   allow-list; **never a shell string**); `shell-exec` a distinct high-risk
   type; `gui-op` structured targeting, coordinate fallback explicit + higher
   risk; canonicalisation (resolve symlinks, collapse `.`/`..`, bind to a fixed
   root, re-canonicalise at every check, deny on any difference — TOCTOU); the
   core invariant: a grant for `program X + argv A` does **not** authorise
   `X + argv B`, `shell → X`, or added composition unless explicitly authorised.
3. `REFERENCE_MONITOR_CONTRACT.md` — complete mediation, non-bypassable,
   tamper-proof, fail-safe defaults; the REQUEST→VALIDATE→CLASSIFY→CHECK
   GOVERNANCE→AUTHORISE/DENY→ISSUE LIVE GRANT→EXECUTE→VERIFY sequence; a closed
   deny-reason vocabulary; `AUTHORIZED(e,g,s,t)` as a formal conjunction; the
   distrust table (reasoner claims, capability self-description, Intention
   status, AI-generated user text, self-declared risk, projections for
   consequential conjuncts, its own past allow, raw strings as scope, requests
   to widen / edit Ring 0 / raise the ceiling, the network).
4. `REVOCATION_MODEL.md` — grants are live revocable handles (RC-5); lifecycle
   ISSUED→ACTIVE→SUSPENDED→REVOKED/EXPIRED→TERMINAL; the outcome vocabulary
   (completed / partially completed / rolled back / failed / interrupted —
   closed, honest, identical everywhere); revocation before / during (between
   effects / mid single effect) / after; per-effect compensation (declared
   tested inverse ⇒ rollback + VERIFY, else PARTIAL and **no rollback claimed**);
   how a partial effect becomes a recorded Event; per effect-kind meaning of
   "stop"/"rollback" (long process, file write/create, file delete/truncate/
   rename, GUI, workflow, automation, capability composition, chronicle-append).
5. `CHRONICLE_LOGICAL_FORMAT.md` — exactly 3 unit kinds (Event / Claim /
   Intention) + a thin Entity Registry; Context / State / Relation / Task / Goal
   / Routine / Project / Memory / Skill / Workflow / Calendar / Dashboard /
   Streak are **not** units; common fields; bitemporal time; structural
   relations recorded as Claims (not promoted — OQ-M1 stays open); supersession
   / AGM belief change = all appends; redaction = a tombstone Event; **OQ-M1,
   OQ-M2, OQ-M3, OQ-M4, OQ-M5, OQ-M11 preserved with both alternatives**.
6. `CHRONICLE_CONTRACT.md` — APPEND / READ / QUERY / REPLAY / VERIFY / PROJECT;
   durable-before-return, monotonic transaction-time, atomic unit append, no
   silent duplicate suppression; the authoritative read (computed from the
   sequence, used by the Monitor + consequential DECIDE — M2) vs the view read
   (current-state cache, may lag, never for consequential decisions); bitemporal
   query forms; deterministic REPLAY; corruption detection with no silent
   repair; one materialised cache (O1), all other views on-demand; the Chronicle
   is the serialisation point (single-writer, no distributed consensus).
7. `ISOLATION_CONTRACT.md` — 14 required Ring-3 properties (no ambient authority;
   explicit-grant-only; **zero network capability by default — the socket
   primitive is withheld, not unconfigured**, L4; filesystem / process
   confinement per grant; Ring-0 region never granted, RC-1; hard resource
   cut-offs; terminable; failure-contained; no Ring-0 path; cannot forge a grant
   or impersonate the Monitor; no persistent inter-action state; lightweight);
   the seL4 confidentiality / integrity / availability framing (timing channels
   **not** covered — documented residual); what happens if isolation fails; the
   reasoning↔effect boundary is real even if Ring-2 and Ring-3 code are
   co-located (O2 rejected); which properties are structural vs empirically
   tested.
8. `GOVERNANCE_FORMAT.md` — the logical structure of the Ring-0 governance
   object: the meta-invariant, the nine-tier policy (tiers 6–9 **cannot be
   expressed as autonomous** — a format property), the **closed effect-class
   vocabulary** (owned here; F1 references it) + permanently-absent classes
   (widen-authority, edit-governance, network-*), risk floors, the
   lethal-trifecta prohibition (L5), authority ceilings (no self-raising),
   aggregate governance constraints (RC-2), emergency-stop behaviour, other
   constitutional invariants; the version metadata (parent link, creation
   metadata, human authorisation, integrity metadata, activation, retirement);
   governance is **not** ordinary MELFINA knowledge and MELFINA cannot author,
   sign, or activate a version; the class→governance maps the Monitor consults.
9. `GOVERNANCE_INTEGRITY.md` — RC-1's abstract integrity contract: a human-held
   signing authority MELFINA does **not** possess in any usable form; a linear,
   append-only, hash-linked version chain; each transition human-signed over
   `hash(content) ‖ parent-id`; rollback-as-forward-only; a separately-protected
   current-head marker (TUF rollback-protection lesson); startup verification
   (walk chain, recompute hashes, verify signatures, check forbidden shapes,
   check head marker) — **any failure ⇒ MELFINA refuses to run**, never runs
   degraded, never auto-repairs; runtime file-change ⇒ emergency stop; version
   pinning; tamper-evident (not tamper-proof) + fully auditable; integrity
   independent of the Chronicle; the required security properties (not Ed25519 /
   RSA / TPM / enclave — an offline key satisfies the contract, hardware is a
   `[OPEN]` upgrade).
10. `VERIFIER_CONTRACT.md` — RC-3: a verifier is a bounded, low-authority,
    preferably-deterministic checker that returns {confirmed / contradicted /
    inconclusive}, **cannot cause effects**, is **not an agent** (no goal state,
    no Intention ownership, no retry-and-act); 11 trust properties;
    "architecturally independent" defined as differing in ≥ 2 of {evidence
    source, checking mechanism, implementation lineage, determinism class}
    **without** claiming statistical / mathematical independence (Knight &
    Leveson 1986 + the 2026 AI-agent replication — common-mode failure is a
    documented residual); the 4-condition single-verifier fallback; high-risk /
    irreversible + no strong verification ⇒ human-gated, not autonomous;
    disagreement ⇒ take the more adverse verdict; verification failure never
    upgrades an outcome.
11. `FOUNDATION_CROSS_CONTRACT_ANALYSIS.md` — nine end-to-end path traces
    (A simple read · B terminal command · C file modification · D long-running
    action revoked midway · E malicious reasoner · F malicious capability ·
    G governance tampering · H local-only violation attempt · I high-risk
    action); per path: authoritative component, trust boundary, authority
    boundary, failure point, recovery, audit record; a contract-vs-contract
    consistency matrix (every shared concept has exactly one owning contract;
    **no contradictions found**); the open seams carried forward.
12. `TECHNOLOGY_SELECTION_CRITERIA.md` — MUST-HAVE / SHOULD-HAVE / MUST-NOT-HAVE
    criteria, each traced to a foundation contract, for: cross-cutting, language,
    Chronicle storage substrate + format, Ring-3 isolation mechanism,
    cryptographic integrity, process / concurrency model, IPC, GUI control,
    terminal control — for the *next* phase (TECHNOLOGY SELECTION) to score
    candidates against. **Chooses nothing.**

Enforceability of every MUST-level invariant is tagged **[SEN]** structurally
enforceable now / **[ID]** implementation-dependent / **[ETL]** empirically
testable later / **[OPEN]**. No model open question resolved (OQ-M1 / OQ-M2 /
OQ-M3 / OQ-M5 / OQ-M9 / OQ-M11 preserved with both alternatives). No requirement
weakened. No technology chosen. No `src/`. `research/`, `requirements/`,
`model/`, and the committed `design/*.md` architecture set untouched (no
contradiction requiring an upstream change was found). **Not committed, not
pushed.**

**Revision 1 (2026-09-10):** a targeted edit pass — NOT a redesign — applying the
adversarial review's corrections into the architecture documents:
- **RC-1** Ring 0 at-rest integrity: cryptographic signature chain rooted in a
  human-held key MELFINA never possesses, verified every startup, invalid ⇒
  refuse to run; Ring 0 path hard-excluded from every File Access grant.
- **RC-2** aggregate-effect governance: per-activity-chain aggregate budget over a
  window; the crossing action re-gates as consequential; a composed workflow is
  risk-classified by the UNION of component authority + scope + aggregate effect;
  composition never launders a high-risk action into low-risk steps.
- **RC-3** verifier trust model (`AUTHORITY_AND_SECURITY_MODEL.md` §4.1, new):
  deterministic where practical, minimal / no ambient authority, read-only unless
  narrowly justified, cannot cause effects, independently testable against
  fixtures, not a second agent; ≥2 architecturally-independent verification
  mechanisms for high-risk actions, with an explicit enumerated single-verifier
  fallback where a second is impractical.
- **RC-4** AU-2 re-tagged **CONTAINMENT-CRITICAL** and made the first LOW-LEVEL
  FOUNDATIONS deliverable: terminal authority = structured actions, not raw shell
  strings; command grants matched against parsed argv / a structured invocation;
  shell interposition is a distinct higher-risk capability.
- **RC-5** capability grants are live, revocable handles the Reference Monitor can
  invalidate mid-execution; after revocation no new effect, running execution
  stopped where possible, reversible effects rolled back, irreversible/partial
  recorded as partial failure; user-visible outcome vocabulary = completed /
  partially completed / rolled back / failed / interrupted.
- **RC-6** reasoner locality is structural: Ring 1 and Ring 2 hold no network
  capability; the Reasoner Interface cannot open a socket; a remote endpoint is
  unreachable through the core capability model — "cannot", not "does not".
- **RC-7** the ~25 named subsystems are **responsibilities**, not necessarily
  separate processes/services/binaries/IPC boundaries; an implementation may
  group them into a few cohesive components; only Ring 0 ↔ all, Ring 1 ↔ the
  sandbox, and reasoning ↔ effect must be real enforced boundaries.
- Small corrections: M1 (context on demand), M2 (Chronicle-contract reads for
  consequential correctness), M3 (Reasoning Contributor trust/lifecycle), M5/O6
  (cache-divergence audit = future concern), M6 (cyclic automation triggers of
  any length), M8 (explicit Reference Monitor ≤-creator-authority check), M9
  (declared-scope-overlap conflict), M10 (structural proposal validation), L4
  (Ring 3 zero network by default), L5 (lethal-trifecta prohibition = a Ring 0
  invariant).
- Optional: **O1 adopted** (one Chronicle + one current-state cache + on-demand
  views), **O2 rejected** (do not collapse Ring 2 / Ring 3 — the reasoning↔effect
  boundary is must-be-real), **O3 adopted** (blackboard = one candidate impl),
  **O4 adopted** (supervisor topology not specified now), **O5 preserved** as an
  explicit design question, **O6 retained** as a future implementation concern.
- **No technology chosen** — the crypto scheme, the action parser, the redundancy
  mechanism, the isolation primitive, storage, and language remain LOW-LEVEL
  FOUNDATIONS choices. No model open question resolved (OQ-M1/OQ-M2/OQ-M5
  untouched). The adversarial review's findings §2–§19 were **not deleted or
  rewritten** — a "Revision 1 — resolution status" section was added before §1.
  `src/`, `research/`, `requirements/`, `model/` untouched. Not committed, not
  pushed.
- Files changed: all 9 `design/*.md` architecture docs +
  `design/ARCHITECTURE_ADVERSARIAL_REVIEW.md` (status section only) +
  `design/README.md` + this file.
- Full finding→correction mapping: `design/ARCHITECTURE_ADVERSARIAL_REVIEW.md`
  "Revision 1 — resolution status"; full traceability:
  `design/TRACEABILITY.md` §2.1.

**Revision 1 — resume-completed (2026-09-11).** The edit pass was interrupted by
a host restart with the bulk of the integration already written. On resume: a
full cross-document consistency check confirmed RC-1…RC-7 + M1/M2/M3/M5/M6/M8/M9/
M10 + L4/L5 are integrated throughout, no model open question (OQ-M1/OQ-M2/OQ-M5)
was resolved, no deferred technology was chosen, and `src/` is untouched. Three
residual propagation gaps were closed — `SYSTEM_ARCHITECTURE.md` §5 startup now
names the RC-1 signature check; §6.1's Event-flow box no longer implies a
maintained "context cache" (M1); a duplicated phrase in §6.2 was fixed — plus one
RC-7 wording variance in `ARCHITECTURAL_ALTERNATIVES.md` AA-1 was harmonised, and
`design/TRACEABILITY.md` §2.1's RC-1 row + the review's resolution-status section
got a resume note. **Verdict: Revision 1 complete; the architecture set is
internally consistent and ready for user review. Still NOT committed, NOT
pushed.**

### SYSTEM DESIGN / ARCHITECTURE MISSION 001 — summary of outcome (2026-09-10)

Designed the architecture that realises the research + requirements + E²CI model.
Architecture defines **boundaries, flows, and invariants** — **no** language,
storage engine, framework, model, or UI is chosen or implied (deferred to the
named later phases). `src/`, `research/`, `requirements/`, `model/` untouched — no
inconsistency requiring an upstream change was found.

Output in `design/` (11 files after Revision 1):
`ARCHITECTURAL_PRINCIPLES.md` (invariants INV-1…14 + principles AP-1…14 + the
mechanism/policy/data/capability/governance/derived-view vocabulary + what is
deferred) · `SYSTEM_ARCHITECTURE.md` (the ring model, subsystem map, all flows,
Diagrams 1–3 & 6, the key decisions AD-1…15, risks) · `ARCHITECTURAL_ALTERNATIVES.md`
(9 choices AA-1…9, candidates/tradeoffs/recommendations/confidence) ·
`RUNTIME_MODEL.md` · `DATA_AND_STATE_MODEL.md` · `AUTHORITY_AND_SECURITY_MODEL.md`
(incl. §4.1 verifier trust model and §9 documented residual limits) ·
`CAPABILITY_MODEL.md` · `FAILURE_AND_RECOVERY.md` (Diagram 7) · `TRACEABILITY.md`
(incl. §2.1 Revision 1 corrections → sources) · `ARCHITECTURE_ADVERSARIAL_REVIEW.md`
(the review + the "Revision 1 — resolution status" section) · `README.md`.

**The architecture in one line:** *a small governed core over an append-only
claim/event substrate, with reasoning and capabilities as optional, isolated,
permissioned outer rings, coordinated by a metareasoning controller that is a
control loop, not an agent loop.*

**Five concentric rings, dependencies strictly inward, nothing outer modifies
anything inner:**
- **Ring 0 — Governance:** the reference monitor / permission gate, the
  self-modification tier policy, the autonomous-action budgets, the emergency
  stop, the meta-invariant. Read-only to every outer ring; changed **only by a
  human editing versioned files** — not a MELFINA operation.
- **Ring 1 — Core Mechanism** (deterministic, local-only, no AI): the **Chronicle**
  (append-only bitemporal Events + Claims + Intentions — the single source of
  truth), Projection Engine, Query, Entity Registry, Pipeline State Machine,
  Audit, Capability Registry (metadata), Policy Store, Notification Gateway,
  Supervisor. **Complete and usable alone** (INV-9 / MEL-REQ-154).
- **Ring 2 — Reasoning** (optional, bounded, may use AI): the Metareasoning
  Controller (blackboard-style control, **not an agent loop**), Context
  Constructor, Reasoning Contributors, Self-Evaluation, the narrow Reasoner
  Interface. **Emits only proposals + claims — never effects.**
- **Ring 3 — Capabilities** (isolated, permissioned, dynamic, replaceable):
  skills, tools, workflows, terminal control, GUI control, file access,
  verifiers. **No ambient authority**; per-action typed grants from the Reference
  Monitor; sandboxed.
- **Ring 4 — (future) Network:** never in the core; off by default; separately
  installed; structurally cannot form the lethal trifecta with core data.

**E²CI maps directly:** `Event` / `Claim` / `Intention` = append-only Chronicle
units; `Entity` = a registry; `Time` = bitemporal coordinates on every unit;
"state" = the one rebuildable current-state cache; `Context` = constructed on
demand, never stored (O1, M1).

**Key decisions:** five inward-only rings (AD-1) · Ring 0 read-only, no write
path, signed at rest with a human-held key (AD-2, AD-14) · Chronicle = single
append-only source of truth, everything else a rebuildable view — one
current-state cache + on-demand views (AD-3) · hybrid persistence (AD-4) ·
reasoning = a metareasoning control loop, not an agent loop (AD-5) · Ring 2 emits
only proposals (AD-6) · capabilities sandboxed, no ambient authority, per-action
grants that are live revocable handles (AD-7, AD-14) · deterministic risk class
from the union of authority + scope + aggregate effect, not lowerable by
reasoning, not launderable by composition (AD-8, AD-15) · self-modification tiers
6–9 are a human path, not a MELFINA operation (AD-9) · one memory, the Chronicle
(AD-10) · a single Notification Gateway (AD-11) · supervision-based failure
isolation, topology deferred (AD-12) · all technology deferred (AD-13).

**THINK→DECIDE→PROPOSE→AUTHORISE→EXECUTE→VERIFY** is a state machine, never
collapsed; the autonomy triad (cognitive / decision / execution) is structurally
separated (reaching a proposed Intention never itself produces an "authorised"
grant).

**The meta-invariant is structural** (Ring 0 has no write path; "widen my
authority" / "edit Ring 0" are absent from the capability namespace; tiers 6–9 are
human-only) — **and documented as necessary, not sufficient**: social-engineering
of the user, unintended Ring-1 bugs, and test-passing-but-badly-generalising
capabilities remain as recorded residual limits (`AUTHORITY_AND_SECURITY_MODEL.md`
§9).

**Grounding pass (~10 sources):** reference monitor / policy-mechanism separation
(Hydra), capability-based OS (seL4, Fuchsia/Zircon, WASI component model), event
sourcing + CQRS, blackboard architecture, microkernel vs modular monolith,
Erlang/OTP supervision + "let it crash".

**Biggest risks (documented, not solved):** the meta-invariant's insufficiency;
projection consistency window; metareasoning-controller predictability;
lightweightness vs ring overhead; local reasoner ceiling; capability sprawl vs
one-maintainer comprehension.

**Not committed or pushed** (per the mission). **Recommended next phase:**
LOW-LEVEL FOUNDATIONS — (1) **the capability grant representation + Reference
Monitor contract, including the structured (parsed-argv) terminal/GUI action
model — CONTAINMENT-CRITICAL (RC-4), revocable mid-execution (RC-5)**;
(2) Chronicle logical format + append/query contract; (3) the Ring 3 isolation
primitive (zero network by default — L4); (4) Ring 0 governance file format +
version-chain verification + the at-rest signing scheme (RC-1); (5) the verifier
trust contract (RC-3); (6) only then a language + project skeleton. **No `src/`
until LOW-LEVEL FOUNDATIONS is accepted.**

We are NOT coding the application. No architecture chosen. No language (C vs C++)
chosen. No storage substrate chosen — and the model explicitly must survive any
choice (relational / graph / document / triple / hypergraph — none implied). No
framework, model, or UI toolkit chosen. `src/` untouched.

### HUMAN / CENTRAL MODEL MISSION 001 — summary of outcome (2026-09-10)

Determined the smallest coherent conceptual model of a human life from which
MELFINA's capabilities emerge. Not built around application categories (tasks,
habits, notes, calendars, projects, goals, reminders, journals, dashboards, music
practice) — those emerge as views.

Output in `model/`:
- `HUMAN_CENTRAL_MODEL.md` — the model (18 sections + `MODEL CHECKPOINT 001`).
- `MODEL_ALTERNATIVES.md` — 4 candidate models (A event-centric · B entity/
  relationship-centric · C state/transition-centric · D three-primitive) compared
  rigorously; conclusion: no single one is sufficient, a synthesis is necessary.
- `MODEL_OPEN_QUESTIONS.md` — `OQ-M1 … OQ-M12`.
- `README.md` — index.

**Leading model — E²CI. Four primitives:**
- **ENTITY** — anything that persists and can be referred to (`kind` is an open
  attribute: person / place / thing / work / concept / capability / workflow /
  source / `self` / `melfina` / …). Grounded in BFO continuant; personal
  knowledge graphs.
- **EVENT** — something that occurs at/over time, with participants in roles,
  possibly bringing about or ending conditions; includes observations, actions,
  sessions, decisions, and *expected* future events. Grounded in BFO occurrent;
  event calculus; W3C PROV.
- **CLAIM** — a statement held by an agent, with an origin and an epistemic
  **status** (observed / reported / inferred / hypothesised / open / stipulated /
  decided / recalled), a **confidence**, full **provenance**, and **bitemporal**
  coordinates (valid-time / transaction-time). *All of MELFINA's knowledge is
  claims.* Relationships and "what holds now" (state) are claims. Grounded in
  epistemic/doxastic logic; PROV; AGM belief revision; bitemporal modelling.
- **INTENTION** — an agent's directedness toward a future condition or action —
  the world-to-mind stance, distinct from a belief by *direction of fit*. Goals,
  tasks, plans, commitments (an intention with a creditor), routines, reminders
  are cases. Grounded in Anscombe/Searle; BDI; Castelfranchi social commitment.

Plus: **TIME** is a bitemporal *dimension* (not a primitive — "calendar"
dissolves into a view). **CONTEXT** is *derived per situation* by a relevance
trade-off (Sperber & Wilson) — never stored.

**Key decisions (open to challenge):** `State` folded into `Claim` (OQ-M2);
`Relation` folded into `Claim` — carrying its own time/provenance/confidence
(OQ-M1); `Intention` kept (direction of fit) — folding it is rejected;
four primitives — three is insufficient, five/six not clearly necessary (OQ-M8).

**Strongest test-case results:** music needs **zero** music-specific primitives
(work/movement/section/passage = `part-of`-linked entities; practice session = a
`session` event; technical problem = a user-worded descriptive claim;
interpretation = `stipulated` claims MELFINA never overwrites; progression = a
`supersedes` chain — no streak, no hours headline). Scientific reasoning maps
cleanly (hypothesis/evidence/experiment are *labels + a discipline*, not model
extensions). The agency pipeline (USER intends → MELFINA reasons → proposes →
USER authorises → MELFINA acts → world changes → MELFINA observes → updates) maps
exactly, and the autonomy triad is naturally separated. The dynamic self-directed
MELFINA is supported without freezing today's agent tech (strategies and skills
are claims/entities, never enums).

**What the model deliberately does NOT contain:** the self-modification
meta-invariant rules (external to MELFINA's reasoning by design); any enforcement
mechanism (permissions are represented, not guarded); any evaluative verdict the
user did not enter; any diagnostic category (ADHD/autism/OCD not reified); any
implementation choice.

**Top unresolved semantic questions:** is `State` a fifth primitive (OQ-M2); is
`Relation` (OQ-M1); is four genuinely minimum-sufficient (OQ-M8, a prototyping
question); recurrence identity (OQ-M3); structure of `confidence` (OQ-M4); how
worries / intrusive thoughts / affect are represented (OQ-M5) — flagged for user
input; "what matters" beyond `Intention` + relevance (OQ-M7).

**Confidence:** Moderate–High that the four *kinds* of thing (referents /
happenings / claims / directedness) are right — converges across BFO, event
calculus, epistemic logic, direction-of-fit, PROV. High that claims carry
provenance + status + bitemporal coordinates, and that context is derived not
stored. Moderate on `State`- and `Relation`-folding and on whether four is
genuinely minimal. Unknown whether E²CI holds *this user's* life well — only
real-world use tells.

`requirements/` was **not edited** — issues with requirements are logged as model
open questions, not changed (research and requirements kept as immutable prior
checkpoints).

### PERSONAL REQUIREMENTS MISSION 001 — summary of outcome (2026-09-10)

Mission = base brief (30 requirement areas, MUST/SHOULD/MAY/MUST NOT, IDs,
conflicts, anti-requirements, musician requirements, AI requirements) +
autonomous-reasoning addendum (cognitive / decision / execution autonomy) +
local-only / network-isolation addendum + autonomous-personal-intelligence
extension (assistant, autonomous automation, adaptation, skills, scientific
thinking, teaching, world understanding, deep reasoning, emotional understanding,
software engineering, computer agency, controlled self-improvement, general
capability, extreme-capability-+-lightweightness, capability model) +
**dynamic-self-directed correction** (do not model MELFINA as a static capability
catalogue; it dynamically determines what capabilities / context / reasoning /
tools a problem needs and, where authorised, constructs new ones).

Output in `requirements/`:
- `REQUIREMENTS_MASTER.md` — **253 requirements** (`MEL-REQ-001…253`) across 55
  areas + **19 anti-requirements** (`MEL-AR-01…19`) + the **MELFINA Capability
  Model** (15 capabilities — a *vocabulary, not a fixed catalogue*) + the
  **autonomy triad** (cognitive / decision / execution, never collapsed) + the
  **action pipeline** (THINK→DECIDE→PROPOSE→AUTHORISE→EXECUTE→VERIFY) + **PART
  IV-B — THE DYNAMIC SELF-DIRECTED SYSTEM** (metareasoning; dynamic context/skill
  selection; capability-gap recognition + capability creation; human–MELFINA
  co-creation; a graded **nine-tier self-modification model** with **THE
  META-INVARIANT** — MELFINA cannot redefine the rules governing its own
  self-modification; dynamic resource allocation + dynamic autonomy;
  self-evaluation; "best is situational, dynamic ≠ unpredictable") + a feasibility
  classification + `REQUIREMENTS CHECKPOINT 001`.
- `CONFLICTS.md` — 10 research-level (RC1–RC10) + 7 vision-level (VC1–VC7) + 5
  dynamic-system (VC8–VC12) tensions as configurable dimensions / constraints /
  safeguards / open questions. Not resolved by fiat.
- `OPEN_QUESTIONS.md` — OQ-1…OQ-21 + the deliberately-undecided list.
- `README.md` — index.

Priority mix: ~120 MUST · ~90 SHOULD · ~20 MAY · ~23 MUST NOT (incl. the 19 AR).

**Highest-priority MUSTs:** local-only core (stronger than "local-first" — the core
has no notion of a remote; MEL-REQ-014, 164–166); user is the authority /
scaffold-don't-steer / more-than-a-tracker (001–005); no diagnostic modes,
descriptive-not-judgemental data model (006, 008); predictable-deterministic core,
quiet by default, no scorekeeping by default, restraint at the hazard surfaces,
graceful degradation (010–013, 015); OCD checking/reassurance safeguards, "I don't
know" is first-class, no compelled engagement (055–060); AI operates under the
permission model not as owner, fully usable with AI disabled, no self-authored
objectives / no core self-modification / no self-replication (149–150, 153–154);
least-authority + capability-based permissions + audit + autonomous-action budgets
+ assume-prompt-injection + emergency stop (179–187); data is local/open/
exportable/deletable/encrypted/durable (160–170); **THE META-INVARIANT — MELFINA
cannot redefine (silently, autonomously, or by persuasion) the rules governing its
own self-modification; those rules are external to its reasoning, monotonic
(authority only narrows without an explicit human act), versioned, audited (235)**;
self-modification tiers 6–9 never autonomous (236); dynamic choices bounded by the
grounding factors, MELFINA does not optimise one fixed notion of "best", dynamic ≠
unpredictable (207, 251–253); autonomy ceiling not self-raisable (245);
self-evaluation not resting on the model's own metacognitive report (248).

**Anti-requirements (19):** compulsive tracking · gamification/streaks ·
notification spam · rigid methodology · endless configuration · reassurance machine
· AI dependency/deskilling · opaque adaptive drift · surveillance · maintenance-
heavy PKM · feature-heavy dashboard · cloud dependency/phone-home · unrestricted
autonomous agent · manipulation/false-intimacy/relationship-replacement · grandiose
claims · self-authored goals / core self-modification · ungrounded/unpredictable
"dynamism" · silent self-privilege / self-modification-rights escalation · a
self-improvement loop that trusts its own judgement.

**Rejected interpretations (recorded):** autonomy = unrestricted execution;
adapt-with = collect-all-data; general intelligence = one giant model;
self-improvement = self-modification/replication (it is instead a *graded* capability,
tiers 6–9 never autonomous); dynamic/self-directed = free to do anything /
unpredictable; MELFINA can eventually govern its own modification permissions;
immutable invariants are a *sufficient* guarantee for a capable self-modifier
(necessary, not proven sufficient); the system can judge for itself that a
self-improvement worked; the 15 capabilities are the complete feature set;
emotional understanding = the system has feelings; "local-first is enough" (→
local-only core); more capability always better / optimise one fixed notion of
"best"; diagnostic research → diagnostic product modes; explanations fix
over-reliance; recursive self-improvement toward superintelligence is the
trajectory (explicitly not a goal).

**Evidence discipline:** every requirement tagged `[E]/[G]/[DI]/[H]/[U]` and
provenance-classed `A` (evidence) / `B` (user goal) / `C` (hypothesis) / `D`
(unknown) / `E` (deliberately undecided). No `[U]`/`[H]` finding was promoted to a
MUST/SHOULD.

**Local-first vs local-only distinction (documented, per the addendum):**
local-first = works offline, syncs when online, cloud assumed present.
**Local-only core (MELFINA's requirement)** = the core has no notion of a remote at
all; removing the network changes nothing about its ability to understand, reason,
remember, retrieve, plan, decide, teach, and act locally; no cloud component it
defers to or degrades toward. Any future network capability is a separate,
isolated, off-by-default add-on that can never form the "lethal trifecta" with
core data and is never a core dependency (MEL-REQ-166, 167).

**Dynamic self-directed system (PART IV-B):** the 15 capabilities are a vocabulary,
not a ceiling; MELFINA reasons per-situation about how deep to reason, how fast,
what context, which skills, whether existing skills suffice, whether to compose,
whether to build a new capability, whether to experiment, whether to act at all.
Grounded in: rational metareasoning / value-of-computation (Russell & Wefald 1991),
adaptive test-time compute (demonstrated), anytime algorithms, adaptive/self-RAG
(dynamic context), skill libraries (Voyager) + LLM tool-making (CREATOR/LATM)
demonstrated in constrained settings, human-AI co-creativity frameworks, safe
dynamic software update, monotonic confinement / external guardrails, the Gödel
machine as the (intractable) safety ideal. Feasibility is mixed and stated per
requirement: adaptive depth/context/resource allocation = demonstrated; capability
creation = demonstrated only in games/benchmarks, open-world unproven; verified
self-generated code ≈ unsolved (→ safety via test+sandbox+rollback+human-gate);
tiers 6–9 self-modification never autonomous; recursive self-improvement toward
superintelligence explicitly not a goal.

**Confidence:** high for local-only / no-scorekeeping / predictability+autonomy /
the autonomy triad + least-authority security / external-structure-is-worth-
building / the dynamic-system *bounds* (grounding factors, the meta-invariant,
"dynamic ≠ unpredictable"). Moderate for metareasoning + dynamic context/resource
allocation, capability creation with gating, and self-modification tiers 1–5. Low
for the neutral-primitives life model (hypothesis), specific assistive-feature
efficacy for this adult, the emotional-understanding *capability*, and the
deep-reasoning/rediscovery aspiration (narrow formal case only). `[U]`: whether
the meta-invariant is *sufficient* against a highly capable self-modifier
(necessary, not proven sufficient — high tiers kept rare + human-driven + audited).
Unknowable now: whether MELFINA actually improves this user's life — only
REAL-WORLD USE answers that.

### RESEARCH MISSION 002 — summary of outcome (2026-09-10)

Deeper pass targeting Mission 001's evidence gaps. Output: `RESEARCH_MASTER.md`

### RESEARCH MISSION 002 — summary of outcome (2026-09-10)

Deeper pass targeting Mission 001's evidence gaps. Output: `RESEARCH_MASTER.md`
**§§20–32 + `RESEARCH CHECKPOINT 002`**; **Second-Pass Evidence Updates** in
`CONFLICTS.md`; **Second Pass Additions** in `BIBLIOGRAPHY.md`.

- **Evidence strengthened:** autism EF meta-analysis primary-verified (g = 0.48,
  235 studies; attenuates but persists into adulthood; informant-report tracks
  real-world difficulty better than lab tests); adult autism life outcomes (EF +
  daily-living skills predict independent living/employment/mental health;
  ~20% "good outcome"); autonomy-support upgraded to **[E]** (SDT meta-analyses);
  cognitive-offloading benefits (2025 meta-analysis); PIM literature (keeping/
  filing is the costly, abandonment-prone part; people prefer navigation+context
  over search for their own data); attention residue (Leroy 2009 — a *credible*
  completion path removes the load, same shape as Masicampo); notification
  batching (Fitz 2019 RCT); AI reassurance risk (IOCDF guidance + MIT–OpenAI
  preprint RCT); self-tracking as genuinely two-sided; reminder efficacy
  RCT-level only in memory-impaired neurological patients (NeuroPage).
- **Claims corrected (none disproven):** notification batching *can* help
  wellbeing (Pass 1 said no evidence); alert-fatigue magnitude was overstated
  (real: ~10% lower acceptance per +5pp repeat-share, Ancker 2017); autism EF
  "stable across lifespan" → attenuates somewhat in adulthood; AuDHD
  co-occurrence figures refined (ADHD-in-autism ≈ 22–34%); ADHD+OCD co-occurrence
  lower/more uncertain than implied, and neurofunctionally opposite; implementation
  intentions for **adults with ADHD** downgraded to **[U]** (child-only evidence).
- **Musician/pianist context (new):** deliberate-practice *amount* explains only
  ~21–26% of music-performance variance and is contested (Ericsson vs
  Macnamara/Hambrick — contradiction preserved); the evidenced leverage is
  **session structure, planning, reflective self-evaluation, and autonomy**, not
  hour-logging; perfectionistic **concerns** (not strivings) drive music
  performance anxiety and overlap with OCD constructs; autonomous motivation
  protects against practice dropout, controlled motivation predicts it.
- **New tier introduced:** **[U] — unknown / insufficient evidence.**
- **Contradictions preserved** (7 listed): deliberate practice; offloading
  long-term effect; mood/self-tracking; notification batching (smartphone vs
  email); "problematic AI use" as a construct; prospective memory in autism;
  adaptive UIs.
- **Largest evidence gap:** the specific **ADHD + autism + OCD combination** is
  unmeasured on every design-relevant variable; controlled assistive-tech
  efficacy in adults with these conditions is sparse to absent.

Confidence levels for the major conclusions are tabulated in `RESEARCH CHECKPOINT
002`. High confidence: EF/PM/time differences warrant external scaffolding; a
trusted capture/resurfacing system frees attention; predictability + autonomy
support as defaults. Low confidence: specific assistive features help adults;
foregrounding practice-hours for a musician; anything specific to the 3-way
combination.

### RESEARCH MISSION 001 — summary of outcome

- **Question:** how to design a personal computing system that reduces cognitive
  friction and supports daily functioning for a person with lived experience of
  ADHD + autism + OCD, without reinforcing harmful patterns. Not a medical
  project; no diagnosis; no treatment.
- **Method:** scoping literature synthesis (web search, Sept 2026), prioritising
  systematic reviews / meta-analyses / clinical & standards-body guidance, then
  peer-reviewed primary research and HCI/assistive-tech venues. Not a systematic
  review.
- **Files created:** `research/RESEARCH_MASTER.md` (19-section report),
  `research/CONFLICTS.md` (the 10 core design tensions), `research/BIBLIOGRAPHY.md`
  (references with provenance tags), `research/README.md` (updated index).

### Most important findings (see RESEARCH_MASTER §1)

1. Executive-function / working-memory / prospective-memory differences persist
   into adulthood (ADHD and autism, medium effects) — strong rationale for
   *external* structure. **[evidence]**
2. Externalising intentions works; a *specific written plan* removes the
   attention cost of an open loop even before the task is done (Masicampo &
   Baumeister 2011). **[evidence]**
3. ADHD time perception is measurably atypical — "externalise time." **[evidence
   for the deficit; clinical guidance for the remedy]**
4. Predictability reduces anxiety in autism; intolerance of uncertainty is
   central to both autism-anxiety and OCD. **[evidence]**
5. Checking, reassurance, logging, and "always-available AI" can become
   compulsions; repeated checking *worsens* memory confidence. **[evidence /
   clinical]**
6. Notifications degrade with volume; deliver at task boundaries; user controls
   interruption. **[evidence]**
7. Gamification effects are small and fragile; streaks convert intrinsic goals
   into loss-avoidance — contraindicated by default for this profile. **[evidence
   / inference]**
8. Users prefer *adaptable* (user-directed) over *adaptive* (system-directed)
   systems; autonomy-supportive framing reduces demand-avoidance. **[evidence /
   clinical]**
9. The defining challenge is **conflicting needs**: the same feature helpful for
   one profile can harm another (10 tensions catalogued in `CONFLICTS.md`). The
   research does not resolve these — they are decisions for the requirements
   phase, and most resolve to *user-set dimensions, not diagnostic presets*.
10. Local-first (Kleppmann 2019) is a coherent architecture philosophy aligned
    with the project's existing principles.

### Provisional research-informed principles (RESEARCH_MASTER §18)

Scaffold don't steer · user is the authority · predictable over clever · external
memory you can trust · time made visible · quiet by default · no scorekeeping ·
restraint at the compulsion surfaces · graceful degradation · personalisation
instead of diagnosis · AI optional and restrained · local/open/durable · neutral
primitives. **Provisional — subject to user review and later phases.**

### Evidence gaps / caveats (RESEARCH_MASTER §2, §17; BIBLIOGRAPHY "known gaps")

- Scoping synthesis, not systematic; English-language, US-indexed search.
- Several primary sources reached only as abstracts (paywalls).
- Most ADHD/autism cognitive research is on children; adult data thinner.
- Little/no controlled evidence for specific assistive features in *adults*
  (visual timers, body doubling, location reminders).
- Essentially no research on an *integrated* life-OS for this specific overlap.
- Long-term effects of heavy cognitive offloading on this population: unknown.
- Not yet searched: music-practice organisation / deliberate practice / flow;
  CSCW work on task managers as a genre; PKM longitudinal outcomes.

### Open questions carried forward (RESEARCH_MASTER §17)

Conflict-resolution design; offloading dependency; adult assistive-feature
efficacy; reminder design for reminder-fatigued users; AI reassurance safeguards;
whether any safe progress representation exists; the life-model primitives;
capacity-state adaptation; music mapping; what sustains long-term engagement.

### Next phase (as recommended at the close of Mission 002; since completed)

**PERSONAL REQUIREMENTS** — done (see the PERSONAL REQUIREMENTS MISSION 001 summary
above). A further (third) research pass remains possible for the `[U]` gaps but
was judged unnecessary to proceed.

### Pipeline

```
DEEP RESEARCH            <-- MISSION 001 + 002 done, pushed
  -> PERSONAL REQUIREMENTS   <-- MISSION 001 (+ correction) done, pushed;
                                REQUIREMENTS EXPANSION MISSION 002 (PART IV-C,
                                general reasoning/knowledge/wisdom) done,
                                committed + pushed (`4f5e97c`)
  -> HUMAN / CENTRAL MODEL   <-- MISSION 001 done, pushed (f3093fc)
  -> SYSTEM DESIGN /         <-- MISSION 001 + ADVERSARIAL REVIEW 001 + REVISION 1
     ARCHITECTURE                done, committed + pushed (`8a40207`)
                                (design/ = 11 files)
  -> LOW-LEVEL FOUNDATIONS   <-- MISSION 001 done, committed + pushed
                                (`19d0135`). design/foundations/ = README +
                                12 technology-independent contracts
  -> TECHNOLOGY SELECTION    <-- MISSION 001 done (1st pass), NOT committed;
                                awaiting review. design/technology/ = README +
                                synthesis + 10 category evaluations +
                                adversarial review + experiment plan +
                                decision log. Recommends, does not mandate;
                                12 decisions flagged for explicit human
                                sign-off (see TECHNOLOGY_SELECTION.md §14)
  -> CORE ENGINE
  -> STORAGE
  -> INTERFACE
  -> AI / ASSISTANT LAYER
  -> AUTOMATION
  -> TESTING / EVALUATION
  -> REAL-WORLD USE
  -> ITERATION
```

Each stage produces written artifacts before the next begins. The directory
layout in section 6 mirrors this pipeline.

## 3. Principles

These constrain every decision. If a choice violates one, it needs an explicit,
recorded justification in `decisions/`.

- **Local-first, privacy-first.** Data lives on the user's machine. The system
  works fully offline.
- **Minimal resource usage.** Small memory and CPU footprint is a feature.
- **High reliability.** Predictable, durable, hard to corrupt. Data outlives the
  software.
- **Excellent functionality.** Minimalism is not an excuse for a weak tool.
- **Long-term maintainability.** One person should be able to hold the whole
  system in their head.
- **Strong user control.** The user can inspect, export, and override everything.
- **No unnecessary cloud dependency.**
- **No unnecessary frameworks or abstraction layers.**
- **Low-level implementation where technically justified** — not everywhere for
  its own sake. C / C++ are the eventual core implementation candidates.
- **CLI / native interfaces before heavy web abstractions.**
- **Extensibility without architectural bloat.**
- **Understand before assembling.** The user wants to understand the system
  fundamentally, not wire libraries together blindly.

## 4. Domain note: music is not a special case (yet)

The user is a classical pianist. Practice sessions, repertoire, musical ideas,
creative projects, and performances must eventually fit **naturally** into the
same underlying life model as everything else.

Do **not** design a bespoke "music feature." First determine the general
abstractions (e.g. what a "session", an "artifact", a "commitment", a "thread of
work" is). Music should fall out of those as a case, not bolt on beside them.

## 5. Working rules for the agent

- Explain briefly what is being done and why.
- Keep every change small and reversible.
- Distinguish **facts**, **recommendations**, and **assumptions** explicitly.
- Preserve a clear checkpoint of project state (this file + git history).
- Ask before any irreversible change.
- Do not install large frameworks or dependencies.
- Do not generate documentation that no one asked for.
- Do not choose a final architecture.
- Stop when the current task is done; do not run ahead into the next phase.

## 6. Repository layout

| Path            | Holds                                                        |
|-----------------|-------------------------------------------------------------|
| `research/`     | Deep-research findings (Missions 001 + 002). Complete for now. |
| `requirements/` | Requirements spec (PERSONAL REQUIREMENTS MISSION 001 + REQUIREMENTS EXPANSION MISSION 002). Both committed + pushed (`4f5e97c`). |
| `model/`        | Human / central life model (HUMAN / CENTRAL MODEL MISSION 001). First pass done, pushed. |
| `design/`       | System architecture (ARCHITECTURE MISSION 001 + ADVERSARIAL REVIEW 001 + REVISION 1). 11 files, committed + pushed (`8a40207`). |
| `design/foundations/` | Technology-independent low-level contracts (LOW-LEVEL FOUNDATIONS MISSION 001). README + 12 contracts, committed + pushed (`19d0135`). |
| `design/technology/` | Technology recommendations (TECHNOLOGY SELECTION MISSION 001). README + synthesis + 10 category evaluations + adversarial review + experiment plan + decision log, first pass done, **not yet committed**. |
| `decisions/`    | Dated, lightweight decision records (one file per decision).|
| `experiments/`  | Throwaway probes and spikes. Never the real system.         |
| `src/`          | The eventual implementation. Placeholder only for now.      |

## 7. Environment snapshot (2026-09-09)

Facts, for reference. Not commitments.

- Ubuntu 24.04, kernel 6.14, x86_64, 12 cores, 16 GB RAM.
- Disk `/`: ~4 GB free at setup time — **low; clear headroom before building.**
- Present: gcc 14.3, g++ 14.3, clang 18.1.3, make, cmake 4.4.2, ninja,
  pkg-config, gdb, git 2.43, python 3.12 (+venv), node 20, rustc/cargo, go,
  jq, ripgrep, tmux.
- Not installed (not needed yet): sqlite3 CLI, valgrind, lldb, clang-format,
  clang-tidy, cppcheck, ctags, pandoc.

## 8. Open decisions (not yet made)

Deferred by design. Requirements are implementation-independent; these are for
SYSTEM DESIGN / ARCHITECTURE and later, and only after HUMAN / CENTRAL MODEL.

- The neutral primitive set / life model (`requirements/OPEN_QUESTIONS.md` OQ-1).
- Core language(s) and their boundaries.
- Storage substrate (flat files, embedded DB, custom format, ...).
- Interface medium(s) — requirements say only "multi-modal, user-choosable,
  consistent with the CLI-first leaning".
- Which local reasoning components / model sizes (OQ-11).
- The permission-grant mechanism (object-capability *style* is required by
  `MEL-REQ-180`; the mechanism is not chosen).
- The default consequential/routine action boundary (OQ-12).
- Whether an isolated network add-on is ever built (OQ-11; `MEL-REQ-167`).
- Whether any optional progress representation ships (OQ-6).
- Licensing (deferred).

The AI/assistant layer's *role* is now specified at requirements level (Parts II,
IV, V of `REQUIREMENTS_MASTER.md`): under the permission model, not owner; the
autonomy triad; optional; local reasoning for the core. *How* it is built is not
decided.

## 9. Checkpoints

- **2026-09-09** — Repository initialised. Branch `main`. Research-first scaffold
  created: directory structure, this file, README, `.gitignore`, `.editorconfig`.
  No dependencies. No code. Awaiting research mission.
- **2026-09-09** — Project renamed **Personal OS → MELFINA**.
- **2026-09-09** — **RESEARCH MISSION 001 complete (first pass).** Created
  `research/RESEARCH_MASTER.md`, `research/CONFLICTS.md`, `research/BIBLIOGRAPHY.md`;
  updated `research/README.md`. Scoping literature synthesis on designing for the
  ADHD + autism + OCD overlap. No requirements, no design, no architecture, no
  dependencies, no code. `src/` untouched.
- **2026-09-09** — Repo connected to GitHub remote
  `https://github.com/Levitationist/melfina` as `origin`; history pushed to
  `origin/main`. Permanent MELFINA git/GitHub workflow rule adopted (task → work →
  verify → update state → checkpoint → review diff → commit → push → verify clean
  tree). Mission 001 accepted by the user as a checkpoint.
- **2026-09-10** — **RESEARCH MISSION 002 complete (second, deeper pass).**
  Appended `RESEARCH_MASTER.md` §§20–32 + `RESEARCH CHECKPOINT 002`; added
  Second-Pass sections to `CONFLICTS.md` and `BIBLIOGRAPHY.md`; updated
  `research/README.md`. Adult-specific evidence, the 3-way overlap, musician/
  pianist context, CSCW/PIM, PKM/offloading trade-offs, controlled adult
  assistive-tech evidence, notification/reminder evidence, AI compulsion risk,
  self-tracking, academic prior art, and an evidence-quality audit of Mission 001
  (2 claims corrected, several refined, 1 partly downgraded to `[U]`; none
  disproven). New tier `[U]`. Contradictions preserved. No requirements, no
  design, no architecture, no language/storage choice, no dependencies, no code.
  `src/` untouched.
- **2026-09-10** — Research Missions 001 + 002 accepted as a checkpoint by the
  user and pushed (`d3160f0` on `origin/main`).
- **2026-09-10** — **PERSONAL REQUIREMENTS MISSION 001 complete (first pass).**
  Created `requirements/REQUIREMENTS_MASTER.md` (203 requirements `MEL-REQ-001…203`
  + 16 anti-requirements `MEL-AR-01…16` + MELFINA Capability Model + autonomy triad
  + action pipeline + feasibility classification + `REQUIREMENTS CHECKPOINT 001`),
  `requirements/CONFLICTS.md` (RC1–RC10 + VC1–VC7 as configurable dimensions /
  constraints / safeguards / open questions), `requirements/OPEN_QUESTIONS.md`
  (OQ-1…OQ-15 + deliberately-undecided list); updated `requirements/README.md` and
  this file. Includes a §18 deep research pass (~40 sources on autonomous agents,
  cognitive architectures, computer-use/coding agents, capability-based security,
  corrigibility, adjustable autonomy, ITS/learning science, local AI, continual
  learning, affective computing, computational creativity, automated discovery,
  agent memory, deskilling, XAI, prompt-injection/lethal-trifecta, BDI, abstention,
  self-improvement). **No architecture. No language (C/C++/Rust/…). No storage
  (SQLite/…). No framework, model, or UI toolkit. No dependencies. No code. `src/`
  untouched.**
- **2026-09-10** — **PERSONAL REQUIREMENTS MISSION 001 — dynamic-self-directed
  correction.** Reframed the capability model as a *vocabulary, not a fixed
  catalogue* (`MEL-REQ-016`); added **PART IV-B — THE DYNAMIC SELF-DIRECTED
  SYSTEM** (`MEL-REQ-204…253`): metareasoning / dynamic strategy selection;
  dynamic context selection as active reasoning; dynamic skill selection +
  capability-gap recognition + capability creation/evolution; human–MELFINA
  co-creation; a graded **nine-tier self-modification model** with **THE
  META-INVARIANT** (`MEL-REQ-235` — MELFINA cannot redefine the rules governing
  its own self-modification; external, monotonic, versioned, audited); dynamic
  resource allocation + dynamic autonomy; dynamic self-evaluation; "best is
  situational, dynamic ≠ unpredictable". Added anti-requirements `MEL-AR-17…19`,
  conflicts `VC8…VC12`, open questions `OQ-16…OQ-21`; extended the feasibility
  classification and `REQUIREMENTS CHECKPOINT 001`. Total now **253 requirements +
  19 anti-requirements**. Includes a dedicated research pass (~28 sources: rational
  metareasoning, adaptive test-time compute, anytime algorithms, LLM metacognition,
  skill-library learning, LLM tool-making, HTN planning, automatic curriculum,
  program synthesis / verified codegen, Gödel machine / Darwin Gödel Machine, safe
  dynamic software update, computational reflection, human-AI co-creativity,
  self-generated workflow orchestration, adaptive/self-RAG, self-improving-agent
  evaluation, monotonic confinement / external guardrails). **Still no
  architecture, no language, no storage, no framework/model/UI, no dependencies,
  no code. `src/` untouched.**
- **2026-09-10** — Research Missions 001+002 and PERSONAL REQUIREMENTS MISSION 001
  (+ correction) accepted as a checkpoint by the user and pushed (`120567a` on
  `origin/main`).
- **2026-09-10** — **HUMAN / CENTRAL MODEL MISSION 001 complete (first pass).**
  Created `model/HUMAN_CENTRAL_MODEL.md` (the model, 18 sections + `MODEL
  CHECKPOINT 001`), `model/MODEL_ALTERNATIVES.md` (4 candidate models compared —
  A event-centric · B entity/relationship-centric · C state/transition-centric ·
  D three-primitive — concluding a synthesis is necessary), `model/
  MODEL_OPEN_QUESTIONS.md` (`OQ-M1…OQ-M12`); updated `model/README.md` and this
  file. **Leading model E²CI: four primitives — ENTITY · EVENT · CLAIM ·
  INTENTION — in a bitemporal TIME dimension, with CONTEXT derived per situation
  (relevance theory), never stored.** All application categories (task / habit /
  note / calendar / project / goal / reminder / journal / dashboard / practice
  session / experiment / decision / hypothesis / capability / automation /
  strategy) emerge as *views or patterns*, not primitives. Grounded in BFO
  continuant/occurrent, event calculus, W3C PROV, epistemic/doxastic logic, AGM
  belief revision, bitemporal modelling, Anscombe/Searle direction of fit,
  Castelfranchi social commitment, activity theory, Sperber & Wilson relevance
  theory, Allen's interval algebra, Conway's self-memory system (grounding pass,
  ~15 sources). Music needs **zero** music-specific primitives; the agency
  pipeline and the dynamic self-directed MELFINA are both supported without
  freezing today's agent tech. The model deliberately does **not** contain the
  self-modification meta-invariant rules, any enforcement mechanism, any
  evaluative verdict, or any diagnostic category. Top open questions: is `State` a
  fifth primitive (OQ-M2); is `Relation` (OQ-M1); is four genuinely
  minimum-sufficient (OQ-M8); how are worries/affect represented (OQ-M5).
  `requirements/` and `research/` **not edited**. **No architecture, no
  technology, no schema, no format, no code. `src/` untouched.**
- **2026-09-10** — Research + Requirements + Human/Central Model accepted as a
  checkpoint by the user and pushed (`f3093fc` on `origin/main`).
- **2026-09-10** — **SYSTEM DESIGN / ARCHITECTURE MISSION 001 complete (first
  pass, NOT yet committed).** Created `design/` (10 files):
  `ARCHITECTURAL_PRINCIPLES.md` (INV-1…14 → AP-1…14 + the mechanism/policy/data/
  capability/governance/derived-view vocabulary + deferred-technology list),
  `SYSTEM_ARCHITECTURE.md` (the five-ring model + subsystem map + all flows +
  Diagrams 1–3 & 6 + key decisions AD-1…15 + risks + "what must NOT be
  implemented yet"), `ARCHITECTURAL_ALTERNATIVES.md` (AA-1…9), `RUNTIME_MODEL.md`,
  `DATA_AND_STATE_MODEL.md`, `AUTHORITY_AND_SECURITY_MODEL.md` (incl. §9 documented
  residual limits), `CAPABILITY_MODEL.md`, `FAILURE_AND_RECOVERY.md` (Diagram 7),
  `TRACEABILITY.md`, `README.md`; updated this file.
  **Architecture:** *a small governed core over an append-only claim/event
  substrate, with reasoning and capabilities as optional isolated permissioned
  outer rings, coordinated by a metareasoning controller that is a control loop,
  not an agent loop.* Five inward-only rings (Governance · Core Mechanism ·
  Reasoning · Capabilities · future Network); E²CI maps directly onto the
  Chronicle (append-only Events/Claims/Intentions = the single source of truth;
  Context + state = rebuildable projections); THINK→DECIDE→PROPOSE→AUTHORISE→
  EXECUTE→VERIFY is a state machine, the autonomy triad structurally separated;
  the self-modification meta-invariant is structural (Ring 0 has no write path;
  tiers 6–9 are a human path, not a MELFINA operation) **and documented as
  necessary-not-sufficient**. Grounding pass (~10 sources): reference monitor /
  Hydra, capability-based OS (seL4/Fuchsia/WASI), event sourcing + CQRS,
  blackboard architecture, microkernel vs modular monolith, Erlang/OTP
  supervision. **No language, storage engine, framework, model, or UI chosen or
  implied — all deferred to the named later phases.** `src/`, `research/`,
  `requirements/`, `model/` **untouched** (no upstream inconsistency found).
  Not committed or pushed (per the mission). Awaiting user review and explicit
  acceptance before LOW-LEVEL FOUNDATIONS.
- **2026-09-10** — **ARCHITECTURE ADVERSARIAL REVIEW 001 complete (first pass).**
  Created `design/ARCHITECTURE_ADVERSARIAL_REVIEW.md` (19 sections). Attacked the
  architecture across all 10 focus areas. **Verdict: the architecture survives —
  0 CRITICAL, 6 HIGH, 11 MEDIUM, 6 LOW; no redesign required.** The 6 HIGH
  findings each get a **minimum correction that strengthens an existing boundary**
  (RC-1…RC-6), plus a required framing fix (RC-7, "subsystems are responsibilities
  not components"):
  - **H1/RC-1:** Ring 0 governance files need cryptographic signing + a hard File
    Access path exclusion (runtime write path is closed; at-rest integrity was
    only version-chain-checked, and the chain could be forged).
  - **H2/RC-2:** aggregate-effect blindness — the routine/reversible class and
    capability composition are governed per-action, not by aggregate effect; a
    sequence of routine actions, or a composition of trivial capabilities, can be
    consequential in aggregate.
  - **H3/RC-3:** VERIFY is a single point of trust — a buggy/compromised Verifier
    undermines the pipeline; needs a Verifier trust spec + ≥2 independent
    Verifiers for high-risk actions.
  - **H4/RC-4:** terminal/GUI grant granularity + action-parsing is
    containment-critical (shell composition defeats "exactly these commands"); AU-2
    must be re-tagged as such; grants checked against parsed argv, not raw strings.
  - **H5/RC-5:** mid-execution permission revocation is unspecified.
  - **H6/RC-6:** the Reasoner Interface's locality is stated as a preference, not a
    structure (contradicts INV-1) — Ring 1/2 have no network capability, so it
    *cannot* bind a remote endpoint; say "cannot", not "does not".
  Optional simplifications (O1 fewer projections · O2 collapse Ring 2/3 into one
  sandboxed region · O3 soften "blackboard" · O4 flat supervisor · O5 move the
  routine/consequential boundary to Ring 0 · O6 projection consistency audit) —
  recommended, not required. **E²CI faithfully preserved; the architecture is
  genuinely dynamic (substance deferred to CORE ENGINE); local-only holds; the
  meta-invariant holds structurally after RC-1, with its residual limits intact
  and honest.** No model open question resolved; no requirement weakened. `src/`,
  `research/`, `requirements/`, `model/` untouched. **Not committed, not pushed.**
  Recommended: a short `ARCHITECTURE MISSION 001 — REVISION 1` applying RC-1…RC-7
  (a targeted edit pass, not a rewrite), then commit the whole `design/` set as
  one checkpoint, then LOW-LEVEL FOUNDATIONS.
- **2026-09-10** — **ARCHITECTURE MISSION 001 — REVISION 1 complete (first pass,
  NOT yet committed).** Targeted edit pass — not a redesign, no `src/` — applying
  the adversarial review's corrections into the architecture documents:
  **RC-1** Ring 0 at-rest cryptographic signing (human-held key, verified every
  startup, invalid ⇒ refuse to run; Ring 0 path excluded from every File Access
  grant); **RC-2** aggregate-effect governance (per-activity-chain aggregate
  budget; composition classified by the union of component authority + scope +
  aggregate; no laundering a high-risk action into low-risk steps); **RC-3**
  verifier trust model (new `AUTHORITY_AND_SECURITY_MODEL.md` §4.1 — deterministic
  where practical, minimal / no ambient authority, read-only, cannot cause
  effects, independently testable, not a second agent; ≥2 architecturally-
  independent verifiers for high-risk, explicit single-verifier fallback
  otherwise); **RC-4** AU-2 re-tagged CONTAINMENT-CRITICAL and made the first
  LOW-LEVEL FOUNDATIONS deliverable (terminal authority = structured actions, not
  raw shell strings; grants matched against parsed argv; shell interposition a
  distinct higher-risk capability); **RC-5** grants are live revocable handles
  the Reference Monitor can invalidate mid-execution, with the outcome vocabulary
  completed / partially completed / rolled back / failed / interrupted; **RC-6**
  reasoner locality made structural (Ring 1/2 hold no network capability — the
  Reasoner Interface *cannot* bind a remote endpoint); **RC-7** the ~25 named
  subsystems stated as responsibilities, not necessarily processes/IPC
  boundaries — only Ring 0 ↔ all, Ring 1 ↔ sandbox, reasoning ↔ effect must be
  real enforced boundaries. Small corrections M1/M2/M3/M5/M6/M8/M9/M10/L4/L5
  integrated. Optional: O1 adopted (one Chronicle + one current-state cache +
  on-demand views), **O2 rejected** (do not collapse Ring 2 / Ring 3), O3 adopted
  (blackboard = one candidate impl), O4 adopted (supervisor topology not
  specified now), O5 preserved as an explicit design question, O6 retained as a
  future implementation concern. `TRACEABILITY.md` gained §2.1 (Revision 1
  corrections → sources) + AD-14/AD-15 + updated §3–§6 rows. The adversarial
  review's findings §2–§19 were **not deleted or rewritten** — a "Revision 1 —
  resolution status" section was added before §1. **No technology chosen** (no
  crypto scheme, no parser, no isolation mechanism, no storage, no language); **no
  model open question resolved** (OQ-M1/OQ-M2/OQ-M5 untouched). `src/`,
  `research/`, `requirements/`, `model/` untouched. **Not committed, not pushed.**
  Recommended next: commit the whole `design/` set (architecture + adversarial
  review + revision) as one checkpoint, push, then LOW-LEVEL FOUNDATIONS.
- **2026-09-11** — **ARCHITECTURE MISSION 001 — REVISION 1 resume-completed +
  validated (still NOT committed, NOT pushed).** The 2026-09-10 edit pass was
  interrupted by a host restart with almost all of the integration written. This
  session did **no redesign and no code** — it verified the present state and
  finished Revision 1:
  · **Cross-document validation.** Read all 11 `design/` docs end to end.
  Confirmed RC-1…RC-7 and M1/M2/M3/M5/M6/M8/M9/M10 + L4/L5 are integrated in every
  document the resolution table names; the RC-5 outcome vocabulary
  (completed / partially completed / rolled back / failed / interrupted) is
  identical everywhere; the "three must-be-real boundaries" statement is
  consistent; E²CI is preserved (Event/Claim/Intention = Chronicle units, Entity
  = registry, Time = bitemporal, Context = on-demand, State = current-state
  cache); the local-only invariant is structural (RC-6 + L4 + Ring 4 outside the
  core); no deferred technology is chosen; OQ-M1/OQ-M2/OQ-M5 and the other §18
  "keep unresolved" items are still open; `src/` and the upstream dirs are
  untouched.
  · **Three residual propagation gaps closed** in `SYSTEM_ARCHITECTURE.md`: §5
  startup now names the RC-1 at-rest signature check (it previously listed only
  the version chain, unlike `RUNTIME_MODEL.md` §2 / `AUTHORITY_AND_SECURITY_MODEL.md`
  §6); §6.1's Event-flow box no longer lists a maintained "context cache" (M1 —
  context is a Ring-2 on-demand build); a duplicated phrase in §6.2 was fixed.
  · **One RC-7 wording variance harmonised** in `ARCHITECTURAL_ALTERNATIVES.md`
  AA-1 (the middle must-be-real boundary now reads "Ring 2 + Ring 3", matching
  `ARCHITECTURAL_PRINCIPLES.md` AP-12 and `SYSTEM_ARCHITECTURE.md` §2/§20).
  · `design/TRACEABILITY.md` §2.1 RC-1 row gained "§5 (startup)"; the review's
  "Revision 1 — resolution status" section gained a dated resume note (no finding
  touched).
  Files changed this session: `design/SYSTEM_ARCHITECTURE.md`,
  `design/ARCHITECTURAL_ALTERNATIVES.md`, `design/TRACEABILITY.md`,
  `design/ARCHITECTURE_ADVERSARIAL_REVIEW.md` (resolution-status note only), and
  this file. **Verdict: Revision 1 complete and internally consistent; awaiting
  user review before the whole `design/` set is committed as one checkpoint,
  then LOW-LEVEL FOUNDATIONS (first deliverable = the RC-4 parsed-action grant
  model).**
- **2026-09-11** — **ARCHITECTURE CHECKPOINT committed + pushed (`8a40207`).**
  `git status` / `git diff` inspected; all 11 `design/*.md` + `PROJECT_STATE.md`
  reviewed; `src/`, `research/`, `requirements/`, `model/`, `decisions/`,
  `experiments/` verified untouched; no secrets / API keys / passwords / tokens /
  PATs / sensitive PII. Staged only the intended files. Commit message exactly
  `design: ARCHITECTURE MISSION 001 + adversarial review 001 + revision 1`.
  Pushed `git push origin main`; local `HEAD` == `origin/main` == `8a40207`;
  clean tree. Next phase: LOW-LEVEL FOUNDATIONS.
- **2026-09-11** — **LOW-LEVEL FOUNDATIONS MISSION 001 complete (first pass, NOT
  committed, NOT pushed).** Still design/research. Created `design/foundations/`:
  `README.md` + 12 technology-independent contracts (see §2 for the per-contract
  summary): `CAPABILITY_GRANT_MODEL.md`, `STRUCTURED_ACTION_MODEL.md`
  (CONTAINMENT-CRITICAL — structured actions, never raw shell strings),
  `REFERENCE_MONITOR_CONTRACT.md`, `REVOCATION_MODEL.md`,
  `CHRONICLE_LOGICAL_FORMAT.md`, `CHRONICLE_CONTRACT.md`, `ISOLATION_CONTRACT.md`,
  `GOVERNANCE_FORMAT.md`, `GOVERNANCE_INTEGRITY.md`, `VERIFIER_CONTRACT.md`,
  `FOUNDATION_CROSS_CONTRACT_ANALYSIS.md` (9 path traces A–I + a consistency
  matrix — no contradictions found), `TECHNOLOGY_SELECTION_CRITERIA.md` (chooses
  nothing). Each contract answers the 10 questions (guarantees / requires /
  trusts / distrusts / enters / leaves / malformed input / failure / authoritative
  / independently verifiable) and tags every MUST-level invariant **[SEN]** /
  **[ID]** / **[ETL]** / **[OPEN]**. Focused research pass (capability security,
  reference monitors, object-capability, secure command invocation, revocable
  capabilities, event sourcing / append-only durability, bitemporal data, crash
  consistency, sandbox isolation / seL4 / WASI, N-version-programming independence
  failure, governance / integrity transparency chains — CT / TUF / Sigstore) —
  principles + failure modes extracted, not cargo-culted. **No language, storage,
  sandbox mechanism, cryptographic primitive, IPC model, GUI toolkit, or local
  model chosen. No API / schema / wire format / grant syntax designed. No parser
  or executable artefact written. OQ-M1 / OQ-M2 / OQ-M3 / OQ-M5 / OQ-M9 / OQ-M11
  preserved with both alternatives. No requirement weakened. `src/`, `research/`,
  `requirements/`, `model/`, and the committed `design/*.md` architecture set
  untouched — no upstream contradiction found.** Awaiting user review before the
  `design/foundations/` set is committed as one checkpoint, then TECHNOLOGY
  SELECTION.
- **2026-09-11** — **LOW-LEVEL FOUNDATIONS MISSION 001 committed + pushed
  (`19d0135`, "design: LOW-LEVEL FOUNDATIONS MISSION 001").** `origin/main`
  advanced from `8a40207` to `19d0135`. Next phase: TECHNOLOGY SELECTION, or
  (as it turned out) a further requirements pass first — see below.
- **2026-09-11** — **REQUIREMENTS EXPANSION MISSION 002 complete (first pass,
  NOT committed, NOT pushed).** Added **PART IV-C — GENERAL REASONING,
  KNOWLEDGE, AND WISDOM** to `requirements/REQUIREMENTS_MASTER.md` (§56–76,
  `MEL-REQ-254`…`MEL-REQ-364`, 111 new requirements: 49 MUST, 28 SHOULD, 34 MUST
  NOT) + 8 new anti-requirements (`MEL-AR-20`…`27`) + `REQUIREMENTS CHECKPOINT
  002`. New companion file `requirements/KNOWLEDGE_AND_REASONING_MODEL.md` (the
  "wisdom" operational definition + honest critique of its own limits; the
  closed analogy/correlation/causation/speculation taxonomy grounded in
  Gentner's structure-mapping theory and Pearl's association/intervention/
  counterfactual causal ladder; the first-principles decomposition pipeline
  with explicit stopping criteria; a 20-item architecture/model/foundation
  compatibility test — all ✓; a dedicated adversarial review of 20 named
  attack scenarios, each closed by restating an existing F1–F10 invariant, none
  requiring a new authority mechanism; and per-area research grounding for the
  mission's 26 research areas with honest `[E]/[G]/[DI]/[H]/[U]` tags,
  including where the literature does *not* justify a strong requirement).
  Updated `requirements/CONFLICTS.md` (+VC13–VC18) and
  `requirements/OPEN_QUESTIONS.md` (+OQ-22–32, +2 deliberately-undecided items)
  and `requirements/README.md`. **Totals: 364 requirements (186 MUST / 116
  SHOULD / 8 MAY / 54 MUST NOT) + 27 anti-requirements.**
  Strengthens genuine internal THINK/DECIDE ownership (§56) **without**
  touching the autonomy triad or the action pipeline; adds cross-domain
  foundational knowledge, first-principles/mathematical reasoning,
  philosophical programming, extended scientific reasoning/teaching/knowledge-
  acquisition, and structured (non-scoring) ethics/law/civics and embodied-
  skill/physiology reasoning — all representable inside the existing E²CI
  ontology, all still local-only, all still fully gated by the existing
  capability-grant/Reference-Monitor/verifier contracts. The two load-bearing
  new sections, **§74 (knowledge ≠ authority)** and **§75–76 (intelligence
  must not become a security bypass)**, contain no new authority mechanism —
  every one of their 18 requirements restates an existing F1/F3/F7/F8/F9/F10
  invariant or an existing MEL-REQ/MEL-AR against a specific reasoning-derived
  route around it. One residual tension was **not** closable by a requirement
  and is recorded, not resolved: `OQ-32` — whether building MELFINA toward
  first-principles reasoning about its own governance changes the answer to
  `OQ-19` (meta-invariant sufficiency) or only makes the question more urgent.
  Verified: no duplicate/gapped `MEL-REQ`/`MEL-AR` IDs (`MEL-REQ-001`…`364` and
  `MEL-AR-01`…`27`, each defined exactly once); `src/` untouched; no secrets or
  sensitive PII added; `model/`, `research/`, and every committed `design/` and
  `design/foundations/` document untouched — no contradiction requiring human
  review was found. **Not committed, not pushed.** Recommended: the user
  reviews this expansion (particularly §74–76 and VC13–VC18/OQ-22–32), then it
  is committed as its own checkpoint, then TECHNOLOGY SELECTION proceeds using
  `design/foundations/TECHNOLOGY_SELECTION_CRITERIA.md` as planned.
- **2026-09-11/12** — **REQUIREMENTS EXPANSION MISSION 002 committed + pushed
  (`4f5e97c`, "requirements: REQUIREMENTS EXPANSION MISSION 002").**
  `origin/main` advanced from `19d0135` to `4f5e97c`. Next phase: TECHNOLOGY
  SELECTION, as recommended.
- **2026-09-11/12** — **TECHNOLOGY SELECTION MISSION 001 complete (first
  pass, NOT committed, NOT pushed).** This session ran across an interruption
  and a resume; the resume instructions briefly conflated this mission's own
  scope with a validation claim ("VC1–VC18 clean") that actually belonged to
  the prior, already-committed REQUIREMENTS EXPANSION MISSION 002 — that
  claim was independently re-checked here (still accurate for `4f5e97c`, and
  explicitly not restated as this mission's own artifact,
  `design/technology/TECHNOLOGY_SELECTION.md` opening note). Determined which
  concrete technologies satisfy the already-committed requirements,
  architecture, and foundation contracts. Created `design/technology/`:
  `README.md`, `TECHNOLOGY_SELECTION.md` (the synthesis), and ten category
  evaluations — `LANGUAGE_EVALUATION.md` (Rust primary; Zig recorded as the
  fallback pending 1.0; single-language core), `CHRONICLE_EVALUATION.md` (a
  custom append-only framed log as the authoritative store + SQLite/WAL as
  the disposable, rebuildable current-state cache), `ISOLATION_EVALUATION.md`
  (Landlock + seccomp-bpf + namespaces + cgroups v2 as the default tier; a
  Firecracker-class microVM as the escalated tier for governance-flagged
  high-risk capability classes), `PROCESS_AND_IPC_EVALUATION.md` (the
  Reference Monitor and the reasoning process each as their own OS process —
  realising RC-7's three must-be-real boundaries as actual process
  boundaries; Unix domain sockets + `SCM_RIGHTS` for IPC, which gives
  OS-resource-shaped grants a kernel-enforced unforgeability property — a
  concrete finding for F1 §19's previously open grant-representation
  question), `TERMINAL_GUI_EVALUATION.md` (`execve`-only invocation +
  `openat2` TOCTOU defence; AT-SPI2 for structured GUI targeting with a
  disclosed Wayland coordinate-fallback gap), `CRYPTO_GOVERNANCE_EVALUATION.md`
  (Ed25519 + SHA-256, an offline signing key, a TPM 2.0 monotonic counter for
  rollback protection where available), `REASONING_COMPUTATION_EVALUATION.md`
  (verifiers as ordinary sandboxed processes with deliberately diverse
  implementations; SymPy/SciPy for math; `llama.cpp`/GGUF for local
  inference, model selection deferred; a two-lane WASI/native capability
  packaging model), `BUILD_AND_SUPPLY_CHAIN.md`, `MIGRATION_PORTABILITY.md`
  (portable state vs. non-portable authority strictly separated; the TPM
  counter explicitly does not transfer automatically). Plus
  `TECHNOLOGY_ADVERSARIAL_REVIEW.md` (36 named attacks against the specific
  technologies chosen — verdict: no attack defeats a foundation-contract
  invariant; one genuinely new gap surfaced and closed, debug/introspection
  interfaces must be gated like any other effect and disabled by default in
  non-development builds), `TECHNOLOGY_EXPERIMENT_PLAN.md` (13 prioritised
  pre-trust experiments, headed by crash-injection testing of the new
  Chronicle log, plus formal-verification opportunities), and
  `TECHNOLOGY_DECISION_LOG.md` (an auditable index of all 28 recommendations
  with alternatives, reversal cost, and confidence). **Verified:**
  compatibility against all of F1–F10 with no invariant weakened; the
  offline-core test; the AI-boundary test (zero ambient authority regardless
  of model behaviour); the dynamic-capability/self-modification boundary
  ("more computational power never becomes more authority", concretely true
  here because authority is gated by grant-possession and effect-class
  membership, neither a function of compute power); no hard MUST/MUST-NOT
  gate was rescued by a SHOULD-tier advantage anywhere. **No implementation
  performed, no dependency installed, `src/` untouched.** 12 decisions
  flagged for explicit human sign-off (language, Chronicle substrate,
  isolation, the Monitor's process boundary, governance integrity, the AI
  runtime boundary); everything else offered as a reviewable, non-blocking
  recommendation (`TECHNOLOGY_SELECTION.md` §14). **Not committed, not
  pushed.** Recommended: human review of the flagged decisions, then this
  checkpoint is committed, then CORE ENGINE begins with the experiment
  plan's priority-1 item (crash-injection testing of the Chronicle log).
