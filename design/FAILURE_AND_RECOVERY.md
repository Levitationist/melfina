# MELFINA — FAILURE AND RECOVERY

**Phase:** SYSTEM DESIGN / ARCHITECTURE — MISSION 001. First pass, pending review.
**Revision 1 applied** — RC-5 (mid-execution revocation + outcome vocabulary),
O4 (supervisor topology not specified now), M5/O6 (cache-divergence audit as a
future concern), M6 (cyclic automation triggers), O1 wording. See
`ARCHITECTURE_ADVERSARIAL_REVIEW.md` "Revision 1 — resolution status".
Baseline `f3093fc`. Read `ARCHITECTURAL_PRINCIPLES.md`, `SYSTEM_ARCHITECTURE.md`,
`RUNTIME_MODEL.md` first. **No technology chosen.**

Covers mission item **Z** (failure/recovery). Grounded in the Erlang/OTP
supervision + "let it crash" pattern (`[E]`) and the append-only-log recovery
model of event sourcing (`[E]`).

---

## 1. The failure philosophy

- **The Chronicle is the ground truth.** Anything else (the current-state cache,
  any on-demand view, the working context, a capability's state) is disposable —
  losing it loses nothing recoverable-from-the-Chronicle.
- **Contain, don't cascade.** A failure is isolated to its supervision subtree.
  A crashed capability does not touch Ring 1; a corrupt current-state cache does
  not touch the Chronicle; a Ring 2 crash does not touch Ring 1.
- **"Let it crash."** Subsystems and capability executions are supervised units.
  On failure they are restarted from a clean state (or abandoned), rather than
  trying to defensively handle every error path in place.
- **Fail honestly, without blame.** A failure produces an Event and, where the
  user needs to know, a neutral notification. **No** guilt UI, **no** "you
  missed", **no** pressure (AP-10; MEL-REQ-047).
- **Atomic appends.** The Chronicle never contains a partial unit. A crash
  mid-append leaves either the whole unit or nothing.

## 2. The supervision tree

```
                         ┌──────────────────┐
                         │  ROOT SUPERVISOR │  (Ring 1)
                         └────────┬─────────┘
            ┌─────────────┬───────┼───────────┬──────────────┐
            ▼             ▼       ▼           ▼              ▼
     ┌───────────┐ ┌───────────┐ ┌─────────┐ ┌───────────┐ ┌────────────┐
     │ Chronicle │ │ Projection│ │ Pipeline│ │  Audit +  │ │ Ring 2/3   │
     │ + Query   │ │  Engine   │ │   SM    │ │ Notif GW  │ │ supervisor │
     │ (critical)│ │(restartbl)│ │(restrtbl│ │(restartbl)│ │(on-demand) │
     └───────────┘ └─────┬─────┘ └─────────┘ └───────────┘ └─────┬──────┘
                         ▼                                       │
                  per-projection                    ┌────────────┼───────────┐
                  workers (one crash                ▼                        ▼
                  ≠ all projections)        Ring 2 (reasoning)      Ring 3 (each
                                            activity supervisor    capability
                                                                   execution
                                                                   supervised)
```

This tree is a **conceptual supervision structure**, not a committed topology
(O4). The concrete supervisor shape — one root, a hierarchy, per-ring supervisors,
an actor-runtime's own tree — is a LOW-LEVEL FOUNDATIONS choice, constrained only
by AP-11 (a failure in one unit does not cascade) and AP-12 (lightweight). What
the architecture fixes is the *isolation boundaries* (Chronicle vs derived views
vs Ring 2 vs Ring 3 executions), not the number or arrangement of supervisors.

**Restart strategies** (per subtree, set conservatively):

| Subtree | Strategy |
|---|---|
| Chronicle + Query | **critical** — a crash here triggers a full safe restart (STARTING sequence); the Chronicle is re-opened and its integrity re-verified |
| Projection Engine / a projection worker | **restart-one** — the failing projection is discarded and rebuilt from its checkpoint; other projections keep running |
| Pipeline SM | **restart** — in-flight actions resume from their last recorded artefact, or abort cleanly to a defined state |
| Audit / Notification Gateway | **restart** — on Audit failure the system pauses *consequential* actions until Audit is back (no unaudited effects), but reads and capture continue |
| Ring 2 activity | **abandon** — a crashed reasoning activity produces a "reasoning did not complete" Claim; nothing was executed (Ring 2 emits only proposals); the user is told neutrally |
| Ring 3 capability execution | **contain + rollback + retry-or-abandon** — see §4 |

## 3. Current-state cache / derived-view failure (the common case)

```
   the cache handler throws / the cache file is corrupt / a rebuild diverges
        │
        ▼
   the Projection Engine discards the cache
        │
        ▼
   replay from the last good checkpoint
        │
   ┌────┴────┐
   ▼         ▼
 replay OK   replay fails
   │         │
   ▼         ▼
 cache back  rebuild from scratch from the whole Chronicle
             (slower; the cache is unavailable meanwhile — consequential reads
              go direct to the Chronicle via its contract, M2)
```

**No user-visible consequence** beyond a possibly slower "what needs attention"
for a moment. The Chronicle — the user's actual life record — is never involved.
An on-demand view that fails simply returns an error for that request and is
recomputed on the next; it has no persistent state to corrupt.

**Silent divergence (M5 / O6).** A cache that has drifted from the Chronicle
without throwing is the subtle case. A periodic consistency audit (recompute a
sample from the Chronicle and compare) that flags divergence and forces a rebuild
is a **future implementation concern**, not an architecture requirement — it is
recorded here so a later phase picks it up. The structural guard that makes this
non-critical: nothing consequential trusts the cache (M2).

## 4. Capability execution failure

```
   a capability crashes / times out / loops / attempts escalation / VERIFY fails
   / its grant is REVOKED mid-execution (RC-5: user act, emergency stop, budget
   or aggregate cut-off, Supervisor detecting escalation, a conflicting action)
        │
        ▼
   Reference Monitor invalidates the live grant handle (atomic); the Capability
   Host stops the execution where possible and denies any further guarded op
        │
        ▼
   for each effect already recorded this action:
        reversible   ─▶ invoke rollback (the pre-authorised inverse: undo the
        │               file write, revert the commit, close the app) ─▶ verify
        │               the rollback
        irreversible ─▶ record as partial; do NOT claim a rollback
        │
        ▼
   the Pipeline SM records ONE definite terminal outcome:
        completed · partially completed · rolled back · failed · interrupted
   (an Event captures what completed, what rolled back, what did not — the
    user-visible state distinguishes all five; the system never reports a
    rollback it cannot guarantee)
        │
        ▼
   Intention → blocked (a Claim records what blocks it)
        │
        ▼
   retry per the action's strategy (bounded retries, Ring-0 budget) OR abandon.
   A revocation-caused stop is NOT auto-retried — the trigger that caused it
   (user, emergency stop, budget) must be cleared first.
        │
        ▼
   the user is informed neutrally (via the Notification Gateway if unsolicited,
   or on their next pull) — factual, no blame
        │
        ▼
   if Ring 2 is active: Self-Evaluation notes the failure against the prediction;
   a capability that fails repeatedly for a task-type is de-prioritised, then
   retired (MEL-REQ-249, 224)
```

**Irreversible actions get extra protection *before* this point** (MEL-REQ-181):
a distinct, deliberate confirmation at AUTHORISE, so the "irreversible / partial"
branch above is rare and always something the user explicitly cleared.

**Mutual / cyclic automation triggering (M6).** Before an automation is
authorised, and on each definition change, the automation-trigger graph is
checked for cycles **of any length** (A triggers B triggers … triggers A), not
just direct self-triggering. A detected cycle blocks authorisation; a cycle that
somehow forms at runtime is caught by the Supervisor + Ring-0 budgets and halted.

## 5. Core service failure

- **Chronicle append failure** (disk full, I/O error): the operation fails
  cleanly; **no partial unit is written**; the caller is told; capture retries
  when space/health returns. The user's data is not corrupted — the failed append
  simply did not happen.
- **Chronicle corruption detected at open** (integrity check fails): MELFINA
  enters a **read-only recovery mode**, reports precisely what is inconsistent,
  and does **not** silently repair. Recovery options (truncate to the last valid
  unit; restore from backup; manual inspection) are presented to the user, who
  decides.
- **Query failure**: transient — retried; persistent — treated as a Chronicle
  subtree failure (critical restart).
- **Pipeline SM failure mid-action**: the action is at a recorded artefact stage
  (THINK/DECIDE/PROPOSE/AUTHORISE/EXECUTE/VERIFY); on restart it resumes from
  there, or — if resumption is unsafe — aborts to a defined state and records it.
  **An action that had reached EXECUTE and whose effect is unknown is treated as
  "may have happened"**: VERIFY runs to determine the actual state; the Chronicle
  records what is observed, not what was intended.

## 6. Reasoning ring failure / unavailability

- The Reasoner Interface fails to bind, or the reasoner crashes: MELFINA reports
  "reasoning unavailable" and **stays at CORE-READY** (`RUNTIME_MODEL.md` §1).
  Capture, memory, retrieval, time support, user-made plans, deterministic
  automations, search — **all continue** (AP-9, INV-9, MEL-REQ-154).
- A reasoning *activity* crashes: abandoned (§2); no effect occurred; the user is
  told neutrally.
- A reasoner produces garbage / obviously-wrong output: caught downstream —
  reasoner output is never an effect, always a proposal/claim that a human or a
  Verifier checks; a pattern of bad output is a Self-Evaluation signal to
  distrust that reasoner/strategy for that task-type.

## 7. Emergency stop (Z, and MEL-REQ-145, 187)

```
   the user triggers Emergency Stop (a path that does NOT go through Ring 2 and
   does NOT require the reasoning ring to be responsive)
        │
        ▼
   Ring 0 asserts the stop signal
        │
        ▼
   · Pipeline SM: refuse all new actions; in-flight EXECUTE actions have their
     grant handles revoked and Capability Hosts torn down; rollback invoked where
     reversible; each ends with one terminal outcome from
     {completed, partially completed, rolled back, failed, interrupted} (RC-5)
   · all Ring 3 executions: terminated
   · Ring 2: activities abandoned
   · background work: paused
   · Ring 1 substrate: driven to a defined safe state (no partial appends;
     projections checkpointed)
        │
        ▼
   MELFINA reports: what was in flight, what was stopped, what state things are in
        │
        ▼
   the stop persists until the user clears it; on clear, MELFINA returns to
   CORE-READY (not automatically resuming the stopped work)
```

## 8. Recovery guarantees (summary)

| Guarantee | How |
|---|---|
| The user's life record is never silently corrupted | append-only, atomic appends, integrity check on open, no silent repair |
| A lost current-state cache / derived view loses nothing | rebuildable from the Chronicle |
| A failure does not cascade | isolation boundaries between Chronicle / derived views / Ring 2 / Ring 3 (supervisor topology deferred — O4) |
| An interrupted consequential action ends in a *known, named* state | pipeline artefacts + VERIFY-on-resume + rollback where reversible; the outcome is exactly one of *completed / partially completed / rolled back / failed / interrupted* (RC-5) |
| A revoked grant stops taking effect at once | grants are live handles the Reference Monitor can invalidate mid-execution (RC-5) |
| The system is usable during partial failure | Ring 1 independent of Ring 2/3; DEGRADED state stays coherent |
| Recovery is not punishing | no guilt/pressure UI; re-entry after any gap is frictionless (MEL-REQ-015, 047) |
| Every failure is auditable | an Event + an Audit record |

## 9. What is deferred

- The concrete restart mechanism **and the supervisor topology** (OS process
  supervision / actor runtime / other; one root or a hierarchy) — LOW-LEVEL
  FOUNDATIONS, against AP-11 (O4 — not specified here).
- The periodic current-state-cache consistency audit (M5 / O6) — a later
  implementation concern, not fixed now.
- The Chronicle integrity mechanism (checksums / hash chain / journaling) —
  STORAGE / LOW-LEVEL FOUNDATIONS.
- The rollback mechanism per capability (each capability declares its inverse) —
  CORE ENGINE + CAPABILITY design.
- Checkpoint frequency and format — STORAGE.
- Concrete retry/backoff policies — CORE ENGINE, as Ring-0/Policy data.
