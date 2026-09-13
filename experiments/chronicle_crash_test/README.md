# chronicle_crash_test — throwaway spike, not real MELFINA code

Implements Phase 1 of
`design/technology/CHRONICLE_CRASH_EXPERIMENT_PROTOCOL.md`: process-crash
fault injection against a minimal implementation of the Chronicle's proposed
frame format (`CHRONICLE_EVALUATION.md` §5), to test the durability,
atomicity, corruption-detection, and crash-recovery claims in
`design/foundations/CHRONICLE_CONTRACT.md` (F6) §2/§5/§8 before any real
data is trusted to the design.

**This is a spike, per `experiments/README.md`: throwaway, never the real
system.** Nothing here is linked to, imported by, or a dependency of any
future real MELFINA implementation. It has zero external dependencies
(pure Rust `std`, no crates, no network fetch needed to build) and writes
only to scratch paths passed on the command line — it does not touch `src/`
and does not touch any real user data.

**Deliberate simplifications, stated for the record (does not affect what
is being tested):**
- Uses standard CRC-32 (IEEE 802.3), not CRC-32C (Castagnoli) as sketched in
  `CHRONICLE_EVALUATION.md` §5 — functionally equivalent for testing
  *whether corruption is detected at all*, which is the property under test
  here; the exact checksum polynomial is a CORE ENGINE-time choice, not
  something this experiment needed to pin down.
- The JSON payload is a trivial `{"seq":N}` string, not a real E²CI unit —
  irrelevant to the four properties under test, which concern the framing
  layer, not the payload's own schema.

## Usage

```
chronicle_crash_test writer <chronicle-path> <ledger-path>
    # appends records in a loop until killed; not meant to be run standalone

chronicle_crash_test run_trials <n> <trial-dir>
    # runs n crash trials, each spawning+killing a fresh writer subprocess,
    # then verifies the resulting chronicle file against the four
    # properties. Prints a per-trial line and a final summary.
```

Results of the actual run performed under human authorization are recorded
in `design/technology/CHRONICLE_CRASH_EXPERIMENT_REPORT.md`, not in this
directory — this directory holds only the test program itself.
