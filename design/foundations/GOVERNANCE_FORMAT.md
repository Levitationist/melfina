# MELFINA — FOUNDATION 8: RING-0 GOVERNANCE LOGICAL FORMAT

**Phase:** LOW-LEVEL FOUNDATIONS — MISSION 001. First pass, pending review.
**Baseline:** `8a40207`. Read F1, F3, F7 first; F9 (integrity) is the companion.

**This document does not choose a concrete file format.** Not JSON, not TOML, not
a binary format, not a schema language. It fixes the **logical structure** of the
Ring-0 governance object — what it contains, how a version relates to its
predecessor, what metadata every version carries — and the rule that **governance
is not ordinary MELFINA knowledge**.

Derives from: **INV-3, INV-4, INV-5, INV-6, INV-13**; **RC-1, RC-2**;
**L5**; **AP-3, AP-5, AP-9**; **MEL-REQ-183, 233, 234, 235, 236, 237, 238, 239,
241, 245, 247**; the meta-invariant (`MEL-REQ-235`); the model's own §12.1
("the meta-invariant rules themselves … are governance, not modelled belief").

---

## 0. Dependencies discovered

- **On F1:** the governance object **owns the closed effect-class vocabulary**
  that F1 §3 defers to this document for, and the **class → risk-floor**
  and **class → required-check** maps the Monitor applies. Defined here in §4.
- **On F3:** the Reference Monitor **reads** governance at CHECK GOVERNANCE; the
  governance object is the source of the tier policy, risk floors, ceilings,
  trifecta rule, aggregate constraints, and emergency-stop behaviour it enforces.
- **On F9:** every property here about *versioning* and *human authorisation*
  depends on F9's integrity chain to be **trustworthy at rest**. This document
  says what a version *is*; F9 says how its authenticity is proven.
- **On F7:** governance sits in the Ring-0 region that F7 P9 says no Ring-3
  instance can reach and RC-1 says is excluded from every `file-*` grant.
- **Discovered constraint:** governance must be **loadable and checkable before
  the reasoning system starts** (F9 startup verification), so its format must be
  parseable by a minimal trusted loader with no dependency on Ring 1 or Ring 2.
  `[SEN]` (ordering) + `[ID]` (loader).

---

## 1. What the governance object IS

> The **governance object** is the constitutional configuration of MELFINA: the
> set of rules that constrain what MELFINA may do and how MELFINA may change
> itself, expressed **outside** MELFINA's belief store, changeable **only** by a
> human-authorised, integrity-verified version transition, and **read but never
> written** by any MELFINA component.

Properties:

- It is **not** a Claim, not an Entity, not an Intention. It is not in the
  Chronicle. The Chronicle *references* governance versions (an Event "governance
  advanced to version N, authorised by the maintainer at T") but does not
  *contain* the governance object. (Model §12.1, INV-3.)
- It is **not reachable by MELFINA's reasoning.** Ring 2 cannot read it as
  material to reason *with* toward changing it; Ring 2 may be *told* the current
  constraints (so it can propose within them), but the object itself is Ring-0.
- **MELFINA does not hold the authority to change it.** No pipeline path, no
  tier, no autonomous action produces a new governance version. Only a human,
  through the out-of-band authorisation path, does (F9). (INV-6, `MEL-REQ-235`,
  `236`.)
- It is **monotonic in restriction where the meta-invariant applies**: a new
  version cannot loosen the self-modification rules relative to what MELFINA
  could already do autonomously — only a human can, and only deliberately
  (INV-6). Other constraints (e.g. a risk floor) a human *may* raise or lower;
  the meta-invariant is the part that ratchets.

---

## 2. The governance object's contents

A governance version contains the following **constitutional elements**. Each is
logical content; none prescribes a syntax.

### 2.1 The meta-invariant

The rule that MELFINA cannot redefine its own self-modification rules
(`MEL-REQ-235`). Expressed as: the elements in §2.2–§2.9 are **governance-tier**,
and a change to any of them requires the human authorisation path (F9), never an
autonomous action, regardless of MELFINA's assessed competence or the assessed
benefit. Necessary; **not proven sufficient** against a highly capable
self-modifier — `[OPEN]`, OQ-19, `MEL-REQ-239`.

### 2.2 The self-modification tier policy

- The **nine tiers** (`MEL-REQ-233`), each with:
  - a **description** of what class of change it covers;
  - an **autonomy setting**: `autonomous` | `proposal-only` | `human-only`;
  - for `autonomous`/`proposal-only` tiers: the **authorisation granularity** —
    per-change, per-session, per-class, or standing (OQ-20, `[OPEN]` — the format
    must be able to *express* each, the choice is deferred);
  - a **risk floor** (§2.4) that changes in this tier cannot go below.
- **Tiers 6–9** (code / subsystem / architecture / self-replacement) are
  **`human-only`** and the format **must not be able to express them as
  `autonomous`** — this is a structural property of the format, not a
  configurable value (`MEL-REQ-236`). `[SEN]`.
- The **quiescent / introspection distinction** (`MEL-REQ-237, 238`):
  introspection and self-description are always allowed; acting on the result at
  tier ≥ some threshold is gated.

### 2.3 The closed effect-class vocabulary  *(owned here; F1 §3 references it)*

The complete set of effect classes a grant may authorise. **Closed** — adding a
class is a governance change. As of this pass:

| Class | Meaning | Default risk floor | Notes |
|---|---|---|---|
| `chronicle-append` | write a unit to the Chronicle | low | shape-checked (F6) |
| `chronicle-read` | authoritative read (F6 §3.1) | none | |
| `local-exec` | run one canonical `process-exec` (F2) | medium | never a shell |
| `local-shell` | run a `shell-exec` (F2) | **high** (floor) | distinct, rarer capability (RC-4, AU-2) |
| `gui-act` | one canonical `gui-op` | medium; **high** if coordinate-fallback | (F2 §2.3) |
| `file-read` | read a canonical path in-scope | low–medium | Ring-0 region **excluded** (RC-1) |
| `file-write` | write/create a canonical path | medium; **high** if delete/truncate/rename | irreversible ops floored high (F2, F4) |
| `capability-mint` | mint a child capability | ≥ the child's class; **≤ minter authority** (M8) | (F1 §9) |
| `notify` | surface a notification to the user | low | |
| `metered-resource` | consume a bounded metered resource | low–medium | counts against budgets |

**Permanently absent** (cannot be added without a constitutional change, and
some not even then):

- `widen-authority` / `raise-ceiling` / `self-grant` — **never** (INV-4).
- `edit-governance` / `write-ring-0` — **never** by any MELFINA path (INV-3, INV-6).
- `network-*` (`socket-open`, `http`, `dns`, …) — **absent from the core
  vocabulary** (RC-6, L4); a Ring-4 concern, not grantable to a core capability.
- any effect that would let tiers 6–9 run autonomously.

### 2.4 Risk floors

A map **effect-class (and sub-condition) → minimum risk class**. The Monitor
classifies an action as `max(declared, floor(class), floor(sub-conditions),
aggregate-escalation)` — a capability can never classify *below* the floor
(`MEL-REQ-227`, RC-2). Irreversible file ops, coordinate-fallback GUI, and
`local-shell` have **high** floors.

### 2.5 The lethal-trifecta prohibition (L5)

A Ring-0 invariant: **no activity chain may simultaneously hold** {access to
private data} ∧ {ingestion of untrusted content} ∧ {an outbound channel}
(`MEL-REQ-126`). Since the core has **no outbound channel at all** (INV-1), this
is currently satisfied structurally; the governance object still states it
explicitly so that any future Ring-4 work is checked against it. The Monitor
evaluates the **union** over an activity chain (RC-2), not per-action.

### 2.6 Authority ceilings

The **maximum authority** any grant may carry, per context / situational-autonomy
level (`MEL-REQ-244, 245`). MELFINA **cannot raise its own ceiling**
(`MEL-REQ-245`) — the format must not express a self-raising ceiling. Situational
autonomy *may* be pre-authorised by the human for a defined scope + window
(`MEL-REQ-244`); the format expresses that as a bounded ceiling with an expiry,
not an open-ended one.

### 2.7 Aggregate governance constraints (RC-2)

- The **per-activity-chain aggregate-effect budget** parameters: what counts as
  one activity chain, the aggregate ceiling(s), and the **cut-off** behaviour
  (hard stop, not throttle — `MEL-REQ-183`).
- The rule that a **composition** is classified by the **union** of authority +
  scope + aggregate effect of its parts, and that a composition of individually
  reversible steps may still be **"practically irreversible in aggregate"** and
  is then floored accordingly (RC-2, F1 §8).
- **No risk laundering**: splitting one high-risk effect into many low-risk
  requests does not lower the classification.

### 2.8 Emergency-stop behaviour

The logical spec of the emergency stop (`MEL-REQ-145, 187`; `FAILURE_AND_RECOVERY.md`
§7):

- What the stop **does**: no new AUTHORISE; revoke live grants (F4); signal
  running Ring-3 instances to halt, hard-terminate if they do not; the Chronicle
  and Ring 1 read-path stay up; MELFINA enters a safe, mostly-read state.
- Who can **trigger** it: the human, always, by an out-of-band mechanism that
  does not depend on Ring 1 or Ring 2 being healthy; the Supervisor / external
  watchdog on a defined class of anomaly.
- What it takes to **clear** it: a human action (never autonomous).
- The **graceful-shutdown** guarantee: shutdown at any point leaves a recoverable
  state (F6 §8, `MEL-REQ-187`).

### 2.9 Other constitutional invariants

The governance object also carries, as stated rules the Monitor / Supervisor
enforce:

- **Local-only** (INV-1, `MEL-REQ-164–169`): the core does no outbound network;
  data does not leave the machine without an explicit, per-instance human action.
- **AI-under-permission** (`MEL-REQ-153`): the reasoning system is a subject of
  the permission model, not its owner; it holds no special authority.
- **Autonomy triad not collapsed** (INV-2, `MEL-REQ-017, 018`): reason / select /
  perform stay separate; a `proposed` Intention never becomes `authorised`
  without a recorded AUTHORISE.
- **Deterministic-where-required** (`MEL-REQ-174`): the authorisation decision
  and other enumerated safety-critical paths are deterministic.
- **No authority from description / status / self-report** (F3 §5).

---

## 3. Versioning structure

Every governance **version** carries this metadata (logical, not syntactic):

| Field | Meaning |
|---|---|
| `version-id` | a stable, ordered identifier for this version |
| `parent-version-id` | the version this one supersedes; **null only for the genesis version** |
| `content` | §2.1–§2.9 |
| `created-at` | when this version was authored (wall clock, human context) |
| `authored-by` | the human maintainer identity (role, not PII in the doc) |
| `rationale` | a human-written note: what changed and why |
| `human-authorisation` | the out-of-band authorisation evidence — see F9 (a signature over the content + parent link, verifiable against the human-held trust root) |
| `integrity-metadata` | the hash of `content`, the hash of `parent-version-id`'s content, and the chained digest — see F9 |
| `activation` | the condition under which this version becomes the active one (e.g. "on next startup after authorisation", or a pinned explicit activation) |
| `retirement` | when/why a version stopped being active (set when a successor activates) |

**Rules:**

- The chain is **linear** (one parent, one active head). No branching governance.
  A "rollback" is a **new version** whose content equals an earlier version's,
  with its own parent link to the current head and its own human authorisation —
  never a pointer move backward (F9). `[SEN]` (the format cannot express a
  backward head move).
- A version is **immutable** once authored. Any change is a new version.
- **Exactly one version is active** at any time. The active version is
  determined at startup by F9's verification + the `activation` condition.
- MELFINA may **read** the active version's content and the chain's history; it
  **cannot author, sign, or activate** a version.

---

## 4. The class → governance maps (owned here)

The Monitor's CHECK GOVERNANCE step (F3 §6) consults, from the active governance
version:

1. **`class → risk-floor`** (§2.4) — for `CLASSIFY`.
2. **`class → required-checks`** — e.g. `file-write(delete|truncate|rename)` and
   `local-shell` and `gui-act(coordinate-fallback)` require a VERIFY plan and, if
   high-risk, ≥ 2 architecturally-independent verifiers (F10, RC-3).
3. **`tier → autonomy-setting`** (§2.2) — for any action that is itself a
   self-modification.
4. **`context → authority-ceiling`** (§2.6).
5. **activity-chain aggregate parameters** (§2.7).
6. **the trifecta predicate** (§2.5) over the activity chain.

All six are **data in the governance object**, not code in the Monitor — so a
constitutional change is a governance-version change, reviewed and human-signed,
not a code edit. `[SEN]` (the maps are governance data) + `[ID]` (the Monitor
correctly applies them).

---

## 5. What governance is NOT

- **Not** a place to store MELFINA's beliefs, preferences, or learned policy.
  Ordinary adaptive policy (which reasoning strategy to try, notification
  timing, phrasing) lives in Ring 1's Policy Store as revisable Claims —
  **not** here. The line: if changing it changes *what MELFINA may do or how it
  may change itself*, it is governance; otherwise it is policy. (INV-3, AP-9.)
- **Not** the enforcement mechanism. The governance object is the *rules*; the
  Reference Monitor + Supervisor + isolation mechanism are the *mechanism*
  (model §12.1). A rule with no enforcement point is a defect (validation).
- **Not** MELFINA-authored. Even the *rationale* and *content* of a new version
  are written by the human. MELFINA may **surface a case** for a change (an
  Intention proposing "governance should change to permit X"), but that Intention
  can only ever reach `proposed`, and its fulfilment is entirely a human act.

---

## 6. Contract Q&A

| Question | Answer |
|---|---|
| **Guarantees** | there is exactly one active, human-authored, integrity-verified constitutional configuration; it carries the tier policy, effect vocabulary, risk floors, ceilings, trifecta rule, aggregate constraints, emergency-stop spec, and constitutional invariants; MELFINA reads it and never writes it; the chain is linear and immutable; tiers 6–9 cannot be expressed as autonomous |
| **Requires** | F9's integrity chain + human trust root; a minimal trusted loader that runs before Ring 1/2; the Monitor applying the §4 maps faithfully |
| **Trusts** | the human maintainer's authorship + authorisation; F9's verification result |
| **Distrusts** | any MELFINA component proposing to write it; a governance file whose integrity check fails (F9 ⇒ refuse to run); MELFINA's assessment that a change is safe/beneficial as grounds to make it |
| **Enters** | (at authoring, out of band) a human-authored version + authorisation; (at runtime) nothing — it is read-only |
| **Leaves** | the active constraints, read by the Monitor / Supervisor / (as a summary) Ring 2 |
| **Malformed input** | a governance object that does not parse, or expresses a forbidden shape (tier 6–9 autonomous, a self-raising ceiling, a `widen-authority` class, a backward head move), is **rejected**; MELFINA **refuses to run** (F9) rather than running under a degraded constitution |
| **Failure** | governance unreadable / unverifiable ⇒ refuse to run (fail-closed, F9); governance readable but the Monitor cannot apply a map ⇒ deny the affected action (fail-closed, F3 §7) |
| **Authoritative** | the active, verified governance version is authoritative for every constraint it states; nothing overrides it at runtime |
| **Independently verifiable** | the *format* properties (no forbidden shapes) by a schema/structural check in the trusted loader; the *authenticity* by F9; the *application* by testing the Monitor against a governance fixture set |

---

## 7. Enforceability summary

| Invariant | Class |
|---|---|
| governance is outside the Chronicle / belief store; not a Claim/Entity/Intention | **[SEN]** (ontology — F5 excludes it) |
| MELFINA has no effect class that writes Ring 0 / governance (INV-3, INV-6) | **[SEN]** (F1 §3 vocabulary) |
| tiers 6–9 cannot be expressed as autonomous (`MEL-REQ-236`) | **[SEN]** (format constraint) + **[ID]** (loader rejects) |
| MELFINA cannot raise its own ceiling (`MEL-REQ-245`) | **[SEN]** (no self-raising ceiling expressible) + **[ID]** |
| the chain is linear, immutable, one active head; rollback = new forward version | **[SEN]** (format) + **[ID]** (loader) + **[F9]** for authenticity |
| exactly one active version, chosen before Ring 1/2 start | **[SEN]** (ordering) + **[ID]** (loader) |
| the §4 class→governance maps are data, not Monitor code | **[SEN]** (design) + **[ID]** (Monitor reads them) |
| the meta-invariant is sufficient against a capable self-modifier | **[OPEN]** — OQ-19, `MEL-REQ-239`; necessary, not proven sufficient |
| authorisation granularity per tier (per-change / session / class / standing) | **[OPEN]** — OQ-20; format must express all, choice deferred |
| local-only holds because the core has no outbound channel | **[SEN]** (INV-1) — the trifecta rule is then structurally satisfied |

---

## 8. Deferred (not decided here)

- The concrete file format (JSON / TOML / binary / a small DSL) — evaluated in
  F12 against: parseable by a minimal dependency-free loader; diffable for human
  review; signable as a byte sequence; hard to misedit.
- The **cryptographic** representation of `human-authorisation` and
  `integrity-metadata` — **F9**.
- The exact tier descriptions and the exact numeric risk floors / ceilings /
  aggregate ceilings — these are **content of the genesis governance version**,
  authored with the human, not fixed in this contract.
- OQ-20 (authorisation granularity), OQ-19 (meta-invariant sufficiency).
- Where the Policy Store / governance line falls for a handful of borderline
  cases (e.g. "may MELFINA change its own notification-frequency cap") — flagged
  for the human at genesis authoring.

## 9. Traceability

| Element | Source |
|---|---|
| the meta-invariant | `MEL-REQ-235`; INV-6 |
| nine tiers; full control set; tiers 6–9 never autonomous | `MEL-REQ-233, 234, 236` |
| quiescent state; introspection allowed, intercession gated | `MEL-REQ-237, 238` |
| invariants necessary but not perfect | `MEL-REQ-239`; OQ-19; VC10, VC12 |
| governance is not modelled belief; enforcement ≠ rules | model §12.1; INV-3 |
| aggregate-effect governance; union classification; no risk laundering | RC-2; `MEL-REQ-183` |
| lethal-trifecta prohibition is a Ring-0 invariant | L5; `MEL-REQ-126` |
| local-only; data does not leave the machine | INV-1; `MEL-REQ-164–169` |
| AI under the permission model, not its owner | `MEL-REQ-153` |
| cannot raise own ceiling; situational autonomy bounded + pre-authorised | `MEL-REQ-244, 245` |
| risk class not self-lowerable | `MEL-REQ-227` |
| emergency stop; graceful shutdown to safe state | `MEL-REQ-145, 187`; `FAILURE_AND_RECOVERY.md` §7 |
| at-rest integrity chain, human-held key | RC-1 → F9 |
| Ring-0 region excluded from every file grant | RC-1; F1 §3; F7 P4 |
