# MELFINA — FOUNDATION 1: CAPABILITY GRANT MODEL

**Phase:** LOW-LEVEL FOUNDATIONS — MISSION 001. First pass, pending review.
**Baseline:** `8a40207`. Read `../ARCHITECTURAL_PRINCIPLES.md`,
`../AUTHORITY_AND_SECURITY_MODEL.md`, `../CAPABILITY_MODEL.md` first.

**This document does not choose a representation.** It fixes what a capability
grant *is*, semantically, and the properties any representation must have. The
representation (opaque token / OS handle / typed resource / policy record checked
per call) and the parser for structured actions are `STRUCTURED_ACTION_MODEL.md`
+ later phases (AU-2, CONTAINMENT-CRITICAL).

Derives from: **RC-2, RC-4, RC-5, RC-6, RC-7**; **INV-2, INV-3, INV-7, INV-8**;
**MEL-REQ-124, 126, 127, 179, 180, 181, 182, 183, 184, 185, 222, 227, 245, 248**;
object-capability model, monotonic confinement, the caretaker/revocation pattern
`[E]`.

---

## 0. Dependencies discovered

- **On F2 (`STRUCTURED_ACTION_MODEL`):** a grant's *scope* for a terminal/GUI/file
  capability is expressed **over a structured action**, not a string. F1 defines
  the grant envelope; F2 defines what fills the `scope` and `action` fields for
  effect-bearing capabilities. F1 must not be read as permitting a string-matched
  command grant.
- **On F3 (`REFERENCE_MONITOR_CONTRACT`):** grants are **issued and evaluated
  only** by the Reference Monitor. F1 defines the object; F3 defines the
  authority to mint and check it.
- **On F4 (`REVOCATION_MODEL`):** the grant *lifecycle* (issued → active →
  suspended → revoked/expired → terminal) is F4. F1 fixes that a grant carries a
  revocation identity and is a live handle, not a bearer ticket.
- **On F5/F6 (Chronicle):** every *issuance*, *narrowing*, *revocation*, and
  *use* of a grant is a recorded Event/Audit entry. F1 fixes what must be
  recorded; F6 fixes the append contract.

---

## 1. What a capability grant IS

> A **capability grant** is a **conveyed, unforgeable, scoped, time-bounded,
> revocable authorisation for a specific bounded set of effects to occur, held by
> being referenced, issued by the Reference Monitor for one authorised
> Intention, and evaluable to exactly one of {this effect may occur, this effect
> may not occur}.**

It is **not**:

- a role, identity, or position ("MELFINA is trusted", "this is the reasoning
  component") — there is **no ambient authority** (`MEL-REQ-179`, AP-7); `[SEN]`.
- a bearer token in the sense of "whoever presents it may act" — it is a **live
  handle bound to a holder and a revocation identity** (RC-5); `[SEN]` for the
  semantics, `[ID]` for unforgeability of the concrete representation.
- a statement MELFINA's reasoning can produce, widen, or reclassify — grants
  originate only at the Monitor (RC-2, `MEL-REQ-227`, INV-2); `[SEN]`.
- persistent by default — a grant exists for the duration of the authorised
  Intention and is invalid afterward (`MEL-REQ-183`); `[SEN]`.

**The minimum question a grant exists to answer:** *"May this exact effect occur,
now, on behalf of this Intention?"* — see §17.

---

## 2. The grant envelope (fields)

Every grant carries the following. Field *meanings* are fixed here; field
*encodings* are deferred. A field marked **(structured, F2)** is filled by the
structured-action model for effect-bearing capabilities.

| Field | Meaning | Notes |
|---|---|---|
| `grant-id` | a fresh, unique, unguessable identifier for this grant instance | not derived from the holder, the Intention, or the action; two grants for the same action are distinct objects |
| `issuer` | always `the Reference Monitor` | there is no other issuer; a grant whose issuer is not the Monitor is not a grant `[SEN]` |
| `recipient` | the executing component (a Ring-3 capability instance, or Ring-1 acting for a user-invoked action) that may exercise this grant | bound; a different component holding the reference cannot exercise it `[ID]` |
| `for-intention` | the `Intention` id (`status = authorised`) this grant serves | one grant serves exactly one authorised Intention; see §8 |
| `authority` | *what kind of power* — the effect class (see §3) | drawn from a fixed, closed vocabulary of effect classes; **not** free text |
| `scope` | *over which resources / targets* the authority applies | Chronicle-unit selectors / path sets / **(structured, F2)** action shapes / window ids / metered-resource ceilings |
| `action-constraints` | *which specific operations* within the authority+scope are permitted, and any parameter bounds | **(structured, F2)** for terminal/GUI/file; enumerated operations for data-model grants |
| `risk-class` | the deterministic classification (§10) that gated this grant's issuance | copied from the Capability Registry / classifier at issue time; **read-only**; recorded |
| `aggregate-charge` | the activity-chain + window this grant's effects are charged against, and the remaining budget at issue (§11) | the Monitor decrements the chain's aggregate on use; a grant whose use would cross the budget is denied at check time |
| `lifetime` | issue instant + an expiry (wall-clock and/or step/among-effects count) | see §5; expiry is a hard bound, independent of revocation |
| `revocation-id` | the handle by which the Monitor invalidates this grant mid-use (RC-5) | see §5, F4; distinct from `grant-id` so a revocation can target a family (§7) |
| `provenance` | the AUTHORISE artefact, the deciding policy/tier rule, the classifier inputs, the human act (if HITL), the issue Event id | full chain; answers "why does this grant exist?" |
| `verification-requirement` | what VERIFY must confirm after the effect, and (for high-risk) that ≥2 independent mechanisms are required (F10) | copied from the risk class + capability declaration |
| `constraints` | any additional Monitor-imposed narrowing (rate, single-use, no-shell, observe-only, dry-run) | monotonic — constraints can only be *added* after issue, never removed |

**A grant with a missing or malformed required field is not a grant.** The
Monitor does not issue one; a holder presenting one is denied (§7, F3 §"malformed
input"). `[SEN]`.

---

## 3. Authority — the effect classes

`authority` is drawn from a **fixed, closed vocabulary of effect classes**. New
kinds of capability do not add effect classes; they compose existing ones. The
classes (the exact list is Ring-0 governance data — `GOVERNANCE_FORMAT.md` §2.3,
which owns this vocabulary; this is the shape):

| Effect class | Grants the power to… | Never includes |
|---|---|---|
| `chronicle-append` | append specified Event/Claim/Intention unit shapes | update, delete, or redact; reading; appending outside the declared shapes |
| `chronicle-read` | read specified Chronicle units / projections / bitemporal queries | any write; reading outside `scope` |
| `local-exec` | cause a **structured** process invocation (F2) within `action-constraints` | a raw shell string; a shell interposition (that is `local-shell`, a distinct, higher-risk class — RC-4); any argv not matching the constraint |
| `local-shell` | cause execution through a shell able to expand / chain / substitute | — (always high-risk; a distinct grant; never implied by `local-exec` — RC-4) `[SEN]` |
| `gui-act` | cause a **structured** GUI operation (target element/window + operation, F2) within constraints | a raw coordinate/keystroke stream where a structured form is practical |
| `file-read` / `file-write` | read / write within a **resolved, canonicalised** path set (F2) | any path resolving into the Ring-0 region — a **hard, non-overridable exclusion** (RC-1) `[SEN]` for the exclusion rule; any path outside `scope` |
| `capability-mint` | create/modify a capability whose declared authority is **≤ the minting context's own authority** (M8) | minting a capability with authority the minter does not hold; any tier-6+ effect `[SEN]` |
| `notify` | enqueue one item to the Notification Gateway within its rules | any other outbound path; bypassing the Gateway's schedule/dedup rules |
| `metered-resource` | consume up to a ceiling of a metered resource (compute time, steps) | exceeding the ceiling (hard cut-off — `MEL-REQ-183`) `[SEN]` |
| *(future)* `network-*` | **not in the core.** A Ring-4 concern; structurally absent from Rings 0–3 (RC-6, L4) | — `[SEN]` |

**Absent from the vocabulary, permanently** (INV-6, `MEL-REQ-235`): any effect
class whose meaning is "widen my authority", "edit Ring-0 governance", "change
the tier policy", "change the risk classifier", "raise the autonomy ceiling"
(`MEL-REQ-245`), or "modify MELFINA's own code / subsystem / architecture"
(tiers 6–9). These are **not well-guarded operations; they do not exist as
operations.** `[SEN]`.

---

## 4. Target / resource identity

`scope` names resources by **stable, unambiguous identity**, never by a
mutable or interpretable string:

- **Chronicle units** — by `unit-id` or by a **declared, bounded selector**
  (a kind + a bounded predicate over immutable fields); never "all Claims".
- **Files** — by a **resolved, canonicalised path set** (F2 §canonicalisation):
  symlinks followed, `.`/`..` collapsed, case/normalisation-form resolved,
  relative parts bound to a fixed root, *before* the grant is minted. The grant
  stores the **resolved** identity; a later resolution that would differ is a
  denial, not a silent re-resolution (TOCTOU defence). `[ID]`.
- **Processes / GUI targets** — by structured descriptors (F2), not by pattern
  match on a command line or a screen region.
- **Metered resources** — by resource name + a numeric ceiling.
- **Windows** — a `window-id` from the Budget Authority for aggregate accounting.

**Rule:** if a resource's identity cannot be resolved to a stable form at issue
time, the grant is **not issued** (`fail closed`). `[SEN]`.

---

## 5. Lifetime, expiry, and "live handle"

A grant is a **live, revocable handle**, not a ticket consumed once at EXECUTE
(RC-5):

- **`lifetime`** is a hard upper bound: a wall-clock expiry **and/or** a bound on
  the number of guarded effects / steps. Reaching it invalidates the grant with
  no further act. `[SEN]` for the semantics; `[ID]` for enforcement.
- **Validity is re-checked at every guarded effect boundary**, not once. The
  executing sandbox (F7) presents the grant reference to the Monitor's check path
  before *each* effect; a grant that has expired, been revoked, or been narrowed
  below the requested effect fails that check. `[ID]` (the "every boundary" check
  is a property the sandbox mechanism must provide).
- **`revocation-id`** lets the Monitor mark the grant invalid **between** two
  guarded effects, atomically, taking effect before the next check (F4). `[SEN]`
  semantics; `[ID]` atomicity.
- A grant is **never renewed in place**. Continuing past `lifetime` requires a
  fresh AUTHORISE and a fresh grant.

**Staleness detection.** A grant is **stale** if any of: `lifetime` passed;
`revocation-id` marked invalid; `for-intention` left `status = authorised` (was
fulfilled/blocked/abandoned); the aggregate budget for its chain/window is
exhausted; the risk class recorded on it no longer matches the Registry (a
capability version changed under it — the grant is invalidated, not silently
re-bound). The Monitor treats a stale grant exactly as a missing grant: **deny,
record**. `[SEN]`.

---

## 6. Attenuation (narrowing)

- A holder MAY **drop** a grant or request a **narrower** one (fewer operations,
  smaller scope, tighter constraint, shorter lifetime). Narrowing is free and
  needs no fresh AUTHORISE. `[SEN]`.
- **Widening is impossible without a fresh AUTHORISE.** There is no operation
  "add authority to this grant". A holder needing more presents a new proposal
  through the pipeline (INV-2, AP-7). `[SEN]`.
- Narrowing produces a **new grant object** deriving from the original
  (recording `derived-from`); the original may be revoked independently, which
  revokes the derived one (§7). `[ID]` (the derivation graph is a mechanism
  property; the *rule* is fixed).
- **Monotonicity is a global invariant:** across the whole system, the effective
  authority reachable from any starting grant can only *decrease* over time
  without a human act at Ring-0 or a fresh AUTHORISE (monotonic confinement,
  `[E]`; `MEL-REQ-222`). `[SEN]` as a rule; `[ETL]` that a given implementation
  actually has no widening path — a red-team target.

---

## 7. Delegation and sub-capabilities

- **Delegation is allowed only downward and only through the Monitor.** A
  capability MAY delegate to a sub-capability (`MEL-REQ-127`), but the
  sub-capability receives its grants **from the Reference Monitor**, deriving from
  the parent's, never by the parent handing over its own reference. Delegation is
  **not a way to escape authorisation** (`MEL-REQ-127`) — the sub-capability's
  action runs the same pipeline and the same Monitor check. `[SEN]` (the rule);
  `[ID]` (that the mechanism has no "hand my handle to a peer" path).
- A delegated grant is **≤ the delegator's grant** on every dimension (authority,
  scope, constraints, lifetime, remaining aggregate budget). `[SEN]`.
- **Revocation is transitive.** Revoking a grant revokes every grant derived from
  it, recursively, atomically from the Monitor's view (this is the seL4
  capability-derivation-tree / KeyKOS "sever" semantics, `[E]`). `[ID]`. The
  contract fixes the *semantics*; the cost (seL4's `revoke` is among its longest
  operations, `[E]`) and the mechanism are LOW-LEVEL FOUNDATIONS build concerns.
- **No caretaker/indirection object is exposed to Rings 2–3 as a way to
  self-manage revocation.** Revocation authority stays at Ring-0 (F3, F4). A
  capability cannot make itself un-revocable. `[SEN]`.

---

## 8. Composition and combination

- **A grant serves exactly one authorised Intention.** A workflow that runs
  several capabilities holds several grants, one per authorised effect, each
  minted for that step. There is no "workflow grant" that is the sum. `[SEN]`.
- **Composition never lowers risk (RC-2).** The workflow-Intention's risk class,
  set before any grant is minted, is the **union** of its component capabilities'
  declared authority, declared scope, and **aggregate effect over the window**.
  The Monitor mints each step's grant only if the *workflow's* class was
  authorised at that level. Splitting a high-risk effect into individually-trivial
  steps does not yield trivial grants — the classifier sees the union. `[SEN]`
  (the union rule); `[ID]` (that the classifier's inputs are the declared
  authority/scope and not a reasoning claim); `[ETL]` (that a real composition
  cannot be crafted to under-declare — a red-team target, mitigated by "sandbox
  ceiling ≡ declared authority": an under-declared capability cannot perform its
  function and fails VERIFY / the fixed suite).
- **Two grants held at once do not combine into a third authority.** Holding
  `file-read(/a)` and `local-exec(cat /b)` does not yield `file-read(/b)`. Each
  guarded effect is checked against the *one* grant that authorises it. `[SEN]`.
- **"Practically irreversible in aggregate"** (N reversible effects whose combined
  rollback is impractical) is an **input to the risk class** (RC-2), so a chain
  of individually-reversible grants that together cannot be undone re-gates as
  consequential before the crossing grant is minted. `[SEN]` (the rule); `[ID]`
  (the aggregate-reversibility metric is a LOW-LEVEL FOUNDATIONS parameter).

---

## 9. Creator authority (M8) — grants that mint capabilities

A `capability-mint` grant is special: its effect is to add a capability to the
Registry with a **declared authority**. The Monitor enforces, structurally
(M8, `MEL-REQ-222`):

> `declared-authority(new-capability) ⊆ { what the user authorised for this
> creation } ∩ { the authority the minting context currently holds }`

- The check is a **Monitor operation**, not a convention the capability-lifecycle
  code is trusted to honour. `[SEN]` (that it is the Monitor's check); `[ID]`
  (that "⊆" is computed over the closed effect-class vocabulary, not inferred).
- A capability **cannot be born holding authority its creator did not hold**, and
  **cannot grant itself more later** (§6 monotonicity). `[SEN]`.
- Tier-6+ effects (own code / subsystem / architecture) are **not mintable** —
  `capability-mint` cannot produce them because they are absent from the effect
  vocabulary (§3). `[SEN]`.

---

## 10. Risk classification — carried, not decided, by the grant

- The `risk-class` field is **copied onto the grant at issue time** from the
  deterministic Ring-1 classifier, whose inputs are the capability's **own
  declared authority + declared scope + aggregate effect** (RC-2, AD-8). The
  Monitor reads the class **from the Registry**, never from a reasoning claim
  (`MEL-REQ-227`). `[SEN]`.
- Ring-2 **cannot lower** it; the classifier is mechanism; the class is
  governance data. `[SEN]`.
- The exact class→governance mapping (which class needs HITL, which needs ≥2
  verifiers, which tightens the aggregate budget) is **Ring-0 governance data**
  (`GOVERNANCE_FORMAT.md`), set at LOW-LEVEL FOUNDATIONS / CORE ENGINE, not here.
  Only the invariants are fixed: **class up ⇒ governance up; class never lowered
  by MELFINA; class recorded on the grant.**

---

## 11. Aggregate-effect budget (RC-2)

- Every grant records the **activity chain** and **window** its effects are
  charged against, and the **remaining aggregate budget** (count and/or
  affected-scope magnitude) at issue.
- On each guarded effect, the Monitor **decrements** the chain's aggregate tally.
  A guarded effect that would cross the chain's aggregate budget is **denied**;
  the next action in that chain is **re-gated as consequential (HITL)** —
  regardless of how the chain is framed as activities or spread across windows
  (RC-2, Budget Authority). `[SEN]` (the rule); `[ID]` (the accounting mechanism,
  window length, and magnitude metric are Ring-0 parameters set later).
- The aggregate tally is maintained by a **deterministic Ring-1 accountant** and
  enforced by an **external watchdog** (RUNTIME_MODEL §4), **not** self-checked by
  the reasoner or the running capability. `[ID]`.

---

## 12. Conflict representation

Two grants (or two in-flight actions) **conflict** when their **declared scopes
overlap** — the resource identities in one grant's `scope` intersect the other's
(M9, conservative default). Absent stronger, proven semantics:

- **overlap ⇒ the pipeline serialises** the two actions; they do not run
  concurrently. `[SEN]` (the rule); `[ID]` (scope-intersection is decidable over
  the identity forms in §4).
- A stronger conflict model (semantic commutativity, disjoint sub-regions) is a
  **later refinement, never a relaxation** of this default (M9). `[OPEN]` how far
  it can be refined safely.
- A conflict is **recorded** (an Event: "action B serialised behind action A,
  scope overlap on <resource>") so the behaviour is auditable, not silent.

---

## 13. Provenance

Every grant's `provenance` chains to: the **AUTHORISE artefact** (user decision
or the policy default for the routine class); the **deciding rule** (which tier /
risk-class / aggregate-budget rule applied); the **classifier inputs**; the
**human act** (for HITL) with its Audit id; and the **issue Event**. From any
grant, "why does this authority exist, and who is accountable?" is answerable by
walking `provenance` (`MEL-REQ-182`, AP-4). `[SEN]` (the requirement to carry it);
`[ID]` (completeness of the chain).

---

## 14. What the grant model trusts / distrusts

| Trusts | Distrusts (must never be an input to issuing or checking a grant) |
|---|---|
| the Reference Monitor's own decision procedure (F3) | any reasoner claim about what authority is "needed" or "safe" or "low-risk" |
| the deterministic risk classifier's output, read from the Registry | a capability's self-description of its own risk or authority |
| the AUTHORISE artefact and (for HITL) the recorded human act | the *status* of an Intention as evidence of authority (a `proposed` Intention conveys nothing; only an AUTHORISE grant does — INV-2) |
| the Chronicle's authoritative state where consequential correctness depends on it (M2) | a projection / current-state cache for a consequential authority decision |
| Ring-0 governance data (the effect vocabulary, class→governance map, aggregate parameters) | a user-visible description generated by AI as the basis for a grant (the grant is minted from structured fields, not prose) |
| the structured, canonicalised action (F2) | a raw command/GUI string as the scope of an effect grant (RC-4) |

`[SEN]` for the whole table — these are shape properties of the contract.

---

## 15. Malformed input

| Malformed thing | Response |
|---|---|
| a grant request with a missing/unknown `authority`, unresolvable `scope`, or an effect class outside the vocabulary | **not issued**; recorded as a denied issuance with the reason; `fail closed` |
| a grant reference presented at a guarded effect that is syntactically invalid, unknown to the Monitor, or not bound to the presenting `recipient` | **deny the effect**, record, do not partially proceed |
| a grant whose `risk-class` field disagrees with the Registry | treated as **stale** → deny, record (§5) |
| a `capability-mint` request whose `declared-authority` is not a subset of the minter's authority | **denied**; recorded (M8) |
| a scope naming a path that resolves into the Ring-0 region | **denied**, non-overridably; recorded as an attempted Ring-0 access (RC-1) |

`fail closed` is the default in every row. `[SEN]`.

---

## 16. Failure

- **Monitor unavailable / cannot decide** ⇒ **no grant is issued and no guarded
  effect proceeds** (fail closed). The pipeline stalls the action at AUTHORISE or
  EXECUTE and records it; the user is informed. There is no "proceed without a
  check". `[SEN]`.
- **Grant record lost / unreadable mid-action** ⇒ the next guarded-effect check
  fails ⇒ the action stops and enters the revocation/partial-outcome path (F4).
  `[SEN]`.
- **Aggregate accountant / watchdog failure** ⇒ autonomous (routine-class)
  actions are **paused** until it is restored (no unbudgeted autonomous effect);
  user-driven HITL actions may proceed under their per-action grant. `[ID]`.

---

## 17. The minimum information to evaluate "may this exact effect occur?"

For a single guarded effect *e* proposed by holder *h* against a grant *g* at
time *t*, the Monitor needs exactly (F3 formalises the predicate):

1. *g* exists, is well-formed, `issuer = Monitor`, `recipient = h`;
2. `for-intention(g)` is still `status = authorised`;
3. *e*'s effect class ⊆ `authority(g)`;
4. *e*'s target(s), **resolved and canonicalised**, ⊆ `scope(g)` and disjoint
   from the Ring-0 region;
5. *e*'s operation + parameters satisfy `action-constraints(g)` **evaluated over
   the structured action** (F2), and no added `constraints(g)` forbid it;
6. *t* is within `lifetime(g)` and `revocation-id(g)` is not marked invalid;
7. charging *e* to `aggregate-charge(g)`'s chain/window stays within budget;
8. no emergency stop is active;
9. for a self-change: the tier classification permits it at this governance level.

**All nine must hold. Any one failing ⇒ deny, record, and (for 6/7/8) enter the
revocation path.** Nothing else — not a reasoner's opinion, not the Intention's
proposal text, not a capability's self-assessment — is consulted. `[SEN]`.

---

## 18. Enforceability summary

| Invariant | Class |
|---|---|
| Grants originate only at the Monitor; no ambient authority | **[SEN]** |
| Effect vocabulary is closed; "widen authority" / "edit Ring 0" absent from it | **[SEN]** |
| Grant serves one authorised Intention; composition classified by union (RC-2) | **[SEN]** rule / **[ID]** classifier inputs / **[ETL]** un-crafted-under-declaration |
| Widening impossible without fresh AUTHORISE; global monotonicity | **[SEN]** rule / **[ETL]** no-widening-path in the built system |
| Grant unforgeable; bound to recipient | **[ID]** (representation) |
| Live handle re-checked at every guarded effect; revocable between effects | **[SEN]** semantics / **[ID]** the "every boundary" check |
| Ring-0 path hard-excluded from every file grant | **[SEN]** |
| Creator-authority ⊆ check (M8) is a Monitor operation | **[SEN]** rule / **[ID]** subset computation |
| Aggregate budget decremented per effect; crossing re-gates as consequential | **[SEN]** rule / **[ID]** accounting parameters |
| Conflict = declared-scope overlap ⇒ serialise (M9) | **[SEN]** rule / **[ID]** intersection decidability |
| `fail closed` everywhere | **[SEN]** |
| Revocation transitive over the derivation graph | **[ID]** |

---

## 19. Deferred (not decided here)

- The concrete **representation** (opaque token / sparse capability with a MAC /
  OS handle / typed WASI-style resource / a policy record keyed by `grant-id`).
- The **derivation-graph** data structure and the cost/algorithm of transitive
  revocation.
- The **aggregate-budget parameters** (window length, magnitude metric, per-chain
  defaults) — Ring-0 governance data.
- The **effect-class vocabulary's exact contents** and the **class→governance
  map** — Ring-0 governance data (`GOVERNANCE_FORMAT.md`).
- The **aggregate-reversibility metric** (when N reversible effects count as
  irreversible-in-aggregate).
- How `scope` selectors over Chronicle units are expressed (F5/F6).

## 20. Traceability

| Element | Source |
|---|---|
| grant is unforgeable, scoped, revocable, held-by-reference, no ambient authority | `MEL-REQ-179`, `MEL-REQ-180`; object-capability model `[E]`; AP-7 |
| live revocable handle, re-checked, invalidated mid-use | RC-5; `MEL-REQ-145`, `MEL-REQ-177`, `MEL-REQ-187` |
| structured action as scope, shell distinct | RC-4; `MEL-REQ-143`–`146` |
| union risk class, no laundering through composition | RC-2, AD-8; `MEL-REQ-227` |
| creator authority ⊆ (M8) as a Monitor check | M8; `MEL-REQ-222` |
| aggregate budget, external watchdog | RC-2; `MEL-REQ-183`; Budget Authority |
| conflict = declared-scope overlap (M9) | M9; `MEL-REQ-010` |
| no network effect class in the core | RC-6, L4; `MEL-REQ-014`, `MEL-REQ-126` |
| "widen authority" / tiers 6–9 absent from the namespace | INV-6; `MEL-REQ-235`, `MEL-REQ-236`, `MEL-REQ-245` |
| grants recorded, provenance-carrying | `MEL-REQ-182`; AP-4 |
