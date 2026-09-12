# MELFINA — TECHNOLOGY SELECTION: CHRONICLE STORAGE EVALUATION

**Phase:** TECHNOLOGY SELECTION MISSION 001. First pass, pending human review.
Scope: **C. Chronicle storage substrate**, **D. physical record format**,
**E. current-state projection/cache mechanism**. Rubric:
`design/foundations/TECHNOLOGY_SELECTION_CRITERIA.md` §2; logical contract:
`design/foundations/CHRONICLE_LOGICAL_FORMAT.md` (F5),
`design/foundations/CHRONICLE_CONTRACT.md` (F6).

---

## 1. What the Chronicle actually needs (recap, not re-derivation)

F6 §1: "a totally-ordered, append-only sequence of E²CI units, durable before
an append returns, from which every other data structure is a rebuildable
projection." Concretely: atomic unit append (F6 §2), monotonic
transaction-time (F5 §2), durability-before-return, corruption detection
without silent repair (F6 §5), crash recovery to a known-good prefix (F6 §8),
full history retained — no compaction discards superseded units (F5 §7–8),
bitemporal query forms (F6 §3.3), one materialised current-state cache + all
other views on demand (F6 §6, O1), single-writer serialisation (F6 §7),
**no server process as the only access path** (`TECHNOLOGY_SELECTION_CRITERIA.md`
§2 MUST-NOT), local files, backup-by-copy (`MEL-REQ-160`, `171`).

**Key framing decision (stated up front, argued below):** *the substrate that
must be authoritative* and *the substrate that provides fast queries* do not
have to be the same technology. F6 §6 already requires exactly one
materialised cache that is rebuildable from the authoritative sequence — this
licenses a **two-tier design**: a minimal, audited, purpose-built append-only
log as the authoritative Chronicle, and a mature embedded query engine as the
disposable, rebuildable cache. This is evaluated as the primary candidate
below, against the single-substrate alternatives.

## 2. Candidates evaluated (authoritative substrate)

| Candidate | Why considered |
|---|---|
| **A custom append-only record log** (framed records: length + type tag + CRC32C + payload, per F6 §2's own sketch) | Exact logical-format control; no relational-impedance mismatch; smallest possible TCB for the one component on the durability-critical path. |
| **SQLite (WAL mode)** | Extraordinarily mature, embedded (no server), extensively fuzzed SQL/storage engine; the default choice for "just use a real database." |
| **LMDB** | Memory-mapped B-tree, MVCC, single-writer/multi-reader by design — a very close structural match to "one writer, many readers." |
| **RocksDB** | LSM-tree, optimised for write-heavy workloads at scale. |
| **PostgreSQL** | Considered and rejected on architectural grounds before scoring (below). |

## 3. Evaluation matrix — authoritative substrate

| Dimension | Custom log | SQLite (WAL) | LMDB | RocksDB |
|---|---|---|---|---|
| **Durable-before-return (MUST)** | **PASS** — one `fsync` per append batch, under direct control | **PASS with a caveat** — `synchronous=FULL` is durable per commit; `synchronous=NORMAL` (a common default people reach for) is **not** durable across power loss — "SQLite's durability settings are a mess" is a documented, named criticism `[E]`; must be pinned explicitly | **PASS** — durable on commit unless `MDB_NOSYNC` is set (must not be set) `[E]` | **PASS**, but durability is entangled with the LSM write path and background compaction, making the "durable before return" moment less simple to audit `[E]` |
| **Atomic unit append (MUST)** | **PASS** — framing (length+CRC+type) makes a torn write self-evidently incomplete, truncated on recovery | **PASS** — full ACID transactions; automatic rollback of a partially-written transaction on next open, **fully automatic, no user action required** `[E]` | **PASS** — LMDB transactions are always ACID | **PASS** — atomic per write-batch |
| **Corruption detection, no silent repair (MUST)** | **PASS by construction** — a bad checksum is unambiguous and the only sane response is "stop and report", matching F6 §5 exactly | **PASS** — `PRAGMA integrity_check` is mature and extensively used; a documented, narrow WAL-reset corruption bug affecting concurrent writers/checkpointers was fixed in 3.51.3 (backported to 3.44.6/3.50.7) — **pin to a patched version** `[E]` | **PASS** — B-tree checksums exist; corruption is detectable, tooling is less turnkey than SQLite's | **PASS**, but LSM-tree corruption modes are more varied (multiple SST files, manifest, WAL) and diagnosis is correspondingly more complex |
| **Crash recovery to a known-good prefix (MUST)** | **PASS by construction** — read until the first bad frame, stop there; simplest possible recovery logic | **PASS** — automatic WAL replay/rollback | **PASS** — automatic | **PASS**, more moving parts (WAL + SST + manifest) to get back in sync |
| **Full history retained, no compaction discards data (MUST — F5 §7-8)** | **PASS by construction** — there is no compaction; redaction is an appended tombstone, never a delete | **PASS** — no compaction unless `VACUUM` is run, which is never invoked on the Chronicle table | **PASS** — no compaction of committed data | **CAUTION** — RocksDB's entire design is built around background compaction merging/discarding overwritten keys; using it as a strictly-append-only, never-compacted log fights the engine's own design intent `[DI]` |
| **Bitemporal query support (F6 §3.3)** | **NONE natively** — would need the cache tier (§5) for any query beyond linear scan | **HIGH** — a real SQL engine with indexing maps naturally onto `at(valid,tx)`, `history-of`, `as-of`, `changes-between`, `provenance-of` | **MEDIUM** — a raw key-value B-tree; range queries are natural, joins/predicates are hand-rolled | **MEDIUM** — similar to LMDB, plus more tuning knobs |
| **TCB size / auditability (AP-5)** | **SMALLEST** — a few hundred lines of framing + fsync logic is realistically fully human-auditable | **LARGE** — a full SQL engine, parser, query planner, and B-tree/WAL implementation; extremely mature but not small `[E]` | **SMALL–MEDIUM** — a focused B-tree engine, no SQL parser, smaller surface than SQLite | **LARGE** — a full LSM engine with background compaction threads, originally built for server-scale workloads |
| **Single-writer serialisation (F6 §7)** | **PASS by construction** | **PASS** — WAL mode is explicitly one-writer/many-readers | **PASS** — LMDB's MVCC model is one-writer/many-readers by design | **PASS**, with more internal concurrency (background compaction) to reason about |
| **No server process (MUST-NOT)** | **PASS** | **PASS** — embedded library | **PASS** — embedded library | **PASS** — embedded library |
| **Portability / inspectability / backup-by-copy (`MEL-REQ-160`, `171`)** | **HIGH** — the format is ours; a plain-text-adjacent framing (§6) is grep-able and diffable even in a degraded state | **HIGH** — a single `.sqlite` file, well-understood, many independent tools can open and salvage it | **HIGH** — a single file, but salvage tooling is more specialist | **LOW–MEDIUM** — a directory of SST/WAL/manifest files, not a single portable artefact |
| **Maturity / fuzzing / real-world hardening** | **NONE — this is new code** | **VERY HIGH** — one of the most deployed and fuzzed pieces of software in existence `[E]` | **HIGH** — long track record (OpenLDAP and others), smaller surface than SQLite | **HIGH** — long track record at Meta-scale, but that scale is not this project's scale |
| **Fit to personal-system scale (AP-12, lightweightness)** | **HIGH** — sized exactly to the problem | **HIGH** — comfortably handles far larger workloads than one person's life record | **HIGH** | **LOW** — built for a scale this project will likely never reach; the complexity is not earning its place (mission §33) |

## 4. Recommendation — a two-tier design

> **PRIMARY: a purpose-built append-only record log as the authoritative
> Chronicle (F6's own §2 sketch, realised), plus SQLite (WAL mode,
> `synchronous=FULL`, a version ≥ 3.51.3 or a version with the WAL-reset fix
> backported) as the rebuildable current-state cache and bitemporal query
> engine (F6 §6).** **Confidence: HIGH for the two-tier shape; MEDIUM for the
> custom-log choice specifically (residual risk noted below); HIGH for SQLite
> in the cache role.**

**Why this wins the hierarchy (safety invariants first, per mission §3):**

1. The **one component that must never lose or silently corrupt data** — the
   authoritative append path — gets the **smallest, most auditable
   implementation possible**, with a corruption/durability story that is
   fully within this project's control to verify (fsync barrier + checksum +
   truncate-on-recovery is well-understood, "boring" engineering, not
   research — matching mission §31's "boring over interesting" test).
2. The genuinely hard, decades-of-engineering problem — **fast, indexed,
   bitemporal queries** — is handed to the technology that has already solved
   it extremely well (SQLite), **in a role where SQLite's own real but narrow
   reliability caveats become low-stakes**: the cache is disposable and
   deterministically rebuildable from the log (F6 §4 REPLAY), so a corrupted
   or out-of-sync cache is an inconvenience, never data loss. This directly
   uses F6 §6's own architecture (one materialised cache, rebuildable,
   authoritative reads never depend on it) rather than fighting it.
3. This is the **smaller-TCB, structurally-safer choice** the mission's §26
   TCB analysis rewards: the authoritative log's TCB is on the order of a few
   hundred lines; SQLite's much larger TCB sits entirely in the
   *non-authoritative* tier, where its failure mode is "rebuild the cache,"
   not "lose the Chronicle."
4. It avoids the **single-substrate failure mode** each alternative has on
   its own: a bare custom log with no query engine would force hand-rolling
   indexing anyway (eventually reinventing a worse SQLite); bare SQLite alone
   as *both* authoritative store and query engine reintroduces its
   `synchronous`-setting durability trap onto the one component that cannot
   afford it.

**Residual risk, stated honestly:** a hand-written append-only log is, by
definition, **new, unfuzzed code** — the one place in this evaluation where
"boring and proven" is genuinely traded for "small and auditable." This is
judged worth it because the format is intentionally minimal (§6 below), but
it is flagged as the **single highest-priority item in the experiment plan**
(`TECHNOLOGY_EXPERIMENT_PLAN.md`: crash-injection, power-loss simulation, and
fuzzing the frame parser before this is trusted with real data).

**Alternative (if the custom-log risk is judged unacceptable after
experimentation): LMDB as the authoritative substrate instead of a custom
log**, keeping SQLite as the cache. LMDB's B-tree engine is smaller and more
focused than SQLite's, has no SQL-parser attack surface, and is a very close
structural match to "one writer, many readers." **Confidence: MEDIUM** — this
is the fallback if the experiment plan finds the custom log's crash-recovery
edge cases harder to get right than expected.

**Rejected:**
- **RocksDB** — an LSM engine's entire design is oriented around background
  compaction that *merges and discards* superseded data, which fights F5
  §7–8's "no compaction ever discards a unit" requirement directly; its
  complexity and server-scale design target are not earning their place at
  this project's scale (mission §33). **Confidence in rejection: HIGH.**
- **PostgreSQL** — requires a running server process as the access path,
  which is a direct MUST-NOT-HAVE violation
  (`TECHNOLOGY_SELECTION_CRITERIA.md` §2) and conflicts with local-only,
  single-machine, no-unnecessary-service design (INV-1, mission §33).
  **Rejected outright**, not merely deprioritised — revisit only if/when a
  genuinely separate, isolated, off-by-default multi-device sync capability
  is ever built (a Ring-4 concern, `MEL-REQ-167`, explicitly out of core
  scope).

## 5. Physical record format (D)

**Recommendation: length-prefixed, checksummed frames wrapping one
JSON-encoded E²CI unit per record — JSON Lines semantics inside a binary
durability frame.**

```
[ 4-byte length ][ 1-byte unit-kind tag ][ 4-byte CRC-32C ][ JSON payload ]
```

- **Why JSON for the payload, not a binary format (CBOR/Protocol
  Buffers/bincode):** the Chronicle is the user's life record
  (`MEL-REQ-160–163`, data ownership) and a **human should be able to `grep`,
  `diff`, or hand-salvage it with a text editor** in a worst-case scenario —
  a design value already stated for governance's format criteria (F8/F9:
  "diffable for human review," "hard to misedit") and equally applicable
  here. A binary payload format is not rejected forever — it is a **DEFER**,
  revisited only if measured parse/storage overhead actually matters at this
  project's scale (unlikely for one person's data over years, per §21's
  resource-efficiency framing).
- **Why the outer frame is still binary, not "just newline-delimited
  JSON":** JSON alone has no built-in corruption boundary — a torn write
  mid-object is not obviously distinguishable from a deliberately-short file
  without an explicit length/checksum. The frame gives F6 §5's "detects
  structural corruption" and §8's "crash recovery to a known-good prefix" an
  unambiguous, cheap mechanical test, independent of JSON's own syntax.
- This is the same well-precedented pattern used by write-ahead logs and
  log-structured storage engines generally (length+type+checksum framing) —
  "boring," not novel, at the framing layer; only the payload choice (JSON
  over binary) is a deliberate trade of a little compactness for a lot of
  human inspectability.

## 6. Current-state projection / cache mechanism (E)

**Recommendation: SQLite, WAL mode, one schema derived mechanically from the
E²CI unit shapes (F5 §2–4), rebuilt by REPLAY (F6 §4) whenever it is missing,
stale beyond a bound, or found inconsistent by VERIFY (F6 §5).** This is
already F6 §6's O1 design ("one materialised current-state cache; all other
views on demand"); this section only fixes which embedded engine realises it.
No other cache technology is needed at this scale — introducing a second
indexing engine (e.g. a dedicated search index) is explicitly **DEFERRED**
until a concrete retrieval-latency problem is measured (mission §33,
"complexity must earn its place").

## 7. Interaction notes (§25 of the mission)

- **Storage × integrity (F9):** the authoritative log's own integrity
  (per-record checksums) is a *different, narrower* concern from Governance's
  cryptographic integrity chain (F9) — they must not be conflated. The
  Chronicle's checksums detect accidental corruption; they carry no
  authentication and are not a substitute for F9's human-signed chain, which
  applies only to the separate Ring-0 governance region.
- **Storage × backup/migration:** a single-file, append-only log and a
  single-file SQLite cache are both trivially copy-based-backup-friendly
  (`MEL-REQ-171`); see `MIGRATION_PORTABILITY.md` for the full treatment.
- **Language × storage:** both choices have mature, actively-maintained Rust
  bindings (`rusqlite` for SQLite; the custom log is native Rust code by
  construction) — no FFI-safety concession is required by this pairing.

## 8. Human review required

Per mission §46. **Recommendation: the two-tier design (custom authoritative
log + SQLite cache), HIGH confidence in the shape, MEDIUM confidence in the
custom-log implementation specifically pending the crash/corruption
experiment plan.** The LMDB-as-authoritative fallback is a concrete,
named alternative if that experiment plan finds problems.

## 9. Sources

[SQLite Write-Ahead Logging](https://www.sqlite.org/wal.html) ·
["SQLite's Durability Settings are a Mess"](https://www.agwa.name/blog/post/sqlite_durability) ·
["SQLite commits are not durable under default settings"](https://avi.im/blag/2025/sqlite-fsync/) ·
[How To Corrupt An SQLite Database File](https://www.sqlite.org/howtocorrupt.html) ·
[LMDB vs RocksDB vs SQLite comparison](https://db-engines.com/en/system/LMDB%3BRocksDB%3BSQLite) ·
[Symas LMDB vs RocksDB](https://stackshare.io/stackups/lmdb-vs-rocksdb).
