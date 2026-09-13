# MELFINA — CHRONICLE CRASH EXPERIMENT REPORT (PHASE 1)

**Status: PHASE 1 EXECUTED, 2026-09-13. Verdict: PASS.** Phase 2 (true
power-loss/disk-barrier simulation) and frame-parser fuzzing were **not**
run, per explicit human authorization scoping this to Phase 1 only.

**Authorization reference:** human authorization message "MELFINA —
AUTHORIZE CHRONICLE EXPERIMENT PHASE 1" (2026-09-13), scoping this run to
process-crash fault injection only, existing Rust toolchain, zero
dependency/package installation, no network, synthetic disposable data
only.

---

## 1. Exact experiment setup

**Program:** `experiments/chronicle_crash_test/` — a new, throwaway,
dependency-free Rust binary crate (pure `std`, no crates.io dependencies,
built with `cargo build --offline` to positively confirm no network access
was used or needed). Source: `experiments/chronicle_crash_test/src/main.rs`.
Build artefacts and all trial data were written **outside the repository**
(`$CLAUDE_JOB_DIR/tmp/chronicle_crash_test/`), never touching `src/` or any
real MELFINA/personal data.

**Frame format under test** (`CHRONICLE_EVALUATION.md` §5, as sketched):

```
[ 4-byte length (LE) ][ 1-byte unit-kind tag ][ 4-byte CRC-32 (LE) ][ JSON-shaped payload ]
```

Deliberate simplifications (stated in `experiments/chronicle_crash_test/README.md`,
repeated here for the record): standard CRC-32 (IEEE 802.3) was used in
place of CRC-32C (Castagnoli) — functionally equivalent for testing whether
corruption is detected at all, which is what this experiment measures; the
payload is a trivial `{"seq":N}` string, not a real E²CI unit — irrelevant
to the framing-layer properties under test.

**Roles:**
- **writer** — opens the target file with `create(true).append(true)`,
  loops forever appending records: writes the 9-byte header in one
  `write_all` + `flush`, waits a randomized 0–3ms delay, writes the payload
  in a second `write_all` + `flush`, then calls `sync_all()` (`fsync`).
  Only *after* `sync_all()` returns `Ok` does it append the same sequence
  number to a separate, independently-fsynced **ledger** file. This ledger
  is the experiment's ground truth for "what had already been durably
  claimed as of the moment of the kill" — it does not rely on stdout
  buffering, which would be a weaker basis for that claim.
- **controller** (`run_trials`) — for each trial: removes any prior file,
  spawns a fresh writer subprocess, sleeps a randomized duration, then calls
  `Child::kill()` (which sends `SIGKILL` on Unix — the writer gets no
  chance to run cleanup code), waits for it to die, then reads both the
  ledger and the chronicle file and checks them against each other.
- **bitflip_check** — a separate, non-crash sub-test: writes 10 clean
  records to completion (writer never killed), flips one bit inside an
  already-fully-written frame's payload, and confirms the reader detects
  it as corrupt rather than silently accepting it.

## 2. Exact fault-injection points

`SIGKILL` was sent at a randomized delay after spawning the writer, with
the delay range varied by trial index to deliberately scatter the kill
point across different phases of the write path:

| Trial index mod 4 | Kill-wait range | Intent |
|---|---|---|
| `== 0` | 0–2,000 µs | biased toward the first few records |
| `== 1` | 0–300 µs | biased toward **before the very first frame completes**, including before the file is even created |
| `== 2`, `== 3` | 0–50,000 µs | broad — scatter across many records, including the ~3ms header/payload gap and the fsync call itself |

Within each record, the header and payload are written in **two separate
`write_all` calls** with a randomized 0–3ms gap between them specifically
so that a kill landing in that gap produces a genuinely torn frame (a
complete, checksummed header with no payload yet on disk) rather than only
ever killing cleanly between whole records.

## 3. Number of runs

- **Run 1:** 200 trials (initial run — see §7, "unexpected behaviour," for
  a test-harness issue found and fixed after this run).
- **Run 2:** 300 trials, after the fix, on a freshly rebuilt binary. **This
  is the run the PASS verdict in §6 is based on.**
- **Bit-flip sub-check:** run twice (once per build), both times before and
  after the fix — outcome unaffected by the fix, since it doesn't involve
  killing anything.
- **Total crash trials across both runs: 500**, each with an independently
  seeded RNG (seeded from wall-clock nanoseconds XORed with the process ID
  and the trial index), so no two trials used the same kill timing.

## 4. Failures observed

**None, in the corrected run (Run 2, 300 trials) or the bit-flip check.**

**Run 1 (200 trials, before the fix)** reported 61 "reader error" lines,
all `No such device or address` / `No such file or directory` — see §7 for
why these were **not** Chronicle-design failures.

Excluding that harness issue, **both runs agree**: `total LOST durable
records = 0` and `checksum-corrupt frame detected` only ever occurred (by
design) in the dedicated bit-flip check, never as a false negative or false
positive in the crash trials.

## 5. Recovery behaviour, Chronicle/log state, and corruption detection —
observed directly

- **Truncated tails were produced and correctly detected in 82 of 300
  trials** (Run 2) — i.e. in roughly a quarter of trials, the kill landed
  mid-frame (mid-header, in the header/payload gap, or mid-payload) and the
  reader correctly reported `TruncatedTail` rather than fabricating or
  guessing at a record. **Zero of these were ever reported as a valid
  record.**
- **Checksum corruption was not encountered in either crash-trial run** —
  expected, since a `SIGKILL` between two `write_all` calls (or during one)
  overwhelmingly produces a *short* file (too few bytes for the declared
  length), not a file with the *right* number of bytes but *wrong* content.
  The checksum-mismatch path exists specifically for bit-rot-style
  corruption (a byte flipped in already-complete data), which the dedicated
  **bit-flip check exercises directly and deterministically**: it wrote 10
  clean records, flipped one bit in the 3rd record's payload, and confirmed
  the reader reported frames 0 and 1 as valid, frame 2 as
  `CorruptChecksum`, and **stopped there** — it did not attempt to parse
  anything past the corruption, matching F6 §5's "detect, do not silently
  repair" requirement exactly. Ran twice, passed both times.
- **Every record a ledger entry claimed was durable was present and valid
  on read, in all 300 trials of Run 2** (and, once the harness's
  misclassification of "file never created" is corrected, in all 200
  trials of Run 1 too — see §7). In several trials the chronicle log
  contained *more* valid records than the ledger claimed (e.g. `ledger=4,
  valid=5`) — this is expected and not a problem: it means the writer was
  killed between a chronicle record's own successful `fsync` and that
  record's separate ledger entry being written, so the ledger under-reports
  slightly. The experiment's pass criterion only checks for the dangerous
  direction (a ledgered/claimed-durable record going missing), and that
  count was 0 throughout.
- **No panics or hangs** were observed in the reader across any of the 500
  crash trials or the 2 bit-flip runs.

## 6. Acceptance/rejection of every Phase-1 criterion

| Protocol criterion (§1 of the protocol document) | Result |
|---|---|
| **Durable-before-return** — a record reported durable (ledgered) after `append()`/`sync_all()` returns survives a subsequent crash | **ACCEPTED** — 0/300 (Run 2) and 0/200-corrected (Run 1) durable records lost |
| **Atomic append** — a record is either fully present or absent after recovery, never half-written and accepted as valid | **ACCEPTED** — 82 genuinely torn frames occurred; 0 were accepted as valid; the reader's stop-at-first-trouble design never returns a partial record as `Valid` |
| **Corruption detection, no silent repair** — a damaged record is detected via its checksum and never silently treated as valid | **ACCEPTED** — bit-flip check: 2/2 pass, exact expected classification (frames before the flip valid, the flipped frame reported corrupt, nothing past it parsed) |
| **Crash recovery to a known-good prefix** — after a crash, the log is readable up to the last complete, checksummed record; nothing before it is lost, nothing after it fabricated | **ACCEPTED** — every trial's valid-record set formed exactly the "everything durable, nothing torn" prefix; no reader error/panic/hang in the corrected run |

**All four criteria are ACCEPTED for the conditions Phase 1 actually
tests** (see §8 for what that does and does not cover).

## 7. Unexpected behaviour (reported, not hidden)

**A test-harness bug, not a Chronicle-design defect, found and fixed during
this run:** in Run 1, when a trial's kill happened *before the writer ever
opened the file for the first time* (which can happen under the very tight
0–300µs kill-wait range), the chronicle file legitimately never came into
existence. The harness's reader treated `File::open` returning `ENOENT` as
a generic reader error and counted it toward the "reader errors (should be
0 for PASS)" tally — 61 of Run 1's 200 trials hit this. This was a
misclassification in the **test program**, not a finding about the
Chronicle design: a file that was never created because nothing was ever
attempted is a trivially-correct empty-log state (0 ledgered, 0 valid, 0
lost), not a failure. **Fixed** by treating `ErrorKind::NotFound`
specifically as "zero frames" rather than an error (`main.rs`, the `Err(e)
if e.kind() == io::ErrorKind::NotFound` branch), and Run 2 was executed
against the corrected binary. Re-inspection of Run 1's raw log (kept,
`$CLAUDE_JOB_DIR/tmp/chronicle_crash_test/run1.log`, not part of the repo)
confirms that of its 61 flagged trials, all 61 were this exact ENOENT case,
and its substantive counters (`LOST = 0`, `checksum-corrupt = 0`) were
already correct even before the fix — the fix corrects the harness's
*reporting*, not a result that was actually wrong.

No other unexpected behaviour was observed.

## 8. Limitations of this experiment — read this before trusting anything
above too far

Per the explicit epistemic rule given with the authorization:

- **This does NOT establish true power-loss durability.** Phase 1 kills a
  process while the OS and disk subsystem stay fully up. It says nothing
  about what happens if the machine loses power before the disk has
  actually persisted data past a volatile write cache — a materially
  different failure mode that `fsync` is also supposed to guard against,
  and which only Phase 2 (explicitly not run, needs a disposable VM or
  device-mapper fault injection not currently set up) can test.
- **This does NOT establish filesystem- or hardware-level durability.** The
  experiment trusts that this machine's actual filesystem and storage
  honour `fsync()` correctly. That assumption itself was not tested and is
  outside Phase 1's scope.
- **This does NOT establish complete Chronicle correctness.** Only the
  four framing-layer properties in §1 of the protocol were tested. Nothing
  here validates the E²CI unit schema, the bitemporal query layer, the
  SQLite cache tier, grant-shape checking on append, or anything else in
  F5/F6 beyond the raw append/read frame mechanics.
- **This does NOT establish production readiness.** The test program is a
  throwaway, single-threaded, single-writer, no-concurrent-readers spike —
  it does not exercise F6 §7's concurrent-reader/queued-writer semantics,
  redaction, supersession, or the real E²CI payload shapes.
- **Kernel-level nuance, stated honestly:** most torn frames observed came
  from killing *between* the two `write_all` calls (header vs. payload),
  not from interrupting a single `write_all`/`fsync` syscall mid-flight —
  Linux does not generally leave a synchronous, non-`O_NONBLOCK` regular-
  file write half-applied at the kernel level just because the calling
  process was `SIGKILL`ed immediately after issuing it. This is a
  reasonable, expected result, but it means Phase 1's torn-frame coverage
  is concentrated at inter-syscall boundaries rather than intra-syscall
  ones — a nuance worth knowing, not a flaw in the result.
- **A CRC-32 (IEEE), not CRC-32C, was used** — see §1. Immaterial to what
  was tested (whether corruption is detected at all), but the exact
  checksum choice for the real implementation remains a CORE ENGINE
  decision, not fixed by this experiment.

**None of the above is claimed to be true. All of it is explicitly
disclaimed.** Phase 1 establishes exactly, and only, what §6's table says
it establishes.

## 9. PHASE 1 VERDICT

# PHASE 1: PASS

**Evidence for this verdict:**
- 300 corrected crash trials (Run 2), 0 lost durable records, 0 reader
  errors, 0 panics/hangs, 82 correctly-detected torn frames, 0 frames
  incorrectly accepted as valid.
- 200 initial crash trials (Run 1), substantively consistent (0 lost, 0
  checksum-corruption false negatives) once a harness misclassification
  (not a design defect) is accounted for.
- 2/2 dedicated bit-flip corruption-detection checks passed, with the exact
  expected per-frame classification.
- All four protocol acceptance criteria (§6) accepted under the conditions
  actually tested.

**What this verdict does NOT mean**, restated per the epistemic rule: it
does not mean the custom Chronicle log is proven safe against true
power-loss, proven correct as a whole, or ready for production use. It
means: **the specific claim Decision 2 was conditioned on — that the
framing design's durability/atomicity/corruption-detection/recovery logic
holds under process-crash conditions — held, across 500 trials and 2
dedicated corruption checks, with no counter-example.**

## 10. Recommended next step (not decided here)

Per `HUMAN_DECISION_PACKAGE.md` Decision 2 and this protocol's own §5:
Phase 1 passing moves the custom-log design **one step closer** to being
trusted with real data, but Phase 2 (true power-loss simulation) remains a
documented, open gap — a judgement call for you on whether to set up the
tooling for it (a disposable VM or device-mapper fault injection, both
requiring installation not yet authorized) before trusting the design
fully, or to accept Phase 1's result as sufficient for now given other
mitigations (e.g. a reliable power supply, journaling filesystem
guarantees you're otherwise comfortable with). **Not decided here — stated
as the open question it is.**

## 11. Traceability

| Element | Source |
|---|---|
| The protocol executed | `design/technology/CHRONICLE_CRASH_EXPERIMENT_PROTOCOL.md` |
| The properties tested | `design/foundations/CHRONICLE_CONTRACT.md` (F6) §2, §5, §8 |
| The frame format | `design/technology/CHRONICLE_EVALUATION.md` §5 |
| The decision this gates | `design/technology/HUMAN_DECISION_PACKAGE.md` Decision 2 |
| The test program | `experiments/chronicle_crash_test/` |
| Human authorization scoping this run | "MELFINA — AUTHORIZE CHRONICLE EXPERIMENT PHASE 1" (2026-09-13) |
