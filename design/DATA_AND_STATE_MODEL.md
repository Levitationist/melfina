# MELFINA — DATA AND STATE MODEL

**Phase:** SYSTEM DESIGN / ARCHITECTURE — MISSION 001. First pass, pending review.
**Revision 1 applied** — O1 (one Chronicle + one current-state cache + on-demand
views), M1 (context on demand), M2 (Chronicle-contract reads for consequential
correctness), RC-1 (Governance Store at-rest signing). See
`ARCHITECTURE_ADVERSARIAL_REVIEW.md` "Revision 1 — resolution status".
Baseline `f3093fc`. Read `ARCHITECTURAL_PRINCIPLES.md`, `SYSTEM_ARCHITECTURE.md`,
`model/HUMAN_CENTRAL_MODEL.md` first.

Covers mission items **E** (data ownership), **M** (persistence boundary),
**O** (memory architecture). **This document does not choose a storage engine,
on-disk format, indexing scheme, or query language.** It defines the *logical*
data model and its guarantees. Those choices are deferred to **STORAGE**,
constrained by the guarantees below.

---

## 1. The logical model: E²CI → the Chronicle

The human/central model's four primitives map to **units appended to the
Chronicle**, plus one registry:

| Model element | Data form | Notes |
|---|---|---|
| **Entity** | a record in the **Entity Registry** | `id` (opaque, stable), `kind` (open), `names`/`aliases` over time. Thin — meaning is in Claims about it. |
| **Event** | a **Chronicle unit** | `id`, `kind`, `time` (point/interval; `occurred` or `expected`), `participants` (Entity ids + roles), `on-behalf-of`, `brings-about`/`ends` (Claim refs), `derived-from`/`informed-by`. |
| **Claim** | a **Chronicle unit** | `id`, `content` (descriptive / relational / about-a-claim / about-an-entity), `holder`, `status` ∈ {observed, reported, inferred, hypothesised, open, stipulated, decided, recalled}, `confidence`, `provenance` (`attributed-to`, `generated-by`, `derived-from`), `valid-time` (interval), `transaction-time` (assigned on append), `supersedes` (Claim ref). |
| **Intention** | a **Chronicle unit** | `id`, `owner`, `target` (a condition and/or an Event), `creditor` (opt — makes it a commitment), `status` ∈ {active, proposed, authorised, suspended, fulfilled, abandoned, blocked}, `plan` (opt — sub-Intentions/Events + relations), `standing` ∈ {one-off, recurring, maintenance}, provenance/valid-time/transaction-time/supersedes as Claim. |
| **Time** | fields on every unit | bitemporal — see §4. |
| **Relation** | a **Claim** of relational content ("X ρ Y") | carries its own holder/status/confidence/provenance/valid-time (model §5, OQ-M1). |
| **State** ("what holds now") | **not stored authoritatively** — the current-state cache (§3), rebuilt from the Chronicle | model §3.3, OQ-M2; the architecture is compatible with either OQ-M2 answer (`ARCHITECTURAL_ALTERNATIVES.md` AA-8). |
| **Context** | **not stored** — a transient Ring-2 derivation, constructed on demand (M1) | model §7. |

**Every unit carries provenance and (for Claims/Intentions) epistemic status.**
There is **no schema path for a bare fact** (AP-4). "MELFINA's beliefs",
"MELFINA's proposals", "MELFINA's self-evaluation", "learned preferences" are all
Chronicle units with `holder`/`owner`/`attributed-to = melfina` — same shape as
the user's (model §12; AP-10).

## 2. The stores

| Store | Content | Discipline | Authoritative? |
|---|---|---|---|
| **Chronicle** | Event / Claim / Intention units | append-only; monotonic transaction-time; atomic appends; no update; no delete (tombstone via redaction Event only) | **YES — the single source of truth** |
| **Entity Registry** | Entity records | append-versioned (name changes, kind refinements are new versions; the id is stable) | YES |
| **Policy Store** | user settings (conflict-axis positions, autonomy/proactivity levels, interruption rules, retention periods, the consequential/routine boundary) | append-versioned; changed only by a user act | YES (for *policy*, not *life data*) |
| **Governance Store** (Ring 0) | permission rules, tier policy, budgets, issued grants | append-versioned; changed only by a human file edit; **not** in the Chronicle; **at-rest integrity chain rooted in a human-held signing key MELFINA never possesses — verified every startup; invalid signature or broken version chain ⇒ refuse to run (RC-1)**; hard-excluded from every File Access grant | YES |
| **Capability Registry** | capability metadata (id, version, declared authority, risk class, reliability claims, lifecycle status) | lifecycle-managed; every version retained | YES (metadata; the *code* lives as component files, versioned) |
| **Audit** | consequential actions, grants/revokes, capability runs, Ring-2 reads, self-changes | append-only; tamper-evident (mechanism deferred) | YES (records *reads* too, which the Chronicle doesn't) |
| **Current-state cache** | the one materialised derived view: "what conditions currently hold" | rebuildable; checkpointed; may lag | **NO — a cache** |
| **On-demand views** | "what needs attention", calendar, learning trajectory, capability reliability, an opted-in progress view, context slices | computed per request from the Chronicle (or the current-state cache); not separately maintained | **NO — derived** |

The conceptual target is **one authoritative Chronicle + one current-state cache
+ on-demand views** — not a proliferation of independently-maintained projections
(O1). **Losing every derived view loses nothing** (AP-5). Losing the Chronicle
loses the user's life record — hence backup (§7).

## 3. Derived views

A **derived view** is a function of the Chronicle (and possibly the
Registry/Policy Store) that produces a query-optimised result. **Exactly one**
derived view is a *maintained materialised cache* — the **current-state cache**.
Every other view is **constructed on demand** and discarded after use (M1, O1).
Whether a specific on-demand view is later promoted to a maintained cache for
performance is a STORAGE decision, not an architecture requirement. Properties
common to all derived views:

- **Idempotent** — re-processing the same Chronicle units yields the same view.
- **Checkpointed** — the current-state cache stores "processed up to
  transaction-time T"; on restart it replays from T. (On-demand views hold no
  checkpoint — they are recomputed each time.)
- **Rebuildable** — deleting the current-state cache and replaying the whole
  Chronicle reconstructs it exactly.
- **May lag** — a bounded consistency window between an append and its reflection
  in a view. Acceptable for **pull** surfaces. **Not** acceptable for the pipeline
  or the Reference Monitor — those read the Chronicle directly (`ARCHITECTURAL_ALTERNATIVES.md`
  AA-2, Risk 2).
- **Never authoritative** — a derived view is never the basis for a consequential
  decision; it is a convenience. DECIDE and the Reference Monitor read current /
  authoritative state through the Chronicle contract where consequential
  correctness matters (M2).

The views:

| View | Maintained? | Definition | Consumers |
|---|---|---|---|
| **current-state cache** | **yes — the one materialised cache** | for each descriptive Claim, is it currently valid (valid-time covers now, not superseded, not ended by an Event)? → the set of currently-holding conditions | Ring 2 (context), "what needs attention", the user |
| **what-needs-attention** | no — on demand | Intentions near a valid-time bound, or `blocked`, or high-relevance to a present focus — **surfaced neutrally, on pull** (MEL-REQ-040, 047) | the user (pull only) |
| **context slices** | no — constructed per situation, then discarded (M1; model §7) | relevance-ranked neighbourhoods of focal entities/events/intentions | Ring 2 Context Constructor |
| **calendar view** | no — on demand | occurred + expected Events and time-bounded Intention targets on a time axis | the user |
| **learning trajectories** | no — on demand | ordered Intentions targeting "understands C" + realising sessions + current-understanding Claims | the user, the teaching contributor — subject to AP-10 (not a checking surface) |
| **capability reliability** | no — on demand (STORAGE may cache for performance) | reliability-by-task-type Claims aggregated per capability | Metareasoning Controller, Capability Lifecycle Manager |
| **progress (opt-in only)** | no — on demand | if the user turns it on: private, non-comparative, non-accumulating, partial-credit, disableable (MEL-REQ-046) | the user, only if enabled — **off by default** |

**No derived view other than the current-state cache is a stored aggregate that
must be maintained** (AP-10): each is computed from the Chronicle on demand; the
"streak store" / "metrics engine" / "dashboard subsystem" do not exist.

## 4. Bitemporal semantics (M)

Two axes on every Chronicle unit:

- **`valid-time`** — the interval over which the content is asserted true *in the
  world* (or the Event occurred, or the Intention's target window is). Any range;
  correctable.
- **`transaction-time`** — assigned by the Chronicle on append. **Monotonic,
  immutable.**

Query forms the storage layer must support (the *interface*, not the
implementation):

| Query | Meaning |
|---|---|
| `at(valid=V, tx=now)` | what MELFINA now believes was true at V |
| `at(valid=V, tx=T)` | what MELFINA believed at time T about V |
| `history-of(unit)` | the `supersedes` chain, all versions retained |
| `as-of(tx=T)` | the whole knowledge state as it was at transaction-time T |
| `changes-between(tx=T1, tx=T2)` | what was appended/superseded in that window |

**"Change the past"** = append a new Claim with an earlier `valid-time` and a
`now` `transaction-time`, `supersedes` the old; the old stays (model §6.1, §11;
AP-5). This is how "historical truth vs current truth" and "revised conclusions"
are represented — never by editing.

**Belief change** (model §9; AGM `[E]`):
- *expansion* = append a consistent Claim.
- *revision* = append a Claim + `supersedes` the conflicting ones.
- *contraction* = append a redaction/retraction Claim; mark units that
  `derived-from` the retracted one for re-evaluation.
All three are appends; nothing is erased.

## 5. Memory architecture (O)

**MELFINA has one memory: the Chronicle.** There is no separate "long-term
store", "vector database", or "episodic buffer" as an architectural primitive.
Retrieval structures (a full-text index, a similarity index, a
recency/importance ranking) — *if they appear* — are **projections/indexes** the
Projection Engine maintains, rebuildable from the Chronicle, none authoritative.

- **Storage ≠ memory** (model §12): a dropped-in file is an `Entity` (kind
  `source`); what MELFINA *knows* from it is Claims (`status = reported`,
  provenance → that Entity + the ingest `observation` Event). The file bytes are
  an artefact; the knowledge is claims.
- **Remembering** = a Query (possibly relevance-ranked) over Chronicle +
  projections. Recalled content carries `status = recalled` (normal fallibility).
- **Forgetting**:
  - *soft* — a Claim's `valid-time` ends, or `confidence` decays, or it is
    `supersede`d → simply not surfaced (relevance). Retained.
  - *hard* — an explicit user deletion → a **redaction Event** that tombstones
    the target unit and cascades the tombstone to every projection. The unit's
    *content* is removed; a minimal record that "a unit was redacted here, by the
    user, at time T" remains for integrity (the append chain must not have a
    hole). (MEL-REQ-162)
- **MELFINA's adaptation memory** (learned preferences) = `inferred` Claims
  `holder = melfina` `about` the user. Explicit, inspectable, editable,
  forgettable (MEL-REQ-118–122). **Not** model weights, **not** hidden state.
  This is what makes MEL-REQ-118 ("non-parametric-first") *structural*.
- **Reliable autonomous consolidation / forgetting** (model C-2, `[U]`) is kept
  as an **optional Projection Engine behaviour** with a human-visible policy in
  the Policy Store — never a silent process. Default: off; soft-forgetting via
  relevance is enough.

## 6. Data ownership (E)

**The user owns everything.** Per `SYSTEM_ARCHITECTURE.md` §4:

- All life data (Chronicle, Registry), all policy, all audit, all capability
  metadata, all issued grants — in an **open, documented, inspectable form**;
  **fully exportable at any time without loss** (MEL-REQ-160).
- The stored format is **readable and usable without MELFINA** — documented
  schema, plain non-proprietary encoding (MEL-REQ-161). "Data outlives the
  software."
- **Full deletion is easy and complete** — any unit, any category, or everything,
  via redaction Events that cascade to projections (MEL-REQ-162).
- Capability *code* is versioned component files the user can inspect and remove.
- **No lock-in** — import/export formats do not trap the user (MEL-REQ-163).

## 7. Backup / restore, retention, encryption

- **Backup** = a copy of the Chronicle + Registry + Policy Store + Governance
  Store + Capability Registry + Audit + capability component files. Derived views
  (including the current-state cache) are **not** backed up (rebuildable). The
  user makes and restores backups **locally, with no cloud service**
  (MEL-REQ-171). A restore = load the backup, rebuild the current-state cache.
- **Retention** (Policy Store, MEL-REQ-065, 169): modest defaults, visible and
  adjustable per category; the user's future access to exhaustive self-logs is an
  OCD-relevant design choice (research `[E]`) — so retention is a *policy*, not a
  hard-coded "keep forever". Soft-forgetting (relevance) handles most of it; hard
  retention limits trigger redaction Events at the boundary.
- **Encryption at rest** (MEL-REQ-168): the whole local data set is encryptable
  with the user holding the key. The mechanism (whole-store / per-file / envelope)
  is a STORAGE/LOW-LEVEL FOUNDATIONS choice; the requirement is the property.
- **Data minimisation / privacy-by-default** (MEL-REQ-169): collect and retain
  only what a stated purpose needs; the most privacy-protective configuration is
  the default.

## 8. What this document does NOT decide (deferred to STORAGE)

- The storage engine / product (relational / graph / document / triple / log /
  custom — **none chosen, none implied**).
- The on-disk format and file layout.
- Indexing (what indexes, how maintained).
- Whether projections are in-memory, on-disk, or hybrid; eager vs lazy
  materialisation (AU-1).
- The query language / API surface.
- The tamper-evidence mechanism for the Audit log.
- The encryption mechanism.

**Constraints STORAGE must honour:** AP-4 (claim-shaped), AP-5 (append-only +
rebuildable projections), INV-1 (local, no server dependency), MEL-REQ-161 (open,
documented, outlives the software), the bitemporal query forms in §4, atomic
appends, and the deletion/redaction semantics in §5.
