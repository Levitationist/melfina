# MELFINA — AUTHORITY AND SECURITY MODEL

**Phase:** SYSTEM DESIGN / ARCHITECTURE — MISSION 001. First pass, pending review.
**Revision 1 applied** — RC-1 (Ring 0 at-rest signing), RC-2 (aggregate-effect
governance), RC-3 (verifier trust model, §4.1), RC-4 (structured terminal
actions), RC-5 (mid-execution revocation), RC-6 (reasoner locality structural),
M8, L4, L5. See `ARCHITECTURE_ADVERSARIAL_REVIEW.md` "Revision 1 — resolution
status".
Baseline `f3093fc`. Read `ARCHITECTURAL_PRINCIPLES.md`, `SYSTEM_ARCHITECTURE.md`
first. **No technology chosen** — the grant/token representation, the action
parser, the Ring-0 signing scheme, and the isolation mechanism are LOW-LEVEL
FOUNDATIONS choices constrained by this
document.

Covers mission items **J** (authorisation flow), **X** (self-modification
governance), **AA** (security/isolation boundaries), and the threat model.

---

## 1. The authority model in one paragraph

MELFINA has **no ambient authority** anywhere. Every ability to read private data,
change the Chronicle, run a capability, control the terminal or GUI, touch a file,
or (future) reach the network is an **explicit, unforgeable, scoped, revocable,
monotonic grant** — held only by holding a reference, never by identity or
position. Grants are issued by **Ring 0's Reference Monitor** for a **specific
authorised action** and expire with it. The rules governing what may be granted,
and the rules governing changes to MELFINA itself, live in **Ring 0**, which every
outer ring can **read** and **none can write**. (Object-capability model `[E]`;
reference monitor `[E]`; monotonic confinement `[E]`.)

## 2. The autonomy triad, enforced (J)

| Layer | What it is | Where it lives | How it is bounded |
|---|---|---|---|
| **Cognitive autonomy** | reason, infer, plan, evaluate options, form a preferred answer | Ring 2 (Metareasoning Controller + Contributors) | Ring-0 budgets (steps/time); produces **claims + proposals only**, never effects |
| **Decision autonomy** | select a preferred course of action (an internal choice) | Ring 2 (the Controller reaches a `proposed` Intention) | reaching it grants **nothing** about executing it |
| **Execution authority** | actually perform the action | the **AUTHORISE grant** in the Governance Store + the Reference Monitor at EXECUTE | scoped to the one action; time-bounded; revocable; monotonic |

**There is no single "agent" permission.** Holding cognitive autonomy does not
grant decision autonomy; holding decision autonomy does not grant execution
authority (INV-2, MEL-REQ-017).

### Authorisation flow

```
   a proposed Intention (from Ring 2, or a user-invoked capability request)
        │
        ▼
   classify (deterministic, Ring 1):
     · base class from declared authority + scope + reversibility
     · for a COMPOSED workflow: class = UNION of every component's authority,
       every component's scope, and the aggregate effect (RC-2). Composition
       can only raise the class, never lower it — a high-risk action cannot be
       laundered into several low-risk steps.
     · AGGREGATE CHECK (RC-2): does this action, added to the routine actions
       already taken in the enclosing activity chain over its window, cross the
       chain's aggregate budget? If yes ⇒ escalate this action to consequential.
       "Practically irreversible in aggregate" (many individually-reversible
       steps that together cannot be undone) is an input to the class.
     · result: routine/reversible/pre-authorised  OR  consequential
        │
   ┌────┴─────────────────────────────┐
   ▼                                  ▼
 routine class                     consequential class
   │                                  │
   ▼                                  ▼
 Policy Store default authorises    route to the USER for an explicit AUTHORISE
 (HOTL: act-then-show)              (HITL: propose-then-wait)
   │                                  │
   └──────────────┬───────────────────┘
                  ▼
   Governance Store records a SCOPED GRANT for this action:
     · exactly what may be read / changed / run / communicated
     · a time bound
     · revocable at any moment without a further act
     · monotonic: it can be narrowed for free; WIDENING needs a fresh AUTHORISE
                  │
                  ▼
   Pipeline SM: Intention → authorised   (still nothing has happened)
                  │
                  ▼
   at EXECUTE: Reference Monitor checks the grant is valid, in scope, within
   budget, not emergency-stopped → PASS or DENY
```

- **The routine/reversible/pre-authorised class** is defined in the Policy Store,
  conservatively by default (err to HITL), a deliberate user act to widen
  (MEL-REQ-019, OQ-12). Autonomous execution is limited to that class; anything
  else is proposed.
- **Aggregate-effect governance (RC-2).** The routine class carries a per-activity-
  chain **aggregate budget** over a window (count and/or effect magnitude). While
  a chain stays under budget its routine actions proceed on the Policy Store
  default; the action that would cross the budget is re-gated as **consequential**
  and routed to the user. This closes the escape where many individually-routine
  actions add up to a consequential — or practically irreversible — effect. The
  budget values are **Ring-0 governance data**; the accounting is deterministic
  Ring 1 (see `RUNTIME_MODEL.md` §4).
- **Composition never launders risk.** A workflow's risk class is the union of its
  components' authority, scope, and aggregate effect. There is no composition that
  is lower-risk than its riskiest constituent authority.
- **"MELFINA may decide no action should be taken"** — a first-class decision
  outcome producing a Claim, not an Intention (MEL-REQ-103).
- **Dynamic autonomy** (MEL-REQ-244–246): within the Policy Store bounds, the
  Metareasoning Controller may choose to be *more* cautious than the bound (more
  verification, lower autonomy) for consequential/irreversible/uncertain
  situations. It **can never choose to be more liberal** than the bound — the
  bound is a ceiling the Controller cannot raise (MEL-REQ-245).

## 3. Capability grants

A grant is a **scoped, typed capability** conveying a *specific* authority:

- names exactly the objects it applies to (these Chronicle units / these paths /
  these commands / this window) and the operations allowed on them. **A terminal
  or GUI action is named as a structured action, never as a raw string (RC-4):** a
  command grant is evaluated against the **parsed argv / structured invocation**,
  not a text match on a command line; **raw shell execution is never equivalent
  to an exact-command grant**; running through a shell that can expand, chain, or
  substitute (**shell interposition**) is a **distinct, higher-risk capability**
  with its own grant type and risk class. GUI actions carry structured
  representations where practical.
- is **unforgeable** — a component cannot manufacture one;
- is held only by *holding the reference* — no ambient "this component is trusted"
  (AP-7);
- is **monotonic** — a component can narrow or drop a grant freely; acquiring
  *more* authority requires the Reference Monitor to issue it, which requires a
  fresh AUTHORISE;
- **cannot exceed the grantor's authority** — a capability MELFINA creates cannot
  hold more than MELFINA held when creating it, nor grant itself more later
  (MEL-REQ-222). **Capability creation or modification passes an explicit
  Reference Monitor check that the creator's authority is sufficient for the
  authority the new/changed capability declares (M8)** — the check is structural,
  not a convention the lifecycle code is trusted to honour;
- is a **live, revocable handle (RC-5)** — not a one-time ticket consumed at
  EXECUTE. The Reference Monitor can **invalidate it mid-execution**. After
  revocation: no new effect is permitted through it; a running execution is
  stopped where possible; reversible effects already made are rolled back where
  possible; irreversible or partial effects are recorded as a **partial
  failure**. User-visible outcome state distinguishes *completed / partially
  completed / rolled back / failed / interrupted* — the system never claims a
  rollback it cannot guarantee.

The **representation** (an opaque token, an OS handle, a WASI-style resource, a
policy record checked per call) and the **action parser** (which parser, which
structured grammar) are deferred to LOW-LEVEL FOUNDATIONS (AU-2). The acceptance
criteria are the properties above.

### Mid-execution revocation (RC-5) — the path

```
   revocation trigger: user act · emergency stop · budget/aggregate cut-off ·
                       Supervisor detecting escalation/loop · a conflicting
                       higher-priority action
        │
        ▼
   Reference Monitor marks the grant invalid   (atomic; takes effect before the
        │                                       capability's next guarded op)
        ▼
   Capability Host: signal the running execution to stop; deny any further
        │           guarded operation it attempts
        ▼
   compensation: for each effect already recorded this action —
        · reversible  ⇒ roll back; record the rollback
        · irreversible ⇒ record as partial; do not pretend otherwise
        ▼
   Pipeline SM records a definite terminal outcome from the vocabulary above;
   an Event captures what completed, what rolled back, what did not
```

## 4. The Reference Monitor (AA)

A single gate, in Ring 0, with the classic reference-monitor properties (`[E]`,
Anderson):

- **Non-bypassable** — there is no path from a proposed action to an effect that
  does not pass it. (Enforced structurally: Ring 3 capabilities receive their
  authority *only* from the Monitor; they start with none.)
- **Always invoked** — checked at EXECUTE for *every* consequential action, every
  time, including automations and capability-lifecycle steps.
- **Evaluable** — small enough to inspect and reason about; a candidate for
  formal checking (AU-5).

It checks, and denies by default unless all hold: (1) a valid grant exists for
this action; (2) the action is within the grant's scope; (3) within Ring-0
budgets (steps/time/invocations/metered resources **and the aggregate budget for
the enclosing activity chain — RC-2**) for the enclosing activity and window;
(4) no emergency stop is active; (5) for a self-change, the tier classification
permits it at this governance level.

## 4.1 Verification trust model (RC-3)

VERIFY (Ring 3 Verifiers) exists so the system does not have to trust a
capability's — or the reasoner's — own word that an action succeeded. A Verifier
that could itself be subverted, or that could cause effects, would defeat this.
The trust model:

| Property | Requirement |
|---|---|
| **Deterministic where practical** | a Verifier is a fixed function of observable state → {pass, fail, indeterminate}. Non-determinism is justified case by case, never the default. |
| **Minimal authority** | only the read grants needed to observe the specific post-condition; nothing broader. |
| **No ambient authority** | like every Ring 3 component — starts with none, receives typed per-check grants. |
| **Read-only** | a Verifier **cannot directly cause an effect**. If a check genuinely needs a side effect (e.g. a probe write), that is narrowly justified, separately granted, and itself logged. |
| **Independently testable** | each Verifier runs against a fixed fixture set and must produce known results; a Verifier that fails its own fixtures is not eligible to gate anything. |
| **Not a second agent** | Verifiers do not plan, do not choose actions, do not call the reasoner. Verification is a bounded check, not an open-ended evaluation loop. |

**Redundancy for high-risk actions.** A high-risk action requires agreement from
**≥ 2 independent verification mechanisms** where feasible. **"Independent" means
architectural** — different code paths, different implementations, different
observation methods — **not** statistical or mathematical independence. Example:
one Verifier reads the Chronicle post-state; a second observes the external
artefact directly. If they disagree, the outcome is **indeterminate** and the
action is treated as not-verified.

**Fallback boundary.** For some actions a second independent mechanism is
genuinely impractical (only one way to observe the result). Those are enumerated
in the Policy Store as **single-verifier actions**; each carries a written
justification, and single-verifier status itself raises the action's governance
(more likely to require HITL, tighter aggregate budget). This exception is
explicit and bounded — never a silent default.

## 5. Self-modification governance (X)

### The nine tiers and their governance

| Tier | Change touches | Who may initiate | Gate |
|---|---|---|---|
| 1 | configuration data (Policy Store) | user; MELFINA may propose | a user confirmation matched to the setting's weight |
| 2 | learned-state (`inferred` Claims about the user) | MELFINA, via §25-adaptation | explicit item, suggestion-confirm, forgettable (MEL-REQ-118–122) |
| 3 | skill creation (a new capability) | MELFINA, via the Capability Lifecycle | compose-before-code; test on the fixed suite; least authority; **an explicit Reference Monitor check that the creator's authority ≥ the new capability's declared authority (M8)**; **AUTHORISE scaled to risk class** (risk class = union of authority + scope + aggregate effect, RC-2) |
| 4 | skill modification | MELFINA, via the Lifecycle | as tier 3 + regression-tested (MEL-REQ-226) |
| 5 | workflow modification (what runs unattended) | MELFINA may propose | user authorises anything changing unattended behaviour |
| 6 | **MELFINA's own code** | **a human only** | not a MELFINA operation — a human edits versioned code with ordinary dev tools; MELFINA may *assist* under the normal pipeline; staged, sandboxed, tested, reversible |
| 7 | a subsystem | **a human only** | as tier 6 + state-quiescence + invariant re-check + a rehearsed rollback |
| 8 | architecture | **a human only** | as tier 7 + a deliberate, documented human decision |
| 9 | self-replacement | **a human only** | MELFINA MUST NOT initiate or perform its own replacement |

- **Tier classification is deterministic** (Ring 1), from *what the proposed
  change touches* (config / learned Claims / a capability component / core code /
  Ring 0). Ring 2 **cannot** reclassify a change downward to dodge a gate
  (MEL-REQ-227) — the classifier's inputs are the change's declared targets, not
  a reasoning claim.
- **Tiers 6–9 are not in the capability namespace.** There is no capability whose
  effect is "modify MELFINA's code / subsystem / architecture" or "replace
  MELFINA". A human does those with a text editor and a compiler.

### The meta-invariant (INV-6, MEL-REQ-235)

MELFINA cannot redefine — silently, autonomously, or by persuasion — the rules
governing whether/how it may modify itself. Structurally:

1. **The rules live in Ring-0 versioned files.** Ring 1–3 can **read** them (to
   explain them to the user, MEL-REQ-201–202). There is **no write path** from any
   ring to Ring 0. The Ring-0 path is **hard-excluded from every File Access
   grant (RC-1)** — MELFINA cannot be granted access to its own governance files,
   so it cannot even stage an edit.
2. **Changing them is a human file edit**, recorded (an Audit Event), with **both
   the cryptographic at-rest integrity chain and the version chain verified at
   every startup (RC-1)**. The integrity chain is rooted in a **human-held signing
   key MELFINA never possesses**; an invalid signature or a broken version chain
   ⇒ **MELFINA refuses to run**. This protects Ring 0 while MELFINA is *stopped*,
   not only while it is running.
3. **Monotonic**: MELFINA's effective self-modification authority can only
   *narrow* without an explicit human act; any *widening* is a deliberate human
   change to the Ring-0 files.
4. **No capability**, at any ring, has the effect "widen my authority" or "edit
   the tier policy". That operation is **absent from the namespace**, not
   well-guarded.

## 6. Governance store integrity

- Every Ring-0 file is append-versioned; each version records who (a human) and
  when.
- **At-rest integrity (RC-1):** the Ring-0 files carry a cryptographic integrity
  chain rooted in a **human-held signing key that MELFINA never possesses**. It
  is verified at every startup **before** the version-chain check. An invalid
  signature ⇒ **refuse to start and report**. This defends against an attacker
  or a buggy process editing the files on disk while MELFINA is not running —
  the "no write path" argument only covers a *running* MELFINA.
- **The Ring-0 path is hard-excluded from every File Access grant.** The File
  Access capability cannot be granted a path that resolves inside the Ring-0
  store, by the user or by MELFINA. MELFINA cannot read *or* write its own
  governance files through a capability.
- The version chain is verified at startup; a broken chain ⇒ **refuse to start
  and report** — never proceed on suspect governance.
- Ring 0 is loaded and armed **before** Ring 1 (`RUNTIME_MODEL.md` §2), so no
  Ring 1 operation ever runs ungoverned.
- **Deferred to LOW-LEVEL FOUNDATIONS:** the signing algorithm, key storage
  (hardware token / passphrase-derived / offline), and the chain format. Only the
  *invariant* — signed, human-rooted, verified every startup, invalid ⇒ refuse —
  is fixed here.

## 7. Threat model

**Assets:** the user's life record (Chronicle); the user's autonomy and agency;
the user's machine; MELFINA's own integrity.

**Assumed adversaries / failure sources:**

| Source | Assumption |
|---|---|
| Adversarial content the user or a capability ingests | **will sometimes carry a successful prompt injection** (`[E]`; INV-8) |
| A compromised or buggy capability | may try to exceed its grant, escalate, loop, or exfiltrate |
| The reasoning ring, if capable enough | may reach conclusions that, if executed, would harm the user's interests; may "over-help" |
| A bug in Ring 1 | could grant unintended authority |
| An attempt to bind a **remote** reasoner (config error, tampering, a persuaded user) | the Reasoner Interface holds **no network capability** — it **cannot** open a socket, so it can bind only a local reasoner (RC-6). Not "disallowed by policy" — unreachable through the core capability model. |
| Social pressure on the user | the user could be persuaded (by MELFINA or by external content relayed through it) into a harmful Ring-0 change |
| Physical / local attacker with disk access | encryption at rest is the mitigation; out of MELFINA's runtime scope otherwise |
| The network | **absent from the core** — the primary mitigation is that the attack surface does not exist locally |

**Structural mitigations (not behavioural trust — mission §8):**

1. **No ambient authority** → a compromised component can do only what its narrow
   per-action grant allows.
2. **Reasoning cannot act** → a prompt-injected reasoner emits a *proposal*,
   which a human (or a conservative policy default for the routine class only)
   must authorise; and it has **no** Chronicle write, capability invocation,
   network, or Ring-0 access. **Reasoner locality is structural (RC-6):** Ring 1
   and Ring 2 hold no network capability, so the Reasoner Interface *cannot*
   reach a remote endpoint — a remote reasoner is not a configurable option.
3. **The lethal trifecta is broken by construction** (MEL-REQ-126): a component
   that ingests untrusted content cannot simultaneously hold private-data read +
   an outbound channel. The core has **no** outbound channel at all (INV-1). Any
   future Ring-4 capability is structurally prevented from holding all three.
   **The prohibition on any component holding all three of {private-data read,
   untrusted-content ingestion, outbound channel} is an explicit Ring-0
   governance invariant (L5)** — the Reference Monitor refuses to assemble such a
   grant set, and no Policy Store edit can permit it; only a deliberate human
   Ring-0 change could, and that change would be plainly recorded.
4. **The Reference Monitor is non-bypassable and always invoked** → every
   consequential effect passes a check that denies by default.
5. **Ring 0 has no write path** → the meta-invariant holds by absence of a path.
6. **Budgets are hard cut-offs** → a runaway reasoner or automation is halted,
   not merely alerted (`[E]`; MEL-REQ-115, 183).
7. **The Chronicle is append-only** → corruption is detectable and non-destructive
   of history; recovery is replay.
8. **Supervision-tree isolation** → a crash or exploit in one subtree does not
   cascade (INV-11).
9. **Audit records reads too** → an attempt to over-access data is visible after
   the fact.

## 8. Future network isolation (AD)

Ring 4 does not exist. If ever built:

- a **distinct, separately-installed component**, off by default, clearly
  indicated when active, fully removable, **never a dependency of any core
  function** (MEL-REQ-167);
- **structurally cannot form the lethal trifecta** with core data: a Ring-4
  capability holds *either* an outbound channel *or* private-data read authority,
  never both, and never while also ingesting untrusted content (MEL-REQ-126).
  The no-all-three rule is a **Ring-0 governance invariant (L5)**, enforced by
  the Reference Monitor, not a Policy Store setting;
- **no Ring-0 access**; all traffic audited;
- the boundary is **architectural, not a config flag** — Ring 4 is a component the
  user physically adds, not a feature toggled on.

Until and unless Ring 4 exists, the **Ring 3 sandbox has zero network capability
by default (L4)** — a capability cannot make a network call even if it tries; the
absence is structural, not a disabled feature.

## 9. Documented residual limits (this is necessary, not sufficient — MEL-REQ-239)

The structural mitigations above do **not** amount to perfect safety. Known gaps:

1. **Social engineering of the user.** A sufficiently capable reasoning ring could
   frame a harmful Ring-0 change as reasonable and get the user to make it. The
   user is the backstop, and the requirements' anti-manipulation safeguards
   (MEL-REQ-122, 139, 158; MEL-AR-06, MEL-AR-08, MEL-AR-14, MEL-AR-18) apply — but
   this is a *limit*, not a solved problem. Mitigation posture: Ring-0 changes are
   rare, deliberate, plainly explained, and never urgent.
2. **Unintended Ring-1 bugs.** A bug in the substrate could grant authority the
   design did not intend. Mitigation posture: keep Ring 0 + Ring 1 small,
   auditable, and a candidate for formal verification (seL4 precedent, `[E]`,
   AU-5). The smaller the trusted core, the smaller this surface.
3. **Test-passing, badly-generalising capabilities.** A capability could pass
   every fixed-suite test yet behave wrongly on novel inputs. Mitigation posture:
   periodic regression + capability-drift monitoring (MEL-REQ-226); easy rollback;
   keep tiers 6–9 human-driven and rare; least authority limits the damage.
4. **The "constitutional behaviour on familiar tests" problem** (`[U]`, carried
   from research): a self-modifying system can preserve rule-compliant behaviour
   on known tests while altering internal abstractions so the rules stop
   generalising. This is why tiers 6–9 are **not MELFINA operations at all** — the
   only entity that changes MELFINA's code is a human.
5. **The user's own vulnerability.** The requirements' entire OCD/perfectionism/
   compulsion analysis means the *user* is a channel an adversary (or MELFINA's
   own misfiring) could exploit. The reassurance-pattern and manipulation
   safeguards (MEL-REQ-058, 104, 139, 186) are the mitigation; their efficacy is
   itself an open question (`requirements` OQ-9).
6. **The "MELFINA may assist tiers 6–9" soft spot.** Tiers 6–9 are human
   operations, but MELFINA may *assist* (draft a diff, explain a subsystem,
   propose a test). A sufficiently capable assistant could shape a human-executed
   change in a direction the human does not fully scrutinise. Mitigation posture:
   the human still runs every step with ordinary tools, the change is staged and
   reversible, and Ring 0's at-rest signing (RC-1) means a governance change in
   particular cannot take effect without a deliberate human signing act. Still a
   *limit*, not a solved problem.

**Revision 1 note.** RC-1 (at-rest signing), RC-2 (aggregate-effect governance),
RC-3 (verifier trust model), and RC-5 (mid-execution revocation) *narrow* several
of these gaps — an on-disk Ring-0 edit, a laundered high-risk action, a subverted
verifier, and an un-stoppable running action are each now structurally harder.
They do not *close* the list. **These limits are recorded so no later phase
pretends they were solved here.**
