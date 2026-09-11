# MELFINA — FOUNDATION 6: CHRONICLE APPEND / QUERY CONTRACT

**Phase:** LOW-LEVEL FOUNDATIONS — MISSION 001. First pass, pending review.
**Baseline:** `8a40207`. Read F5 (`CHRONICLE_LOGICAL_FORMAT.md`) first.

**This document does not choose a storage engine, on-disk format, index, or query
language.** It fixes the **abstract operations** on the Chronicle and the
guarantees each must provide: `APPEND`, `READ`, `QUERY`, `REPLAY`, `VERIFY`,
`PROJECT`.

Derives from: **INV-3, INV-5**; **AP-3, AP-5**; **RC/M2**; **MEL-REQ-170, 171,
173, 174, 021, 020**; append-only log durability (fsync, record framing, torn
writes), event sourcing + CQRS, bitemporal query `[E]`.

---

## 0. Dependencies discovered

- **On F5:** the units APPEND takes and QUERY returns are the three unit kinds of
  F5. `transaction-time` assignment (mentioned in F5) is defined here.
- **On F3:** the Reference Monitor's authoritative reads (M2) go through the
  **READ / QUERY** operations here, specifically the "fresh authoritative read"
  mode (§3), never a projection.
- **On F4/F10:** the atomic-append guarantee here is what makes "record what
  actually happened" reliable across an interrupting crash; VERIFY-on-resume
  (F10) uses `VERIFY` + `REPLAY` here.
- **On F1:** a `chronicle-append` / `chronicle-read` grant's `scope` selects
  units by the identity/selector forms of F5; APPEND enforces that the appended
  units match the granted shapes.
- **Discovered constraint:** the *current-state cache* is the **one** materialised
  projection (O1); every other view is `PROJECT`-on-demand. The contract must
  make "consequential reads never trust a lagging view" (M2) a first-class mode,
  not a caller's discipline.

---

## 1. The authoritative object

> **The Chronicle is a totally-ordered, append-only sequence of units (F5),
> durable before an append returns, from which every other data structure in
> MELFINA is a rebuildable projection.**

- **Total order** is by `transaction-time`, assigned by `APPEND` (§2).
- **Append-only:** no operation updates or removes a unit in place. "Redaction"
  (F5 §8) is itself an appended `redaction` Event that tombstones content while
  leaving the chain intact.
- **Authoritative:** losing every projection loses nothing (AP-5). Losing the
  Chronicle loses the user's life record — hence local backup (`MEL-REQ-171`),
  whose mechanism is a TECHNOLOGY SELECTION concern (F12 §2), not fixed here.

`[SEN]`.

---

## 2. APPEND

**Input:** one or more well-formed units (F5), optionally as an **atomic group**
(a unit + the Claims it `brings-about`, or a `supersedes` pair).

**Guarantees:**

| Guarantee | Detail | Class |
|---|---|---|
| **Atomicity** | the whole unit (or the whole atomic group) lands, or nothing does. There is **never** a partially-written unit visible to any reader. A crash mid-append leaves the last consistent prefix. | **[ID]** (record framing + checksum + a durability barrier; e.g. a length+type+CRC frame so a torn tail is detected and ignored on the next open — `[E]`) |
| **Monotonic `transaction-time`** | `APPEND` assigns a `transaction-time` strictly greater than every prior unit's; the assignment is **immutable**. Wall-clock is *used* for it but monotonicity is preserved across clock adjustments (a logical component — a Lamport-style counter — backs it; L1). | **[SEN]** semantics / **[ID]** the monotone counter |
| **Durability before return** | when `APPEND` returns success, the unit is on stable storage (the equivalent of `fsync` on the log — `[E]`). "Captured data is not silently lost" (`MEL-REQ-170`). | **[ID]** |
| **Shape-checked against the grant** | an `APPEND` performed under a `chronicle-append` grant is rejected if a unit does not match the granted unit shapes (F1). Ring-1 mechanism appends (capture, action results) are not grant-gated but are still F5-schema-checked. | **[SEN]** rule / **[ID]** shape matching |
| **Capture is one action** | a user capture appends its minimal Event/Claim **immediately**; `kind`, links, next action are **later** Claims (`MEL-REQ-020`, `021`). `APPEND` never blocks capture on classification. | **[SEN]** |
| **No duplicate suppression by default** | `APPEND` does **not** dedupe by content — identical content ⇒ distinct `unit-id`s (F5 §2). Idempotency is the *caller's* concern via an optional `idempotency-key` (§7). | **[SEN]** |

**On malformed input:** the unit is **not appended**; a `malformed-unit` record
is appended instead (so the attempt is visible); the caller is told. `fail
closed` — a malformed unit never lands. `[SEN]`.

**On failure** (disk full, I/O error): `APPEND` **fails cleanly** — no partial
unit is written; the caller is told; capture retries when health returns
(`FAILURE_AND_RECOVERY.md §5`). The user's data is not corrupted — the failed
append simply did not happen. `[SEN]` semantics / `[ID]`.

---

## 3. READ / QUERY

Two modes, deliberately distinct:

### 3.1 Authoritative read (the "Chronicle contract" of M2)

**Input:** a `unit-id` or a **bounded selector** (F5 §4) + a bitemporal
coordinate.

**Guarantee:** returns the answer computed **directly from the authoritative
Chronicle sequence** as of a specified `transaction-time` (default: the latest
appended). It **never** consults the current-state cache or any lagging
projection. This is the mode the **Reference Monitor** and the **DECIDE/classify
step** use for any conjunct whose correctness is consequential (M2). It may be
slower; that is accepted (`ARCHITECTURAL_ALTERNATIVES.md` AA-2, Risk 2).

`[SEN]` (that this mode exists and is the one consequential paths use); `[ID]`
(its implementation).

### 3.2 View read (pull surfaces, non-consequential framing)

**Input:** a named view + parameters.

**Guarantee:** returns from the **current-state cache** or a `PROJECT`-on-demand
view (§6). **May lag** the Chronicle by a bounded consistency window. Acceptable
for the user's pull surfaces ("what needs attention", calendar, a learning
trajectory) and for Ring-2 context framing. **Never** the basis for a
consequential decision (§3.1). `[SEN]`.

### 3.3 Bitemporal query forms

The contract requires these query *capabilities* (F5 §5); the mechanism is
deferred:

| Form | Meaning |
|---|---|
| `at(valid = V, tx = now)` | what MELFINA now believes was true at V |
| `at(valid = V, tx = T)` | what MELFINA believed at time T about V |
| `history-of(unit-id)` | the `supersedes` chain; every version |
| `as-of(tx = T)` | the whole knowledge state at `transaction-time` T |
| `changes-between(tx = T1, tx = T2)` | what was appended / superseded in that window |
| `provenance-of(unit-id)` | the `generated-by` / `derived-from` walk (F5 §6) |

`[SEN]` (the required forms) / `[ID]` (the query engine).

---

## 4. REPLAY

**Input:** a starting `transaction-time` (or the beginning) and a target
projection definition.

**Guarantee:** deterministically reconstructs a projection by processing the
units from the start point in `transaction-time` order. **Idempotent** —
re-processing the same units yields the identical projection (INV-3, AP-3).
`REPLAY` from the beginning **exactly reconstructs** any projection, including the
current-state cache; this is the recovery ground truth (`FAILURE_AND_RECOVERY.md
§3`). `[SEN]` semantics / `[ID]`.

---

## 5. VERIFY (integrity)

**Input:** the Chronicle (typically at open / startup).

**Guarantee:** detects, without repairing:

- **structural corruption** — a torn/partial unit at the tail (framing/checksum
  mismatch), an out-of-order `transaction-time`, a break in the append chain
  (including a redaction that removed the shell, F5 §8);
- **provenance dangling** — a `derived-from` / `supersedes` / `brings-about`
  pointing at a `unit-id` that does not exist (as a *report*, not a hard error —
  it may be a legitimately redacted target).

**On detected corruption:** MELFINA enters a **read-only recovery mode**, reports
**precisely** what is inconsistent, and does **not silently repair**
(`MEL-REQ-170`, `FAILURE_AND_RECOVERY.md §5`). Recovery options (truncate to the
last valid unit; restore from backup; manual inspection) are presented to the
user, who decides. `[SEN]` for "detect + report + no silent repair"; `[ID]` for
the detection mechanism (hash chain / checksums / journaling — F-build / STORAGE).

**A tamper-evidence property is required of the eventual mechanism** (`MEL-REQ-203`)
— an append chain such that a modification or deletion of a past unit is
**detectable** (a hash-chain or Merkle structure over units, in the manner of a
transparency log — `[E]`). This is a property of the *Audit* trail especially
(F-build); for the Chronicle it is `[ID]` and constrained here.

---

## 6. PROJECT

**Input:** a projection definition + the Chronicle (from a checkpoint or the
beginning).

**Guarantees:**

| Guarantee | Detail |
|---|---|
| **one materialised cache** | the **current-state cache** is the single maintained projection (O1). It stores "processed up to `transaction-time` T"; on restart it `REPLAY`s from T. | **[SEN]** |
| **all other views are on-demand** | "what needs attention", calendar, learning trajectory, capability reliability, an opted-in progress view — computed per request from the Chronicle (or the current-state cache), **not separately maintained** (O1, M1). No checkpoint; recomputed each time. | **[SEN]** |
| **Context is NOT projected here** | Context is a per-situation Ring-2 derivation (F5, M1) — the Projection Engine does not build or hold it. | **[SEN]** |
| **rebuildable** | deleting any projection and `REPLAY`ing reconstructs it exactly (AP-5). Loss of every projection loses nothing. | **[SEN]** semantics / **[ID]** |
| **may lag** | a bounded consistency window between an `APPEND` and its reflection in the current-state cache. Acceptable for view reads; **not** for authoritative reads (§3.1). | **[SEN]** |
| **redaction cascades** | a `redaction` Event tombstones the target across the current-state cache and any computed view (F5 §8). | **[ID]** cascade completeness |
| **silent divergence** | a cache that drifts from the Chronicle without throwing is the subtle case. A periodic consistency audit (recompute a sample by direct Chronicle query, flag divergence, force a rebuild) is a **future implementation concern (M5 / O6)**, not an architecture requirement. The structural guard: nothing consequential trusts the cache (M2). | **[ID]** / **[OPEN]** (whether the audit is ever built) |

---

## 7. Concurrency and ordering

- **The Chronicle is the serialisation point.** `APPEND`s are totally ordered;
  concurrent producers **queue** — a simple, predictable consistency model for a
  single-user system without distributed-consensus machinery (`RUNTIME_MODEL.md`
  §6). `[SEN]` semantics / `[ID]`.
- **Projections are built by consumers of the append stream** and can lag
  independently; a slow projection worker does not block a fast one. `[SEN]`.
- **Idempotent append:** a caller that must not double-append (e.g. an automation
  retrying) supplies an `idempotency-key`; `APPEND` with a key that has already
  landed returns the existing `unit-id` and appends nothing. The key space and
  retention are a build concern. `[ID]`.
- **Reads are consistent to a `transaction-time` snapshot** — a query specifies
  (or defaults to) a `tx` coordinate and sees a stable prefix; a concurrent
  append does not change a query already in progress. `[ID]`.

---

## 8. Crash recovery requirements

| Requirement | Detail | Class |
|---|---|---|
| **atomic appends** | the Chronicle never contains a partial unit; a crash mid-append leaves the last consistent prefix (§2) | **[ID]** (framing/checksum + durability barrier — `[E]`) |
| **integrity check on open** | `VERIFY` runs at every open; corruption ⇒ read-only recovery mode + precise report + no silent repair (§5) | **[SEN]** rule / **[ID]** |
| **projection recovery** | `REPLAY` from the last good checkpoint; if replay fails, rebuild from scratch from the whole Chronicle; consequential reads go direct (§3.1) meanwhile (`FAILURE_AND_RECOVERY.md §3`) | **[SEN]** semantics / **[ID]** |
| **in-flight action recovery** | an action that had reached EXECUTE and whose effect is unknown is treated as **"may have happened"**: VERIFY-on-resume (F10) determines the actual state; the Chronicle records **what is observed, not what was intended** (`FAILURE_AND_RECOVERY.md §5`) | **[SEN]** |
| **no dependence on network time** | scheduled work runs off wall-clock against `valid-time` targets; `transaction-time` monotonicity survives clock adjustment (INV-1, `RUNTIME_MODEL.md §7`) | **[SEN]** |

---

## 9. Contract Q&A

| Question | Answer |
|---|---|
| **Guarantees** | append is atomic + durable-before-return + monotonically ordered; the sequence is the single source of truth; every projection is `REPLAY`-rebuildable and disposable; an authoritative-read mode exists that never touches a lagging view; corruption is detected + reported + never silently repaired; capture is never blocked on classification |
| **Requires** | stable storage with a durability barrier; record framing that makes torn writes detectable; a monotone logical counter for `transaction-time`; F5-well-formed units |
| **Trusts** | its own append order and durability barrier; a unit's F5 schema conformance (checked at append) |
| **Distrusts** | a projection / cache for a consequential read (M2); the intended effect of an in-flight action across a crash; wall-clock as a monotonicity guarantee; a caller's claim that a unit is well-formed (re-checks) |
| **Enters** | proposed units (APPEND); selectors + bitemporal coordinates (READ/QUERY); projection definitions (PROJECT/REPLAY) |
| **Leaves** | a `unit-id` + assigned `transaction-time` (APPEND); query results as-of a `tx` snapshot (READ/QUERY); a reconstructed projection (REPLAY/PROJECT); an integrity report (VERIFY) |
| **Malformed input** | a malformed unit is not appended; a `malformed-unit` record is appended instead; a query with an invalid selector returns an error, not a guess |
| **Failure** | append fails cleanly (no partial unit); corruption ⇒ read-only recovery mode + user decides; a failed projection returns an error for that request and is recomputed next time; core reads/capture continue during projection failure |
| **Authoritative** | the append-only unit sequence; the authoritative-read mode; `VERIFY`'s corruption report |
| **Independently verifiable** | yes — replay determinism (REPLAY twice ⇒ identical projection); atomicity + torn-write detection (fault-injection at append); "authoritative read never consults a projection" (instrumentation / code audit); the bitemporal query forms against a fixture history with known answers |

---

## 10. Enforceability summary

| Invariant | Class |
|---|---|
| append-only; no update/remove in place; redaction is an appended tombstone | **[SEN]** |
| total order by monotone immutable `transaction-time` | **[SEN]** rule / **[ID]** counter |
| atomic append; no partial unit ever visible; torn tail detected on open | **[ID]** |
| durable before return | **[ID]** |
| authoritative-read mode exists and is what consequential paths use (M2) | **[SEN]** rule / **[ID]** |
| one materialised projection (current-state cache); all else on-demand (O1) | **[SEN]** |
| Context not projected by the Projection Engine (M1) | **[SEN]** |
| every projection `REPLAY`-rebuildable; loss of all projections loses nothing | **[SEN]** semantics / **[ID]** |
| corruption detected + reported + never silently repaired | **[SEN]** rule / **[ID]** detection |
| tamper-evidence of the append chain (`MEL-REQ-203`) | **[ID]** (hash-chain / Merkle — F-build) |
| capture never blocked on classification | **[SEN]** |
| serialisation point = the Chronicle; single-writer discipline; no distributed consensus | **[SEN]** semantics / **[ID]** |
| silent-divergence audit of the cache | **[OPEN]** (M5 / O6) |

---

## 11. Deferred (STORAGE / LOW-LEVEL FOUNDATIONS)

- The storage engine, on-disk format, encoding, indexing, query language / API.
- The record framing (header layout, checksum algorithm) and the durability
  barrier (`fsync` / `fdatasync` / `O_DSYNC` / a journal).
- The `transaction-time` counter's exact form (hybrid logical clock vs a pure
  counter + a best-effort wall-clock, L1).
- Whether the current-state cache is in-memory / on-disk / hybrid; eager vs lazy
  (AU-1).
- Checkpoint frequency and format.
- The tamper-evidence mechanism for the Chronicle and the Audit log.
- The idempotency-key space and retention.
- Whether the silent-divergence consistency audit (M5 / O6) is built, and when.

## 12. Traceability

| Element | Source |
|---|---|
| append-only; nothing overwritten; deletion a separate act | INV-5; AP-5; `MEL-REQ-160`, `162`, `170`; model §11 |
| data durability, no silent loss or corruption; recovery path | `MEL-REQ-170`, `173` |
| local backup/restore, no cloud | `MEL-REQ-171` |
| deterministic components where practical; predictable core | `MEL-REQ-174`; INV-3; AP-3 |
| consequential reads go to authoritative state, not a lagging view | M2; `ARCHITECTURAL_ALTERNATIVES.md` AA-2 |
| one Chronicle + one current-state cache + on-demand views | O1; `DATA_AND_STATE_MODEL.md` §2–§3 |
| Context is a Ring-2 on-demand build, not projected | M1 |
| tamper-evident audit trail | `MEL-REQ-203`; transparency-log pattern `[E]` |
| capture is one action, no forced classification | `MEL-REQ-020`, `021` |
| append durability + framing + torn-write detection | WAL / append-log crash-consistency literature `[E]` |
| silent-divergence audit is a future concern | M5 / O6; adversarial review §5 M5 |
