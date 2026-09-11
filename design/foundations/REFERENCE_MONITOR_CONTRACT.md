# MELFINA — FOUNDATION 3: REFERENCE MONITOR CONTRACT

**Phase:** LOW-LEVEL FOUNDATIONS — MISSION 001. First pass, pending review.
**Baseline:** `8a40207`. Read F1 (`CAPABILITY_GRANT_MODEL.md`) and F2
(`STRUCTURED_ACTION_MODEL.md`) first.

**This document does not choose a mechanism.** It fixes the Reference Monitor as
a *security contract*: the properties it must have, the decision procedure it
must run, what crosses its boundary, and — most importantly — **what it must
never trust**.

Derives from: **AP-6, AP-7**; **INV-2, INV-3, INV-6, INV-7, INV-8**;
**MEL-REQ-018, 153, 156, 179–187, 227, 235, 245**; the reference-monitor concept
(Anderson 1972 — NEAT + complete mediation + tamper-proof + verifiable; TCSEC B3),
Saltzer & Schroeder (complete mediation, fail-safe defaults, least privilege,
economy of mechanism), monotonic confinement `[E]`.

---

## 0. Dependencies discovered

- **On F1:** the Monitor is the **sole issuer** of grants and the **sole
  evaluator** of grant checks. F1 defines the grant object; F3 defines the
  authority that mints and checks it.
- **On F2:** the Monitor's "action within the authorised proposal" step is, for
  effect-bearing capabilities, a **structural match against a canonical action**.
- **On F4:** the Monitor holds the revocation authority; a revocation trigger
  reaches the Monitor, which invalidates the grant.
- **On F6:** every issuance/denial/revocation the Monitor performs is a
  Chronicle/Audit append. The Monitor **reads authoritative state through the
  Chronicle contract** where consequential correctness depends on it (M2).
- **On F8/F9:** the Monitor's rules (the effect vocabulary, the class→governance
  map, the tier policy, the aggregate parameters, the lethal-trifecta
  prohibition, the Ring-0 exclusion) are **Ring-0 governance data** it *reads*;
  it has no write path to them.
- **Discovered constraint:** the Monitor must be reachable and decisive **before
  Ring 1 performs any consequential operation** — it is loaded and armed in
  startup step 1 (RUNTIME_MODEL §2), before the Ring-1 substrate. `[SEN]`.

---

## 1. What the Reference Monitor is

> The **Reference Monitor** is the single, always-invoked, non-bypassable gate
> through which **every** consequential effect passes to be decided **allow /
> deny**, denying by default, using only explicit authority and structured
> facts, and recording every decision.

It is **Ring-0 governance**. It is **not** a subsystem MELFINA runs, extends, or
reasons about. It is a **mechanism + a set of rules**; the rules are versioned
files a human edits (F8, F9); the mechanism enforces them.

**It is the enforcement point for:** the autonomy triad (INV-2), least authority
(INV-7), the meta-invariant (INV-6), aggregate-effect governance (RC-2), the
lethal-trifecta prohibition (L5), and the self-modification tier policy.

---

## 2. The properties it MUST have

The classic reference-monitor properties (Anderson 1972, `[E]`), each with its
MELFINA-specific acceptance criterion and enforceability class:

| Property | MELFINA acceptance criterion | Class |
|---|---|---|
| **Complete mediation** | there is **no code path** from a proposed consequential effect to that effect occurring that does not pass a Monitor check. Every guarded effect boundary (F1 §5) calls it. | **[SEN]** as a design rule; **[ID]** that the built system has no unmediated path; **[ETL]** confirmed by red-team + a coverage argument over the effect classes |
| **Non-bypassable** | a component cannot reach an effect by *not asking* — Ring-3 capabilities start with **zero authority** and receive effect-power **only** as Monitor-issued grants (F1, F7); there is no "raw syscall" path out of the sandbox. | **[ID]** (a property of the isolation mechanism, F7); **[SEN]** for the rule |
| **Tamper-proof** | the Monitor's code and its rule-set (Ring-0 governance) cannot be altered by any Ring-1/2/3 operation at runtime **or at rest** (RC-1, F9); attempts are denied and recorded. | **[SEN]** (no write path in the effect vocabulary) + **[ID]** (at-rest signing, F9) |
| **Always invoked** | the Monitor is armed **before** Ring 1 starts (RUNTIME_MODEL §2); if it is unavailable, **no consequential effect proceeds** (fail closed, §7). | **[SEN]** for the ordering + fail-closed rule; **[ID]** that the runtime enforces the ordering |
| **Evaluable / verifiable** | the Monitor is **small enough to inspect and reason about**, a candidate for formal checking (AU-5); its decision procedure is **deterministic** given its inputs (§4). | **[ID]** (smallness is a build discipline) + **[ETL]** (formal or exhaustive checking, later) |
| **Fail-safe defaults** | the default answer is **deny**; a decision is `allow` only if **every** condition is affirmatively met. | **[SEN]** |
| **Economy of mechanism** | the Monitor does the *minimum*: match structured facts to explicit rules. It does **not** reason, plan, learn, call a model, or interpret intent. | **[SEN]** for the rule; **[ID]** for keeping it minimal in the build |

---

## 3. The decision boundary — what crosses it

**Into the Monitor** (a decision request), for one guarded effect *e*:

1. the **canonical structured action** for *e* (F2) — the *what* and *where*, as
   resolved identities, not strings;
2. the **grant reference** *g* being exercised (F1) — or, at issue time, the
   AUTHORISE artefact and the classifier output;
3. the **presenting component** identity (F1 `recipient`);
4. the **activity chain + window** *e* is charged to;
5. the **current authoritative state** the Monitor itself reads (M2) — not
   supplied by the caller: the Chronicle-contract read of "what holds now" where
   reversibility/precondition correctness matters, the emergency-stop signal, the
   Registry's risk class, the Ring-0 rule-set, the aggregate tally.

**Out of the Monitor** (a decision), exactly one of:

- **`allow`** — plus (at issue time) a minted grant; (at check time) nothing but
  the permission to proceed with *e*, and a decremented aggregate tally;
- **`deny`** — plus a **reason code** (from a closed set: `no-grant`,
  `out-of-scope`, `wrong-recipient`, `expired`, `revoked`, `over-budget`,
  `emergency-stop`, `tier-forbidden`, `trifecta`, `ring0-excluded`,
  `malformed`, `monitor-unavailable`, `state-unreadable`), and a recorded Event.

There is **no third outcome**. There is no `allow-with-warning`, no
`ask-the-model`, no `probably-fine`. `[SEN]`.

---

## 4. The decision procedure

The Monitor's authorisation predicate. `AUTHORIZED(e, g, s, t)` holds **iff every
conjunct holds** (extends F1 §17, adds the governance/trifecta conjuncts):

```
AUTHORIZED(e, g, s, t)  ⇔
      wellformed(g) ∧ issuer(g) = Monitor ∧ recipient(g) = presenter(e)
    ∧ status(for-intention(g)) = authorised
    ∧ effect-class(e) ∈ authority(g)
    ∧ canon-targets(e) ⊆ scope(g)  ∧  canon-targets(e) ∩ Ring0-region = ∅
    ∧ operation(e) ⊨ action-constraints(g)      -- structural match over F2
    ∧ ¬ any added-constraint(g) forbids operation(e)
    ∧ within(t, lifetime(g))  ∧  ¬ revoked(revocation-id(g), t)
    ∧ aggregate-charge(g).remaining ≥ cost(e)
    ∧ ¬ emergency-stop-active(s)
    ∧ ( is-self-change(e) → tier-policy-permits(classify-tier(e), governance-level) )
    ∧ ( ¬ would-complete-a-trifecta(grant-set-of(chain-of(g)) ∪ {g}) )       -- L5
    ∧ ( is-capability-mint(e) → declared-authority(e.new) ⊆ authority(g) ∩ user-authorised(e) )  -- M8
```

- **Deterministic** in its inputs: same `(e, g, s, t)` and same Ring-0 rule-set ⇒
  same decision (`MEL-REQ-253` at the mechanism level). `[SEN]` intent; `[ID]`
  that no non-deterministic input leaks in.
- **The state `s` the Monitor reads is authoritative** (M2): it reads "what holds
  now" via the Chronicle contract, not a projection, for any conjunct whose
  correctness is consequential (`status(for-intention)`, reversibility inputs to
  the tier/risk check, the emergency stop). `[SEN]` rule; `[ID]`.
- **`classify-tier` and `classify-risk` are deterministic Ring-1 mechanism**
  whose inputs are the change's/capability's **declared targets and authority**,
  never a reasoning claim (`MEL-REQ-227`, AD-8). The Monitor reads the result;
  it does not ask the reasoner. `[SEN]`.

---

## 5. What the Monitor MUST NOT trust

This is the heart of the contract. The Monitor's decision **never** takes any of
the following as an input:

| Must NOT trust | Why | Instead it uses |
|---|---|---|
| **a reasoner's claim** that an action is safe, low-risk, necessary, or authorised | a prompt-injected or mistaken reasoner would assert exactly this (INV-8) | the deterministic classifier + the AUTHORISE artefact |
| **a capability's self-description** of its own authority or risk | a compromised capability lies (INV-8, `MEL-REQ-185`) | the Registry's recorded declared authority + risk class |
| **the `status` of an Intention as evidence of execution authority** — `proposed` conveys nothing | collapsing propose→authorise is the autonomy-triad failure (INV-2) | a Monitor-issued grant tied to `status = authorised` + a recorded AUTHORISE |
| **a user-visible description generated by AI** (a proposal's prose, a summary) | prose can misrepresent the structured effect | the canonical structured action (F2) — the Monitor decides on structure, not narrative |
| **self-declared risk / self-declared "this is routine"** | `MEL-REQ-227` — MELFINA cannot reclassify its own change downward | the external classifier; the Policy-Store routine/consequential boundary (conservative default) |
| **a projection / current-state cache** for a consequential conjunct | it may lag (M2) | a fresh authoritative Chronicle read |
| **its own past `allow` as a licence to re-allow** without re-checking | a grant is a live handle; conditions change (revocation, budget, stop) | re-evaluation at **every** guarded effect boundary |
| **a raw command/GUI string as the scope of an effect** | shell/pixel ambiguity (RC-4) | the canonical structured action |
| **a request to widen a grant, edit Ring-0, or raise the autonomy ceiling** | these operations do not exist (INV-6) | deny — but there is nothing to deny against, because no such effect class exists (F1 §3) |
| **the network** as reachable from Rings 0–3 | RC-6, L4 — structurally absent | there is no `network-*` effect class in the core vocabulary |

`[SEN]` for the whole table — each row is a shape property of the contract, not a
behaviour the Monitor is asked to remember.

---

## 6. The request → verify sequence (mission Part 3)

The Monitor participates at two points of the pipeline; the full sequence:

```
  REQUEST      a proposed Intention reaches the Pipeline SM
     │
  VALIDATE     Pipeline SM structural validation (M10): well-formed target,
     │         resolvable references, scope expressible in the grant vocabulary,
     │         action canonicalisable (F2). Malformed ⇒ reject, record, STOP.
     │
  CLASSIFY     deterministic Ring-1 classifier: risk class from the UNION of
     │         declared authority + scope + aggregate effect (RC-2); tier class
     │         from what a self-change touches. NOT lowerable by Ring 2.
     │
  CHECK        the Pipeline SM routes AUTHORISE by class:
  GOVERNANCE     · routine/reversible/pre-authorised + within aggregate budget
     │             ⇒ policy default (HOTL: act-then-show)
     │           · consequential OR aggregate-crossing OR high-tier
     │             ⇒ route to the user (HITL: propose-then-wait)
     │
  AUTHORISE /  a recorded AUTHORISE artefact (a human decision, or the policy
  DENY         default for the routine class). DENY ⇒ recorded, Intention blocked.
     │
  ISSUE LIVE   the Monitor mints the grant (F1) for this authorised Intention:
  GRANT        scoped, time-bounded, revocation-id, aggregate-charge, risk-class,
     │         verification-requirement. Intention → status = authorised.
     │         (Nothing has happened yet — authorisation ≠ execution.)
     │
  EXECUTE      at every guarded effect boundary inside the sandbox (F7):
     │         the Monitor evaluates AUTHORIZED(e, g, s, t) (§4). PASS ⇒ the
     │         effect proceeds and the aggregate tally decrements. FAIL ⇒ deny,
     │         record, and (for revoked/over-budget/stopped) enter the
     │         revocation/partial-outcome path (F4).
     │
  VERIFY       Verifiers (F10) confirm the effect actually occurred, within the
               authorised canonical scope; high-risk ⇒ ≥2 independent mechanisms
               agree. Result recorded with the outcome vocabulary.
```

`[SEN]` for the sequence and the never-collapsed stages (INV-2, AD-6); `[ID]` for
the Pipeline SM and classifier implementations.

---

## 7. Malformed input and failure — fail closed

| Situation | Response |
|---|---|
| a decision request with a missing/invalid grant reference, an unknown effect class, an uncanonicalisable action, a scope that does not resolve | **deny**, reason code, record; the pipeline stops the action |
| the Monitor process is down / not yet armed / cannot load its rule-set | **no consequential effect proceeds**; the pipeline holds the action at AUTHORISE/EXECUTE; the user is told; Ring-1 non-consequential reads and capture continue (INV-9). `monitor-unavailable`. |
| the Monitor cannot read authoritative state (Chronicle unreadable for a consequential conjunct) | **deny** (`state-unreadable`); do not fall back to a projection for a consequential decision (M2) |
| the aggregate accountant / watchdog is down | autonomous (routine-class) actions **pause**; per-action HITL grants may still be checked and used |
| the rule-set fails its integrity check at startup (F9) | **MELFINA refuses to run** — the Monitor is not armed, so no consequential effect is possible anyway |
| the Monitor's own code is detected as altered | refuse to run; report; never self-repair (F9) |

**In every case the safe direction is "deny / do not proceed".** There is no
input, failure, or ambiguity that yields `allow`. `[SEN]`.

---

## 8. Contract Q&A

| Question | Answer |
|---|---|
| **Guarantees** | every consequential effect is decided allow/deny by an always-invoked, non-bypassable, deny-by-default gate using only explicit authority + structured facts + authoritative state; every decision is recorded; there is no path to an effect that skips it |
| **Requires** | it is armed before Ring 1 (RUNTIME_MODEL §2); an isolation mechanism with no unmediated effect path (F7); a total canonicaliser (F2); a Chronicle-contract read path (F6); an intact Ring-0 rule-set (F9) |
| **Trusts** | its own deterministic decision procedure; the Ring-0 governance rule-set (read-only); the deterministic classifier's output read from the Registry; the canonical structured action; its own fresh authoritative reads |
| **Distrusts** | everything in §5 |
| **Enters** | a decision request: canonical action + grant ref (or AUTHORISE artefact) + presenter + chain/window |
| **Leaves** | exactly one of `allow` (+ a minted grant at issue time) or `deny` (+ reason code); plus a recorded decision |
| **Malformed input** | **deny**, reason `malformed`, record; never coerce |
| **Failure** | **fail closed** — no consequential effect proceeds; recorded; Ring-1 non-consequential operation and capture continue |
| **Authoritative** | the Monitor's decision is authoritative for whether an effect may occur; the Ring-0 rule-set is authoritative for the rules; the Chronicle is authoritative for the state it reads |
| **Independently verifiable** | yes — the decision predicate (§4) is a pure function of `(e, g, s, ruleset, t)` and can be tested exhaustively over the closed effect-class vocabulary and a fixture set of grant/action/state combinations, including an adversarial set; the "complete mediation" property is checked by red-team + a coverage argument |

---

## 9. Enforceability summary

| Invariant | Class |
|---|---|
| deny-by-default; exactly two outcomes; economy of mechanism (no reasoning) | **[SEN]** |
| never trusts reasoner claims / capability self-description / proposal prose / self-declared risk / a projection / a raw string (§5) | **[SEN]** |
| autonomy triad not collapsed: `proposed` ≠ authority; grant only from an AUTHORISE | **[SEN]** |
| meta-invariant: no effect class "widen authority" / "edit Ring 0" / "raise ceiling" | **[SEN]** |
| lethal-trifecta prohibition enforced as a grant-set check the Monitor runs (L5) | **[SEN]** rule / **[ID]** the trifecta predicate over the effect classes |
| creator-authority ⊆ (M8) is a Monitor conjunct | **[SEN]** |
| reads authoritative state (M2), not a projection, for consequential conjuncts | **[SEN]** rule / **[ID]** |
| complete mediation — no unmediated effect path | **[ID]** (isolation mechanism, F7) / **[ETL]** (red-team + coverage) |
| non-bypassable — Ring 3 starts with zero authority | **[ID]** (isolation mechanism) |
| tamper-proof at runtime and at rest | **[SEN]** (no write path) + **[ID]** (F9 signing) |
| always invoked — armed before Ring 1; unavailable ⇒ fail closed | **[SEN]** rule / **[ID]** startup ordering |
| evaluable / small / formally checkable | **[ID]** build discipline / **[ETL]** later checking |
| meta-invariant sufficiency against a highly capable self-modifier | **[OPEN]** (OQ-19, `MEL-REQ-239`) — necessary, not proven sufficient |

---

## 10. Deferred (not decided here)

- The Monitor's implementation (in-process module / separate protection domain /
  kernel-mediated) and the isolation mechanism it relies on (F7).
- The concrete grant representation and check algorithm.
- The exact reason-code set beyond the closed list above (Ring-0 data).
- The formal-methods commitment (whether the Monitor is machine-checked — AU-5).
- The trifecta predicate's exact form over the effect classes (Ring-0 data, L5).
- The classifier algorithm (`CAPABILITY_MODEL.md` §5; CORE ENGINE).

## 11. Traceability

| Element | Source |
|---|---|
| non-bypassable, always-invoked, evaluable, tamper-proof, complete mediation | AP-6; reference-monitor concept (Anderson 1972), Saltzer & Schroeder `[E]` |
| deny by default; fail-safe defaults | Saltzer & Schroeder `[E]`; `MEL-REQ-179` |
| the action pipeline THINK→…→VERIFY, never collapsed; a distinct artefact per stage | `MEL-REQ-018`; INV-2; AD-6 |
| AI operates under the permission model, not as owner | `MEL-REQ-153` |
| least authority; capability-based, explicit scopes | `MEL-REQ-179`, `MEL-REQ-180` |
| risk class external, not self-lowerable | `MEL-REQ-227`; RC-2; AD-8 |
| the meta-invariant: rules external, monotonic, versioned, audited | `MEL-REQ-235`; INV-6 |
| MELFINA cannot raise its own autonomy ceiling | `MEL-REQ-245` |
| autonomous-action budgets as hard cut-offs | `MEL-REQ-183`; RC-2 |
| lethal-trifecta prohibition as a Ring-0 governance invariant | L5; `MEL-REQ-126` |
| assume adversarial content wins; contain blast radius | INV-8; `MEL-REQ-185` |
| every consequential action / grant / access logged | `MEL-REQ-156`, `MEL-REQ-182` |
| meta-invariant necessary-not-sufficient | `MEL-REQ-239`; OQ-19 |
