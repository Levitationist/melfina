# MELFINA — ARCHITECTURAL ALTERNATIVES

**Phase:** SYSTEM DESIGN / ARCHITECTURE — MISSION 001. First pass, pending review.
**Revision 1 applied** (targeted adversarial-review corrections — see
`ARCHITECTURE_ADVERSARIAL_REVIEW.md` "Revision 1 — resolution status").
Baseline `f3093fc`. Read `ARCHITECTURAL_PRINCIPLES.md` and `SYSTEM_ARCHITECTURE.md`
first.

For each significant architectural choice: candidates, advantages, disadvantages,
implications for MELFINA, failure modes, recommendation, confidence.
**No implementation technology is chosen** — these are *structural* choices.
Uncertainty is preserved, not papered over. `[E]` established · `[DI]` inference ·
`[H]` hypothesis · `[U]` unknown.

---

## AA-1 — Overall structure: how are subsystems separated?

**Candidate A — Monolithic process.** One address space; subsystems are modules;
calls are function calls.
- **+** simplest; lowest overhead; easiest for one maintainer to build first;
  fastest.
- **−** a compromised capability or a runaway reasoner is *inside* the trust
  boundary; no structural containment; a crash can take the whole thing down.
- **MELFINA implication:** violates AP-8 (structural containment), AP-11 (failure
  isolation), AP-2 (reasoning cannot act — hard to enforce in-process), INV-8
  (assume injection wins).
- **Failure modes:** one prompt-injected reasoning call reads private data and
  invokes a destructive capability in the same process; one infinite loop hangs
  everything.
- **Verdict: rejected.**

**Candidate B — Full microkernel: every subsystem an isolated component, all
communication via message passing / IPC.**
- **+** maximal isolation; every boundary is enforced; matches seL4/Fuchsia
  precedent (`[E]`); a compromised component is fully contained.
- **−** IPC overhead on every internal call; substantial operational and
  conceptual complexity; hard for one maintainer to hold; startup and latency
  costs.
- **MELFINA implication:** violates AP-12 (lightweight, one-maintainer, minimal
  deps) at the *literal* level; over-engineered for a single-user local system.
- **Failure modes:** the system is so complex it is never finished, or its
  performance forces the user off it; the IPC layer itself becomes a bug source.
- **Verdict: rejected as a literal design; its *principle* kept.**

**Candidate C — Concentric rings: a small always-resident deterministic core +
governance, with reasoning and capabilities as isolated, on-demand outer rings.
Isolation strength scales with ring (Ring 3 fully sandboxed; Ring 1 in-process
modules).**  *(the chosen structure)*
- **+** structural containment exactly where it is needed (untrusted content,
  capability execution, the reasoner) without paying IPC cost everywhere;
  the always-resident part stays small (AP-12); the autonomy triad and
  "AI-is-not-the-whole-system" fall out of the ring boundaries; one maintainer can
  hold Ring 0 + Ring 1.
- **−** the ring boundaries must be genuinely enforced (a real isolation
  primitive for Ring 3 — deferred, AU-2); the Reasoner Interface and the
  capability grant mechanism are new surfaces to get right.
- **MELFINA implication:** satisfies AP-1…AP-14; defers the *isolation mechanism*
  (process / VM / WASM / other) to LOW-LEVEL FOUNDATIONS without prejudicing the
  structure.
- **Failure modes:** if Ring 3 isolation is weak in practice, it degrades toward
  Candidate A for capabilities — mitigated by making isolation a first-class
  LOW-LEVEL FOUNDATIONS deliverable with AP-7/AP-8 as acceptance criteria.
- **Recommendation:** **C.** **Confidence: High** for the structure; **Moderate**
  that the isolation mechanism will be both strong and lightweight (AU-5).

> **Responsibilities vs components (RC-7 / adversarial review O2).** The ~25 named
> subsystems are **responsibilities**, not a mandate for 25 processes, services,
> binaries, or IPC boundaries. An implementation MAY group them into a small
> number of cohesive components. Only three boundaries **must** be real, enforced
> isolation (not merely a code convention): **Ring 0 ↔ everything else**; **Ring 1
> (+ its resident interfaces) ↔ the sandboxed outer region (Ring 2 + Ring 3 —
> no Chronicle write, no capability invocation from Ring 2, no network, no
> Ring 0)**; and the **reasoning (Ring 2) ↔ effect (Ring 3) boundary** — Ring 2
> emits only proposals and claims and holds no capability grant; Ring 3 holds
> per-action grants and can cause effects. Revision 1 explicitly **rejects**
> collapsing Ring 2 and Ring 3
> (O2): co-locating their code in one OS process is allowed, erasing the authority
> distinction is not.

---

## AA-2 — Persistence: how is state stored and history kept?

**Candidate A — Pure event-sourced.** The append-only Chronicle is the only
stored thing; *all* current state and views are computed by replaying events;
nothing is materialised.
- **+** perfect fidelity to E²CI (bitemporal, append-only, provenance);
  time-travel is native; audit is free; no cache-invalidation class of bugs.
- **−** "retrieving current state requires aggregating all relevant events…
  slow and unpredictable" (`[E]`); the personal-assistant "what needs attention"
  read becomes a whole-history scan; operational tooling (replay, checkpoints)
  needed anyway.
- **MELFINA implication:** the everyday reads (§13.5 of the model) would be too
  slow; violates AP-3's spirit (predictable *performance*) and MEL-REQ-197 (low
  latency where practical).
- **Failure mode:** the system feels sluggish for exactly the queries the user
  makes most; the user stops trusting it to "just know".
- **Verdict: rejected alone; the *log* is kept as the authoritative layer.**

**Candidate B — Mutable current-state, with a side history/audit log.** The
primary store holds "the world as it is now"; changes overwrite; a separate log
records what changed for audit.
- **+** fast reads; familiar; simple queries.
- **−** bitemporal "what did I believe at transaction-time T about valid-time V"
  is not native — it must be reconstructed from the side log, which is exactly the
  event-sourcing read problem moved to the harder case; provenance becomes
  metadata, not structure (violates AP-4); "the past is appended to, not
  overwritten" (P-5) is violated by construction; conflict (`contradicts`) has
  nowhere natural to live.
- **MELFINA implication:** violates AP-4, AP-5; breaks the model's §11 change
  semantics and §8 provenance-as-structure.
- **Failure mode:** MELFINA can't reliably answer "what changed my mind and when";
  a bug in the overwrite path silently corrupts the user's life record with no
  recovery.
- **Verdict: rejected.**

**Candidate C — Hybrid: the Chronicle (append-only Events/Claims/Intentions) is
authoritative; there is **one materialised current-state cache**, and other views
(attention, calendar, learning trajectory, capability reliability) are **built on
demand** — all rebuildable, all allowed to lag, none a second source of truth.**
The conceptual target is *one Chronicle + one current-state cache + on-demand
views*, not a proliferation of separately-maintained projections (O1).
*(the chosen shape; the CQRS + event-sourcing pattern, `[E]`)*
- **+** the guarantees of A (fidelity, time-travel, provenance, audit,
  recoverability) with the read performance of B for the common case; loss of
  every projection loses nothing; new views (a learning trajectory, a capability
  reliability report) are new projections without touching the write model.
- **−** a consistency window between an append and its projection (bounded,
  usually sub-second); projection/checkpoint/replay tooling to build and operate
  (but modest for a single-user local system — no distributed consumers); the
  materialisation strategy (eager / lazy / hybrid) is itself a choice (AU-1).
- **MELFINA implication:** satisfies AP-3, AP-4, AP-5; anything *consequential*
  (the pipeline, the Reference Monitor) reads the Chronicle directly, never a
  possibly-stale view; the *engine* and *format* remain deferred to STORAGE.
- **Failure modes:** a projection bug shows a wrong view → contained (the view is
  discarded and rebuilt; the Chronicle is untouched); the consistency window
  causes a just-captured item to be briefly absent from "what needs attention" →
  acceptable for a pull surface, and consequential reads bypass the view.
- **Recommendation:** **C (hybrid).** **Confidence: Moderate–High.** The open
  sub-question (eager vs lazy materialisation) is AU-1, for STORAGE + prototyping.

---

## AA-3 — Reasoning coordination: how does dynamic behaviour work?

**Candidate A — A single autonomous agent loop** (observe → think → act → observe…).
- **+** simple; the dominant contemporary pattern.
- **−** the mission **explicitly forbids** collapsing THINK→…→VERIFY into "a
  generic agent loop"; it fuses cognitive/decision/execution autonomy (violates
  INV-2); hard to bound, hard to explain, prone to the documented agent failure
  modes (premature action, over-helpfulness, context pollution — `[E]`).
- **Verdict: rejected by mission constraint and by INV-2.**

**Candidate B — A fixed pipeline of reasoning stages** (interpret → plan →
evaluate → decide), always run in order, always the same depth.
- **+** predictable; easy to audit.
- **−** not *dynamic* — cannot vary reasoning depth, context breadth, strategy, or
  which contributors run, per situation (violates MEL-REQ-204–213, VC8); wastes
  effort on simple problems, under-serves hard ones.
- **Verdict: rejected — fails the dynamic-self-directed requirements.**

**Candidate C — A metareasoning *controller* + a set of reasoning contributors + a
transient working context; the controller opportunistically selects which
contributors to run, how deep, with how much context, and whether to act / wait /
ask / do nothing.** *(the chosen structure)*

> **"Blackboard" is one candidate implementation, not an architectural
> requirement (O3).** The blackboard pattern (Hearsay-II lineage, `[E]` — "well
> suited to problems where the solution path is not known in advance") is the
> reference model for *how* a controller and contributors could share a working
> context. The architecture commits only to: a controller that makes logged,
> bounded strategic choices; contributors that can be added/created without
> changing the controller; and a transient working context. The concrete
> coordination mechanism (blackboard, direct invocation, a queue, other) is a
> CORE ENGINE design choice.
- **+** *is* dynamic strategy/context/skill selection (MEL-REQ-204–216);
  the controller *is* the metareasoning layer; "whether to act at all" is a
  first-class control decision (MEL-REQ-103); bounded by Ring-0 budgets;
  every strategic choice is a logged control decision → explainable (MEL-REQ-208,
  253); contributors can be added/created without changing the controller.
- **−** the controller's selection *policy* is itself non-trivial and must be made
  predictable enough (Risk 3 in `SYSTEM_ARCHITECTURE.md` §28); the working context
  is transient state the controller manages (AU-3).
- **MELFINA implication:** satisfies VC8–VC12; keeps reasoning as *proposal
  generation* (AD-6); the controller algorithm is deferred to CORE ENGINE where it
  gets its own design + evaluation.
- **Failure modes:** a poorly-tuned controller thrashes (mitigated by budgets +
  "stop when marginal value drops") or is unpredictable (mitigated by logging +
  grounding-factor inputs + a conservative default under meta-uncertainty).
- **Recommendation:** **C (a metareasoning controller + contributors).**
  **Confidence: Moderate–High** for the structure; the controller *algorithm* and
  the concrete coordination mechanism (blackboard or otherwise) are genuine open
  design problems (deferred to CORE ENGINE).

---

## AA-4 — Where do the self-modification governance rules live?

**Candidate A — As Claims in the Chronicle**, like everything else MELFINA knows.
- **+** uniform; MELFINA can reason about its own rules.
- **−** if the rules are Claims, the reasoning ring can *propose editing them*,
  and the only thing stopping a bad edit is another rule that is also a Claim…
  turtles. Violates INV-6 directly (model §12.1 explicitly excludes this).
- **Verdict: rejected.**

**Candidate B — As data behind a "modify governance" capability**, gated by a
permission check.
- **+** consistent with the capability model; the gate is explicit.
- **−** the gate becomes the single highest-value target; a prompt-injection or a
  bug that gets past *one* check escalates fully; "a model can be talked out of
  its own instructions but cannot override an external check" (`[E]`) argues for
  *no such operation existing*, not a well-guarded one.
- **Verdict: rejected.**

**Candidate C — In a Ring-0 governance store: versioned files that Ring 1–3 can
**read** (to explain the rules) and have **no write path** to; changed only by a
human editing them with ordinary tools; the Reference Monitor enforces them and
is non-bypassable, always invoked, evaluable.** *(the chosen structure; reference
monitor `[E]` + monotonic confinement `[E]`)*
- **+** structural: there is no MELFINA operation whose effect is "edit Ring 0";
  the meta-invariant holds by *absence* of a path, not by strength of a guard;
  monotonic (authority only narrows without a human widening act); every change
  is a recorded human act; the smaller Ring 0/1, the smaller the residual bug
  surface (AU-5).
- **+ at-rest integrity (RC-1).** The "no write path" argument covers a running
  MELFINA. It does **not** cover an attacker (or a buggy capability) editing the
  Ring 0 files directly on disk while MELFINA is stopped. Revision 1 closes this:
  the Ring 0 governance files carry a cryptographic integrity chain rooted in a
  **human-held signing key that MELFINA never possesses**; the chain is verified
  at every startup; an invalid signature or a broken version chain ⇒ **MELFINA
  refuses to run**. The Ring 0 path is additionally **hard-excluded from every
  File Access grant** — MELFINA cannot be granted access to its own governance
  files, so it cannot even stage an edit for a human to later sign. *(Only the
  invariant is fixed here; the signing scheme, key storage, and chain format are
  LOW-LEVEL FOUNDATIONS.)*
- **−** **necessary, not sufficient** (MEL-REQ-239): a capable reasoning ring
  could socially engineer the *user* into a Ring-0 change, or exploit an
  unintended Ring-1 bug. These limits are **documented, not solved**
  (`AUTHORITY_AND_SECURITY_MODEL.md` §9).
- **MELFINA implication:** self-modification tiers 6–9 are **not MELFINA
  operations** — a human changes MELFINA's code/subsystems/architecture; MELFINA
  may *assist* under the normal pipeline but cannot *perform* T6+.
- **Failure modes:** documented residual limits above; plus "the user, over time,
  rubber-stamps Ring-0 changes" — mitigated by making them rare, deliberate, and
  framed as such.
- **Recommendation:** **C.** **Confidence: High** that this is the right structure;
  **Moderate** on residual sufficiency — carried forward honestly.

---

## AA-5 — The local AI integration boundary

**Candidate A — The model as orchestrator / source of truth / memory.** The AI
holds context, decides, remembers, and calls tools.
- **−** the mission (§8) forbids exactly this; violates AP-3 (determinism), AP-4
  (claim-shaped truth), AP-9 (usable without AI), and every containment invariant;
  makes the whole system as reliable as the model's worst day.
- **Verdict: rejected by mission constraint.**

**Candidate B — The model as one contributor among many, behind a narrow typed
interface (context projection + question → claims/proposals + confidence +
trace), with no Chronicle write, no capability invocation, no network, no Ring-0
access; swappable or absent.** *(the chosen structure)*
- **+** "AI is not the sole source of truth/memory/authority/verification/safety"
  is *structural* — the model simply lacks those capabilities; the system works
  with the model disabled (AP-9); a small model, a large model, a symbolic engine,
  or none, all bind to the same interface; local by requirement, declines rather
  than reaches out (MEL-REQ-155).
- **+ locality is structural, not policy (RC-6).** The Reasoner Interface lives in
  the local core (Ring 1) and Ring 1 and Ring 2 hold **no network capability** —
  the interface cannot open a socket, so it can bind **only** a local reasoner. A
  remote endpoint is not "disallowed by configuration"; it is **unreachable
  through the core capability model**. The correct verb is *cannot*, not *does
  not*. (This is why Ring 4 must stay outside the core: a network capability in
  Ring 1/2 would silently reopen this path.)
- **−** the local reasoner ceiling (OQ-11) is real — if it's too weak MELFINA
  declines a lot; the interface must carry enough (a reasoning trace, calibrated
  uncertainty) for the rest of the system to use the output responsibly.
- **Recommendation:** **B.** **Confidence: High** for the structure; the *ceiling*
  is `[U]`, deferred to AI/ASSISTANT LAYER.

*(No Candidate C — the design space here is essentially binary: the model governs,
or it is a bounded contributor. The requirements decide it.)*

---

## AA-6 — Capability isolation posture

**Candidate A — Trust capabilities; validate their outputs.**
- **−** assume prompt injection / adversarial content wins (AP-8, `[E]`); a
  malicious or compromised capability with ambient authority is game over before
  any output check; ~50% of shared trigger-action rules have integrity flaws
  (`[E]`).
- **Verdict: rejected.**

**Candidate B — No ambient authority; per-action typed grants; sandboxed
execution; hard resource limits; monotonic (a capability cannot acquire authority
beyond its grant).** *(the chosen posture; ocap + WASI component-model pattern,
`[E]`)*
- **+** a compromised capability can do only what its narrow grant allows, for the
  duration of one authorised action; blast radius is contained by construction;
  the Reference Monitor is always invoked before execution.
- **− AU-2 is CONTAINMENT-CRITICAL (RC-4).** Terminal and GUI authority is the
  sharpest edge of this posture. The grant model MUST treat terminal authority as
  **structured actions, not raw shell strings**: a command grant is evaluated
  against a **parsed argv / structured invocation**, never a text match on a
  command line; **raw shell execution is never equivalent to an exact-command
  grant**; **shell interposition** (running through a shell that can expand,
  chain, or substitute) is a **distinct, higher-risk capability**. GUI actions
  get structured representations where practical. Getting this wrong lets a
  capability launder arbitrary execution through a narrow-looking grant.
- **−** every capability must *declare* its authority needs precisely (design
  discipline); the grant/handle representation and the action parser are
  LOW-LEVEL FOUNDATIONS design problems (AU-2); more moving parts than "just run
  it".
- **Recommendation:** **B.** **Confidence: High.** The *mechanism* (process / VM /
  WASM / other) and the *parser* are deferred with AP-7/AP-8 and RC-4 as
  acceptance criteria.

---

## AA-7 — Notification / outbound posture

**Candidate A — Any subsystem may notify the user** (a cross-cutting concern).
- **−** the hazard surfaces (interruption, repetition, nudging) then have no
  single choke point; every new feature can add a prompt; drifts toward
  MEL-AR-03 (notification spam).
- **Verdict: rejected.**

**Candidate B — A single Notification Gateway is the only push path; everything
else is pull; the Gateway enforces user-governed, per-source, per-context,
predictably-scheduled delivery with no repetition-escalation and no
behaviour-change nudging.** *(the chosen posture)*
- **+** one governed choke point; the restraint requirements (MEL-REQ-011,
  041–047) are enforced *once*, structurally; Fitz-2019-style batched predictable
  delivery (`[E]`) is the default.
- **−** requires discipline (no back doors); the "what would have prompted me"
  pull surface must be good enough that users don't feel they're missing things.
- **Recommendation:** **B.** **Confidence: High.**

---

## AA-8 — Where does "current state" (what holds now) live?

**Candidate A — A first-class stored current-state object**, updated in place as
events arrive. *(what a `State` primitive, model OQ-M2, would suggest)*
- **+** fastest "what holds now" and "what needs attention"; natural for planning
  preconditions.
- **−** an update-in-place store is exactly Candidate B of AA-2 (rejected); it
  becomes a second source of truth that can diverge from the Chronicle.
- **Verdict: rejected as *authoritative*.**

**Candidate B — Current-state is *the one* rebuildable, checkpointed cache built
from the Chronicle's `brings-about`/`ends` links; every other view is built on
demand from the Chronicle (or from this cache).** *(the chosen approach;
consistent with AA-2 C and O1 — one Chronicle, one current-state cache, on-demand
views)*
- **+** one source of truth (the Chronicle); one materialised cache to keep
  honest, not many; the cache can be as eager or lazy as STORAGE decides; if the
  model later makes `State` a primitive, this becomes a *checkpointed* projection,
  not a new subsystem (AU-4).
- **−** states with no known initiating event (model §3.3) are Claims without a
  `brings-about` link — the projection handles them as standing descriptive Claims
  with a valid-time; fine, but a reminder that the model's OQ-M2 is unresolved.
- **Recommendation:** **B.** **Confidence: Moderate–High.** Explicitly does **not**
  resolve model OQ-M2 — the architecture is compatible with either answer.

---

## AA-9 — Do capabilities depend on the reasoning ring?

**Candidate A — Yes: capabilities are only ever invoked by the metareasoning
controller.**
- **−** then the system is unusable with AI disabled for anything active
  (violates AP-9 / MEL-REQ-154); a user who just wants to run a deterministic
  automation or a direct capability can't.
- **Verdict: rejected.**

**Candidate B — No: a capability can be invoked (a) by the user directly through
the pipeline, or (b) by an authorised automation, or (c) via a proposal from the
reasoning ring — all three routes go through the same pipeline + Reference
Monitor.** *(the chosen approach)*
- **+** AP-9 holds — deterministic capabilities and user-driven action work with
  Ring 2 off; the reasoning ring is additive, not load-bearing.
- **Recommendation:** **B.** **Confidence: High.**

---

## Summary table

| # | Choice | Recommended | Confidence | Key deferred sub-question |
|---|---|---|---|---|
| AA-1 | subsystem structure | concentric rings | High | isolation mechanism (AU-2, AU-5) |
| AA-2 | persistence | hybrid — one Chronicle + one current-state cache + on-demand views (O1) | Mod–High | eager vs lazy materialisation (AU-1) |
| AA-3 | reasoning coordination | metareasoning controller + contributors (blackboard = one candidate impl, O3) | Mod–High | the controller algorithm + coordination mechanism (CORE ENGINE) |
| AA-4 | self-mod governance | Ring-0 read-only files, no write path, **+ at-rest signing (RC-1)** | High struct / Mod sufficiency | residual limits (documented); signing scheme (LLF) |
| AA-5 | AI boundary | bounded contributor behind a narrow interface; **locality structural — cannot bind a remote reasoner (RC-6)** | High | local reasoner ceiling (OQ-11) |
| AA-6 | capability isolation | no ambient authority, per-action grants, sandbox; **terminal authority = structured actions not raw shell (RC-4, containment-critical)** | High | grant representation + action parser (AU-2) |
| AA-7 | notification | single Gateway, pull-by-default | High | — |
| AA-8 | "current state" | the one current-state cache, not authoritative (O1) | Mod–High | model OQ-M2 (unresolved, compatible either way) |
| AA-9 | capabilities ↔ reasoning | independent (three invocation routes) | High | — |
