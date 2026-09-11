# MELFINA — FOUNDATION 4: REVOCATION MODEL

**Phase:** LOW-LEVEL FOUNDATIONS — MISSION 001. First pass, pending review.
**Baseline:** `8a40207`. Read F1–F3 first.

**This document does not choose a stop/rollback mechanism.** It fixes the
lifecycle of a live grant, the semantics of revocation at each point in an
action, the **outcome vocabulary**, and how a partial effect becomes a recorded
Event/Claim.

Derives from: **RC-5**; **INV-2, INV-3, INV-11**; **MEL-REQ-145, 172, 173, 177,
181, 187, 142, 157**; the caretaker/revocation-by-indirection pattern, seL4
capability-derivation-tree revoke, "let it crash" recovery `[E]`.

---

## 0. Dependencies discovered

- **On F1:** the grant carries a `revocation-id` and is a live handle
  re-validated at every guarded effect. F4 defines the *transitions* of that
  handle.
- **On F3:** the Reference Monitor holds the revocation authority. A revocation
  trigger reaches the Monitor; the Monitor marks the grant invalid; the next
  `AUTHORIZED(e,g,s,t)` check fails on `revoked(...)`.
- **On F6:** a partial/interrupted effect is written to the Chronicle as an
  Event with the outcome vocabulary; F6 fixes the atomic-append contract that
  makes "record what actually happened" reliable even across the interrupting
  crash.
- **On F7:** "stop the running execution where possible" is a property the
  isolation mechanism must provide (a terminable sandbox); F4 fixes the
  *semantics* of the stop, F7 the *capability* to stop.
- **On F10:** after a revocation-caused stop, VERIFY-on-resume determines the
  *actual* post-state; the recorded outcome is what VERIFY observes, not what was
  intended.
- **Discovered constraint:** "rollback" is meaningful only for effects a
  capability has **declared a tested inverse for**. Whether an effect is
  reversible is a property of the capability + the effect, resolved *before*
  EXECUTE (at classification). F4 must not promise rollback generically.

---

## 1. The grant lifecycle

A grant moves through these states. Transitions are performed by the Reference
Monitor (or by expiry, which needs no act).

```
                 issue (Monitor mints for an authorised Intention)
                        │
                        ▼
                  ┌───────────┐   activate (first guarded effect begins)
                  │  ISSUED   │──────────────────────────┐
                  └───────────┘                          ▼
                        │                          ┌───────────┐
                 (never activated;                 │  ACTIVE   │
                  Intention resolved,              └─────┬─────┘
                  or lifetime passed)                    │
                        │              ┌─────────────────┼─────────────────┐
                        ▼              ▼                 ▼                 ▼
                  ┌───────────┐   suspend           revoke            expire
                  │  EXPIRED  │   (pause; no        (Monitor marks    (lifetime
                  │  / STALE  │   new effect,       invalid;          bound
                  └───────────┘   resumable         irreversible)     reached)
                        │              │                 │                 │
                        └──────────────┴────────┬────────┴─────────────────┘
                                                ▼
                                        ┌───────────────┐
                                        │   TERMINAL     │  one outcome from §4
                                        │  (invalid;     │  is recorded; the
                                        │   Intention    │  Intention → fulfilled
                                        │   resolved)    │  / blocked / abandoned
                                        └───────────────┘
```

- **ISSUED → ACTIVE:** the executing component begins the first guarded effect.
- **ACTIVE → SUSPENDED:** a pause (e.g. a conflicting higher-priority action, a
  resource-pressure hold). No new effect proceeds; the grant is resumable to
  ACTIVE without a fresh AUTHORISE **only if** it has not expired and nothing
  narrowed it below the pending effect. Resume of a *revocation-caused* stop is
  **not** automatic (§5).
- **any → REVOKED (terminal):** the Monitor marks `revocation-id` invalid,
  atomically. This is a one-way transition.
- **any → EXPIRED (terminal):** `lifetime` reached.
- **A grant is never renewed in place** (F1 §5). Continuing past a terminal state
  requires a fresh AUTHORISE and a fresh grant.

`[SEN]` for the state set and the one-way revocation; `[ID]` for atomicity of the
Monitor's mark.

---

## 2. Revocation triggers

A revocation is initiated by any of (RC-5, F3):

| Trigger | Source |
|---|---|
| **user act** | the user stops a pending or in-progress action (`MEL-REQ-177`) |
| **emergency stop** | the Ring-0 Emergency-Stop signal, reachable without Ring 2 (`MEL-REQ-145`, `MEL-REQ-187`) |
| **budget / aggregate cut-off** | a per-action budget or the activity-chain aggregate budget is crossed (RC-2) — enforced by the **external watchdog**, not self-checked |
| **Supervisor detecting escalation / loop / no-progress** | the sandbox loop detector or the Supervisor (INV-11) |
| **a conflicting higher-priority action** | scope overlap where serialisation requires stopping the lower-priority one (M9) |
| **grant became stale** | the Intention left `status = authorised`; a capability version changed under the grant; the risk class no longer matches (F1 §5) |
| **the Monitor itself** | on any failed re-check it cannot safely continue |

**None of these triggers is the reasoner.** The reasoner can *propose* stopping
(an Intention), but revocation is a Monitor/watchdog/user act. `[SEN]`.

---

## 3. What revocation does — the path

```
  a revocation trigger
        │
        ▼
  the Reference Monitor marks revocation-id INVALID
        │   (atomic; ordered before the target's next guarded-effect check —
        │    "before its next guarded op", seL4-style)
        ▼
  no NEW effect is permitted through the grant:
    the next AUTHORIZED(e, g, s, t) check fails on ¬revoked(...) ⇒ deny
        │
        ▼
  the running execution is STOPPED WHERE POSSIBLE (F7):
    · signal the sandbox to stop; deny every further guarded operation it attempts
    · if the sandbox cannot be stopped cleanly, it is terminated (contained;
      the Chronicle and core are untouched — INV-11)
        │
        ▼
  COMPENSATION, per effect already recorded for this action:
    · effect has a declared, tested inverse AND is currently reversible
        ⇒ invoke the inverse; VERIFY the inverse; record "rolled back"
    · effect is irreversible, or its inverse is not available, or the inverse
      itself fails
        ⇒ record as PARTIAL; do NOT claim a rollback that cannot be guaranteed
        │
        ▼
  the Pipeline SM records ONE definite terminal outcome (§4);
  an Event captures what completed, what rolled back, what did not;
  the Intention → blocked / abandoned (a Claim records what blocks it)
```

- **"Stopped where possible"** is honest: some effects (a spawned process that
  has already forked and detached, a network send in a future Ring-4 capability,
  a GUI action already committed by the target application) cannot be un-started.
  The contract says *stop what can be stopped, contain the rest, record it all*.
  `[SEN]` for the discipline; `[ID]`/`[ETL]` for how much a given mechanism can
  actually stop.
- **Rollback is never promised where it cannot be guaranteed.** `[SEN]`.

---

## 4. The authoritative outcome vocabulary

Every consequential action — whether it finished normally, was revoked, crashed,
or was emergency-stopped — reaches **exactly one** terminal outcome, drawn from a
**closed set**:

| Outcome | Meaning |
|---|---|
| **completed** | every intended effect occurred and VERIFY confirmed it, within the authorised canonical scope |
| **partially completed** | some intended effects occurred and were verified; others did not; **no rollback was performed** (or none was needed for the completed part); the record names which effects landed and which did not |
| **rolled back** | one or more effects occurred and were then **undone via a declared, tested inverse that VERIFY confirmed**; the world is at (or verified-equivalent to) its pre-action state for those effects |
| **failed** | the action did not achieve its intended effect; either nothing consequential occurred, or what occurred was rolled back, or the residue is recorded as partial-and-failed |
| **interrupted** | the action was stopped by a user act or emergency stop before completion; its partial state is recorded exactly as in *partially completed*, and it is **not auto-retried** (§5) |

- **The user-visible state distinguishes all five.** The system **never** reports
  `completed` for a partial action, and **never** reports `rolled back` unless a
  verified inverse actually ran. `[SEN]` for the vocabulary and the honesty rule.
- This vocabulary is **identical** in F1, `AUTHORITY_AND_SECURITY_MODEL.md §3`,
  `CAPABILITY_MODEL.md §2`, `RUNTIME_MODEL.md §6`, `FAILURE_AND_RECOVERY.md
  §4/§7/§8`, and here. It is a fixed contract term. `[SEN]`.

---

## 5. Revocation at each point in an action

| When revocation lands | Behaviour |
|---|---|
| **before EXECUTE** (grant issued, no effect yet) | the grant goes terminal without activation; no effect occurred; outcome **failed** (or **interrupted** if user/e-stop); Intention → blocked; nothing to roll back; nothing to record beyond the denial |
| **during EXECUTE, between guarded effects** | the next check denies; no further effect; compensation runs for effects already landed (§3); outcome **partially completed** / **rolled back** / **interrupted** per what landed and what was undone |
| **during a single guarded effect that is mid-flight** | the sandbox is signalled to stop; if the effect is atomic at the OS level it either fully lands or does not (F6 append is atomic; a file write may be torn and is then detected and treated as not-landed); the effect is recorded as landed or not-landed based on **VERIFY's re-read of the actual post-state**, never on intent |
| **after the last effect but before VERIFY** | VERIFY still runs (on resume if needed) to establish the actual post-state; outcome is **completed** if all effects verify, else **partially completed** |
| **after VERIFY / completion** | the grant is already terminal (`completed`); a late revocation has nothing to act on; if the user wants the *effect* undone, that is a **new** proposed action (a compensating Intention) through the full pipeline — not a retroactive revocation |

- **A revocation-caused stop is NOT auto-retried.** The trigger that caused it
  (a user stop, an emergency stop, a crossed budget) must be **cleared first**,
  and re-attempting the action is a **new** pipeline pass with a **new**
  AUTHORISE (`FAILURE_AND_RECOVERY.md §4`). `[SEN]`.
- **Non-revocation failures** (a capability crash, a timeout, a VERIFY failure)
  MAY be retried per the action's bounded retry strategy (Ring-0 budget), which
  is distinct from revocation. `[SEN]`.

---

## 6. How a partial effect becomes a recorded Event

- The Pipeline SM appends a **PARTIAL FAILURE / INTERRUPTED Event** (F6) that
  records: the action's canonical intended effect; which guarded effects **landed
  and verified**; which **did not**; which were **rolled back** (with the inverse
  that ran and its VERIFY result); the **trigger** (user / e-stop / budget /
  crash / conflict); and the resulting **outcome term** (§4).
- The affected `Intention` moves to `status = blocked` (a Claim records what
  blocks it) or `abandoned`. **Failure of an intention does not falsify anything**
  (model §4.4) — no "success/failure" verdict, no guilt surface (AP-10,
  `MEL-REQ-047`).
- The append of this record is **atomic** (F6): a crash *during* the recording
  leaves either the whole record or nothing; recovery re-runs VERIFY-on-resume to
  reconstruct the outcome (`FAILURE_AND_RECOVERY.md §5`).
- **Audit** additionally records the revocation act, its trigger, and the grant
  that was invalidated (`MEL-REQ-182`).

`[SEN]` for what must be recorded; `[ID]` for the record's structure (F5/F6).

---

## 7. Effect kinds and what "stop / roll back" means for each

| Effect kind | Stop | Roll back |
|---|---|---|
| **long-running child process** | signal → terminate (F7); wait bounded, then hard-kill; reap | only if the capability declared a tested inverse (e.g. "the command was `mkdir X`; inverse `rmdir X` if empty"); many process effects have **no** inverse → **partial** |
| **file write / create** | stop before the next write op; a torn in-flight write is detected (F6-style framing / checksum) and treated as **not landed** | reversible only if a **pre-image was captured** as part of the action (a declared inverse: restore the pre-image / delete the created file); otherwise **partial** |
| **file delete / truncate / rename** | these are irreversible-class; they got the extra deliberate confirmation at AUTHORISE (`MEL-REQ-181`), so a mid-action revocation of a *chain* containing one records the delete as **landed, irreversible** | not reversible unless the action explicitly staged a copy first |
| **GUI action** | stop before the next `gui-op`; an op already dispatched to the target application may be **committed by that application** and cannot be recalled | reversible only if there is a declared inverse `gui-op` (e.g. "undo") and the target still supports it |
| **multi-step workflow** | stop after the current step; do not start the next | roll back **step by step in reverse**, each via its declared inverse; the first step with no inverse ⇒ the remainder is **partial**; the record names the boundary |
| **automation run** | the watchdog invalidates the grant; the current run stops | as the workflow above; the automation is **not** re-fired automatically (the trigger that stopped it must clear) |
| **capability composition / sub-capability** | revoking the parent grant revokes the child grants **transitively** (F1 §7); each child stops | each child's effects roll back per its own declared inverse |
| **`chronicle-append`** | the append is atomic (F6) — it either fully landed or did not; a partial append does not exist | a landed Chronicle unit is **never deleted to "roll back"**; a compensating unit is appended (a redaction / correction Event) — this is append-only history (INV-5), so "rollback" of a Chronicle effect means "append a superseding/redacting unit", recorded as such |

`[SEN]` for the "no inverse ⇒ partial, recorded honestly" rule; `[ID]` for each
mechanism's actual stop granularity; `[ETL]` for how reliably a given inverse
restores state — the inverse itself is VERIFY'd.

---

## 8. Contract Q&A

| Question | Answer |
|---|---|
| **Guarantees** | a revoked grant permits no new effect; a running execution is stopped where possible and contained where not; reversible effects with a declared, tested inverse are rolled back and re-verified; every consequential action reaches exactly one of {completed, partially completed, rolled back, failed, interrupted}; the record names what landed, what rolled back, what did not; nothing is auto-retried after a revocation-caused stop |
| **Requires** | the Monitor's atomic invalidation (F3); a terminable/containable sandbox (F7); atomic append (F6); VERIFY-on-resume (F10); per-capability declared tested inverses where rollback is claimed |
| **Trusts** | VERIFY's re-read of the actual post-state; the Monitor's invalidation ordering; the atomicity of the Chronicle append |
| **Distrusts** | the intended effect as evidence of what happened; a capability's claim that it "cleaned up"; a rollback that was not VERIFY'd; the reasoner as a revocation authority |
| **Enters** | a revocation trigger (user / e-stop / budget / Supervisor / conflict / staleness / Monitor) |
| **Leaves** | one terminal outcome term + a recorded Event describing landed/rolled-back/not-landed effects + an Audit entry for the revocation act |
| **Malformed input** | a revocation targeting an unknown or already-terminal grant is a no-op, recorded; it never *un*-revokes anything |
| **Failure** (the stop mechanism cannot stop; the inverse fails) | contain (terminate the sandbox; Chronicle/core untouched); record as **partially completed** / **failed**; do not claim more than was verified |
| **Authoritative** | VERIFY's observed post-state is authoritative for what happened; the recorded outcome term is authoritative for the user-visible result; the Monitor's `revoked` mark is authoritative for the grant |
| **Independently verifiable** | yes — a fixture set of {effect kind, revoke at point P, declared inverse present?/absent?} → {expected outcome term, expected record contents}; the "no new effect after revoke" property is red-teamed by attempting a guarded effect on a revoked grant |

---

## 9. Enforceability summary

| Invariant | Class |
|---|---|
| revoke is one-way; no new effect after; re-checked at every boundary | **[SEN]** rule / **[ID]** atomicity + "every boundary" |
| outcome vocabulary is closed and identical across all docs; honesty rule (never claim `completed`/`rolled back` falsely) | **[SEN]** |
| stop where possible; contain where not; Chronicle/core untouched by a contained stop | **[SEN]** rule / **[ID]** sandbox terminability |
| rollback only for a declared, tested, VERIFY'd inverse; else partial | **[SEN]** rule / **[ETL]** inverse fidelity |
| revocation-caused stop is not auto-retried; trigger must clear first | **[SEN]** |
| the reasoner is never a revocation authority | **[SEN]** |
| Chronicle "rollback" = append a superseding/redacting unit, never delete (INV-5) | **[SEN]** |
| partial-effect record is atomic; VERIFY-on-resume reconstructs the outcome | **[ID]** (F6) |
| a landed irreversible effect (delete/send) is recorded as such; it had the extra confirmation at AUTHORISE | **[SEN]** rule / **[ID]** the classifier tagging reversibility |

---

## 10. Deferred (not decided here)

- The stop mechanism (signal/terminate/checkpoint) and how much it can stop for
  each effect kind and platform.
- The per-capability **inverse declaration** format and how the lifecycle tests
  it (CORE ENGINE + CAPABILITY design).
- The **aggregate-reversibility** metric (F1 §8) — when N reversible effects
  become irreversible-in-aggregate.
- Concrete retry/backoff policies for non-revocation failures (Ring-0 / Policy
  data).
- The pre-image / staging discipline for making file/GUI effects reversible.

## 11. Traceability

| Element | Source |
|---|---|
| grants are live revocable handles; invalidate mid-execution; no new effect; stop where possible; roll back reversible; record irreversible/partial; outcome vocabulary | RC-5; `MEL-REQ-145`, `MEL-REQ-177`, `MEL-REQ-181`, `MEL-REQ-187` |
| partial failure is contained and visible; recover to a known state | `MEL-REQ-172`, `MEL-REQ-173`; INV-11 |
| code actions verified and reversible; consequential AI actions reversible where possible | `MEL-REQ-142`, `MEL-REQ-157` |
| emergency stop, graceful shutdown, safe state | `MEL-REQ-145`, `MEL-REQ-187` |
| failure carries no guilt surface | `MEL-REQ-047`; AP-10 |
| append-only history; "rollback" of a Chronicle effect is a new unit | INV-5; model §11 |
| transitive revocation over the derivation tree | seL4 / KeyKOS revoke `[E]`; F1 §7 |
