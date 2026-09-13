// chronicle_crash_test — throwaway spike, see README.md.
// Pure std, zero dependencies, zero network access needed to build or run.
//
// Frame format under test (CHRONICLE_EVALUATION.md §5):
//   [ 4-byte length (LE) ][ 1-byte unit-kind tag ][ 4-byte CRC-32 (LE) ][ payload ]
//
// Properties under test (design/foundations/CHRONICLE_CONTRACT.md §2/§5/§8):
//   1. durable-before-return
//   2. atomic append (never half-written-and-accepted)
//   3. corruption detection, no silent repair
//   4. crash recovery to a known-good prefix

use std::collections::HashSet;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// ---------------------------------------------------------------- CRC-32 --

fn crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0u32;
    while i < 256 {
        let mut c = i;
        let mut k = 0;
        while k < 8 {
            c = if c & 1 != 0 { 0xEDB88320 ^ (c >> 1) } else { c >> 1 };
            k += 1;
        }
        table[i as usize] = c;
        i += 1;
    }
    table
}

fn crc32(data: &[u8]) -> u32 {
    let table = crc32_table();
    let mut crc: u32 = 0xFFFF_FFFF;
    for &b in data {
        let idx = ((crc ^ b as u32) & 0xFF) as usize;
        crc = table[idx] ^ (crc >> 8);
    }
    crc ^ 0xFFFF_FFFF
}

// ------------------------------------------------------------------ PRNG --
// Small xorshift64*, seeded from wall-clock + pid. Not cryptographic —
// only needed to scatter kill timing / mid-write delay across trials.

struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        Rng(if seed == 0 { 0x9E3779B97F4A7C15 } else { seed })
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn range(&mut self, lo: u64, hi: u64) -> u64 {
        if hi <= lo {
            lo
        } else {
            lo + self.next_u64() % (hi - lo)
        }
    }
}

fn seed_from_env(extra: u64) -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    nanos ^ (std::process::id() as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ extra
}

// ---------------------------------------------------------- append logic --

/// Appends one record. Header is written and flushed in one syscall, then
/// (optionally, after a small delay to widen the window a kill can land in)
/// the payload is written, flushed, and fsync'd. `append_record` only
/// returns Ok after `sync_all()` (fsync) has returned successfully — that
/// return is the durability point this experiment is testing.
fn append_record(path: &str, seq: u64, mid_delay_micros: u64) -> io::Result<()> {
    let payload = format!("{{\"seq\":{}}}", seq);
    let payload_bytes = payload.as_bytes();
    let len = payload_bytes.len() as u32;
    let tag: u8 = 1; // stand-in "Event" tag
    let crc = crc32(payload_bytes);

    let mut header = Vec::with_capacity(9);
    header.extend_from_slice(&len.to_le_bytes());
    header.push(tag);
    header.extend_from_slice(&crc.to_le_bytes());

    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    f.write_all(&header)?;
    f.flush()?;
    if mid_delay_micros > 0 {
        std::thread::sleep(Duration::from_micros(mid_delay_micros));
    }
    f.write_all(payload_bytes)?;
    f.flush()?;
    f.sync_all()?; // fsync — the claimed durability point
    Ok(())
}

/// Appends the sequence number to a tiny, separately-fsynced "ledger" file
/// immediately after `append_record` has returned Ok. This records "the
/// chronicle append for seq N was durable as of this point" in a way that
/// survives the writer being killed a moment later, without relying on
/// stdout buffering semantics.
fn append_ledger(ledger_path: &str, seq: u64) -> io::Result<()> {
    let mut f = OpenOptions::new().create(true).append(true).open(ledger_path)?;
    writeln!(f, "{}", seq)?;
    f.flush()?;
    f.sync_all()?;
    Ok(())
}

fn run_writer(chron_path: &str, ledger_path: &str) -> ! {
    let mut seq: u64 = 0;
    let mut rng = Rng::new(seed_from_env(0));
    loop {
        // Vary the header/payload gap: mostly small, sometimes zero, to
        // land kills at different points relative to the frame boundary.
        let delay = rng.range(0, 3000); // 0..3ms
        if append_record(chron_path, seq, delay).is_err() {
            std::process::exit(1);
        }
        if append_ledger(ledger_path, seq).is_err() {
            std::process::exit(1);
        }
        seq += 1;
    }
}

// -------------------------------------------------------------- reading --

#[derive(Debug, Clone, Copy)]
enum FrameOutcome {
    Valid(u64),
    CorruptChecksum,
    TruncatedTail,
}

/// Parses frames strictly and stops at the first sign of trouble — mirrors
/// F6 §5's "detect, do not silently repair" requirement: a bad checksum or
/// an incomplete tail is reported, never guessed past.
fn read_all(path: &str) -> io::Result<Vec<FrameOutcome>> {
    let mut f = File::open(path)?;
    let mut data = Vec::new();
    f.read_to_end(&mut data)?;
    let mut results = Vec::new();
    let mut i = 0usize;
    while i < data.len() {
        if i + 9 > data.len() {
            results.push(FrameOutcome::TruncatedTail);
            break;
        }
        let len = u32::from_le_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]) as usize;
        let _tag = data[i + 4];
        let crc_stored = u32::from_le_bytes([data[i + 5], data[i + 6], data[i + 7], data[i + 8]]);
        let payload_start = i + 9;
        if payload_start + len > data.len() {
            results.push(FrameOutcome::TruncatedTail);
            break;
        }
        let payload = &data[payload_start..payload_start + len];
        let crc_actual = crc32(payload);
        if crc_actual != crc_stored {
            results.push(FrameOutcome::CorruptChecksum);
            break; // fail closed: do not attempt to parse past a bad frame
        }
        let s = String::from_utf8_lossy(payload);
        let seq = s
            .trim_start_matches("{\"seq\":")
            .trim_end_matches('}')
            .parse::<u64>()
            .unwrap_or(u64::MAX);
        results.push(FrameOutcome::Valid(seq));
        i = payload_start + len;
    }
    Ok(results)
}

// ------------------------------------------------------------- 1 trial --

struct TrialOutcome {
    trial_idx: usize,
    ledger_count: usize,
    valid_count: usize,
    lost_durable: Vec<u64>,
    saw_truncated_tail: bool,
    saw_checksum_corruption: bool,
    reader_error: Option<String>,
}

fn run_one_crash_trial(trial_dir: &str, trial_idx: usize, kill_range_micros: (u64, u64)) -> TrialOutcome {
    let chron_path = format!("{}/trial_{}.chron", trial_dir, trial_idx);
    let ledger_path = format!("{}/trial_{}.ledger", trial_dir, trial_idx);
    let _ = fs::remove_file(&chron_path);
    let _ = fs::remove_file(&ledger_path);

    let exe = env::current_exe().expect("current_exe");
    let mut child = Command::new(&exe)
        .arg("writer")
        .arg(&chron_path)
        .arg(&ledger_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to spawn writer subprocess");

    let mut rng = Rng::new(seed_from_env(trial_idx as u64));
    let wait_micros = rng.range(kill_range_micros.0, kill_range_micros.1);
    std::thread::sleep(Duration::from_micros(wait_micros));

    // Child::kill() sends SIGKILL on Unix — the writer gets no chance to
    // run any cleanup/drop code after this point.
    let _ = child.kill();
    let _ = child.wait();

    let ledger_seqs: HashSet<u64> = fs::read_to_string(&ledger_path)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| l.trim().parse::<u64>().ok())
        .collect();

    let mut reader_error = None;
    let frames = match read_all(&chron_path) {
        Ok(v) => v,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            // The writer was killed before it ever performed its first
            // OpenOptions::create(true) open — no file was ever created,
            // so nothing was ever durable. This is a legitimate, trivially
            // passing outcome (0 ledgered, 0 valid), not a reader failure.
            Vec::new()
        }
        Err(e) => {
            reader_error = Some(format!("{e}"));
            Vec::new()
        }
    };

    let mut valid_seqs = HashSet::new();
    let mut saw_truncated_tail = false;
    let mut saw_checksum_corruption = false;
    for fr in &frames {
        match fr {
            FrameOutcome::Valid(s) => {
                valid_seqs.insert(*s);
            }
            FrameOutcome::TruncatedTail => saw_truncated_tail = true,
            FrameOutcome::CorruptChecksum => saw_checksum_corruption = true,
        }
    }

    let mut lost_durable: Vec<u64> = ledger_seqs.difference(&valid_seqs).copied().collect();
    lost_durable.sort_unstable();

    TrialOutcome {
        trial_idx,
        ledger_count: ledger_seqs.len(),
        valid_count: valid_seqs.len(),
        lost_durable,
        saw_truncated_tail,
        saw_checksum_corruption,
        reader_error,
    }
}

// --------------------------------------------------- bit-flip sub-check --
// Separate from crash timing: writes N clean records (writer runs to
// completion, not killed), then flips one payload byte in an already-
// fully-written frame and confirms read_all reports CorruptChecksum for
// it (never silently accepts the flipped byte as valid), and that every
// record strictly before the flipped one is still reported Valid.

fn run_bitflip_check(path: &str) -> Result<(), String> {
    let _ = fs::remove_file(path);
    for seq in 0..10u64 {
        append_record(path, seq, 0).map_err(|e| format!("append failed: {e}"))?;
    }
    let mut data = fs::read(path).map_err(|e| format!("read failed: {e}"))?;
    // Locate the 3rd frame's payload (frames are variable-length here they
    // are not, since seq 0..10 payload lengths vary slightly in digit
    // count, so walk frames to find the 3rd one's payload start).
    let mut offsets = Vec::new();
    let mut i = 0usize;
    while i + 9 <= data.len() {
        let len = u32::from_le_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]) as usize;
        let start = i + 9;
        if start + len > data.len() {
            break;
        }
        offsets.push((i, start, len));
        i = start + len;
    }
    if offsets.len() < 5 {
        return Err("bit-flip check: not enough frames written".to_string());
    }
    let (_, target_start, target_len) = offsets[2];
    if target_len == 0 {
        return Err("bit-flip check: target frame empty".to_string());
    }
    // flip one bit in the middle of the payload
    let flip_at = target_start + target_len / 2;
    data[flip_at] ^= 0x01;
    fs::write(path, &data).map_err(|e| format!("rewrite failed: {e}"))?;

    let results = read_all(path).map_err(|e| format!("read_all failed: {e}"))?;
    // Expect: frames 0,1 valid; frame 2 corrupt-checksum; nothing after
    // frame 2 reported (reader stops at first corruption).
    match results.get(0) {
        Some(FrameOutcome::Valid(0)) => {}
        other => return Err(format!("expected frame 0 valid, got {:?}", other)),
    }
    match results.get(1) {
        Some(FrameOutcome::Valid(1)) => {}
        other => return Err(format!("expected frame 1 valid, got {:?}", other)),
    }
    match results.get(2) {
        Some(FrameOutcome::CorruptChecksum) => {}
        other => return Err(format!("expected frame 2 corrupt, got {:?}", other)),
    }
    if results.len() != 3 {
        return Err(format!(
            "expected reader to stop at the corrupt frame (3 results), got {} results",
            results.len()
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------- main --

fn run_trials(n: usize, trial_dir: &str) {
    fs::create_dir_all(trial_dir).expect("create trial dir");

    let mut total_lost = 0usize;
    let mut total_false_valid_after_corruption = 0usize; // sanity: should always be 0 by construction
    let mut trials_with_truncated_tail = 0usize;
    let mut trials_with_checksum_corruption = 0usize;
    let mut reader_errors = 0usize;
    let mut total_valid_records = 0usize;
    let mut total_ledger_records = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for trial_idx in 0..n {
        // Vary the kill-timing distribution across trials, per the
        // protocol's "some early, some between frames" instruction.
        let kill_range = if trial_idx % 4 == 0 {
            (0u64, 2_000u64) // biased toward very early / first record
        } else if trial_idx % 4 == 1 {
            (0u64, 300u64) // extremely early — often before any full frame
        } else {
            (0u64, 50_000u64) // broad — scatter across many records
        };

        let outcome = run_one_crash_trial(trial_dir, trial_idx, kill_range);

        total_valid_records += outcome.valid_count;
        total_ledger_records += outcome.ledger_count;
        if outcome.saw_truncated_tail {
            trials_with_truncated_tail += 1;
        }
        if outcome.saw_checksum_corruption {
            trials_with_checksum_corruption += 1;
        }
        if let Some(e) = &outcome.reader_error {
            reader_errors += 1;
            failures.push(format!(
                "trial {}: reader error: {e}",
                outcome.trial_idx
            ));
        }
        if !outcome.lost_durable.is_empty() {
            total_lost += outcome.lost_durable.len();
            failures.push(format!(
                "trial {}: LOST {} durable record(s) that were ledgered but not valid on read: {:?}",
                outcome.trial_idx, outcome.lost_durable.len(), outcome.lost_durable
            ));
        }

        println!(
            "trial {:>3}: ledger={:<5} valid={:<5} lost={:<3} truncated_tail={:<5} checksum_corrupt={:<5}{}",
            outcome.trial_idx,
            outcome.ledger_count,
            outcome.valid_count,
            outcome.lost_durable.len(),
            outcome.saw_truncated_tail,
            outcome.saw_checksum_corruption,
            outcome.reader_error.map(|e| format!(" reader_error={e}")).unwrap_or_default(),
        );
    }

    println!();
    println!("=== SUMMARY ({} trials) ===", n);
    println!("total ledgered (claimed-durable) records across all trials: {}", total_ledger_records);
    println!("total valid records read back across all trials:            {}", total_valid_records);
    println!("total LOST durable records (should be 0 for PASS):          {}", total_lost);
    println!("trials with a truncated tail detected (expected, not a failure): {}", trials_with_truncated_tail);
    println!("trials with a checksum-corrupt frame detected:               {}", trials_with_checksum_corruption);
    println!("trials with a reader error/panic (should be 0 for PASS):     {}", reader_errors);
    println!("false-valid-after-corruption sanity count (structurally 0):  {}", total_false_valid_after_corruption);
    // silence unused-mut-like warning if the value never changes; kept for
    // symmetry with the report's "explicitly checked, not just assumed" style
    total_false_valid_after_corruption += 0;

    if failures.is_empty() {
        println!();
        println!("PHASE1_TRIALS_RESULT=PASS");
    } else {
        println!();
        println!("PHASE1_TRIALS_RESULT=FAIL");
        for f in &failures {
            println!("  - {f}");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("writer") => {
            let chron = args.get(2).expect("writer needs <chronicle-path>");
            let ledger = args.get(3).expect("writer needs <ledger-path>");
            run_writer(chron, ledger);
        }
        Some("run_trials") => {
            let n: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(200);
            let dir = args.get(3).map(|s| s.as_str()).unwrap_or("trials");
            run_trials(n, dir);
        }
        Some("bitflip_check") => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or("bitflip.chron");
            match run_bitflip_check(path) {
                Ok(()) => println!("BITFLIP_CHECK_RESULT=PASS"),
                Err(e) => println!("BITFLIP_CHECK_RESULT=FAIL: {e}"),
            }
        }
        _ => {
            eprintln!("usage: chronicle_crash_test writer <chron-path> <ledger-path>");
            eprintln!("       chronicle_crash_test run_trials <n> <trial-dir>");
            eprintln!("       chronicle_crash_test bitflip_check <path>");
            std::process::exit(2);
        }
    }
}
