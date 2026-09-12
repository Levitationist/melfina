# MELFINA — TECHNOLOGY SELECTION: MIGRATION AND PORTABILITY

**Phase:** TECHNOLOGY SELECTION MISSION 001. First pass, pending human review.
Scope: **Z. Migration/backup/state portability mechanism**, mission §20/§36.

**Governing principle (stated verbatim in the mission, adopted without
qualification):** *portable state* is separated from *machine-specific
authority/capability*. **A migrated MELFINA instance must NOT automatically
inherit hardware capabilities merely because the state says they existed
previously.**

---

## 1. What transfers unchanged (portable by construction, given this
mission's other choices)

| Artefact | Why it transfers cleanly |
|---|---|
| The Chronicle's authoritative log file (`CHRONICLE_EVALUATION.md` §5) | A framed sequence of JSON-payload records — no architecture-dependent binary layout, no endianness concerns |
| The governance chain's content and signatures (F9) | Ed25519 signatures and SHA-256 hashes are architecture-independent; the *content* of the chain is fully portable |
| A capability package's manifest and provenance metadata (`REASONING_COMPUTATION_EVALUATION.md` §5) | Plain, portable metadata |
| Model weight files (GGUF) | GGUF is an explicitly portable, self-describing binary format `[E]` |

## 2. What requires conversion or re-derivation (never blindly copied)

| Artefact | Why | Handling |
|---|---|---|
| The SQLite current-state cache | Not treated as portable **on principle**, even though SQLite's file format is in fact cross-architecture-compatible — because F6 §4 already provides a deterministic, cheap alternative (REPLAY) that removes any need to trust a copied binary artefact at all | **Always rebuilt fresh from the authoritative log after migration**, never copied. This is the simplest possible policy and costs nothing extra, since the cache is disposable by design (F6 §6) |
| Compiled binaries (P0/P1/P2, capability packages compiled natively) | Architecture-specific machine code | Rebuilt for the target architecture from source (`BUILD_AND_SUPPLY_CHAIN.md` §7's ARM64 secondary-target support exists for exactly this) |
| Any capability's `execve` target paths (F1 §4's resolved, canonicalised path set) | A path resolved and canonicalised on machine A (`/usr/bin/foo` at a specific inode, following a specific symlink chain) is **not guaranteed to exist or resolve identically on machine B** | **Every file-scoped and process-scoped grant is treated as expired on migration** and must be freshly re-authorised and re-resolved on the new machine — never copied forward as if still valid |

## 3. What must NOT transfer automatically — the mission's core migration
principle, made concrete

| Artefact | Why it must not transfer silently | Realisation |
|---|---|---|
| **The TPM-backed governance current-head marker** (`CRYPTO_GOVERNANCE_EVALUATION.md` §5) | A monotonic counter is, by design, **bound to the specific physical TPM chip** — new hardware has no history of it. Silently "trusting" an absent or zero counter on first boot on new hardware would be exactly the kind of downgrade-attack surface F9 C7 exists to close | On detecting new hardware (a TPM with no prior MELFINA counter state), the trusted loader **refuses to auto-establish trust** and requires an explicit, out-of-band human re-authorisation step — a deliberate "first boot on this hardware" ceremony, not a silent pass-through. This is a **direct, positive application of F9's fail-closed doctrine to the migration case specifically** — an unfamiliar case is treated with the same suspicion as a corrupted one, not with default trust |
| **Live/active capability grants** (F1 §5) | A grant is a live handle scoped to a *running system's* state (F4 §1); a Chronicle record that "this grant was once issued" is a **historical Event**, not a standing authorisation | Migration always starts with **zero live grants** — the Monitor (P0) issues none until freshly authorised on the new machine, regardless of what the historical record shows |
| **Detected hardware capabilities** (a GPU backend for local inference, a specific accessibility service, a specific display server) | Assuming a previously-detected capability is still present is exactly the "state says it existed previously" failure mode the mission names explicitly | Every hardware-dependent capability (GPU backend selection for `llama.cpp`, the Wayland-vs-X11 GUI path, TPM presence) is **re-detected fresh on every startup**, not cached from a prior run's Chronicle record |

## 4. Migration verification sequence

1. Copy the Chronicle log file and the governance chain files to the new
   machine (the only artefacts that transfer as-is, §1).
2. Run F9 §3's full startup verification **from scratch** — migration gets
   **no shortcut** here; an unfamiliar-hardware current-head marker triggers
   the explicit re-authorisation ceremony (§3).
3. Rebuild the SQLite cache via REPLAY (F6 §4) — never restored from a
   backup copy of the cache file, even if one exists, to keep the "cache is
   always disposable and rebuildable" property exercised and trustworthy
   rather than assumed.
4. Re-detect all hardware-dependent capabilities fresh (§3).
5. Re-establish every file/process/GUI-scoped grant through the ordinary
   AUTHORISE pipeline (F3) — none are carried over as pre-authorised.
6. Only after 1–5 succeed does MELFINA resume normal operation on the new
   machine.

## 5. Backup (a narrower, more frequent case than full migration)

An ordinary backup (copying the Chronicle + governance files to another
location on the *same* machine, or to external media, without changing
hardware) is a strict subset of the above: it needs none of §3's
re-authorisation ceremony (the TPM/hardware context is unchanged), but
**restoring** from a backup after data loss should still run the same full
F9 verification (step 2) and the same cache-rebuild-never-restore policy
(step 3) — a backup is not assumed trustworthy without verification merely
because it came from "the same machine, earlier."

## 6. Interaction notes (§25 of the mission)

- **Storage × backup/migration:** the Chronicle's single-file, plain-framed
  design (`CHRONICLE_EVALUATION.md`) is what makes "copy the file" a
  sufficient backup mechanism at all — a design with a directory of
  interdependent SST/manifest files (the rejected RocksDB option) would have
  made backup meaningfully harder, reinforcing that rejection from a
  different angle.
- **Governance × OS security:** the TPM-binding decision
  (`CRYPTO_GOVERNANCE_EVALUATION.md` §5) is the one place this mission's
  choices actively **complicate** migration — accepted deliberately, because
  the security property it buys (rollback resistance) is judged more
  valuable than migration convenience, and the complication is handled by an
  explicit human ceremony rather than either silently failing or silently
  trusting.

## 7. Human review required

Per mission §46, not explicitly listed as a top-tier decision, but the
TPM-migration interaction (§3) is flagged for explicit human awareness given
it directly affects how migrating to new hardware will feel in practice
(an extra, deliberate step, not a one-command restore).

## 8. Sources

No new external research citations — this document applies already-cited
foundation contracts (F1, F3, F4, F6, F9) and this mission's own prior
decisions (Chronicle format, TPM-backed head marker) to the migration
scenario; it does not depend on any additional external claim.
