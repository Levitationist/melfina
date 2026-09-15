# MELFINA — CHRONICLE PHASE 2 (MECHANISM A) EXECUTION PROCEDURE

**Status: PREPARED, NOT YET RUN — REVISED 2026-09-15.** This is the
human-reviewable procedure for Phase 2 Mechanism A (dm-flakey fault
injection), per `CHRONICLE_CRASH_EXPERIMENT_PROTOCOL.md` §2. **No
privileged command in this document has been executed by Claude.**

**Revision note.** The first version of this document used `sudo chmod
666` (world-writable) on the flakey device node and assumed `$LOOPDEV`
would still be set in whatever shell later ran teardown. A dry run of §A
(the human ran only the `chmod` line, out of order, before the device
existed) surfaced both problems concretely: the command failed harmlessly
(`No such file or directory`, confirmed independently, no side effect) but
made clear the ordering and shell-persistence assumptions needed fixing,
not just the permission mode. This revision:
- replaces `chmod 666` with owner-only `chown <uid>:<gid>` + `chmod 600`;
- adds an explicit existence check for the device node **before** any
  permission change is attempted, anywhere in §A;
- re-derives the loop device from the kernel (`losetup -j <backing
  file>`) at every point that needs it, rather than trusting a shell
  variable to have survived between separate invocations — including at
  teardown, which is now safe to run from a brand-new shell;
- adds guard checks throughout that `exit 1` with a clear message rather
  than silently continuing on an unexpected result;
- makes teardown idempotent (safe to re-run) and gives explicit guidance
  for what to do if a step fails instead of leaving the human stuck.

---

## 0. Why this design (read before running)

Phase 1 killed a process while the OS/disk stayed up. Phase 2 needs to
answer a different question: **what if `fsync()` returns success but the
data never actually reaches stable storage** — the literal definition of a
power-loss-class failure. `dm-flakey`'s `drop_writes` feature does exactly
this at the block layer: during a configured "down" window, writes are
silently discarded while still appearing to succeed to the caller above it.

**Design choice made while preparing this (and the reasoning for it):**
Mechanism A runs **directly against the raw flakey block device — no
filesystem, no mount, anywhere in this procedure.** Layering a real
filesystem (ext4) on top would additionally risk the *filesystem's own*
metadata being corrupted by dropped writes, which would test ext4's crash
consistency, not the Chronicle's frame-format logic — a different
question, and one that would also make safe, clean teardown harder (a
corrupted mount can need `fsck` before it will even unmount). Testing the
raw device instead keeps the blast radius to one disposable device-mapper
node and directly isolates the property actually in question.

**A real code change this required, found and fixed before touching any
device (see §C):** Phase 1's writer relied on `O_APPEND` to a regular file
and a reader that used `read_to_end` (which stops exactly at real EOF).
Neither works against a fixed-size block device: `O_APPEND` on a device
seeks to the device's constant total size, not to "how far our own writes
have gotten"; and reading to the device's full size would include
never-written, zero-filled space that — checked — would misparse as
spurious valid empty frames (CRC-32 of an empty payload is `0`, which
matches a zeroed stored checksum). Both are fixed in the new code: explicit
offset tracking on every write, and a sentinel tag-byte check that treats
non-written space as "end of data," never as a valid frame. This was unit
tested against a synthetic fixed-size regular file (zero privilege, no
real device) before this procedure was written: **PASS, 483/483 records
verified intact.**

## A. Commands you run manually, with sudo

**Run each numbered step and read its output before typing the next one.**
Several steps are guards that intentionally stop (`exit 1`) rather than
continue on an unexpected result — that is deliberate, not a bug in the
procedure.

```bash
# ---- A0. Baseline — capture current state, to diff against at teardown ----
sudo dmsetup ls
sudo losetup -a

# ---- A1. Verify the exact backing file BEFORE touching it ----
BACKING="/home/levy/.claude/jobs/de309c9f/tmp/chronicle_crash_test/mechA/mecha_backing.img"
MECHA_DIR="/home/levy/.claude/jobs/de309c9f/tmp/chronicle_crash_test/mechA"

test -f "$BACKING" || { echo "STOP: backing file missing at $BACKING"; exit 1; }
SIZE=$(stat -c%s "$BACKING")
[ "$SIZE" -eq 268435456 ] || { echo "STOP: backing file is $SIZE bytes, expected exactly 268435456 (256 MiB) — do not proceed"; exit 1; }
echo "OK: backing file confirmed at $BACKING, exactly 256 MiB, not a real device path"

# ---- A2. Allocate a loop device for it ----
LOOPDEV=$(sudo losetup -f --show "$BACKING")
[ -n "$LOOPDEV" ] || { echo "STOP: losetup did not return a device path"; exit 1; }
case "$LOOPDEV" in
  /dev/loop[0-9]*) ;;
  *) echo "STOP: '$LOOPDEV' does not look like /dev/loopN — refusing to continue"; exit 1 ;;
esac
echo "Loop device allocated: $LOOPDEV"

# ---- A2b. Record it, AND independently re-derive it from the kernel, and
#      confirm both agree. This is what makes teardown (§D) safe even from
#      a brand-new shell that never saw $LOOPDEV. ----
echo "$LOOPDEV" > "$MECHA_DIR/loopdev.txt"
DERIVED=$(sudo losetup -j "$BACKING" | cut -d: -f1)
[ "$LOOPDEV" = "$DERIVED" ] || { echo "STOP: recorded ($LOOPDEV) and kernel-reported ($DERIVED) loop devices disagree"; exit 1; }
echo "OK: $LOOPDEV confirmed as the kernel's own record for this backing file"

# ---- A3. Load dm-flakey (present on disk already; confirmed not loaded
#      before this procedure; a small leaf module — no other mapping on
#      this system depends on it) ----
sudo modprobe dm-flakey

# ---- A4. Create the flakey mapping. Geometry: 256 MiB / 512 = 524288
#      sectors. up=2s, down=1s, 1 feature argument: drop_writes (writes
#      issued during a "down" second are silently discarded while still
#      returning success — this is the fault being tested). ----
SECTORS=524288
sudo dmsetup create melfina_mecha_flakey \
  --table "0 $SECTORS flakey $LOOPDEV 0 2 1 1 drop_writes"

# ---- A5. Confirm the device node EXISTS before doing anything else to
#      it. Do NOT run A7 if this fails. ----
for i in 1 2 3 4 5; do
  [ -e /dev/mapper/melfina_mecha_flakey ] && break
  sleep 0.2
done
test -b /dev/mapper/melfina_mecha_flakey || { echo "STOP: /dev/mapper/melfina_mecha_flakey was not created — do NOT run any chown/chmod"; exit 1; }
echo "OK: /dev/mapper/melfina_mecha_flakey exists as a block device"

# ---- A6. Confirm the mapping's exact table BEFORE changing permissions ----
sudo dmsetup table melfina_mecha_flakey
# Expected, exactly: 0 524288 flakey <the $LOOPDEV from A2> 0 2 1 1 drop_writes
# If the device path shown differs from $LOOPDEV above, STOP — do not continue.

# ---- A7. LEAST-PRIVILEGE permissions — owner-only, mode 600. NOT 666.
#      chown/chmod resolve /dev/mapper/<name> to whatever real device node
#      it actually is on this distro (a symlink or a plain node), so this
#      reaches the real device either way. ----
sudo chown "$(id -u):$(id -g)" /dev/mapper/melfina_mecha_flakey
sudo chmod 600 /dev/mapper/melfina_mecha_flakey
stat -c '%U:%G %a %F' /dev/mapper/melfina_mecha_flakey
# Expected: <your username>:<your group> 600 block special file
```

**Only after A7's `stat` output shows `600` owned by your own account**
should you tell Claude to proceed to §B. Claude's commands never need
`$LOOPDEV` directly (they only ever touch the fixed
`/dev/mapper/melfina_mecha_flakey` path) — it's needed again only at
teardown (§D), which re-derives it itself rather than assuming this
shell's variable is still around.

## B. Unprivileged commands Claude runs

Everything here operates only on `/dev/mapper/melfina_mecha_flakey` (made
accessible, owner-only, by A7) and on files under
`$CLAUDE_JOB_DIR/tmp/chronicle_crash_test/mechA/` — never on `src/`, never
on any real device path, never on the real Chronicle (which does not exist
yet — there is no production code to touch). **No mount is ever performed
anywhere in this procedure** (§0).

```bash
BIN="$CLAUDE_JOB_DIR/tmp/chronicle_crash_test/target/debug/chronicle_crash_test"
DEV="/dev/mapper/melfina_mecha_flakey"
MECHA_DIR="$CLAUDE_JOB_DIR/tmp/chronicle_crash_test/mechA"
LEDGER="$MECHA_DIR/run1.ledger"

# ---- B1. Confirm the device exists, is a block device, and carries
#      exactly the least-privilege ownership/mode §A7 set — not just "is
#      it writable" but "is it *owner-only* writable". ----
test -b "$DEV" || { echo "STOP: $DEV is not a block device"; exit 1; }
PERM=$(stat -c '%U:%G %a' "$DEV")
echo "Device ownership/mode: $PERM"
if [ "$PERM" != "$(id -un):$(id -gn) 600" ]; then
  echo "STOP: unexpected ownership/mode on $DEV ($PERM) — expected $(id -un):$(id -gn) 600. Do not proceed."
  exit 1
fi
echo "OK: least-privilege ownership/mode confirmed"

# ---- B2. For the record only: report which loop device this run is
#      backed by, re-derived fresh, never assumed from an earlier shell. ----
losetup -j "/home/levy/.claude/jobs/de309c9f/tmp/chronicle_crash_test/mechA/mecha_backing.img" 2>/dev/null || echo "(cannot list loop devices unprivileged — informational only, not required to proceed)"

# ---- B3. Run the writer for 45 seconds, offset 0, within the 256 MiB
#      budget (matching the backing file's real size exactly, so it can
#      never write past the device). ~15 up/down cycles (2s+1s each). ----
"$BIN" mecha_write "$DEV" 0 268435456 45 "$LEDGER"

# ---- B4. Verify: for every record the ledger claims was durably
#      appended, seek to its exact offset and check the frame is intact. ----
"$BIN" mecha_verify "$DEV" "$LEDGER"
```

## C. What B's commands actually do (the experiment/test logic)

- **`mecha_write`** opens the device for writing (no `create`, no
  `truncate` — the node already exists at fixed size), and in a loop:
  builds one frame (`[4-byte length][1-byte tag=1][4-byte CRC-32][JSON
  payload]`), `seek`s to the next explicit offset (starting at the given
  base offset), writes the frame, calls `fsync` (`sync_all()`), and **only
  after that returns `Ok`** appends `seq,offset` to a separately-fsynced
  ledger file (on the *host* filesystem, not the flakey device — the
  ledger is our external ground truth and must not itself be subject to
  the fault we're injecting). This repeats for the requested duration or
  until it would run past the assigned byte budget.
- **`mecha_verify`** reads the ledger, and for each `(seq, offset)`,
  independently seeks to that exact offset on the device and re-derives
  what the frame *should* contain from `seq` alone (the payload format is
  deterministic), then checks the actual bytes on the device against that
  — length, tag, checksum, and content. This is a direct, per-record check
  by known position, not a sequential scan — so a single dropped write
  earlier in the range cannot make later, genuinely-intact records
  falsely appear "unreadable" the way a naive linear parser would.
- **Why this is the right test for dm-flakey specifically:** during a
  "down" second, the write B3 issues will still return success from
  `write()`/`sync_all()`'s point of view (dm-flakey lies at the block
  layer, below where those syscalls observe anything), but the actual
  bytes are discarded before reaching the loop-backed file. If our design
  is sound, `mecha_verify` should still report every ledgered record as
  intact — because by the time `mecha_write` logs a record to the ledger,
  its `sync_all()` already returned, and if the *design* were wrong we'd
  see exactly a ledgered-but-corrupted-or-missing record here. **If any
  are lost, that is the specific, real signal Phase 2 exists to catch** —
  a durability claim that dm-flakey proved false.
- **Behaviour if `mecha_write`/`mecha_verify` itself crashes or hangs:**
  it holds no lock and creates no privileged state — the worst case is the
  device is left open by a stuck process, which §D1 checks for explicitly
  before teardown proceeds.

## D. Teardown — you run these (privileged)

**Run these regardless of B's result — pass or fail, this must be torn
down. Safe to run in a brand-new shell: nothing here depends on any
variable set in an earlier one — the loop device is re-derived from the
kernel, not assumed.**

```bash
BACKING="/home/levy/.claude/jobs/de309c9f/tmp/chronicle_crash_test/mechA/mecha_backing.img"
MECHA_DIR="/home/levy/.claude/jobs/de309c9f/tmp/chronicle_crash_test/mechA"

# ---- D0. Re-derive the loop device from the kernel — authoritative, not
#      assumed. The recorded file (§A2b) is used only as a cross-check. ----
LOOPDEV_KERNEL=$(sudo losetup -j "$BACKING" 2>/dev/null | cut -d: -f1)
LOOPDEV_RECORDED=$(cat "$MECHA_DIR/loopdev.txt" 2>/dev/null || true)
echo "Kernel says backing file is attached to: '${LOOPDEV_KERNEL:-<none>}'"
echo "Earlier recorded value was:              '${LOOPDEV_RECORDED:-<none>}'"
if [ -n "$LOOPDEV_KERNEL" ] && [ -n "$LOOPDEV_RECORDED" ] && [ "$LOOPDEV_KERNEL" != "$LOOPDEV_RECORDED" ]; then
  echo "STOP: these disagree — investigate manually before tearing anything down."
  exit 1
fi
LOOPDEV="$LOOPDEV_KERNEL"   # kernel value is authoritative

# ---- D1. Check nothing still has the mapping open. If something does
#      (e.g. a stuck/crashed test run), investigate and stop it before
#      removing anything underneath it — do not force D2 past this. ----
sudo fuser -v /dev/mapper/melfina_mecha_flakey 2>&1 || echo "(nothing has it open — expected)"

# ---- D2. Remove the flakey mapping. Idempotent: safe to re-run.
#      If this fails with "device busy," do not force it — find and stop
#      whatever D1 showed, then re-run this exact command. ----
if sudo dmsetup ls 2>/dev/null | grep -q '^melfina_mecha_flakey'; then
  sudo dmsetup remove melfina_mecha_flakey
else
  echo "(melfina_mecha_flakey not present — nothing to remove)"
fi

# ---- D3. Detach the loop device — only if D0 actually found one.
#      Idempotent: safe to re-run. ----
if [ -n "$LOOPDEV" ]; then
  sudo losetup -d "$LOOPDEV"
else
  echo "(no loop device found associated with the backing file — nothing to detach)"
fi

# ---- D4. Optional: unload dm-flakey. Harmless to leave loaded with no
#      active mappings — this is NOT required cleanup, unlike D2/D3. ----
sudo modprobe -r dm-flakey 2>/dev/null || true

# ---- D5. Delete the disposable backing file and the recorded-loopdev
#      note (unprivileged; the ledger and any run logs are kept for
#      review, not deleted here). ----
rm -f "$BACKING" "$MECHA_DIR/loopdev.txt"

# ---- D6. Verify teardown against the §A0 baseline. ----
sudo dmsetup ls                                  # must NOT list melfina_mecha_flakey
sudo losetup -a                                  # must NOT list the loop device from D0
ls /dev/mapper/ 2>&1 | grep -i mecha || echo "OK: no melfina_mecha_flakey device node remains"
```

Compare D6's output against the **A0 baseline** — the only acceptable diff
is that everything created in §A is now gone; anything else present in the
"before" snapshot should still be present unchanged.

## E. Safety checklist

**Before running §A:**
- [ ] `$BACKING` (A1) is under `$CLAUDE_JOB_DIR/tmp/`, **not** anywhere
      under `/home/levy/projects/`, and **not** a real block device path
      like `/dev/sd*` or `/dev/nvme*`.
- [ ] `sudo losetup -a` (A0) shows no pre-existing loop device you might
      confuse with the one A2 allocates.
- [ ] `sudo dmsetup ls` (A0) shows no existing `melfina_mecha_flakey`
      (a collision would mean something unexpected already uses that name).

**During §A, before proceeding to §B:**
- [ ] A5 confirms the device node exists **before** A7's chown/chmod runs
      — if A5's check fails, A7 must not be run at all.
- [ ] A6's `dmsetup table` output shows the loop device *this same
      session* just allocated in A2 — not a guessed or hardcoded path —
      and shows `524288`, `2`, `1`, `1 drop_writes` exactly.
- [ ] A7's `stat` output shows mode `600`, owned by your own account —
      **not** `666`, **not** any group/other bit set.
- [ ] Confirm no mount is performed anywhere in §A or §B (there is none in
      this procedure by design — this is a check that the procedure was
      followed as written, not a command to run).

**Before running §D (teardown):**
- [ ] B3/B4 have both fully exited (no `chronicle_crash_test` process
      still running against the device) — D1's `fuser -v` should show
      nothing before D2 proceeds.

**After §D:**
- [ ] D0's kernel-derived and recorded loop-device values agreed (or the
      procedure stopped and you investigated before continuing).
- [ ] `melfina_mecha_flakey` is gone from `dmsetup ls` (D6).
- [ ] The loop device from D0 is gone from `losetup -a` (D6).
- [ ] The backing file and `loopdev.txt` have been deleted (D5).
- [ ] Nothing else in the A0 baseline changed.

**Standing guarantees, restated explicitly:**
- No real MELFINA data can be touched by this procedure: there is no
  production Chronicle implementation in `src/` yet, and every path this
  procedure ever writes to is either the fixed disposable device
  (`/dev/mapper/melfina_mecha_flakey`) or a file under
  `$CLAUDE_JOB_DIR/tmp/chronicle_crash_test/mechA/`.
- The backing file remains disposable: fixed at exactly 256 MiB (checked
  at A1), created sparse (0 blocks allocated until written), and
  explicitly deleted at D5 — never left behind.
- `dmsetup`/`losetup` resources cannot be silently left behind: D0
  re-derives state from the kernel rather than trusting any shell
  variable, D2/D3 are idempotent (safe to re-run if a first attempt only
  partially completed), and D6 verifies against the A0 baseline rather
  than just assuming success.

## What evidence to bring back

Paste (or let Claude re-read, since these are on this same machine) the
**full stdout** of:
- §A5, §A6, §A7 (the existence/table/permission confirmations)
- §B3 and §B4 (the write run and the verify run)
- §D0, §D1, and §D6 (teardown confirmation)

Claude will write the experiment report (setup, exact fault points, PASS/
FAIL/INCONCLUSIVE with evidence, limitations, per the same epistemic
discipline as the Phase 1 report) from that output plus its own local copy
of `run1.ledger` — **no committing until you've reviewed it.**

## Confirmation: no system/package/production change has been made

- No package installed (everything used in §A was already present on this
  machine, confirmed by inspection before this procedure was first
  written).
- No privileged command has been executed by Claude — §A and §D are
  written for **you** to run.
- No kernel module has been loaded yet; no loop device or dm mapping
  currently exists — reconfirmed just now.
- `src/` is untouched; there is no production Chronicle implementation yet
  for anything to have modified.
- The only filesystem changes so far: new/modified files under
  `experiments/chronicle_crash_test/` (the crate source — see `git status`
  below) and one disposable 256 MiB **sparse** file plus a small text note
  under `$CLAUDE_JOB_DIR/tmp/`, both outside the repository entirely.
- The one command actually executed against the real system since the
  prior revision was a single, harmless, out-of-order `sudo chmod 666
  /dev/mapper/melfina_mecha_flakey` (run by the human, not Claude), which
  failed immediately with "No such file or directory" because no device
  existed yet — independently reconfirmed to have left no trace (no loop
  device, no dm mapping, no module loaded).

```
$ git status --porcelain
 M experiments/chronicle_crash_test/src/main.rs
?? design/technology/CHRONICLE_CRASH_EXPERIMENT_PHASE2_PROCEDURE.md
```

## Traceability

| Element | Source |
|---|---|
| Phase 2 definition and why it's needed | `CHRONICLE_CRASH_EXPERIMENT_PROTOCOL.md` §2 |
| Tooling inventory this procedure is built from | read-only inspection (`qemu`, `dmsetup`, `losetup`, `dm-flakey.ko.zst`, `/dev/kvm` all present; no `disk`-group membership; no passwordless sudo) |
| The new writer/reader logic and why Phase 1's couldn't be reused as-is | `experiments/chronicle_crash_test/src/main.rs`, the "MECHANISM A" section |
| Self-test validating the new code with zero privilege | `mecha_selftest` — run against a synthetic file, **PASS**, 483/483 records verified intact |
| Least-privilege permission correction | this revision, prompted by the human's out-of-order `chmod` dry run |
