# MELFINA — FOUNDATION 9: GOVERNANCE VERSION-CHAIN + INTEGRITY CONTRACT

**Phase:** LOW-LEVEL FOUNDATIONS — MISSION 001. First pass, pending review.
**Baseline:** `8a40207`. Read F8 first (it defines *what* a governance version
is); this document defines how its **authenticity and continuity are proven**.

**This document does not choose a cryptographic primitive or hardware.** Not
Ed25519, not RSA, not ECDSA, not a hash function, not a TPM, not a secure
enclave, not a hardware token, not a specific filesystem or key store. It fixes
the **security properties** any mechanism must deliver.

Derives from: **RC-1** (the adversarial-review correction that added the at-rest
integrity chain); **INV-3, INV-6, INV-13**; **AP-3, AP-5**; **MEL-REQ-179, 201,
202, 203, 235, 241, 247**; Certificate Transparency / RFC 6962 (append-only,
tamper-evident, signed tree heads), TUF (root of trust held offline, threshold
signing, explicit rollback protection), Sigstore/Rekor (transparency log),
signed hash chains `[E]`.

---

## 0. Dependencies discovered

- **On F8:** consumes the version metadata F8 §3 defines (`version-id`,
  `parent-version-id`, `content`, `human-authorisation`, `integrity-metadata`).
- **On F7:** the verified governance region is the Ring-0 region F7 P9 isolates
  and RC-1 excludes from every `file-*` grant.
- **On F3:** the Reference Monitor only trusts governance content **after** this
  contract's startup verification has passed; a failed verification means the
  Monitor has no rules to enforce ⇒ MELFINA does not run.
- **On F6:** the Chronicle records governance transitions as Events (an audit
  reference), but is **not** the integrity mechanism — the chain stands on its
  own so that a compromised Chronicle cannot forge governance history, and a
  compromised governance store cannot be excused by the Chronicle.
- **Discovered constraint:** verification must run in a **minimal trusted
  loader** with no dependency on Ring 1, Ring 2, or Ring 3, because those layers
  must not start until governance is trusted. `[SEN]` (ordering) + `[ID]`.

---

## 1. The trust root

> There exists a **human-held signing authority** — the *governance trust root* —
> that MELFINA **does not possess in any form**. MELFINA cannot read it, derive
> it, reconstruct it, or use it. A governance version is authentic **iff** it
> carries a valid authorisation that verifies against this trust root.

Properties the mechanism must deliver:

| # | Property | Class |
|---|---|---|
| T1 | the signing authority is **not present** on the running system in a usable form — not in a file MELFINA can read, not in memory MELFINA can reach, not behind an API MELFINA can call to sign arbitrary content | **[ID]** (key custody) + **[ETL]** (audit that no code path reaches it) |
| T2 | MELFINA **can verify** a signature against the corresponding public trust anchor — verification needs only public material, which MAY be embedded | **[ID]** |
| T3 | the trust anchor MELFINA holds for verification is itself **integrity-protected** — MELFINA cannot silently swap in an anchor it controls (else it could self-authorise). The anchor's authenticity is established at install time by the human and is part of what startup verification checks. | **[ID]** + **[ETL]** |
| T4 | compromise of the **running system** does not yield the signing authority — an attacker who fully controls MELFINA still cannot mint a governance version that verifies | **[ID]** (custody) + **[ETL]** (red-team) |

**What "MELFINA does not possess it" does NOT require:** a hardware token or
enclave. An offline key on the maintainer's separate machine, used only when
authoring a new version, satisfies T1–T4. Hardware custody is a **[OPEN]**
upgrade (AU-6), not a mandate.

---

## 2. The version chain

The governance history is a **linear, append-only, hash-linked chain** (F8 §3):

```
genesis ← v1 ← v2 ← … ← v_head          (each arrow = parent-version-id + hash link)
```

Required properties:

| # | Property | Meaning | Class |
|---|---|---|---|
| C1 | **linear** | one parent per version; exactly one head; no branches | **[SEN]** (F8 format cannot express a branch) + **[ID]** |
| C2 | **hash-linked** | each version's `integrity-metadata` binds `hash(content)` **and** `hash(parent's content)` (or the parent's chained digest), so altering any past version breaks every link after it | **[ID]** (choice of hash) |
| C3 | **each transition human-authorised** | `human-authorisation` on version N is a signature, verifying against the trust root (§1), over *at least* `hash(content_N) ‖ parent-version-id_N` — so a version cannot be re-parented or its content swapped without re-signing | **[ID]** |
| C4 | **immutable** | no in-place edit of any version, ever; a change is a new head | **[SEN]** (F8) + **[ID]** (store is append-only / the loader rejects a mutated version) |
| C5 | **rollback-as-forward** | reverting to an earlier policy = a **new** version at the head whose content matches the earlier one, with its own parent link + its own signature; the head pointer never moves backward | **[SEN]** (F8 cannot express a backward move) + **[ID]** |
| C6 | **monotonic sequence** | `version-id`s are totally ordered; the loader rejects a chain with a gap, a duplicate, or a non-increasing step | **[ID]** |
| C7 | **rollback attack resistance** | an attacker cannot present an **old, validly-signed** version as current to downgrade the constitution — the loader checks the head against a **separately protected "current head" marker** (a signed pointer, or a monotonic counter the attacker cannot decrement) | **[ID]** — this is the TUF rollback-protection lesson `[E]`; **[ETL]** (red-team) |

---

## 3. Startup verification

Before the reasoning system starts, the trusted loader performs, and **all must
pass**:

1. **Locate** the governance store and the protected current-head marker.
2. **Verify the trust anchor** MELFINA holds is the human-installed one (T3).
3. **Walk the chain** from genesis (or a pinned checkpoint) to the head:
   - each version parses and expresses **no forbidden shape** (F8 §6: no tier
     6–9 autonomous, no self-raising ceiling, no `widen-authority` class, no
     backward head move);
   - each `integrity-metadata` hash matches recomputed `hash(content)`;
   - each parent link's hash matches the parent's actual content (C2);
   - each `human-authorisation` verifies against the trust root (C3);
   - the sequence is monotonic and gap-free (C6).
4. **Check the head** matches the protected current-head marker (C7).
5. **Resolve the active version** via its `activation` condition (F8 §3).

**Outcome:**

| Result | Action |
|---|---|
| all pass | the active governance version is loaded; the Monitor may enforce it; Ring 1 → 2 → 3 may start |
| any hash mismatch, any invalid signature, a forbidden shape, a broken parent link, a non-monotonic sequence, a head/marker mismatch, a missing trust anchor | **MELFINA refuses to run.** It does not start the reasoning system. It does not "run with the last known-good governance" silently. It reports the specific failure to the human and waits. (RC-1: "invalid chain / signature ⇒ refuse to run.") |
| the governance store is **absent** (first install) | genesis authoring flow (human-driven, out of band); MELFINA does not synthesise a genesis version |
| partial / torn write in the store (crash during authoring) | detected as a hash/parse failure ⇒ refuse to run ⇒ human re-authors or restores; **never** auto-repaired |

`[SEN]` for "refuse to run is the only non-pass outcome"; `[ID]` for the loader
correctly implementing steps 1–5.

---

## 4. Runtime

- Governance is **read-only at runtime** (F8). There is no runtime path to
  advance a version; a new version is authored **offline / out of band** and
  takes effect at the **next verified startup** (or a human-triggered
  re-verification), never hot-swapped into a running system. `[SEN]`.
- The Monitor MAY **re-verify** the active version's hash periodically and on any
  governance read, cheaply (hash check, no signature walk). A mismatch at runtime
  (the file changed underneath a running MELFINA) is a **stop-condition**:
  emergency stop (F8 §2.8), report, refuse to continue consequential operation
  until a clean restart verifies. `[ID]` + `[ETL]`.
- **Version pinning:** a deployment MAY pin to a specific `version-id`; startup
  then additionally checks the resolved active version equals the pin, and
  refuses to run on a mismatch (so an unexpected — even validly signed —
  advance does not take effect unnoticed). `[ID]`.

---

## 5. Auditability

- Every **verified startup** appends a Chronicle Event: "governance version V
  verified and activated at transaction-time T", with the chain head hash. (An
  audit reference — not the integrity mechanism itself; §0.)
- Every **failed verification** appends (if the Chronicle is available) an Event
  and always produces a durable, human-readable report outside the Chronicle
  (since the Chronicle may itself be suspect in a compromise).
- The **full chain is inspectable** by the human at any time: every version, its
  content, its parent, its author, its rationale, its signature status
  (`MEL-REQ-201–203`, tamper-evident audit trail).
- The chain is **tamper-evident, not tamper-proof**: an attacker with write
  access to the store can delete or corrupt it, but **cannot do so undetectably**
  — verification fails and MELFINA refuses to run (CT / Rekor model `[E]`).

---

## 6. Required security properties (the technology-independent list)

Any mechanism selected in a later phase must provide:

| Property | Delivered by (category, not choice) |
|---|---|
| **authenticity** — only the human trust root can create a version that verifies | asymmetric signature over `hash(content) ‖ parent-id`, key held off the running system |
| **integrity** — any alteration of any version is detected | collision-resistant hash; chained links (C2) |
| **continuity** — no version can be inserted, removed, or re-ordered undetectably | monotonic `version-id` + parent-hash links + gap check (C6) |
| **rollback resistance** — an old signed version cannot be replayed as current | separately-protected current-head marker / monotonic counter (C7) |
| **anchor integrity** — MELFINA cannot substitute its own verification anchor | human-installed anchor, checked at startup (T3) |
| **fail-closed** — unverifiable governance ⇒ no operation | the loader's only non-pass outcome is "refuse to run" (§3) |
| **recoverability** — a corrupted store can be restored from a human-held copy without weakening the chain | the human re-installs a signed chain; genesis/checkpoint pinning |
| **auditability** — the human can inspect and verify the entire history | chain is inspectable + tamper-evident (§5) |
| **independence from the Chronicle** — governance integrity does not rely on Chronicle integrity and vice versa | the chain self-verifies; the Chronicle only *references* it |

---

## 7. Contract Q&A

| Question | Answer |
|---|---|
| **Guarantees** | at startup, MELFINA runs **only** under a governance configuration whose every version is human-authored, hash-linked, monotonically sequenced, and signature-verified against a trust root MELFINA does not possess; any failure of that ⇒ MELFINA refuses to run; governance is not hot-swapped; the chain is tamper-evident and fully auditable |
| **Requires** | a human-held signing authority off the running system; a human-installed verification anchor; a minimal trusted loader ordered before Ring 1/2/3; a collision-resistant hash; a protected current-head marker; a human-held backup of the chain |
| **Trusts** | the human trust root; the human-installed anchor; the loader's own code (part of the trusted computing base — kept minimal, AP-5) |
| **Distrusts** | the running system's own integrity (T4 — compromise must not yield the key); the governance store's storage medium (assume it can be corrupted / rolled back); the Chronicle as a source of governance truth; any MELFINA component offering to author or activate a version; a validly-signed *old* version presented as current |
| **Enters** | (offline) a human-authored, human-signed new version; (at startup) the stored chain + head marker + anchor |
| **Leaves** | a pass/fail verdict + the resolved active version; a Chronicle Event on success; a durable report on failure |
| **Malformed input** | any parse failure, hash mismatch, bad signature, forbidden shape, broken link, sequence anomaly, or head/marker mismatch ⇒ **refuse to run** + specific report; never auto-repair, never run degraded |
| **Failure** | fail-closed: no reasoning system start, no consequential operation; runtime detection of a changed file ⇒ emergency stop; recovery is a human re-install + clean restart |
| **Authoritative** | the verified chain + protected head marker are jointly authoritative for which governance version is active; nothing at runtime overrides |
| **Independently verifiable** | yes — the human can independently re-run the same chain walk with the public anchor on a separate machine; the loader logic is small enough to audit; rollback resistance and key inaccessibility are red-team targets `[ETL]` |

---

## 8. Enforceability summary

| Invariant | Class |
|---|---|
| MELFINA does not possess the signing authority (T1, T4) | **[ID]** (key custody) + **[ETL]** (audit no path reaches it) |
| MELFINA can verify but not mint a governance version (T2) | **[ID]** |
| MELFINA cannot substitute its own verification anchor (T3) | **[ID]** + **[ETL]** |
| chain is linear, immutable, monotonic, hash-linked (C1–C6) | **[SEN]** (F8 format) + **[ID]** (loader + store) |
| rollback / downgrade resistance (C7) | **[ID]** (head marker) + **[ETL]** (red-team) — TUF lesson `[E]` |
| invalid chain / signature ⇒ refuse to run (RC-1) | **[SEN]** (only non-pass outcome) + **[ID]** (loader) |
| governance not hot-swapped; new version at next verified startup | **[SEN]** (no runtime write path) |
| runtime file-change detection ⇒ emergency stop | **[ID]** + **[ETL]** |
| tamper-evident (not tamper-proof); fully auditable | **[ID]** — CT / Rekor model `[E]` |
| integrity independent of the Chronicle | **[SEN]** (design — self-verifying chain) |
| hardware key custody (TPM / enclave / token) | **[OPEN]** — AU-6; offline key satisfies the contract, hardware is an upgrade |
| formal verification of the loader | **[OPEN]** — AU-5 |

---

## 9. Deferred (not decided here)

- The signature scheme (Ed25519 / RSA-PSS / ECDSA / a threshold scheme) and the
  hash function — F12, against §6.
- Key custody: offline file on the maintainer's machine vs hardware token vs
  enclave (AU-6) — the contract is satisfied by the weakest (offline file); the
  choice is a `[OPEN]` risk/effort trade-off.
- The current-head-marker mechanism (a second signature over `(head-id,
  counter)` / a monotonic hardware counter / an OS-protected file) — F12.
- The governance store location and filesystem, and how the Ring-0 path
  exclusion (RC-1) is enforced against every `file-*` grant — F12 + F7.
- The trusted loader's language and form (it is TCB — must be minimal and
  auditable, AP-5) — F12.
- Whether the loader / chain verification is formally verified (AU-5).
- Genesis authoring workflow details (a human-process document, not a
  foundation contract).

## 10. Traceability

| Element | Source |
|---|---|
| at-rest cryptographic integrity chain for Ring 0; human-held key MELFINA never possesses; verified every startup; invalid ⇒ refuse to run | **RC-1**; `ARCHITECTURE_ADVERSARIAL_REVIEW.md` finding; `AUTHORITY_AND_SECURITY_MODEL.md` §6 |
| Ring-0 path hard-excluded from every File Access grant | RC-1; F1 §3; F7 P4; F8 §2.3 |
| the meta-invariant; MELFINA cannot rewrite its constitutional governance | `MEL-REQ-235`; INV-6; F8 §1 |
| tamper-evident audit trail; human can inspect history | `MEL-REQ-201, 202, 203` |
| least authority; minimal trusted computing base | `MEL-REQ-179`; AP-5 |
| deterministic where practical (verification is deterministic) | `MEL-REQ-174` |
| rollback protection; offline root of trust; threshold signing option | TUF `[E]` |
| append-only, tamper-evident, signed heads | Certificate Transparency / RFC 6962; Sigstore/Rekor `[E]` |
| recover to a known state; no silent data loss | `MEL-REQ-170, 173` |
