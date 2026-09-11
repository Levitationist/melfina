# MELFINA — FOUNDATION 10: VERIFIER TRUST CONTRACT

**Phase:** LOW-LEVEL FOUNDATIONS — MISSION 001. First pass, pending review.
**Baseline:** `8a40207`. Read F2, F3, F4, F7 first.

**This document does not choose a verifier implementation** or a set of concrete
verifiers. It fixes what a **verifier is trusted to be**, what "architecturally
independent" means for MELFINA (without over-claiming), and what happens on every
failure mode of the verification step.

Derives from: **RC-3** (the adversarial-review correction that added the verifier
trust model); **INV-2, INV-8, INV-10, INV-12**; **AP-4, AP-8**; **MEL-REQ-018,
157, 174, 181, 226, 248, 250**; Knight & Leveson 1986 (N-version programming:
27 independently written programs showed **significant common-mode failure** —
independence of *effort* does not yield independence of *failure*) and its 2026
replication with AI coding agents (**429 coincident failures observed vs 115
predicted** under an independence assumption) `[E]`; the reference-monitor
"evaluable / verifiable" property `[E]`.

---

## 0. Dependencies discovered

- **On F3:** VERIFY is the last step of the Monitor's pipeline (F3 §6). The
  Monitor decides *whether* a verifier is required and *how many*; this contract
  says what the verifier must be.
- **On F4:** the outcome vocabulary (completed / partially completed / rolled
  back / failed / interrupted) is what VERIFY confirms or contradicts. A
  verifier's job is to check the **claimed** outcome against **observed**
  reality.
- **On F2:** a verifier reads the **canonical structured action** and the
  **canonical post-state**, not a capability's prose description of what it did.
- **On F7:** verifiers **are Ring-3 components** and run under the isolation
  contract — minimal grants, read-only where possible, no ambient authority,
  terminable, contained.
- **On F8:** the governance object's `class → required-checks` map (F8 §4) says
  which actions need a VERIFY plan and which need ≥ 2 independent verifiers.
- **Discovered constraint:** because a verifier must sometimes read state a
  capability wrote, and that state may be outside the Chronicle (a file, a GUI
  surface), a verifier needs **its own narrow read grants** — issued by the
  Monitor, distinct from the acting capability's grants, and **read-only**
  wherever the effect class permits observation without mutation. `[SEN]` (the
  intent) + `[ID]` (read-only enforcement).

---

## 1. What a verifier IS

> A **verifier** is a bounded, low-authority, preferably-deterministic checker
> that takes an explicit claim ("action A produced outcome O") plus explicit
> evidence, and returns one of {**confirmed**, **contradicted**, **inconclusive**}
> with its reasons. It **cannot cause effects**. It is **not** an agent: it does
> not plan, choose goals, retry, or act — it only checks.

---

## 2. The trust properties

Every verifier must satisfy all of these:

| # | Property | Acceptance criterion | Class |
|---|---|---|---|
| V1 | **minimal authority** | a verifier holds only the narrow read grants needed to observe the evidence for its one check; no write, no exec, no network, no capability-mint | **[SEN]** (grant shape) + **[ID]** (Monitor issues read-only) |
| V2 | **no ambient authority** | like any Ring-3 instance (F7 P1): nothing until grants are wired | **[ID]** |
| V3 | **cannot cause effects** | a verifier has **no** effect-causing grant; if a check *requires* mutation to observe (rare), that is a separate action with its own authorisation, not something the verifier does | **[SEN]** (grant shape) |
| V4 | **read-only where possible** | for `file-*`, `chronicle-*`, and observable `gui` state, the verifier reads; it never needs write to verify | **[SEN]** + **[ID]** |
| V5 | **deterministic where practical** | given the same evidence, a verifier returns the same verdict; non-deterministic checks (e.g. an AI judgement) are marked as such, carry lower weight, and **cannot be the sole verifier for a high-risk or irreversible action** (RC-3, `MEL-REQ-174`) | **[ID]** + **[ETL]** |
| V6 | **independently testable** | a verifier can be run offline against a fixture corpus of (action, post-state, expected verdict) cases; it is part of the regression harness (`MEL-REQ-226`) | **[ETL]** (the corpus is built here) |
| V7 | **explicit inputs and outputs** | inputs: the canonical action, the claimed outcome, the evidence references, the read grants. Output: verdict ∈ {confirmed, contradicted, inconclusive} + reason + the evidence it actually read. Nothing implicit. | **[SEN]** (interface shape) |
| V8 | **provenance** | the verdict is recorded with which verifier produced it, its version, what it read, and when — so a later reviewer can retrace it | **[ID]** (recorded as a Claim, F5) |
| V9 | **failure is a verdict, not an exception** | a verifier that cannot complete returns **inconclusive** with a reason; it does not throw and vanish, and it does not guess | **[ID]** |
| V10 | **not a second agent** | a verifier has no goal state, no Intention ownership, no ability to propose or trigger actions, no memory across checks beyond its fixture/config; it cannot "decide to investigate further" and act | **[SEN]** (no Intention-owner role, no effect grant) + **[ETL]** |
| V11 | **bounded** | wall-clock, resource, and evidence-size limits like any Ring-3 instance (F7 P6); exceeding them ⇒ **inconclusive (timeout)** | **[ID]** |

---

## 3. "Architecturally independent" — defined without over-claiming

RC-3 requires, for **high-risk** actions, **≥ 2 architecturally-independent
verification mechanisms where feasible**. What that means here:

**Architecturally independent** = the verifiers differ in **mechanism, evidence
source, and failure mode** such that a single fault, bug, blind spot, or
adversarial manipulation is **unlikely to defeat both in the same way**.
Concretely, two verifiers are architecturally independent if they differ on
**at least two** of:

- **evidence source** — e.g. one reads the filesystem directly; the other reads
  the Chronicle Event the capability appended; a third observes via an OS-level
  audit facility;
- **checking mechanism** — e.g. exact byte/hash comparison vs a structural
  property check vs a policy predicate;
- **implementation lineage** — different code, different author/toolchain, not a
  shared library for the load-bearing logic;
- **determinism class** — a deterministic structural check paired with (never
  replaced by) a heuristic one.

**What this is NOT:** a claim of **statistical or mathematical independence** of
failure. Knight & Leveson showed, and the 2026 AI-agent replication reinforced,
that independently produced checkers **fail together far more than an
independence assumption predicts** (`[E]`). MELFINA therefore:

- **does not multiply probabilities** to claim a combined assurance level;
- treats a **second verifier as defence-in-depth**, not as a guarantee;
- requires the two to **disagree loudly** rather than assuming agreement is
  confirmation (§4);
- documents that common-mode verifier failure is a **residual risk** `[U]`.

**"Where feasible" / the enumerated single-verifier fallback (RC-3):** a single
verifier is acceptable **only** when *all* of:

1. the action's governance risk class is **not** high and **not** irreversible
   (F8 §2.4); **and**
2. a second architecturally-independent mechanism genuinely does not exist for
   this evidence type (enumerated in the capability's VERIFY plan, reviewed at
   capability authorisation, `MEL-REQ-226`); **and**
3. the single verifier is **deterministic** (V5); **and**
4. the fallback is **recorded** on the action (an explicit "verified by one
   mechanism because …" Claim) so it is visible in audit.

For a **high-risk or irreversible** action where a second mechanism is
infeasible: the action does **not** proceed autonomously — it is escalated to
the human (`MEL-REQ-157, 181`). "Cannot verify strongly ⇒ do not do it
autonomously."

---

## 4. Failure modes of the verification step

| Situation | Contract response |
|---|---|
| **verifier confirms** | the claimed outcome (F4 vocabulary) stands; recorded as a `confirmed` Claim with provenance (V8); the action closes |
| **verifier contradicts** | the claimed outcome is **rejected**; the recorded outcome becomes what the verifier observed (or `failed` / `partially completed` if unclear); the discrepancy is a **stop-condition** for that capability — the Supervisor flags it; repeated contradiction ⇒ retire the capability (`MEL-REQ-224, 249`); the user is informed for any consequential action |
| **verifier inconclusive** | the outcome is recorded as **unverified**; for a low-risk reversible action MELFINA may proceed but marks the result low-confidence; for a **consequential** action, unverified ⇒ treat as **not safely completed** — escalate to the human, do not report success |
| **two verifiers agree (confirm)** | outcome stands; note the common-mode residual (§3) — agreement is not proof |
| **two verifiers disagree** | **fail toward caution**: take the **more adverse** verdict (contradicted > inconclusive > confirmed); record both verdicts + both evidence sets; escalate to the human for any consequential action; **never** silently pick the favourable one |
| **verifier times out** (V11) | inconclusive (timeout); handled as "inconclusive" above; the Monitor may re-run once with a longer bound for a non-adversarial timeout, then stop |
| **evidence unavailable** (the state to check is gone / unreadable) | inconclusive (no-evidence); for a consequential action ⇒ escalate; record that verification could not be performed |
| **verification is impossible for this action type** | this must be known **at capability authorisation** (the VERIFY plan, `MEL-REQ-226`); such an action is **not** eligible for autonomous execution if consequential — it is human-gated (`MEL-REQ-157`) |
| **the action is irreversible AND verification is inconclusive** | the worst case: MELFINA must **not** have reached here autonomously (F8 risk floor + §3 rule). If it did (a classification bug), that is itself a stop-condition and an incident |
| **the verifier itself crashes / is compromised** (F7 P8) | contained; its verdict is discarded (treated as inconclusive); a second verifier's verdict, if present, is not automatically trusted more — the incident is flagged; the capability's results since the last clean verification are reviewed |

**Principle:** verification failure never upgrades an outcome. The verification
step can only **confirm** or **downgrade** confidence in a claimed outcome, never
raise it beyond what a clean `confirmed` gives.

---

## 5. Where verifier output goes

- A verdict is recorded as a **Claim** (F5) with `holder = melfina`, status
  `observed` (it observed evidence) or `inferred` (it reasoned over evidence),
  provenance `generated-by <verifier-id@version>`, and `evidence-for` /
  `evidence-against` links to the outcome Claim it checks.
- The verdict feeds **Self-Evaluation** (`MEL-REQ-248`: self-evaluation is not
  the model's own report — a verifier is an independent-of-the-reasoner check,
  which is exactly the point).
- A `contradicted` verdict feeds the **capability lifecycle** (trust
  down-weighting, retirement) and the **Supervisor** (stop-condition).
- Verdicts are **auditable**: the human can see, per action, what was checked,
  by what, reading what, and what it concluded.

---

## 6. Contract Q&A

| Question | Answer |
|---|---|
| **Guarantees** | every action requiring verification is checked by an explicit, low-authority, effect-free checker against real evidence; high-risk/irreversible actions get ≥ 2 architecturally-independent checks or do not run autonomously; a verifier cannot act, cannot cause effects, and cannot become an agent; failure yields `inconclusive`, never a silent pass; disagreement fails toward caution |
| **Requires** | the Monitor to gate which actions need verification and how many verifiers (F3, F8); narrow read grants issued to the verifier; a fixture corpus + regression harness; the capability's VERIFY plan reviewed at authorisation |
| **Trusts** | the evidence it directly reads (a file's bytes, a Chronicle Event, an observable GUI state) — and only after canonicalisation (F2); its own deterministic logic |
| **Distrusts** | the acting capability's prose account of what it did; an AI-generated description of the outcome (F3 §5); a single non-deterministic verdict for a serious action; agreement between two verifiers as *proof* (common-mode, §3); its own heuristic components |
| **Enters** | the canonical action, the claimed F4 outcome, evidence references, read-only grants |
| **Leaves** | a verdict ∈ {confirmed, contradicted, inconclusive} + reason + the evidence actually read + verifier id/version/time — recorded as a Claim |
| **Malformed input** | a verification request missing the action, the claimed outcome, or evidence references is rejected by the Monitor before a verifier runs; a verifier given unreadable evidence returns `inconclusive (no-evidence)` |
| **Failure** | fail toward caution: inconclusive for a serious action ⇒ escalate + do not report success; disagreement ⇒ take the more adverse verdict; a crashed verifier ⇒ contained, verdict discarded, incident flagged |
| **Authoritative** | the verifier's observation of actual post-state is authoritative **over the capability's claim**; for a contested outcome the recorded truth is what verification observed, not what the capability reported |
| **Independently verifiable** | yes by construction — a verifier is the independent check; the verifiers themselves are tested against a fixture corpus (V6); the human can re-run any verdict against the same evidence |

---

## 7. Enforceability summary

| Invariant | Class |
|---|---|
| a verifier holds no effect-causing grant (V1, V3, V10) | **[SEN]** (grant shape) |
| no ambient authority; read-only where possible; bounded (V2, V4, V11) | **[SEN]** + **[ID]** (F7) |
| deterministic where practical; a non-deterministic verifier cannot be sole for high-risk/irreversible (V5, RC-3, `MEL-REQ-174`) | **[ID]** + **[ETL]** |
| explicit inputs/outputs; verdict ∈ closed set; failure = `inconclusive` not exception (V7, V9) | **[SEN]** (interface) + **[ID]** |
| ≥ 2 architecturally-independent mechanisms for high-risk where feasible (RC-3) | **[ID]** (build them) + **[ETL]** (test independence assumptions) |
| "architecturally independent" ≠ statistically independent; common-mode is a residual risk | **[U]** — documented, not eliminated (Knight & Leveson + 2026 replication `[E]`) |
| single-verifier fallback only under the 4 enumerated conditions (§3) | **[SEN]** (rule) + **[ID]** (Monitor + capability authorisation enforce) |
| high-risk/irreversible + no strong verification ⇒ human-gated, not autonomous | **[SEN]** (F8 floor + rule) + **[ID]** |
| verification failure never upgrades an outcome | **[SEN]** (rule) + **[ID]** |
| disagreement ⇒ take the more adverse verdict, escalate | **[ID]** |
| a verifier cannot become a second autonomous agent (V10) | **[SEN]** (no Intention-owner role, no effect grant) + **[ETL]** (review that it stays so) |
| verdicts are recorded as Claims with provenance and are auditable (V8) | **[SEN]** (F5) + **[ID]** |

---

## 8. Deferred (not decided here)

- The actual set of verifiers and their implementations.
- The evidence-source mechanisms (direct filesystem read / Chronicle read /
  OS-level audit facility / GUI-state observation) — F12 + the isolation
  mechanism.
- Whether any verifier uses a local model, and if so how its non-determinism is
  bounded and weighted (it can never be sole for high-risk — RC-3).
- The fixture corpus + regression harness form (`MEL-REQ-226`) — built in a
  later phase, maintained thereafter.
- The precise `class → required-checks` numbers — content of the genesis
  governance version (F8 §4, §8).
- How many verifiers is "enough" beyond 2 for the most severe actions — `[OPEN]`.

## 9. Traceability

| Element | Source |
|---|---|
| verifier trust model: deterministic where practical, minimal/no ambient authority, read-only, cannot cause effects, independently testable, not a second agent; ≥ 2 architecturally-independent verifiers for high-risk; explicit single-verifier fallback | **RC-3**; `AUTHORITY_AND_SECURITY_MODEL.md` §4.1 |
| VERIFY is a distinct, non-collapsible pipeline stage | `MEL-REQ-018`; INV-2 |
| consequential AI actions confirmed and reversible; irreversible actions stronger protection | `MEL-REQ-157, 181` |
| self-evaluation is not the model's own report | `MEL-REQ-248`; INV-12 |
| deterministic where practical | `MEL-REQ-174` |
| capability changes regression-tested; risk class not self-lowerable | `MEL-REQ-226, 227` |
| capabilities that misbehave are de-prioritised then retired | `MEL-REQ-224, 249` |
| N-version independence failure (effort independence ≠ failure independence) | Knight & Leveson 1986; 2026 AI-agent replication `[E]` |
| reference monitor must be evaluable / verifiable | Anderson 1972 `[E]` |
| assume adversarial content sometimes wins; defence in depth | `MEL-REQ-185`; INV-8; AP-8 |
