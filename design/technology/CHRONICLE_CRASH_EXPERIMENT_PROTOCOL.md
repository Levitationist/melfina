# MELFINA — CHRONICLE CRASH / POWER-LOSS EXPERIMENT PROTOCOL

**Status: PROTOCOL PREPARED, NOT YET RUN.** This document specifies
experiment #1 from `TECHNOLOGY_EXPERIMENT_PLAN.md` in enough concrete detail
to actually execute it — but **no experiment code has been written, no
binary has been built, and nothing has been run.** This is the gate
Decision 2 (`HUMAN_DECISION_PACKAGE.md`) is conditioned on: the custom
Chronicle append-only log (`CHRONICLE_EVALUATION.md` §5) is not trusted with
real data until this passes.

**Why this is the highest-priority experiment:** every other technology
recommendation in this mission composes existing, extremely mature
mechanisms (Rust's compiler, Landlock, SQLite, Ed25519). The Chronicle's
authoritative log format is the **one piece of genuinely new code** in the
whole stack. This experiment is where that specific risk gets tested before
anything real depends on it.

---

## 1. What is being tested

The claims made in `CHRONICLE_EVALUATION.md` §5 and F6 (`CHRONICLE_
CONTRACT.md`) about the frame format:

```
[ 4-byte length ][ 1-byte unit-kind tag ][ 4-byte CRC-32C ][ JSON payload ]
```

Specifically:
1. **Durable-before-return** — once an append call returns, the record
   survives a subsequent crash (F6 §2).
2. **Atomic append** — a record is either fully present or absent after
   recovery, never half-written and accepted as valid (F6 §2).
3. **Corruption detection, no silent repair** — a damaged record is
   detected via its checksum and never silently treated as valid (F6 §5).
4. **Crash recovery to a known-good prefix** — after a crash, the log is
   readable up to the last complete, checksummed record; nothing before
   that point is lost, nothing after it is fabricated (F6 §8).

## 2. Experiment design — two phases

### Phase 1 — process-crash fault injection (no installation needed)

Simulates the dominant real-world failure mode for a single-user desktop
system: **the writing process dies** (a panic, an OOM-kill, a `SIGKILL`, a
power-button-forced shutdown that the OS itself survives) while the OS and
disk stay up. This does **not** require a VM — it can run directly, using
only the already-installed Rust toolchain (`rustc`/`cargo`, confirmed
present per `PROJECT_STATE.md` §7), entirely inside `experiments/` (per the
repository's own convention: "throwaway probes and spikes, never the real
system").

**Design:**
1. A minimal, throwaway Rust program (not part of any future real
   MELFINA code) implements exactly the frame format above: an `append(record: &[u8]) -> io::Result<()>` function that serialises the frame and calls `write_all` + `flush` + `sync_all` (`fsync`), and a `read_all() -> Vec<Result<Record, CorruptionError>>` function that reads frames until EOF or a bad checksum/length.
2. A **writer** subprocess appends records in a tight loop (e.g. a
   monotonically increasing counter as the JSON payload), reporting each
   record's sequence number to stdout **immediately after its `append` call
   returns** (i.e. after the claimed durability point).
3. A **controller** process launches the writer, lets it run for a random
   short interval, then sends it `SIGKILL` at an unpredictable point —
   including deliberately mid-write (e.g. by pausing the writer with
   `SIGSTOP` at a byte-level probability point instrumented into the write
   path for this test build only, then `SIGKILL`ing it) to maximise the
   chance of catching a genuinely torn write, not just a clean stop between
   records.
4. After the kill, a fresh process opens the log with `read_all()` and
   checks:
   - every sequence number the writer reported as durable **before** the
     kill is present and uncorrupted;
   - no record is present whose checksum doesn't match (i.e. `read_all()`
     never returns a "valid" record that wasn't actually intact);
   - if a torn frame exists at the very end (the write in progress when
     killed), it is reported as corruption/truncation, never as a
     successfully-parsed record;
   - the reader never panics or hangs on a torn tail.
5. Repeat for **at least 200 randomized kill-timing trials**, varying the
   record size and the kill-point distribution (some early in a frame's
   write, some at the length-prefix boundary, some at the checksum
   boundary, some between frames).

**Pass criterion:** zero trials in which a record reported as durable is
lost, zero trials in which a corrupted/torn record is accepted as valid, and
zero panics/hangs on read. **Fail criterion:** any single violation of the
above — which would trigger the recorded fallback (LMDB as the
authoritative substrate, per Decision 2).

### Phase 2 — true power-loss / disk-barrier simulation (needs a decision
before it can run)

Phase 1 tests process death with the OS and disk subsystem intact. It does
**not** test what happens if the machine loses power **before the disk
itself has actually persisted data past its write cache** — a different,
narrower, but real failure mode `fsync` is specifically supposed to guard
against.

Testing this properly needs either:
- a **disposable virtual machine** whose disk can be forcibly, abruptly
  killed (e.g. `qemu` with `-S`/`quit` mid-write, or simply `kill -9` on the
  VM process itself, which drops any data the VM's own guest OS thought was
  written but the host had not yet flushed) — the safe, standard way to
  simulate this without touching a real disk; or
- **device-mapper fault injection** (`dm-flakey`, or a similar block-layer
  fault-injection facility) on a **loopback file**, not a real partition —
  also safe, but requires kernel module / `device-mapper` tooling that may
  not be loaded by default.

**Neither of these is installed or set up on this machine right now, and
setting either up is exactly the kind of "installing software or changing
the system" this protocol is required to stop and ask about before doing.**

## 3. Fuzzing the frame parser (a related, separate near-term item)

`TECHNOLOGY_EXPERIMENT_PLAN.md` item 2 also calls for fuzzing the frame
parser with `cargo fuzz`. **`cargo-fuzz` is not currently installed** (it
requires a Rust nightly toolchain component and `libFuzzer`) — this is a
second, separate thing that needs an explicit install decision, kept
distinct from Phase 1 above because it tests a different property
(parser robustness against arbitrary/adversarial byte sequences, not
crash-timing behaviour).

## 4. STOP — what is needed to actually run each part

| Part | Needs installing anything? | Status |
|---|---|---|
| **Phase 1** (process-crash fault injection) | **No** — `rustc`/`cargo` already present per the environment snapshot; everything else is a throwaway program under `experiments/` | **Ready to write and run — asking for explicit go-ahead before doing so**, since it involves writing and executing new code (even if throwaway and sandboxed to `experiments/`), which this session was told to treat conservatively |
| **Phase 2** (true power-loss/disk-barrier simulation) | **Yes** — a disposable VM (e.g. `qemu`) or device-mapper fault-injection tooling, neither present now | **Blocked pending your decision** on which mechanism to set up, and confirmation that installing it is authorised |
| **Frame-parser fuzzing** (`cargo fuzz`) | **Yes** — a Rust nightly component + `cargo-fuzz` | **Blocked pending your decision** on whether to install it now or defer it alongside Phase 2 |

**Recommendation, not a decision made unilaterally:** run Phase 1 first (no
install needed, directly tests the specific claim Decision 2 is conditioned
on) and treat Phase 2 + fuzzing as a follow-up once Phase 1's result is in —
a Phase-1 failure would already trigger the LMDB fallback and make Phase 2
moot for the custom-log design; a Phase-1 pass is still not a full pass
(Phase 2 tests a different, real failure mode) but is the more informative
next data point for the least additional setup.

## 5. What happens after this runs

- **Phase 1 passes cleanly (200/200 trials):** recorded as evidence the
  custom log's crash-recovery logic is sound against process-death; Decision
  2's condition is one step closer to satisfied, but not fully — Phase 2
  remains a documented gap until it's actually run or a decision is made not
  to bother (e.g. if the eventual deployment target has battery-backed
  storage or other mitigations making true power-loss less of a concern —
  a judgement call for you, not decided here).
- **Phase 1 fails (any violation):** the custom log design is not trusted
  with real data; `CHRONICLE_EVALUATION.md` and `HUMAN_DECISION_PACKAGE.md`
  Decision 2 are updated to reflect the fallback to LMDB as the
  authoritative substrate, and this document's Phase 1 design is revisited
  for the *cause* of the failure before deciding whether the fallback is
  necessary or the custom log's implementation just needs a fix.

## 6. Traceability

| Element | Source |
|---|---|
| The frame format under test | `CHRONICLE_EVALUATION.md` §5 |
| The four properties tested | `design/foundations/CHRONICLE_CONTRACT.md` (F6) §2, §5, §8 |
| Why this is the highest-priority experiment | `TECHNOLOGY_EXPERIMENT_PLAN.md` item 1 |
| The condition this experiment gates | `HUMAN_DECISION_PACKAGE.md` Decision 2 |
| "Run dangerous experiments inside appropriate isolation" | `TECHNOLOGY_EXPERIMENT_PLAN.md` §2, mission §38's own instruction |
