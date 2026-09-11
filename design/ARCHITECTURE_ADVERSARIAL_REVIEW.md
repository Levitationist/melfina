# MELFINA — ARCHITECTURE ADVERSARIAL REVIEW 001

**Phase:** SYSTEM DESIGN / ARCHITECTURE — adversarial review of MISSION 001.
**Status:** first pass, pending review. A **"Revision 1 — resolution status"**
section (below, before §1) was added after ARCHITECTURE MISSION 001 — REVISION 1
applied the corrections. **The findings §2–§19 are unchanged** — nothing deleted
or rewritten; they record the architecture *as reviewed*. **Date:** 2026-09-10.
**Baseline:** `origin/main = f3093fc`; architecture documents in `design/`
(uncommitted). **Not committed. Not pushed. No `src/` changes.**

**Purpose:** to *break* the architecture, not to agree with it. Findings are
graded **CRITICAL / HIGH / MEDIUM / LOW** and classified **REAL CONTRADICTION /
ARCHITECTURAL GAP / UNRESOLVED DESIGN QUESTION / IMPLEMENTATION-LATER DETAIL /
ACCEPTABLE TRADEOFF / FALSE POSITIVE**. Every finding cites evidence.

**Rules honoured:** no requirement silently weakened; the Human/Central Model not
rewritten; no implementation technology introduced (except to demonstrate why a
boundary is required); no model open question silently resolved; no guarantee
invented that cannot be justified.

---

## Revision 1 — resolution status (added post-review; no findings deleted or rewritten)

**ARCHITECTURE MISSION 001 — REVISION 1** applied the required corrections
RC-1…RC-7 and the small corrections M1/M2/M3/M5/M6/M8/M9/M10/L4/L5 as a targeted
revision pass (no redesign, no `src/`). This section maps each finding to where
the correction now lives. The findings below (§2–§19) are **unchanged** — they
record the architecture *as reviewed*; this table records what changed *in
response*.

| Finding | Severity | Status after Revision 1 | Where integrated |
|---|---|---|---|
| **H1** Ring 0 at-rest integrity | HIGH | **Addressed (RC-1).** Cryptographic integrity chain over the Ring 0 files, rooted in a human-held signing key MELFINA never possesses; verified every startup before the version-chain check; invalid ⇒ refuse to run. Ring 0 path hard-excluded from every File Access grant. | `ARCHITECTURAL_PRINCIPLES.md` AP-6, §3; `SYSTEM_ARCHITECTURE.md` §2/§17/§20; `AUTHORITY_AND_SECURITY_MODEL.md` §5–§6; `DATA_AND_STATE_MODEL.md` §2; `RUNTIME_MODEL.md` §2; `CAPABILITY_MODEL.md` §7; `ARCHITECTURAL_ALTERNATIVES.md` AA-4; `TRACEABILITY.md` §2.1 |
| **H2** aggregate-effect governance | HIGH | **Addressed (RC-2).** Per-activity-chain aggregate budget over a window; the crossing action re-gates as consequential. Composed workflows classified by the UNION of component authority + scope + aggregate effect; composition never lowers the class. "Practically irreversible in aggregate" is a risk-class input. | `SYSTEM_ARCHITECTURE.md` §2/§6.3/§12/§16/§21/§25; `AUTHORITY_AND_SECURITY_MODEL.md` §2/§4; `CAPABILITY_MODEL.md` §2/§3/§4/§5/§6; `RUNTIME_MODEL.md` §4; `TRACEABILITY.md` §2.1 (AD-8, AD-15) |
| **H3** verifier single point of trust | HIGH | **Addressed (RC-3).** Verifier trust model specified: deterministic where practical, minimal + no ambient authority, read-only unless narrowly justified, cannot cause effects, independently testable against fixtures, not a second agent. High-risk actions need ≥2 architecturally-independent verification mechanisms; an explicit, enumerated, justified single-verifier fallback with raised governance where a second mechanism is impractical. | `AUTHORITY_AND_SECURITY_MODEL.md` §4.1 (new); `SYSTEM_ARCHITECTURE.md` §2/§6.6; `CAPABILITY_MODEL.md` §4; `ARCHITECTURAL_PRINCIPLES.md` §3; `TRACEABILITY.md` §2.1 |
| **H4** terminal/GUI grant granularity | HIGH | **Addressed (RC-4).** AU-2 re-tagged **CONTAINMENT-CRITICAL** and made the first LOW-LEVEL FOUNDATIONS deliverable. Terminal authority = structured actions, not raw shell strings; command grants evaluated against parsed argv / structured invocation; raw shell execution never equivalent to an exact-command grant; shell interposition is a distinct higher-risk capability; GUI actions structured where practical. | `SYSTEM_ARCHITECTURE.md` §2/§6.5/§12/§26/§27/§29/§30; `AUTHORITY_AND_SECURITY_MODEL.md` §3; `CAPABILITY_MODEL.md` §2/§5/§8; `ARCHITECTURAL_ALTERNATIVES.md` AA-6; `ARCHITECTURAL_PRINCIPLES.md` §3; `TRACEABILITY.md` §2.1 |
| **H5** mid-execution revocation | HIGH | **Addressed (RC-5).** Grants are live, revocable handles the Reference Monitor can invalidate mid-execution. After revocation: no new effect; running execution stopped where possible; reversible effects rolled back where possible; irreversible/partial recorded as partial failure. User-visible outcome vocabulary: completed / partially completed / rolled back / failed / interrupted — never a rollback that cannot be guaranteed. | `AUTHORITY_AND_SECURITY_MODEL.md` §3 (path added); `SYSTEM_ARCHITECTURE.md` §2/§6.5/§12; `CAPABILITY_MODEL.md` §2; `RUNTIME_MODEL.md` §4/§6; `FAILURE_AND_RECOVERY.md` §2/§4/§7/§8; `TRACEABILITY.md` §2.1 |
| **H6** reasoner locality is preference not structure | HIGH | **Addressed (RC-6).** Stated structurally: Ring 1 and Ring 2 hold no network capability; the Reasoner Interface cannot open a socket; a remote endpoint is unreachable through the core capability model. Wording changed to "cannot", not "does not". | `ARCHITECTURAL_PRINCIPLES.md` AP-9; `SYSTEM_ARCHITECTURE.md` §2/§22/§23; `AUTHORITY_AND_SECURITY_MODEL.md` §7; `RUNTIME_MODEL.md` §2; `ARCHITECTURAL_ALTERNATIVES.md` AA-5; `TRACEABILITY.md` §2.1 |
| **M7 / complexity** ~25 subsystems vs one-maintainer | MEDIUM (systemic) | **Addressed (RC-7).** The named subsystems are stated as **responsibilities**, not necessarily separate processes/services/binaries/IPC boundaries; an implementation may group them into a few cohesive components. Only three boundaries must be real, enforced isolation: Ring 0 ↔ all; Ring 1 ↔ the sandboxed outer region; reasoning ↔ effect. | `ARCHITECTURAL_PRINCIPLES.md` AP-12; `SYSTEM_ARCHITECTURE.md` §2 header/§20/§26; `ARCHITECTURAL_ALTERNATIVES.md` AA-1; `FAILURE_AND_RECOVERY.md` §2; `TRACEABILITY.md` §2.1 |
| **M1** context as a maintained projection | MEDIUM | **Addressed.** Context is constructed on demand, discarded after use — not a continuously-maintained authoritative projection. | `ARCHITECTURAL_PRINCIPLES.md` §2; `SYSTEM_ARCHITECTURE.md` §2/§8; `DATA_AND_STATE_MODEL.md` §1/§3; `RUNTIME_MODEL.md` §3/§5 |
| **M2** consequential reads off a lagging projection | MEDIUM | **Addressed.** DECIDE and the Reference Monitor read current/authoritative state through the Chronicle contract where consequential correctness matters. | `SYSTEM_ARCHITECTURE.md` §6.3/§28; `DATA_AND_STATE_MODEL.md` §3; `FAILURE_AND_RECOVERY.md` §3 |
| **M3** Reasoning Contributor lifecycle/trust unclear | MEDIUM | **Addressed.** Core contributors = trusted Ring-2 code; novel/self-authored strategies = Ring-3 lifecycle capabilities under the full pipeline. | `SYSTEM_ARCHITECTURE.md` §2 (Reasoning Contributors row) |
| **M5** silent projection divergence | MEDIUM | **Acknowledged as a future implementation concern (with O6).** A periodic consistency audit is recorded for a later phase, not made an architecture requirement; the structural guard is that nothing consequential trusts the cache (M2). | `SYSTEM_ARCHITECTURE.md` §28; `FAILURE_AND_RECOVERY.md` §3/§9 |
| **M6** mutual/cyclic automation triggering | MEDIUM | **Addressed.** The automation-trigger graph is checked for cycles of any length before authorisation and on each definition change; runtime cycles are caught by the Supervisor + Ring-0 budgets. | `SYSTEM_ARCHITECTURE.md` §11; `CAPABILITY_MODEL.md` §6; `FAILURE_AND_RECOVERY.md` §4 |
| **M8** capability-creation authority check implicit | MEDIUM | **Addressed.** Capability creation/modification passes an explicit Reference Monitor check that the creator's authority ≥ the declared authority of the new/changed capability. | `SYSTEM_ARCHITECTURE.md` §16; `AUTHORITY_AND_SECURITY_MODEL.md` §3/§5; `CAPABILITY_MODEL.md` §4 |
| **M9** conflict detection semantics | MEDIUM | **Addressed.** Conflict is judged conservatively: two actions conflict if their declared scopes overlap, unless stronger semantics are available; overlap ⇒ serialise. | `SYSTEM_ARCHITECTURE.md` §6.3; `RUNTIME_MODEL.md` §6 |
| **M10** proposals not structurally validated pre-authorise | MEDIUM | **Addressed.** Proposals pass structural validation (well-formed Intention, declared scope, provenance) before entering AUTHORISE. | `SYSTEM_ARCHITECTURE.md` §2 (Pipeline SM)/§6.3 |
| **M11** and remaining MEDIUM/LOW not listed here | MEDIUM/LOW | Retained as originally recorded; addressed where a RC/M above covers them, otherwise carried forward to the named later phase. | §4–§5 below |
| **L4** Ring 3 sandbox network posture | LOW | **Addressed.** Ring 3 sandbox has zero network capability by default — structural absence, not a disabled feature. | `ARCHITECTURAL_PRINCIPLES.md` §3; `SYSTEM_ARCHITECTURE.md` §2/§6.5; `AUTHORITY_AND_SECURITY_MODEL.md` §8; `RUNTIME_MODEL.md` §2; `CAPABILITY_MODEL.md` §2 |
| **L5** lethal-trifecta prohibition status | LOW | **Addressed.** Made an explicit Ring 0 governance invariant — the Reference Monitor refuses to assemble a grant set holding all three of {private-data read, untrusted-content ingestion, outbound channel}; no Policy Store edit can permit it. | `SYSTEM_ARCHITECTURE.md` §2/§20/§23; `AUTHORITY_AND_SECURITY_MODEL.md` §7/§8 |

**Optional changes (§17) — decisions:** O1 **adopted** (one Chronicle + one
current-state cache + on-demand views); O2 **rejected** (do not collapse Ring 2
and Ring 3 — the reasoning↔effect boundary is one of the three must-be-real
boundaries; implementation grouping is allowed, erasing the authority distinction
is not); O3 **adopted** ("blackboard" reframed as one candidate implementation,
not an architectural requirement); O4 **adopted** (no supervisor topology
specified now — only the isolation boundaries are fixed); O5 **preserved as an
explicit design question** (routine/consequential policy not moved into Ring 0;
AU-6); O6 **retained as a future implementation concern** (cache consistency
audit), not an architecture requirement.

**Unresolved items (§18) remain unresolved** — Revision 1 resolved no model open
question (OQ-M1, OQ-M2, OQ-M5 untouched) and did not choose any deferred
technology (no crypto scheme, no parser, no isolation mechanism, no storage
engine, no language). The meta-invariant's *sufficiency* is still `[U]`;
Revision 1 narrows gaps H1/H2/H3/H5, it does not close the residual-limits list
in `AUTHORITY_AND_SECURITY_MODEL.md` §9.

**Revision 1 — resume completion (2026-09-11).** The Revision 1 edit pass was
interrupted by a host restart. On resume, a full cross-document consistency check
confirmed RC-1…RC-7 and M1/M2/M3/M5/M6/M8/M9/M10 + L4/L5 are integrated
throughout. Three residual propagation gaps in `SYSTEM_ARCHITECTURE.md` were
closed — §5 startup now names the RC-1 signature check (matching
`RUNTIME_MODEL.md` §2 and `AUTHORITY_AND_SECURITY_MODEL.md` §6), §6.1's Event-flow
box no longer lists a maintained "context cache" (M1), and a duplicated phrase in
§6.2 was fixed — and one RC-7 wording variance in `ARCHITECTURAL_ALTERNATIVES.md`
AA-1 was harmonised. No finding was altered; no model question resolved; no
technology chosen; `src/` untouched.

---

## 1. Executive verdict

**The architecture survives. No CRITICAL findings. No redesign required.**

The foundational invariants hold **structurally**, not by model obedience:
- **local-only core** — Ring 1/Ring 2 have no network capability; the only place a
  socket can exist is a future Ring 4 component the user physically adds;
- **the autonomy triad** — reaching a `proposed` Intention never itself produces
  an execution grant; a prompt-injected reasoner can only emit a proposal;
- **the append-only Chronicle** — the single source of truth; projections are
  disposable;
- **AI is not the whole system** — Ring 2 emits only claims/proposals; the system
  is complete with AI disabled;
- **the self-modification meta-invariant** — Ring 0 has no runtime write path;
  tiers 6–9 are absent from the capability namespace.

**Six HIGH findings** require **minimum corrections that strengthen existing
boundaries — none is a redesign:**

| # | HIGH finding | Correction |
|---|---|---|
| H1 | Ring 0 governance files are protected against *runtime* writes but not against *at-rest* tampering beyond a version-chain check | cryptographically sign the version chain with a human-held key; hard-exclude the Ring 0 path from any File Access grant |
| H2 | The routine/reversible class and capability composition are governed **per-action**, not by **aggregate effect** — a sequence of individually-routine actions, or a composition of trivial capabilities, can be consequential in aggregate | per-window aggregate budget across *all* routine actions; classify a composed workflow by the *union* of its components' authorities |
| H3 | The VERIFY stage is a **single point of trust** — a buggy or compromised Verifier undermines the pipeline's verification guarantee | specify Verifier trust requirements (deterministic, no ambient authority, minimal); allow ≥2 independent Verifiers for high-risk actions |
| H4 | Terminal/GUI **grant granularity and action-parsing** is where containment actually lives; it is deferred (AU-2) without flagging how load-bearing it is — shell composition defeats "exactly these commands" | flag AU-2 as containment-critical; require the grant vocabulary to be expressible against parsed, not raw, actions |
| H5 | **Mid-execution permission revocation** semantics are unspecified — the fate of a running capability's grant and a half-done action | grants are live handles the Reference Monitor can invalidate mid-action; on revocation, stop + roll back if reversible + record partial state |
| H6 | The Reasoner Interface's locality is stated as **preference** ("declines rather than reaches out"), not **structure** | state it structurally: Ring 1/2 have no network capability, so the interface *cannot* bind a non-local endpoint regardless of configuration |

**One systemic concern** (MEDIUM, M7): the architecture decomposes into ~25
subsystems, in tension with MEL-REQ-192 ("one maintainer holds the whole system
in their head"). The decomposition is *conceptual* (responsibilities, not
processes) but the documents do not say so clearly enough.

**Unresolved (correctly preserved, cannot be closed at architecture level):** the
metareasoning controller's predictability (MEL-REQ-253); the meta-invariant's
ultimate sufficiency against a capable reasoner + a persuadable user; the grant
representation (AU-2); model OQ-M1/OQ-M2; whether MELFINA should assist with
tiers 6–9 at all.

---

## 2. Critical findings

**None.**

No finding undermines a foundational invariant to the point of making it
impossible, and no finding makes a MUST requirement impossible. The closest
candidate, H1 (Ring 0 at-rest integrity), is mitigated by an existing mechanism
(startup version-chain verification → refuse to run on a broken chain), so
runtime tampering does not affect the running system and is detected at next
startup. It is rated HIGH, not CRITICAL, because the required correction
(signing) is small and additive.

---

## 3. High-severity findings

### H1 — Ring 0 files: runtime write path is closed; at-rest integrity is under-specified
**Classification: ARCHITECTURAL GAP.**

**Evidence.** `ARCHITECTURAL_PRINCIPLES.md` AP-6 and
`AUTHORITY_AND_SECURITY_MODEL.md` §5–§6: "the rules live in Ring-0 versioned
files. Ring 1–3 can **read** them… there is **no write path** from any ring to
Ring 0." `CAPABILITY_MODEL.md` §7: "there is NO such capability [to edit Ring 0]…
that path is absent." `AUTHORITY_AND_SECURITY_MODEL.md` §6: "the version chain is
verified at startup; a broken chain ⇒ refuse to start."

**Attack.** The "no write path" is a claim about the *runtime API*. It says
nothing about the *filesystem*. Ring 3's **File Access** capability is granted by
*path*, not by *effect*. If a File Access grant's path resolves to (or contains, or
is a symlink to, or `..`-traverses to) the directory holding the Ring 0
governance files, a capability can overwrite them directly on disk. The stated
structural defence ("no capability whose *effect* is 'edit Ring 0'") does not
apply, because File Access is not named by effect.

**What actually holds.** Runtime tampering does **not** change the *running* Ring
0 (it is loaded and resident from startup). It takes effect only at the next
startup, where the version-chain check runs. So the real residual risk is
narrower: **a tamperer who also produces a valid-looking version chain.** If the
chain is only "each version references the prior hash", forging a consistent chain
is possible for anyone who can write the files.

**Severity: HIGH.** It weakens the *structural* claim of the meta-invariant
(MEL-REQ-235) — "MELFINA cannot redefine… the rules" becomes "MELFINA cannot
redefine the *running* rules, and a tampered file set is caught at next startup
*if* the chain cannot be forged."

**Required correction (strengthens, not redesigns):**
1. The Ring 0 file location is a **hard, non-overridable exclusion** in the File
   Access capability itself — not a Policy Store setting, not a grant the user
   could accidentally issue. Enforced in the capability's code, verified by the
   Reference Monitor rejecting any grant whose resolved path intersects the Ring 0
   region.
2. The Ring 0 version chain is **cryptographically signed** by a key the running
   MELFINA does not hold (only the human). MELFINA verifies the signature at
   startup and **refuses to run on an invalid signature**. Tampering then becomes
   *detectable and blocking*, not merely *detectable if the chain wasn't forged*.
3. (Optional) Store Ring 0 files on a medium MELFINA's process cannot write at all
   (read-only mount, immutable attribute) — an implementation-later reinforcement.

### H2 — Aggregate-effect blindness: the routine class and composition are governed per-action
**Classification: ARCHITECTURAL GAP.**

**Evidence.** `SYSTEM_ARCHITECTURE.md` §6.3: routine/reversible/pre-authorised
actions get a "policy default authorises (act-then-show, HOTL)". §21 / Ring 0
Budget Authority: "hard ceilings… **per activity and per window**".
`CAPABILITY_MODEL.md` §3: "a composition is itself a **workflow-Entity** — a
`plan` of sub-capability invocations". §5 risk classification is described
per-capability, from *its* declared authority.

**Attack A (routine aggregation).** Each individual file move / rename / small
edit classifies as routine/reversible. The metareasoning controller emits 500 of
them. Individually every one is HOTL-authorised by the policy default. In
aggregate they reorganise the user's entire home directory — an action that, as a
single consequential proposal, the user would have scrutinised. If the controller
spreads them across time windows or frames them as separate "activities", the
per-activity / per-window budgets do not bind the aggregate.

**Attack B (composition risk laundering).** Compose three individually-trivial
capabilities (each: "read one file", "transform text", "write one file") into a
workflow whose aggregate effect is "rewrite every config file in the project".
If the risk classifier classifies each *step* trivially and never the *workflow*,
the composition has laundered a high-risk operation into a chain of low-risk ones.

**Severity: HIGH.** It undermines MEL-REQ-019 (the routine/consequential
distinction), MEL-REQ-181 (irreversible-action protection — 500 reversible actions
can be practically irreversible), MEL-REQ-220/227 (compose-before-code must not
become risk-laundering), and the whole "predictable, nothing consequential
without authorisation" stance (INV-3).

**Required correction:**
1. The routine/reversible/pre-authorised class carries a **per-window aggregate
   budget across all routine actions of an activity chain** (count, and total
   affected scope). Crossing it **escalates the next routine action to
   consequential** (HITL), regardless of activity/window framing.
2. A **composed workflow is risk-classified by the union of its components'
   declared authorities and the union of their affected scopes** — computed by the
   deterministic Ring-1 classifier, not per-step. A composition never has a lower
   risk class than its highest-risk component, and gains a class if the *breadth*
   of affected scope crosses a threshold.
3. "Practically irreversible" is a risk-class input: N reversible actions whose
   combined rollback is impractical (or whose scope is large) classify up.

### H3 — VERIFY is a single point of trust
**Classification: ARCHITECTURAL GAP.**

**Evidence.** `SYSTEM_ARCHITECTURE.md` §6.6, §15: "Verifiers (deterministic,
separate from the reasoner)"; VERIFY produces the Events recorded as the action's
effect. `CAPABILITY_MODEL.md`: Verifiers are Ring 3 capabilities. There is no
provision for a wrong Verifier.

**Attack.** A Verifier is a capability — it can be buggy, or (via the lifecycle) a
subtly wrong one could be created/authorised. A Verifier that returns "verified"
for an action that did not achieve its effect (or that returns "verified" for an
out-of-scope action) breaks the pipeline's central safety property: the system
records what it was *told* happened, not what happened. Because VERIFY's output
becomes an appended Event (authoritative, in the Chronicle), a wrong VERIFY
permanently corrupts the user's life record with a false "this succeeded".

**Severity: HIGH.** VERIFY is load-bearing for MEL-REQ-113, 142, 172, 221, 248 and
for the failure/recovery guarantees. A single wrong Verifier defeats them all for
the actions it verifies.

**Required correction:**
1. **Verifier trust spec:** a Verifier MUST be deterministic; MUST have no
   ambient authority and only a read grant for the state it checks; SHOULD be
   among the smallest, most-audited capabilities; MUST NOT itself perform any
   effect. A Verifier that fails these is not a Verifier.
2. **Redundancy for high-risk actions:** an action classified high-risk requires
   **≥2 independent Verifiers** (different implementations where practical) to
   agree; a disagreement is treated as "not verified" and triggers the failure
   path.
3. **Verifier self-check:** the pipeline periodically runs Verifiers against
   known-good and known-bad fixtures (part of the fixed regression suite) and
   distrusts a Verifier that starts disagreeing with the fixtures.

### H4 — Terminal/GUI grant granularity and parsing is containment-critical and under-flagged
**Classification: UNRESOLVED DESIGN QUESTION (with an architectural
under-emphasis).**

**Evidence.** `SYSTEM_ARCHITECTURE.md` §12: grants "name exactly the commands /
windows / paths in scope". `AUTHORITY_AND_SECURITY_MODEL.md` §3: the grant
"representation… is deferred to LOW-LEVEL FOUNDATIONS (AU-2)". §7 threat model
assumes prompt injection wins.

**Attack.** A grant "run `git` in repo X". The reasoner proposes an action the
pipeline lowers to a shell string. Shell composition — `git status; rm -rf ~`,
`git $(curl …)`, `git log | sh`, backticks, `&&`, subshells — means "exactly the
command `git`" is not a syntactic property of the raw string. If the Reference
Monitor's scope check matches the *raw string prefix* or a naive tokenisation, the
appended `; rm -rf ~` sails through. The entire containment of terminal/GUI
control rests on the grant being checked against a **parsed, structured
representation** of the action, and on the grant vocabulary being able to express
"only these argv vectors, no shell metacharacters, no interpolation".

**Severity: HIGH.** If AU-2 resolves to a coarse grant model, MEL-REQ-141–146
(computer control under authority) is "covered on paper" but the boundary is
bypassable. The architecture treats AU-2 as a routine deferral; it is a
foundational one.

**Required correction:**
1. Re-tag AU-2 as **containment-critical**, not a routine deferral.
2. The architecture states a constraint LOW-LEVEL FOUNDATIONS must meet: a
   terminal/GUI grant is checked against a **structured, parsed action**
   (e.g. an argv vector with no shell interposition), never a raw string;
   the grant vocabulary can express "no shell metacharacters / no interpolation /
   only this argv shape".
3. Terminal control MUST NOT invoke actions through a shell interpreter by
   default; direct process execution with an explicit argv is the safe form;
   shell invocation is itself a distinct, higher-risk grant.

### H5 — Mid-execution permission revocation is unspecified
**Classification: ARCHITECTURAL GAP.**

**Evidence.** `AUTHORITY_AND_SECURITY_MODEL.md` §3: a grant "is **revocable** at
any time; revocation takes effect before the next operation." §2: "a grant can be
*narrowed* or *revoked* at any time without a further act". `SYSTEM_ARCHITECTURE.md`
§12: "any pending or in-progress action is stoppable by the user."
`FAILURE_AND_RECOVERY.md` §7 covers the Emergency Stop but not a *targeted*
revocation.

**Attack / ambiguity.** The user revokes a grant *while the capability holding it
is mid-action* (not an Emergency Stop — a single grant). "Takes effect before the
next operation" — is that the capability's next *internal* operation, or the next
*pipeline* action? If the capability got a token at EXECUTE start and is now
30 seconds into a long file operation, does it keep the token to completion?
What happens to the half-written file / half-applied change?

**Severity: HIGH.** Ambiguity here means MEL-REQ-177 (the user can override any
in-progress action) and MEL-REQ-181 (irreversible-action protection) are not
actually guaranteed for long-running actions.

**Required correction:**
1. A grant is a **live handle** the Reference Monitor can invalidate; the
   Capability Host checks handle validity at every I/O boundary of the running
   capability (or the Host mediates all the capability's effects and checks per
   effect).
2. On mid-action revocation: the Host **stops the capability**, invokes rollback
   if the action is reversible, and records the partial state as a failure Event
   (identical to the mid-execution-failure path in `FAILURE_AND_RECOVERY.md` §5).
3. Irreversible actions, per H2, get extra scrutiny at AUTHORISE precisely so
   this path is rare and always something the user explicitly cleared.

### H6 — The Reasoner Interface's locality is a preference, not a structure
**Classification: REAL CONTRADICTION (with the local-only invariant, as written).**

**Evidence.** `SYSTEM_ARCHITECTURE.md` §22: "The reasoner behind it **may be** a
large local model, a small local model, a symbolic engine… **Local by
requirement** (MEL-REQ-155, INV-1): if a task exceeds the local reasoner, MELFINA
returns a clearly-marked lower-capability local result **or declines** — it does
**not** silently reach out." `AUTHORITY_AND_SECURITY_MODEL.md` §7: Ring 2 gets
"the Reasoner Interface" and "no network".

**Attack.** "Does not silently reach out" and "or declines" are *behavioural*
descriptions. If the Reasoner Interface is a configuration point ("point it at
your reasoner"), and Ring 2's no-network property is not *enforced on the
interface itself*, then pointing the interface at `https://some-api/` is a
supported configuration that breaks INV-1 — not by a bug, by a setting. INV-1
says the core "MUST NOT require… remote API dependency" and "MUST NOT… silently
communicate with external servers" — a configurable remote reasoner endpoint
makes the *core's reasoning* network-dependent by configuration.

**Severity: HIGH.** It is the difference between "local-only is structural" and
"local-only is the default setting." INV-1 and MEL-REQ-164 require the former.

**Required correction:**
1. State the enforcement structurally: **Ring 1 and Ring 2 have no network
   capability. The Reasoner Interface, being resident in the local core, cannot
   open a socket. It can bind only to a local process / local library / local
   file-backed model.** A remote reasoner is not a configuration option of the
   core; it would have to be a Ring 4 component, which is out of scope and
   off-by-default.
2. `SYSTEM_ARCHITECTURE.md` §22 and `AUTHORITY_AND_SECURITY_MODEL.md` §7 updated
   to say "cannot", not "does not".

---

## 4. Medium-severity findings

### M1 — "Context slices" listed as a standing projection drifts toward context-as-stored
**Classification: REAL CONTRADICTION (minor).**
`DATA_AND_STATE_MODEL.md` §3 lists "context slices" among the "standard
projections" with consumers, alongside a note "(rebuilt per situation, then
discarded)". The model (§7) is emphatic: context is "derived, never stored." A
*maintained* projection updated on every append is closer to stored context than
to per-situation derivation. **Correction:** state that the Context Constructor
builds context **on demand** from *other* projections + Query; context is not
itself a maintained projection; remove it from the "standard projections" list or
mark it explicitly transient-only.

### M2 — The DECIDE/classify step may read a stale projection for a consequential classification
**Classification: ARCHITECTURAL GAP.**
`SYSTEM_ARCHITECTURE.md` §6.3: classification is "deterministic classifier +
Policy Store's consequential/routine boundary". Whether an action is "reversible"
often depends on **current state** ("is this file already tracked?", "is this app
already open?"). Current-state is a projection that "may lag" (`ARCHITECTURAL_ALTERNATIVES.md`
AA-2/AA-8). If DECIDE reads the lagging projection, a consequential classification
is made on possibly-stale data. §28 Risk 2 says "anything consequential reads the
Chronicle directly" — but DECIDE is not obviously covered by "consequential
reads." **Correction:** the classify/DECIDE step reads current-state **from the
Chronicle directly** (or from a projection with a freshness guarantee), never
from a best-effort lagging view.

### M3 — Reasoning Contributors' ring placement and lifecycle status is ambiguous
**Classification: UNRESOLVED DESIGN QUESTION.**
`SYSTEM_ARCHITECTURE.md` §2 lists "Reasoning Contributors" in the **Ring 2** table
with class "**capability (of reasoning)**". Ring 3's table lists capabilities. If
contributors are trusted Ring 2 code, then "MELFINA dynamically creates new
reasoning strategies" (MEL-REQ-206) is not architecturally supported — the
strategy set is fixed. If contributors are Ring 3 lifecycle-managed capabilities,
they belong in Ring 3 and should be sandboxed. The docs straddle. **Correction
(recommended, or explicitly defer with this default):** the *core* contributors
(interpret / infer / plan / evaluate / derive / discipline / teach) are small,
audited, trusted Ring 2 code; *additional or novel* reasoning strategies MELFINA
constructs are **Ring 3 capabilities under the full lifecycle** and run
sandboxed. State this so reasoning-strategy dynamism is genuine but bounded.

### M4 — The routine/consequential boundary is Ring 1 policy but is autonomy-governance-relevant
**Classification: UNRESOLVED DESIGN QUESTION.**
`SYSTEM_ARCHITECTURE.md` §2: Policy Store (Ring 1) holds "the consequential/routine
boundary". But that boundary directly sets how much MELFINA may do without asking
— an autonomy-ceiling concern. A reasoner that persuades the user to widen it
(a tier-1 policy change) is influencing something governance-adjacent while the
architecture treats it as ordinary policy. **Question to preserve:** should the
routine/consequential boundary (and per-area autonomy ceilings) live in Ring 0
(human-file-edit only, monotonic) rather than Ring 1 (user act, versioned)?
Arguments both ways; the review does not resolve it. If it stays in Ring 1, the
Notification Gateway / interface must treat "widen your autonomy" proposals with
the same restraint as reassurance patterns (MEL-REQ-058, 158).

### M5 — Silent projection divergence (a logic bug, not corruption) has no detection
**Classification: ARCHITECTURAL GAP.**
`DATA_AND_STATE_MODEL.md` §3: projections are "idempotent" and "rebuilding from
scratch reconstructs it exactly." True — but if the projection *logic* has a bug,
rebuilding reproduces the *same wrong output*. "Rebuildable" is not "correct."
There is no cross-check between a projection and a direct Chronicle query.
**Correction (optional but recommended):** a periodic, low-priority
**consistency audit** that recomputes a sample of projection outputs by an
independent direct Chronicle query and flags divergence. Cheap for a single-user
system.

### M6 — Mutual recursion between two automations may evade a naive self-trigger loop detector
**Classification: ARCHITECTURAL GAP.**
`SYSTEM_ARCHITECTURE.md` §11 / `CAPABILITY_MODEL.md` §6: "a loop detector + the
Supervisor halt oscillating / self-triggering automations." A *single* automation
whose output triggers itself is easy to catch. Automation A whose output Event
triggers automation B, whose output triggers A, is a two-node cycle a
self-trigger check misses. **Correction:** the loop detector operates on the
**automation-trigger graph** (which automation's outputs can satisfy which
automation's triggers), detecting cycles of any length, plus a global
"automations-fired-this-window" budget (Ring 0) as a backstop.

### M7 — Subsystem count (~25) vs MEL-REQ-192; "responsibilities not components" is not stated
**Classification: ACCEPTABLE TRADEOFF (with a documentation gap).**
`SYSTEM_ARCHITECTURE.md` §2 enumerates ~25 subsystems. MEL-REQ-192: "one maintainer
should be able to hold the whole system in their head." `ARCHITECTURAL_PRINCIPLES.md`
AP-12 targets smallness *of the resident core*, and §16 (AD-1) notes the capability
model is conceptual — but the *architecture's own subsystem list* is not flagged
the same way. A reader could take "25 subsystems" as "25 processes/modules."
**Correction:** add an explicit statement to `SYSTEM_ARCHITECTURE.md` §2 and
`README.md`: *"These are **responsibilities**, not necessarily separate processes,
modules, or files. A conforming implementation could realise Ring 0 + Ring 1 as a
small number of modules in a single process; several listed subsystems (Chronicle
+ Query + Entity Registry + Projection Engine ≈ 'the store'; Pipeline SM +
Reference Monitor ≈ 'the gate'; Audit + Notification Gateway ≈ 'the edge') are
naturally one component each."* This is a framing fix, not a structural change —
the responsibilities and their boundaries are correct.

### M8 — The "≤ creator authority" cap on created capabilities is implied, not an explicit check
**Classification: ARCHITECTURAL GAP (minor).**
MEL-REQ-222: a created capability "cannot hold more authority than MELFINA held
when creating it, nor grant itself more later." `CAPABILITY_MODEL.md` §4 AUTHORISE
step describes risk-class-scaled governance but does not name a "≤ creator's
current authority" check. **Correction:** the Reference Monitor's check at a
capability-creation AUTHORISE explicitly caps the new capability's maximum
possible grant at the *intersection* of {what the user authorises} and {what the
creating context currently holds}.

### M9 — Conflict detection between concurrent pipeline actions is unspecified
**Classification: IMPLEMENTATION-LATER DETAIL (flagged).**
`RUNTIME_MODEL.md` §6: "it does not interleave two consequential actions that
could conflict; it may run independent, non-conflicting actions concurrently."
How "conflict" is determined (scope overlap? semantic analysis?) is not stated.
For scope-declared actions, overlap of declared scopes is a sound conservative
test. **Correction:** state the conservative default — two actions conflict if
their declared affected scopes overlap; the pipeline serialises them; semantic
refinement is a CORE ENGINE concern.

### M10 — Proposal structural validation is assumed, not specified
**Classification: ARCHITECTURAL GAP (minor).**
A malformed proposal (missing target, malformed scope, dangling references)
reaching AUTHORISE is not addressed. **Correction:** the Pipeline SM validates a
proposal's structure (well-formed target, resolvable references, a scope
expressible in the grant vocabulary) before AUTHORISE; a malformed proposal is
rejected with a recorded Event and never reaches the user or EXECUTE.

### M11 — OS-level crash reporting could exfiltrate Chronicle data from process memory
**Classification: IMPLEMENTATION-LATER DETAIL.**
INV-1 / MEL-REQ-165: no crash-reporting-to-vendor *by MELFINA*. But the host OS's
crash reporter (Apport, WER, macOS) can capture MELFINA's memory — which holds
Chronicle content and, if unlocked, keys — and offer to send it. Outside MELFINA's
runtime, but a real leak path. **Correction:** LOW-LEVEL FOUNDATIONS should mark
the MELFINA process to opt out of OS crash reporting / core dumps where the OS
allows, and the docs should recommend the user disable it. Also argues for
keeping decrypted sensitive data in memory for the shortest time.

---

## 5. Low-severity findings

- **L1 — transaction-time is a monotonic logical clock, not a wall-clock
  timestamp.** `RUNTIME_MODEL.md` §7 requires monotonicity across clock
  adjustments, which forces a Lamport-style logical component. The model calls
  transaction-time "when this claim was recorded" — a monotonic counter that
  *approximates* wall-clock. Minor semantic drift. **IMPLEMENTATION-LATER
  DETAIL** — STORAGE should record both the logical order and the best-effort
  wall-clock.
- **L2 — `brings-about` / `ends` relation-Claims have special mechanical status in
  Ring 1** (the Projection Engine consumes them to derive current-state). This
  makes them quasi-primitive — exactly what model OQ-M1 anticipates.
  **UNRESOLVED DESIGN QUESTION — correctly preserved.** The architecture should
  note that a small set of relations (`brings-about`, `ends`, `supersedes`,
  `part-of`) are *mechanically load-bearing* even though modelled as Claims.
- **L3 — "local-only" cannot fully control a networked filesystem mount.** If the
  user's data directory is on NFS/SMB, "local writes" traverse the network.
  **ACCEPTABLE TRADEOFF** — STORAGE should detect and warn; the invariant is about
  MELFINA's *intent*, not the OS storage layer.
- **L4 — Ring 3 sandbox network isolation is implied, not explicit.** "No ambient
  authority" should be spelled out to include "zero network capability,
  structurally; a socket cannot be opened from Ring 3."
  **ARCHITECTURAL GAP (minor)** — one sentence.
- **L5 — the "no {outbound + private-read} to one component" rule for Ring 4 is an
  assertion, not located.** It should be an explicit **Ring 0 governance rule** so
  it is enforced by the Reference Monitor, not left as prose.
  **ARCHITECTURAL GAP (minor).**
- **L6 — cyclical / subjective-time domains may strain the linear bitemporal
  model.** A domain organised around seasons, or around felt time, would be
  represented awkwardly. **ACCEPTABLE TRADEOFF / edge case** — the model's linear
  bitemporal time is a deliberate simplification (model §6); cyclical patterns are
  representable as recurring Intentions + Allen relations, if imperfectly.

---

## 6. Attacks the architecture successfully resisted

| Attack | Why it fails |
|---|---|
| **Grant forgery** | grants are unforgeable *by construction* (AU-2 pending, appropriately flagged); a component cannot manufacture one |
| **Capability privilege escalation** | monotonic grants + the Reference Monitor always invoked; a capability cannot acquire authority beyond its grant, and any escalation *attempt* is caught at VERIFY and the capability retired |
| **Proposal / authorization confusion** | distinct `status` field; consequential reads hit the Chronicle directly, not a lagging projection; the proposed→authorised transition is an append, not a mutable flag |
| **Under-declare authority to get a low risk class** | self-defeating — the sandbox ceiling *equals* the declared authority, so an under-declared capability cannot perform its actual function and fails VERIFY / the fixed test suite |
| **Reasoning writes the Chronicle directly** | structurally impossible — Ring 2 holds only Query (read) + the Reasoner Interface; no write path |
| **AI as the source of truth / memory / authority** | structurally impossible — Ring 2 emits only claims/proposals; the system is complete and usable at CORE-READY with AI disabled (INV-9) |
| **Chronicle silent corruption** | detected at open (integrity check); read-only recovery mode; **no silent repair**; the user decides the recovery path |
| **Partial execution / power loss mid-action** | atomic appends (no partial unit); on resume, VERIFY determines the *actual* state; the action is treated as "may have happened", not "intended" |
| **Runaway reasoning / infinite loop** | Ring 0 Budget Authority as an **external** hard cut-off (steps + wall-clock), not self-checked; "stop when marginal value drops" as a secondary |
| **Single-automation self-trigger recursion** | loop detector + budgets (two-node mutual recursion is M6, an incremental fix) |
| **Failed verification** | rollback if reversible; Intention → blocked; recorded honestly; **no hiding** (MEL-REQ-047) |
| **Emergency stop unreachable because the reasoner is hung** | the Emergency Stop path does not go through Ring 2 and does not require the reasoning ring to be responsive |
| **Projection loss** | loses nothing — rebuildable from the Chronicle; consequential reads bypass projections anyway |
| **Diagnostic-category / feature ontology leakage** | the data model has exactly four unit types (Event/Claim/Intention + Entity Registry); no music/task/habit/metrics subsystem exists (the only mentions are lines saying they *don't* exist) |
| **Domain-specific primitive creep for a new domain** | `kind` and relations are open; music/learning/SWE/science mapped with zero domain primitives (model §13); a novel domain fits the same way |

---

## 7. Governance-boundary analysis

**Structural (holds by construction):**
- Ring 2 → Ring 0: **no path.** Ring 2 emits claims/proposals; it has Query
  (read) + the Reasoner Interface only. It cannot write the Chronicle, invoke a
  capability, or touch Ring 0.
- Ring 3 → Ring 0 (runtime): **no path.** Capabilities receive narrow per-action
  grants; "widen my authority" and "edit Ring 0" are absent from the capability
  namespace.
- Reaching a decision → execution authority: **structurally separated.** The
  `proposed` → `authorised` transition requires an AUTHORISE artefact + a
  Governance Store grant; no reasoning output produces one.
- Grant monotonicity: **structural** (pending AU-2's representation) — authority
  narrows for free; widening needs a fresh AUTHORISE.

**Reliant on obedience / not yet structural / gaps:**
- **Ring 3 → Ring 0 (at-rest):** H1 — a File Access grant to the wrong path can
  overwrite Ring 0 files on disk. Correction required (signing + hard path
  exclusion).
- **Aggregation:** H2 — the routine class and composition are governed
  per-action, not by aggregate effect. Correction required.
- **Policy interpretation:** the Reference Monitor's "action within the authorised
  proposal" check is an *interpretation* step; for terminal/GUI it depends
  entirely on parsing the action structurally (H4).
- **Reasoning influencing governance-adjacent policy:** M4 — the
  routine/consequential boundary is Ring 1 policy a reasoner can propose widening;
  the user is the only backstop (a documented residual limit).
- **The human as the T6–9 weak link:** "MELFINA may assist" a human editing Ring 1
  code means a rubber-stamping human effectively lets MELFINA author a core
  change. Documented (`AUTHORITY_AND_SECURITY_MODEL.md` §9), preserved.

**Verdict:** the governance boundary is *mostly* structural. The two gaps that
matter (H1 at-rest integrity, H2 aggregation) are correctable within the current
design. The residual reliance on the user's judgement for policy-widening and
T6–9 assistance is inherent and honestly documented.

---

## 8. E²CI integrity analysis

| Question (from the mission) | Finding |
|---|---|
| Does the Chronicle preserve the Human/Central Model? | **Yes.** Event/Claim/Intention are the append-only unit types; Entity is a registry; Time is bitemporal coordinates on every unit; every unit carries provenance + (for Claims/Intentions) epistemic status. No unit type outside the four. |
| Are Event/Claim/Intention genuinely distinct? | **Yes**, and the architecture uses the distinctions: Events drive change (`brings-about`/`ends`); Claims carry epistemic status + confidence; Intentions carry `owner`/`target`/`status` and are the only thing Ring 2 emits as a proposal. Direction of fit (model P-7) is respected — a proposal is an Intention, not a Claim. |
| Is Entity consistent? | **Mostly.** Entity is thin (id + open `kind` + names). One wrinkle: `self` and `melfina` are Entities of special kinds *and* the two fixed `holder`/`owner` perspectives — model OQ-M9, unresolved, and the architecture inherits the ambiguity without worsening it. |
| Are relationships becoming hidden primitives? | **Partially — L2.** Most relations are ordinary Claims. But `brings-about`, `ends`, `supersedes`, `part-of` are *mechanically consumed* by Ring 1 (the Projection Engine, the Query bitemporal logic). This is model OQ-M1 biting exactly where predicted. The architecture does not resolve it (correct) but should acknowledge the special mechanical status. |
| Is State becoming a primitive? | **No** — current-state is a projection, not authoritative (AA-8), compatible with either answer to model OQ-M2. **But M1/M2** show the *discipline* is fragile: "context slices" as a standing projection, and DECIDE possibly reading a lagging current-state, both erode "derived, never authoritative." Correctable. |
| Are projections becoming authoritative? | **At risk — M2.** The stated rule ("consequential reads hit the Chronicle") must explicitly cover the DECIDE/classify step, which it does not clearly. |
| Do storage concerns leak into the model? | **Minor — L1.** transaction-time becomes a monotonic logical clock (an implementation reality) rather than a literal timestamp. The model's semantics survive; the wording drifts. |
| Does the architecture force today's implementation concepts onto the model? | **No.** No "LLM", "prompt", "context window", "agent", "tool call", "vector store", "fine-tune" appears as a model or architecture concept. Strategies and skills are Claims/Entities, not enums. The Reasoner Interface is deliberately generic. |
| Can future unknown domains fit without domain primitives? | **Yes** — resisted (§6). The one caveat is L6 (cyclical/subjective time), an edge case the linear bitemporal model handles imperfectly. |

**Verdict:** E²CI is faithfully preserved. The live risks are *discipline* risks
(M1, M2) — the architecture must not let projections quietly become the thing
consequential decisions read — and one honestly-inherited unresolved question
(OQ-M1, via L2). No model open question was resolved by this review.

---

## 9. Complexity analysis

**Three kinds of complexity, assessed separately.**

### Conceptual complexity
The architecture is: **1 substrate (append-only Chronicle) + 1 gate (pipeline +
Reference Monitor) + 1 governance region (Ring 0) + 2 sandboxed extension tiers
(reasoning, capabilities) + 1 derived-view layer.** That is ~6 concepts. The
"~25 subsystems" is a *responsibility* decomposition of those 6. **M7** is the
finding: this is not stated, so it reads heavier than it is. With M7's
one-paragraph fix, the conceptual load is within "one maintainer holds it."

### Which boundaries are what

| Boundary | Conceptual? | Runtime? | Security? | Removable? |
|---|---|---|---|---|
| Ring 0 ↔ everything | yes | **yes** (loaded/armed first) | **yes** (the meta-invariant) | **no** — removing it destroys MEL-REQ-235 |
| Ring 1 ↔ Ring 2 | yes | **yes** | **yes** (Ring 2 has no Chronicle write / capability invoke / network) | **no** — removing it fuses the autonomy triad and makes AI the source of truth |
| Ring 1 ↔ Ring 3 | yes | **yes** | **yes** (sandbox, no ambient authority) | **no** — removing it means a compromised capability is inside the trust boundary |
| Ring 2 ↔ Ring 3 | yes | **partly** (Ring 2 cannot invoke a capability) | **yes** (different authority levels) | **collapsible** — Ring 2 and Ring 3 could share one isolation mechanism at two authority levels; they are not two *tiers* so much as two trust levels in "the sandboxed outer region." **Recommended optional simplification (O2).** |
| Ring 3 ↔ Ring 4 | yes | future | **yes** (the lethal-trifecta break) | **no** (if Ring 4 ever exists) |
| within Ring 1: Chronicle / Query / Projection Engine / Entity Registry | yes | **no** — naturally one component ("the store") | shared | **collapsible in implementation** |
| within Ring 1: Pipeline SM / Reference Monitor placement | yes | Monitor is Ring 0, SM is Ring 1 | **yes** | **no** — the Monitor must be in the governed region |
| within Ring 1: Audit / Notification Gateway | yes | one component ("the edge") | Audit is integrity-sensitive | **collapsible in implementation** |

### Is the machinery justified?

| Machinery | Justified? | Note |
|---|---|---|
| **Supervision tree** | **concept yes; elaborate tree no** | "let it crash + restart" removes defensive error-handling everywhere — a net *simplicity* win. But a *hierarchical* tree is over-spec for ~10 Ring-1 services. **O4: start with a flat supervisor; add hierarchy only if the subsystem count grows.** |
| **Projections / CQRS** | **yes, but reducible** | pure event-sourced was rightly rejected (slow reads). But "N maintained projections" imports the CQRS operational tax the literature warns about (`[E]`). For a *single-user, single-consumer* system, **one materialised current-state cache + on-demand computed views** is enough. **O1: reduce "N projections" to "one current-state cache + on-demand views."** |
| **Blackboard controller** | **structure yes; the name over-commits** | the requirement is a *dynamic* controller (VC8–12). "Blackboard" specifies an *opportunistic* coordination discipline that may be more than MELFINA needs. **O3: soften to "a dynamic controller; blackboard-style opportunistic contribution is one candidate, to be chosen in CORE ENGINE."** |
| **Capability lifecycle** | **yes** | directly required (PART IV-B); every step is a governance gate; not reducible without weakening MEL-REQ-217–227. |
| **The Reference Monitor** | **yes, non-negotiable** | the meta-invariant and least-authority depend on it. |
| **The pipeline state machine** | **yes** | THINK→…→VERIFY must not be a loop (mission); the SM is the minimum that keeps the stages distinct and stoppable. |

**Verdict:** the *boundaries* are justified — each removable one, if removed,
weakens a real invariant (the table above). The *machinery* has three optional
simplifications (O1 projections, O3 blackboard naming, O4 flat supervision) and
one optional structural simplification (O2 collapse Ring 2/3 into one sandboxed
region at two authority levels). None is required. **The one required framing fix
is M7** (say "responsibilities, not components").

---

## 10. Dynamic-system analysis

**Is this genuinely dynamic, or a disguised static agent framework?**

| Capability (mission) | Architecturally supported? | How / caveat |
|---|---|---|
| dynamic context selection | **yes** | the Context Constructor builds context per situation from Query, decides inclusion/exclusion, records why (auditable). Not a stored structure. (M1 must be fixed so "context slices" isn't a standing projection.) |
| dynamic reasoning depth | **scaffolded** | the controller decides depth; Ring-0 budgets bound it. The *depth-choice algorithm* is CORE ENGINE — at architecture level this is "there is a controller that can go shallow or deep and stop early," not the mechanism. |
| dynamic strategy selection | **scaffolded, with M3** | strategies are Claims ("melfina will approach P via A"), not enums. Whether MELFINA can *create* a new strategy depends on M3 (are contributors fixed Ring 2 code or lifecycle capabilities?). Needs the M3 decision. |
| dynamic capability selection | **yes** | the controller consults the Registry + the reliability projection and picks/composes. |
| capability-gap recognition | **yes, heuristic deferred** | a `Claim` "no capability covers T"; the *recognition heuristic* (real gap vs "the model wants to build") is `requirements` OQ-17, CORE ENGINE. Gated by compose-before-code + test + risk-scaled AUTHORISE regardless. |
| compose-before-code | **yes** | the first branch of the lifecycle; enforced. (H2: composition must be aggregate-risk-classified.) |
| capability creation under governance | **yes** | the full lifecycle; risk class deterministic and not self-lowerable (AD-8); ≤ creator authority (M8 makes it explicit). |
| capability retirement | **yes** | a redaction Event ends "active"; the Entity + history stay; a replacement supersedes. |
| dynamic workflow adaptation | **yes** | a workflow is a `plan` (data); `supersedes` on the workflow-Entity as it changes; tier-5 governance for unattended changes. |
| dynamic resource allocation | **scaffolded** | Ring-0 budgets + a controller that can stop early / choose cheaper strategies under pressure (DEGRADED state). The *value-of-computation allocation algorithm* is CORE ENGINE. |
| choosing to do nothing | **yes** | a first-class decision outcome — a Claim, not an Intention (MEL-REQ-103). |

**Is it a disguised static list?** The *reasoning contributor set* is the one
place that could be static (M3). Everything else — capabilities, strategies,
workflows, context — is data or lifecycle-managed. **With the M3 decision made
(core contributors trusted + novel ones as lifecycle capabilities), the
architecture is genuinely dynamic, not a fixed agent framework.** The honest
caveat: much of the *substance* of dynamism (the depth-choice algorithm, the
relevance computation, the value-of-computation allocation, the gap heuristic) is
**deferred to CORE ENGINE** — at the architecture level, what exists is the
*scaffolding that makes those algorithms possible and bounded*, plus the guarantee
that they emit only proposals and are logged/explainable. That is the correct
division of labour for an architecture phase, but the review notes it so nobody
reads "dynamic" as "already solved."

---

## 11. Authority-pipeline attack results

THINK → DECIDE → PROPOSE → AUTHORISE → EXECUTE → VERIFY.

| # | Scenario | Containment | Recovery | Auditability | User visibility | Safe to continue? |
|---|---|---|---|---|---|---|
| 1 | **malicious reasoner** | proposal only; needs AUTHORISE; a genuinely destructive action won't classify routine (deterministic Ring-1 classifier). **Hole: aggregation (H2).** | n/a (no effect until authorised) | full — the proposal + its reasoning trace are appended | the proposal is shown (HITL) or shown-after (routine) | **yes**, once H2 is fixed |
| 2 | **malformed proposal** | **M10 — validation assumed, not specified.** | reject + record | should be recorded | reject reason shown | yes, once M10 is fixed |
| 3 | **compromised capability** | sandbox + per-action grant; can do only the grant's scope for one action. **Depends on grant granularity (H4).** | Host teardown; rollback if reversible; Event | full (Audit records the invocation + grant used + effect) | neutral notification / next pull | **yes** — contained; the capability is retired if it attempted escalation |
| 4 | **stale authorization** (revoked before EXECUTE) | Reference Monitor checks grant validity at EXECUTE → DENY | Intention → blocked; recorded | full | shown | yes |
| 5 | **authorization replay** (reuse a grant for a 2nd action) | grants are single-use, action-scoped, time-bounded (pending AU-2). | n/a | full | n/a | yes |
| 6 | **race condition** (two conflicting actions) | the SM serialises conflicting consequential actions. **M9 — conflict detection unspecified; conservative default = declared-scope overlap.** | n/a | full | n/a | yes, with M9's conservative default |
| 7 | **partial execution** | atomic appends; VERIFY-on-resume determines actual state; "may have happened" | rollback if reversible; honest recording | full | shown | yes |
| 8 | **failed verification** | rollback if reversible; Intention → blocked | as above | full | neutral notification | yes |
| 9 | **permission revoked *during* execution** | **H5 — unspecified.** | undefined until H5 is fixed | full (the revocation is recorded) | should be shown | **not until H5 is fixed** |
| 10 | **privilege-escalation attempt** | monotonic grants + Monitor always invoked → the escalation *cannot succeed*; the attempt is caught at VERIFY | capability retired | full | shown | yes |
| 11 | **recursive automation** (single) | loop detector + Ring-0 budget | halt; record | full | shown | yes. **Mutual recursion: M6.** |
| 12 | **lying / broken verifier** | **H3 — single point of trust; no redundancy.** | undefined — a false "verified" becomes an authoritative Event | the Verifier's output is recorded, but as truth | the user sees "succeeded" | **not until H3 is fixed** |

**Summary:** the pipeline resists 8 of 12 cleanly. Three need the incremental
fixes H3, H5, M9/M10; one (aggregation, feeding scenario 1) needs H2. **None
requires a redesign of the pipeline** — the THINK→…→VERIFY structure and the
autonomy-triad separation hold; the gaps are in *what VERIFY trusts*, *what
happens on mid-action revocation*, and *aggregate-effect governance*.

---

## 12. Local-only attack results

| Attack vector | Result |
|---|---|
| hidden network dependency in a core component | **none found** — Ring 1 has no network capability; startup, capture, memory, retrieval, time support, deterministic automations, search all work with networking disabled (CORE-READY is a complete state). |
| telemetry / analytics | **structurally absent** — no such subsystem; INV-1 / MEL-REQ-165. |
| cloud fallback | **structurally absent** — nothing degrades toward a remote; the local reasoner declines rather than reaching out (H6 makes this structural rather than behavioural). |
| crash reporting | **MELFINA: absent.** **OS-level: M11** — the host OS crash reporter can capture process memory (Chronicle + keys). Correction: opt the process out of OS crash reporting; keep decrypted data resident briefly. |
| update mechanism | **absent from the core** — no update checks (INV-1); updates are a human action with ordinary tools (consistent with T6–8 being a human path). |
| plugin communication | capabilities communicate via the pipeline + typed grants, not arbitrary IPC; no capability-to-capability channel except through a composed workflow the Monitor sees. |
| Ring 2 network access | **none** — Ring 2 has Query + the Reasoner Interface only, no network capability. **H6:** make the Reasoner Interface's inability-to-reach-remote structural, not a preference. |
| Ring 3 accidental network access | **L4** — "no ambient authority" should explicitly include "zero network capability; a socket cannot be opened from Ring 3." One sentence. |
| future Ring 4 isolation | **L5** — the "no {outbound + private-read} to one component" rule should be an explicit **Ring 0 governance rule**, not prose. |
| core-data → network leak (any path) | the only path is a future Ring 4 component the user physically adds; it is audited, off-by-default, and structurally trifecta-broken. Plus M11 (OS crash reporter) as an out-of-band path. |
| networked filesystem mount | **L3** — "local" writes could traverse NFS/SMB; STORAGE should detect and warn; the invariant is about MELFINA's intent. |

**Verdict:** the local-only invariant **holds** for MELFINA's own behaviour, with
two wording fixes (H6, L4), one Ring-0 rule to make explicit (L5), and two
out-of-MELFINA's-control paths to mitigate/warn about (M11 OS crash reporter, L3
network filesystem). **No core component secretly needs the network.**

---

## 13. Self-modification attack results

Tiers T1 config · T2 learned state · T3 skill creation · T4 skill modification ·
T5 workflow modification · T6 code · T7 subsystem · T8 architecture · T9
self-replacement.

| Attack | Result |
|---|---|
| **Where do the self-modification rules live?** | Ring 0 versioned files. **Runtime write path: closed.** **At-rest: H1** — a File Access grant to the wrong path can overwrite them on disk; runtime tampering doesn't affect the running system and is detected at startup, but the version chain can be forged unless signed. **Correction: sign the chain; hard-exclude the path.** |
| **Who can change them?** | a human editing the files. After H1: a human editing the files *and holding the signing key*. |
| **Can reasoning influence them indirectly?** | Ring 2 can *propose* a T1 policy change (routine/consequential boundary — M4) or persuade the user toward a Ring-0 edit (documented residual limit §9). Not directly. |
| **Can a capability modify them?** | Not via effect (absent from the namespace). Via File Access path — **H1**. |
| **Can a generated capability bootstrap stronger authority?** | **No** — MEL-REQ-222; a created capability's max grant is capped at the intersection of {user authorises} ∩ {creator's current authority}. **M8: make the cap an explicit Monitor check.** Transitively, nothing in a creation chain exceeds the original human grant. |
| **Can composition bypass the risk classifier?** | **Yes — H2 Attack B** (currently). Fix: classify a composed workflow by the union of component authorities + affected scope. |
| **Can risk ever be lowered by MELFINA?** | The classifier is deterministic (Ring 1); its input is the capability's declared authority, which MELFINA authors — but **under-declaring is self-defeating** (the sandbox ceiling = the declaration; an under-declared capability can't function and fails testing). So risk cannot be *usefully* lowered. **Preserve the invariant "sandbox ceiling ≡ declared authority" — it is what makes AD-8 sound.** |
| **T6–9 bootstrap?** | Not MELFINA operations; a human does them. **Soft spot: "MELFINA may assist"** a human editing Ring 1 — a rubber-stamping human lets MELFINA effectively author a core change. Documented (§9), preserved as a trade-off. The safest variant (MELFINA does *not* assist T6–9 at all) is very restrictive; the review does not force it but flags it as the strongest available hardening. |
| **Meta-invariant absolute?** | **No — and the architecture already says so** (`AUTHORITY_AND_SECURITY_MODEL.md` §9, MEL-REQ-239). Necessary, not sufficient. The review confirms: after H1's fix, the meta-invariant holds structurally at runtime and at rest; the residual limits are (1) social engineering of the user, (2) unintended Ring-1 bugs, (3) test-passing-but-badly-generalising capabilities, (4) the T6–9 "assist" soft spot. All documented. **No claim of absolute safety is made or should be.** |

**Verdict:** the self-modification model **holds** with two corrections (H1
signing + path exclusion; H2 aggregate classification of compositions) and one
clarification (M8). The meta-invariant is structural after H1. The documented
residual limits are real and correctly not papered over.

---

## 14. Failure-mode analysis

| Failure | Authoritative state | Trustworthy state | Recoverable? | Must stop? | Rebuild path |
|---|---|---|---|---|---|
| **Chronicle corruption** | the last valid appended unit | everything up to the corruption | yes (truncate to last valid / restore backup — user decides) | **yes** — enter read-only recovery; no silent repair | user-chosen |
| **partial write** | prevented — atomic appends; either the unit or nothing | all | yes (the failed append simply didn't happen) | no | n/a |
| **projection corruption** (data) | the Chronicle | the Chronicle | yes | no | replay from checkpoint; if that fails, rebuild from scratch |
| **projection divergence** (logic bug) | the Chronicle | the Chronicle | **M5 — undetected**; rebuild reproduces the same wrong output | no | consistency audit (recommended, M5) |
| **corrupted capability component** | the Registry metadata + the Chronicle | all core state | yes | no (that capability only) | restore a prior version; the capability is contained meanwhile |
| **runaway automation** | the Chronicle (its Events up to the halt) | all | yes | that automation | loop detector + Ring-0 budget halt; **M6 for mutual recursion** |
| **infinite / deadlocked reasoning** | the Chronicle (unchanged — Ring 2 emitted nothing) | all | yes | that activity | external Budget Authority watchdog kills it; "reasoning did not complete" Claim |
| **deadlocked controller** | the Chronicle | all | yes | that activity | **must be an external watchdog, not self-checked (M-note in FA8)** |
| **malicious skill** | the Chronicle | all core state | yes | that skill's execution | sandbox teardown + rollback + retire; **grant granularity (H4)** is the real bound |
| **prompt injection** (into a reasoning contributor) | the Chronicle | all core state | yes | the affected activity | the reasoner emits a *proposal* → HITL / classifier / VERIFY catch it; **aggregation (H2)** is the escape |
| **stale context** | n/a — context is transient, rebuilt per situation | n/a | yes (rebuild) | no | Context Constructor rebuilds |
| **stale authorization** | the Governance Store (the current grant/revocation) | all | yes | the action (DENY at EXECUTE) | n/a. **Mid-execution: H5.** |
| **verifier failure** (crash) | the Chronicle | all | yes | that verification | retry / a second Verifier (**H3** for a *lying* Verifier) |
| **inconsistent materialized state** | the Chronicle | the Chronicle | yes | no | rebuild; **M5** for silent logic-bug divergence |
| **disk full** | the last successful append | all | yes | consequential actions pause; capture retries | resume when space returns |
| **process crash** | the last successful append | all | yes | — | STARTING sequence; Chronicle integrity re-verified; in-flight pipeline actions resume from their last artefact or abort cleanly |
| **power loss** | the last durably-appended unit (atomic) | all | yes | — | as process crash + the storage layer's atomicity guarantee (deferred) |
| **interrupted execution** | the Chronicle; VERIFY-on-resume determines the real world state | all | yes (rollback if reversible) | the action | "may have happened" handling |

**Invariant across every row:** **the Chronicle is the authoritative and
trustworthy state; everything else is rebuildable; failure is contained to a
supervision subtree; nothing is surfaced with pressure or blame.** The three gaps
— M5 (silent projection divergence), M6 (mutual automation recursion), H3 (lying
Verifier) — are additive fixes, not structural failures.

---

## 15. Lightweightness analysis

**Conceptual complexity:** ~6 core concepts (substrate / gate / governance region
/ 2 extension tiers / derived views); ~25 *responsibilities* decomposed from them.
**M7** (say "responsibilities, not components") brings this within "one
maintainer holds it."

**Implementation complexity (real code to write):** the Chronicle + bitemporal
query; the Reference Monitor + grant system; a sandbox mechanism; a supervisor;
projection/checkpoint/replay; the pipeline state machine; the capability
lifecycle. This is a **substantial** system — more than a weekend project, less
than an OS. The optional simplifications reduce it: **O1** (one current-state
cache, not N projections) removes most of the CQRS tax; **O4** (flat supervisor)
removes the tree; **O2** (one sandboxed outer region at two authority levels)
removes a tier. With O1+O2+O4 the implementation surface is meaningfully smaller
without losing a single security boundary.

**Runtime resource cost (idle):**
- Resident: Ring 0 + the Ring 1 substrate. With O1 that's ≈ the Chronicle, the
  Query path, one current-state cache, the pipeline SM, Audit, the Notification
  Gateway, the Supervisor, the Registry, the Policy Store — realisable as a
  handful of modules in one process.
- Idle background: projection catch-up (bounded, yields), checkpointing (periodic),
  notification-window flush, due automations, spaced-review resurfacing,
  opportunistic regression tests (skipped under load). **No polling of Ring 2. No
  model kept warm. No network.**
- **Risk:** a poorly-scoped implementation makes each "subsystem" a process that
  polls. **AP-12 + M7 + O1/O2/O4** are the guardrails; concrete idle CPU/RAM
  budgets are set in LOW-LEVEL FOUNDATIONS.

**Verdict:** the architecture is *not* accidentally microservice-shaped —
provided M7 is stated and O1/O2/O4 are taken (or explicitly declined with
reason). The security boundaries that *must* be runtime boundaries (Ring 0 ↔ all;
Ring 1 ↔ sandboxed extensions) are few. The rest is decomposition for clarity.
**AP-12 is achievable; the docs currently oversell the component count.**

---

## 16. Required architectural changes

All six are **corrections that strengthen an existing boundary** — none is a
redesign. Each names the affected invariant/requirement.

| # | Change | Affects | From |
|---|---|---|---|
| **RC-1** | Ring 0 governance files: (a) cryptographically sign the version chain with a human-held key; refuse to run on an invalid signature; (b) hard, non-overridable exclusion of the Ring 0 path from any File Access grant (in the capability's code + a Reference Monitor rejection rule). | MEL-REQ-235 (the meta-invariant); INV-6 | H1 |
| **RC-2** | Add aggregate-effect governance: (a) a per-window aggregate budget across *all* routine actions of an activity chain; crossing it escalates the next action to consequential regardless of activity/window framing; (b) a composed workflow is risk-classified by the **union** of its components' declared authorities and affected scope, never per-step; (c) "practically irreversible in aggregate" is a risk-class input. | MEL-REQ-019, 181, 220, 227; INV-3 | H2 |
| **RC-3** | Verifier trust spec: deterministic, no ambient authority, read-only on the checked state, minimal, performs no effect; high-risk actions require ≥2 independent Verifiers agreeing; Verifiers are self-checked against known fixtures in the regression suite. | MEL-REQ-113, 142, 172, 221, 248 | H3 |
| **RC-4** | Re-tag AU-2 (grant representation) as **containment-critical**. State the constraint for LOW-LEVEL FOUNDATIONS: terminal/GUI grants are checked against a **parsed, structured action** (argv, no shell interposition), not a raw string; the grant vocabulary can express "no shell metacharacters / no interpolation / this argv shape"; shell invocation is a distinct higher-risk grant. | MEL-REQ-141–146; INV-7, INV-8 | H4 |
| **RC-5** | Specify mid-execution revocation: grants are live handles the Reference Monitor can invalidate; the Capability Host checks handle validity at every effect boundary; on mid-action revocation the capability is stopped, rolled back if reversible, and the partial state recorded as a failure. | MEL-REQ-177, 181 | H5 |
| **RC-6** | State the Reasoner Interface's locality **structurally**: Ring 1/Ring 2 have no network capability; the interface *cannot* bind a non-local endpoint regardless of configuration; a remote reasoner would be a Ring 4 component (out of scope). Update `SYSTEM_ARCHITECTURE.md` §22 and `AUTHORITY_AND_SECURITY_MODEL.md` §7 to say "cannot", not "does not". | INV-1; MEL-REQ-155, 164, 165 | H6 |

**Plus the required framing fix:**

| # | Change | From |
|---|---|---|
| **RC-7** | `SYSTEM_ARCHITECTURE.md` §2 + `README.md`: state that the ~25 subsystems are **responsibilities**, not necessarily separate processes/modules; give the natural groupings ("the store", "the gate", "the edge"). | M7 |

**Also fold in the small gaps** (one sentence / clause each): M2 (DECIDE reads
current-state from the Chronicle, not a lagging view), M3 (decide contributors'
ring/lifecycle status), M8 (explicit ≤-creator-authority Monitor check), M9
(conflict = declared-scope overlap, conservative default), M10 (proposal
structural validation before AUTHORISE), M1 (context is on-demand, not a
maintained projection), L4 (Ring 3 sandbox = zero network capability), L5
(the trifecta rule is a Ring 0 governance rule).

---

## 17. Optional architectural changes (not required)

| # | Change | Benefit | Cost of not doing it |
|---|---|---|---|
| **O1** | Reduce "N maintained projections" to **one materialised current-state cache + on-demand computed views**. | removes most CQRS operational tax; smaller resident footprint; simpler to hold in one head | acceptable — the architecture is still sound with N projections, just heavier |
| **O2** | Collapse Ring 2 and Ring 3 into **one sandboxed outer region at two authority levels** (reasoning-authority, capability-authority) sharing one isolation mechanism. | one fewer tier; one isolation mechanism to build and audit | acceptable — two tiers is not wrong, just more |
| **O3** | Soften "blackboard control" to **"a dynamic controller; blackboard-style opportunistic contribution is one candidate coordination pattern, to be chosen in CORE ENGINE."** | avoids over-committing the coordination discipline before CORE ENGINE evaluates it | acceptable — "blackboard" is a reasonable default, just premature to fix |
| **O4** | Start with a **flat supervisor** ("restart these N services"); add a hierarchical tree only if the subsystem count grows. | less machinery now | acceptable — a tree is not harmful, just more than needed at ~10 services |
| **O5** | Move the **routine/consequential boundary and per-area autonomy ceilings to Ring 0** (human-file-edit, monotonic) rather than Ring 1 policy. | closes M4 — autonomy-ceiling changes become as governed as the meta-invariant | acceptable to leave in Ring 1 *if* the interface treats "widen autonomy" proposals with reassurance-pattern-level restraint |
| **O6** | Add a periodic **projection consistency audit** (recompute a sample by direct Chronicle query, flag divergence). | closes M5 (silent logic-bug divergence) | acceptable — divergence is rare and eventually noticed; the audit just makes it fast |

---

## 18. Questions that should remain unresolved

Preserved deliberately; **do not** resolve these to make the architecture look
finished.

1. **The metareasoning controller's predictability** (MEL-REQ-253, Risk 3). "Same
   situation + grounding ⇒ consistent, explainable choice" is required but not
   demonstrable at the architecture level. CORE ENGINE, with its own evaluation.
2. **The meta-invariant's ultimate sufficiency.** After RC-1, it holds
   structurally at runtime and at rest. It does **not** defend against a capable
   reasoner persuading the user, an unintended Ring-1 bug, or a
   test-passing-but-badly-generalising capability. `AUTHORITY_AND_SECURITY_MODEL.md`
   §9; MEL-REQ-239. Cannot be closed at any phase — only mitigated.
3. **Whether MELFINA should assist with T6–9 at all** (FA7 soft spot). The
   strongest hardening (no assistance) is very restrictive. Trade-off; the user
   should decide.
4. **The grant representation and parsing granularity** (AU-2, now RC-4-flagged as
   containment-critical). LOW-LEVEL FOUNDATIONS.
5. **Model OQ-M1 (`Relation` a primitive?) and OQ-M2 (`State` a primitive?)** —
   the architecture is compatible with either answer (AA-8; L2). Do not resolve
   here.
6. **Model OQ-M5 (worries / intrusive thoughts / affect)** — if it becomes a
   primitive, it is another Chronicle unit type, not a new ring. Do not resolve
   here.
7. **Whether Reasoning Contributors are trusted code or lifecycle capabilities**
   (M3) — the review *recommends* a split but flags it as a genuine open design
   question if the user prefers otherwise.
8. **Where the Ring 0 / Ring 1 line falls for autonomy-relevant policy** (M4,
   O5) — arguments both ways; a real decision for the user / SYSTEM DESIGN
   follow-up.
9. **Semantic conflict detection between actions** (M9) — the conservative default
   (declared-scope overlap) is stated; refinement is CORE ENGINE.
10. **Concrete idle CPU/RAM budgets** — LOW-LEVEL FOUNDATIONS.

---

## 19. Final recommendation

**Accept the architecture as the working baseline, subject to applying the seven
required corrections (RC-1…RC-7) and folding in the listed small gaps.** None is a
redesign; all strengthen boundaries that already exist. After them:

- the local-only invariant is **structural** (RC-6);
- the self-modification meta-invariant is **structural at runtime and at rest**
  (RC-1) — with its documented residual limits intact and honest;
- aggregate-effect governance closes the routine-class / composition escape (RC-2);
- the VERIFY stage is no longer a single point of trust (RC-3);
- terminal/GUI containment rests on parsed actions, and AU-2 is correctly
  elevated to containment-critical (RC-4);
- mid-execution revocation is defined (RC-5);
- the subsystem count is correctly framed as responsibilities (RC-7).

**The optional simplifications O1–O4** (fewer projections, collapse Ring 2/3,
soften "blackboard", flat supervisor) are **recommended** — they materially reduce
implementation and runtime weight without touching a security boundary — but the
architecture is sound without them.

**The architecture does not fail.** It is coherent, the boundaries that must be
real are real, E²CI is faithfully preserved, it is genuinely dynamic (with the
substance appropriately deferred to CORE ENGINE), and it is honest about what it
cannot guarantee. The corrections above are the difference between "the
architecture *says* it cannot happen" and "the architecture *structurally makes
it impossible*" for the six places where that gap currently exists.

**Recommended next action:** the user reviews this document; then a short
`ARCHITECTURE MISSION 001 — REVISION 1` applies RC-1…RC-7 + the small gaps to the
`design/` files (a targeted edit pass, not a rewrite), after which the whole
architecture set is committed and pushed as one checkpoint. **Then** LOW-LEVEL
FOUNDATIONS, with AU-2 / RC-4 as its first and most containment-critical
deliverable.

---

*This review examined all ten focus areas. Every CRITICAL/HIGH finding cites
architecture, model, or requirements evidence. No model open question was
resolved; no requirement was weakened. `src/` unchanged; only this review
document was added.*
