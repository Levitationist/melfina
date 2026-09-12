# MELFINA — TECHNOLOGY SELECTION: GOVERNANCE INTEGRITY / CRYPTOGRAPHY

**Phase:** TECHNOLOGY SELECTION MISSION 001. First pass, pending human review.
Scope: **K. Governance integrity / cryptographic implementation**,
**L. Signing-key custody mechanism**. Rubric:
`design/foundations/TECHNOLOGY_SELECTION_CRITERIA.md` §4; logical contract:
`design/foundations/GOVERNANCE_INTEGRITY.md` (F9).

---

## 1. Signature scheme

> **Recommendation: Ed25519 via `ed25519-dalek` (Rust). Confidence: HIGH.**

| Candidate | Evaluation |
|---|---|
| **Ed25519 (`ed25519-dalek`)** | **Selected.** Independently security-audited (Quarkslab, 2019, covering the dalek-cryptography family) `[E]`; deterministic signing (no per-signature randomness requirement to get right, removing a whole class of historical ECDSA-nonce-reuse-style failures by construction); **signing keys are zeroed on drop by default** (`zeroize`), a direct, concrete match for F9 T1/T4's "compromise of the running system must not yield reusable key material lingering in memory" `[E]`; small, fast, widely deployed and reviewed (SSH, TLS 1.3, Signal, and most modern package-signing schemes use Ed25519). A formally-verified backend variant exists (`ed25519-dalek-fiat`, using arithmetic extracted from Coq proofs) as a documented upgrade path if ever warranted — **`[OPEN]`, not required now**. |
| **RSA-PSS** | Rejected as primary — larger keys and signatures, slower, more historical implementation-pitfall surface (padding-oracle-class bugs have recurred across RSA implementations industry-wide), and no functional advantage for this use case (one human signer, no legacy interoperability requirement). Not ruled out if a specific external tool the maintainer wants to integrate only supports RSA — **DEFER**. |
| **ECDSA (P-256/secp256k1)** | Rejected — shares Ed25519's elliptic-curve foundation without its determinism/misuse-resistance advantages; no reason to prefer it here. |
| **A threshold/multi-signature scheme** | Not selected now (single human signer) but **explicitly recorded as the upgrade path F9 §6 already anticipates** ("a threshold / multi-signature option… even if single-signer at first") — revisit only if a second trusted human party is ever introduced. |

## 2. Hash function

> **Recommendation: SHA-256. Confidence: HIGH.**

Chosen over BLAKE3 (a strong, modern, faster alternative) on **conservatism
grounds specifically**: F9 §6 requires "standard, well-reviewed primitives —
no bespoke cryptography," and governance-chain verification is not a
performance-sensitive path (F9 §3's chain walk happens once at startup and,
per F9 §4, cheaply on subsequent governance reads — a human authors a new
governance version rarely, not per-transaction). SHA-256 is the most
standardised, most widely implemented, hardware-accelerated-on-virtually-
every-modern-CPU (`SHA-NI`) option, and its use here has no downside worth
trading for BLAKE3's speed advantage. **BLAKE3 is recorded as the
alternative** if a future measured bottleneck ever justifies it — none is
expected.

## 3. Signing-key custody mechanism (L)

> **Recommendation: an offline Ed25519 key, stored on a medium never
> attached to the machine running MELFINA day-to-day, used only through a
> small, standalone signing tool (the same `ed25519-dalek` library, a
> separate minimal binary) at governance-authoring time. Confidence: HIGH
> for the offline-key baseline; the hardware-backed upgrade (§4) is
> `[OPEN]`.**

This directly satisfies F9 T1–T4 at **zero additional hardware cost**:

- **T1 (not present on the running system):** the key never touches the
  machine MELFINA runs on; it lives on separate media (e.g. a USB drive kept
  physically disconnected, or the maintainer's separate machine).
- **T2 (MELFINA can verify):** the corresponding **public** key is embedded
  in MELFINA's trusted loader (F9 §1) — verification needs no secret
  material.
- **T3 (anchor integrity):** the embedded public key is set at build/install
  time by the human, not writable by any MELFINA runtime path — a build-time
  constant, not a configuration file MELFINA could be tricked into
  rewriting.
- **T4 (compromise of the running system does not yield the key):** true by
  construction — there is nothing to yield; the key was never there.

**The standalone signing tool is deliberately minimal and separate from
MELFINA itself** — it is not a MELFINA subsystem, has no dependency on
MELFINA's own build, and its only job is: read a candidate governance
version file, prompt the human to review it, sign `hash(content) ‖
parent-version-id` with the offline key, and write the signature out. Its
smallness is itself a security property (a smaller TCB for the one tool that
ever touches the actual secret key).

## 4. Hardware-backed key custody — an `[OPEN]` upgrade, not a requirement

An offline software key already satisfies F9's contract. A hardware token
(FIDO2/U2F device supporting Ed25519, e.g. a security key, or a TPM-backed
key) would additionally protect against a **stolen offline-key file** being
usable without the physical device present. This is recorded as `AU-6`'s
upgrade path, genuinely optional: **`[OPEN]`**, to be decided by the human
based on their own risk tolerance and hardware availability — this mission
does not mandate it.

## 5. The current-head marker / rollback protection (F9 C7) — the one place
hardware genuinely earns a recommendation

F9 C7 requires a **separately-protected current-head marker** so a validly-
signed *old* governance version cannot be replayed as current (a downgrade
attack). This is architecturally distinct from key custody and deserves its
own analysis:

| Candidate | Evaluation |
|---|---|
| **A TPM 2.0 monotonic counter** | **Recommended where available.** A TPM's monotonic counter is purpose-built for exactly this problem: a value that can only increase, survives an OS reinstall or a filesystem-level rollback (restoring old files does not roll the counter back), and is queryable via well-supported, standard tooling (`tpm2-tools`) on Linux. TPM 2.0 chips are near-ubiquitous on hardware from the last several years (the project's own environment snapshot lists a 2026-era x86-64 machine, which very likely has one). **Confidence: HIGH if present; this is the one place in the whole crypto/governance evaluation where hardware is the clearly superior mechanism**, not merely an optional hardening. |
| **A signed marker file plus human vigilance** (fallback) | If no TPM is available: the current head's version number is prominently reported to the human at every startup (F9 §5's auditability requirement already requires this); a human who would notice an unexpected downgrade is a real, if weaker, defence. This satisfies F9's contract (fail-closed on a detected mismatch, auditable) but not as strongly as a hardware counter — **honestly rated MEDIUM, not HIGH.** |
| **A remote timestamp/transparency-log service** (Certificate-Transparency/Rekor-style) | Rejected for the *current-head marker* specifically — it requires network reachability, which conflicts directly with INV-1/local-only. The **conceptual** lesson of transparency logs (append-only, tamper-evident, independently verifiable) is already incorporated into F9's chain design itself; it does not need an actual remote service. |

**Recommendation: detect TPM 2.0 presence at install time; use its
monotonic counter for the current-head marker if present; fall back to the
signed-marker-plus-human-vigilance scheme if not, with the fallback's weaker
guarantee explicitly surfaced to the human at install time (not silently
accepted as equivalent).** `[ID]` pending a concrete availability check on
the target hardware, `[OPEN]` on whether the human wants to rely on it.

## 6. The trusted loader itself (build/language note)

The loader that performs F9 §3's startup verification is part of the trusted
computing base and should be **the smallest, most auditable piece of code in
the entire system** — smaller even than P0 (`PROCESS_AND_IPC_EVALUATION.md`),
since it runs *before* P0 exists and establishes whether P0 may run at all.
**Recommendation: written in the same primary language (Rust) as the rest of
the system** (no separate language is justified here — the earlier
Rust-vs-OCaml consideration for "decision logic that benefits from
exhaustiveness checking" applies to the *Monitor's* `AUTHORIZED()` predicate,
not to this loader, whose job is comparatively simple: walk a chain, check
hashes, verify signatures, check a monotonic counter), using **only**
`ed25519-dalek`, a SHA-256 implementation, and (if available) a TPM binding
crate as dependencies — deliberately not the same dependency tree as the
rest of MELFINA, to keep this component's own supply-chain surface as small
as its logical scope.

## 7. Interaction notes (§25 of the mission)

- **Governance × OS security:** the TPM-counter approach (§5) is the one
  place this evaluation recommends binding to a specific hardware feature;
  it is designed to degrade gracefully (§5's fallback) rather than making
  TPM presence a hard requirement, preserving portability
  (`MIGRATION_PORTABILITY.md`).
- **Crypto × language:** `ed25519-dalek`'s Rust-native implementation avoids
  an FFI boundary for the one code path where an FFI-boundary bug would be
  most consequential (signature verification gating whether MELFINA runs at
  all).
- **Storage × integrity:** the governance object's own file(s) are **not**
  the Chronicle (`CHRONICLE_EVALUATION.md`) and use a separate, much simpler
  storage mechanism (plain files, since governance changes are rare,
  human-authored events, not a high-throughput append log) — F9 §0 already
  requires this independence, and the technology choice preserves it by
  construction (no shared storage engine between the two).

## 8. Human review required

Per mission §46, explicitly listed. **Recommendation: Ed25519
(`ed25519-dalek`) + SHA-256; an offline key with a small standalone signing
tool; a TPM 2.0 monotonic counter for the current-head marker where
available, with an explicitly weaker fallback otherwise.** The
hardware-key-for-signing question (§4) is left open for the human's own risk
decision.

## 9. Sources

[Security evaluation of dalek-cryptography libraries (Quarkslab, 2019)](https://blog.quarkslab.com/resources/2019-08-26-audit-dalek-libraries/19-06-594-REP.pdf) ·
[ed25519-dalek documentation](https://docs.rs/ed25519-dalek) ·
[ed25519-dalek-fiat (formally-verified backend)](https://github.com/novifinancial/ed25519-dalek-fiat) ·
`design/foundations/GOVERNANCE_INTEGRITY.md` (F9, carried forward — TUF
rollback-protection lesson, Certificate Transparency / Rekor model already
cited there).
