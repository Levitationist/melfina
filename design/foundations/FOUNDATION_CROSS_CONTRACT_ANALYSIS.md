# MELFINA — FOUNDATION 11: FOUNDATION CROSS-CONTRACT ANALYSIS

**Phase:** LOW-LEVEL FOUNDATIONS — MISSION 001. First pass, pending review.
**Baseline:** `8a40207`. Read F1–F10 first.

**Purpose.** The ten preceding contracts are written to stand alone. This
document traces **nine complete paths** end-to-end across all of them, to show
the contracts compose without gap or contradiction, and to name — for each path —
the authoritative component, the trust boundary crossed, the authority boundary
crossed, the failure point, the recovery, and the audit record. It closes with a
**contract-vs-contract consistency matrix** and the **open seams**.

Notation: **F1** = CAPABILITY_GRANT_MODEL, **F2** = STRUCTURED_ACTION_MODEL,
**F3** = REFERENCE_MONITOR_CONTRACT, **F4** = REVOCATION_MODEL, **F5** =
CHRONICLE_LOGICAL_FORMAT, **F6** = CHRONICLE_CONTRACT, **F7** =
ISOLATION_CONTRACT, **F8** = GOVERNANCE_FORMAT, **F9** = GOVERNANCE_INTEGRITY,
**F10** = VERIFIER_CONTRACT.

The pipeline (`MEL-REQ-018`): **THINK → DECIDE → PROPOSE → AUTHORISE → EXECUTE →
VERIFY**. THINK/DECIDE/PROPOSE are Ring 2; AUTHORISE is Ring 0 (the Monitor);
EXECUTE is Ring 3 under a live grant; VERIFY is a Ring-3 verifier gated by Ring 0.

---

## Path A — Simple read ("what did I say I'd do about the landlord?")

| Aspect | Detail |
|---|---|
| **Flow** | Ring 2 Context Constructor needs material → issues a **Query (view read)** against the current-state projection (F6 §3.2). No Intention, no grant, no effect. Result: Claims/Events/Intentions with `holder ∈ {self, source}` about the landlord. |
| **Authoritative component** | the **Chronicle** (F6). The projection is authoritative *enough* for reasoning input (M1 — context is constructed on demand from it); it is **not** used for a consequential decision. |
| **Trust boundary crossed** | Ring 2 ↔ Ring 1 (read only). Ring 2 receives data; it gains **no authority** (F7 §7). |
| **Authority boundary crossed** | none. A read needs `chronicle-read` semantics but no *grant* for a pure reasoning-context read (F1 §3: `chronicle-read` risk floor `none`); the Reasoner Interface mediates it. |
| **Failure point** | projection stale or unavailable → Ring 2 gets an explicit "as-of" marker or falls back to an authoritative read (F6 §3.1); Chronicle unreadable → reasoning degrades gracefully, MELFINA says so (INV-9). Never fabricates. |
| **Recovery** | projection rebuild from the authoritative sequence (F6 §4, §6); no data loss (append-only). |
| **Audit record** | reads are **not** individually appended by default (F6 §2 "no duplicate suppression"/§9 — reads are not units). If a read informs a later recorded inference, the inference Claim carries `derived-from` / `informed-by` provenance (F5 §6) pointing at what was read. |

**Cross-contract check:** F6 (view vs authoritative read) + F5 (reads are not
units) + F7 (Ring 2 gains no authority from reading). Consistent.

---

## Path B — Terminal command ("archive last month's invoices": run a known script)

| Aspect | Detail |
|---|---|
| **Flow** | Ring 2 PROPOSES an Intention "run the archive routine", `status = proposed`, with a **structured `process-exec`** proposal (F2 §2.1): resolved absolute program path, `argv` vector, fixed env allow-list, cwd, resource limits, `child-authority = nothing`. → Monitor **AUTHORISE** (F3 §6): VALIDATE (structural, M10) → CLASSIFY (`local-exec`, floor `medium`, F8 §2.3) → CHECK GOVERNANCE (tier n/a; ceiling ok; aggregate budget ok; not trifecta) → CHECK AUTHORITY/SCOPE against a **pre-authorised scope** the user granted for this routine (`MEL-REQ-144`) → **allow** + mint one **live grant** (F1 §2, F4 §1) bound to *exactly* this canonical action. → EXECUTE: Ring-3 capability instance loaded (F7), grant wired, `execve`-style invocation — **never a shell** (F2 §1, §5). → VERIFY (F10): a verifier reads the archive directory + the Chronicle Event, confirms the expected files moved. |
| **Authoritative component** | the **Reference Monitor** for "may this run?"; the **verifier's observation** for "what actually happened" (F10 §6); the **Chronicle** for the durable record. |
| **Trust boundary crossed** | Ring 2 → Ring 0 (proposal, carrying **no authority** — F3 §5: Intention `status` is not authority evidence); Ring 0 → Ring 3 (grant handed to the sandbox); Ring 3 → OS (the actual exec, inside isolation, F7). |
| **Authority boundary crossed** | proposal→authorised (the triad's decision/execution split, INV-2). The grant authorises `program X + argv A` **only** — not `X + argv B`, not `shell → X`, not an added pipe (F2 §5 the core invariant). |
| **Failure point** | canonicalisation mismatch at check time vs issue time → **deny** (F2 §3, TOCTOU); script exits non-zero → outcome `failed` (F4 §4); verifier `inconclusive` → recorded unverified, and since this is consequential-ish, low confidence + surfaced (F10 §4). |
| **Recovery** | archive is (per its VERIFY plan) reversible → on `failed`/`contradicted` a declared inverse may restore, outcome `rolled back` (F4 §3); else `partially completed`, **no rollback claimed** (F4 §4 honesty rule). |
| **Audit record** | an **EXECUTE Event** (F5 §4.1) with participants (agent = the capability, on-behalf-of the authorised Intention), `brings-about` links to resulting file-state Claims; the **grant id + revocation id** (F1 §2); the **verifier verdict Claim** (F10 §5); one **terminal outcome** from the F4 vocabulary. |

**Cross-contract check:** F2 (structured, not string) + F1 (grant bound to the
canonical action) + F3 (classify + authorise) + F7 (sandboxed exec) + F4
(outcome) + F10 (verify) + F5/F6 (record). Consistent. The key seam — "grant for
argv A ≠ argv B" — is owned by **F2 §5** and *enforced at* **F3 CHECK SCOPE**
using **F1 §4** canonical target identity. Owner and enforcement point both
named. ✓

---

## Path C — File modification ("update my budget spreadsheet's Q3 totals")

| Aspect | Detail |
|---|---|
| **Flow** | Ring 2 PROPOSES a `file-op` (F2 §2.4): operation `write`, resolved absolute path, bound to a fixed root. → AUTHORISE: CLASSIFY `file-write`, floor `medium` (not delete/truncate/rename, so not the `high` sub-floor — F8 §2.3). CHECK GOVERNANCE: path is **not** in the Ring-0 region (RC-1 / F7 P4 / F8 §2.3 exclusion — checked here). Aggregate budget ok. → grant minted, **re-canonicalised at the check** and resolution bound into the grant (F2 §3 acceptance criteria). → EXECUTE in the sandbox with a `file-write` grant to that one path. → VERIFY: a verifier reads the file back, checks the Q3 cells changed and nothing else did (F10 V4 read-only). |
| **Authoritative component** | Monitor for permission; **the file's actual post-state as read by the verifier** for what happened; Chronicle for the record. |
| **Trust boundary crossed** | Ring 0 → Ring 3 (grant); Ring 3 → filesystem (mediated by F7 P4 — the instance sees only this path). |
| **Authority boundary crossed** | proposal→authorised; and the **scope** boundary: the grant covers **this path**, not its directory, not a sibling, not a symlink target resolved elsewhere (F2 §3: resolve every symlink; any difference at re-check ⇒ deny). |
| **Failure point** | a symlink swapped between check and open (TOCTOU) → F2 §3 requires re-canonicalise at every check and deny on difference → **deny / abort**; write partially completes then the process dies → outcome `partially completed`, verifier sees a half-written file, **no rollback claimed** unless a pre-write copy existed and is restored (`rolled back`). |
| **Recovery** | if the capability's VERIFY plan declared a pre-write snapshot, restore → `rolled back` + VERIFY the restore (F4 §3). Otherwise `partially completed`, surfaced to the user with the observed state. |
| **Audit record** | EXECUTE Event; the canonical path (F1 §4); grant + revocation id; pre/post evidence the verifier read (F10 §5, V8); terminal outcome. |

**Cross-contract check:** the Ring-0 path exclusion appears in **RC-1**, **F1
§3**, **F7 P4**, **F8 §2.3**, and is **enforced at F3 CHECK GOVERNANCE**. Four
statements, one enforcement point, no contradiction. ✓

---

## Path D — Long-running action revoked midway (a multi-step workflow, user hits stop)

| Aspect | Detail |
|---|---|
| **Flow** | A workflow grant is ACTIVE (F4 §1); the capability is mid-way through step 3 of 5. The **user triggers emergency stop** (F8 §2.8) or revokes this grant. → Monitor marks the **revocation-id INVALID**, atomically, ordered **before the next guarded operation** (F4 §3). → no new effect authorised; the running instance is **signalled to halt**, hard-terminated if it does not (F7 P7). → steps 1–2 completed; step 3 was mid-effect. → per-effect compensation (F4 §3): step 3's effect kind determines whether a declared inverse runs. → **one terminal outcome**: `interrupted` (F4 §4) — or `partially completed` if some steps stand and cannot be undone. |
| **Authoritative component** | the **Monitor** (owns the revocation-id validity — F4 §3); the **verifier / observed state** for what each step actually left behind; the **Chronicle** for the partial-effect Event. |
| **Trust boundary crossed** | Ring 0 → Ring 3, in reverse: the Monitor's revocation propagates into the sandbox as a halt signal; the sandbox cannot refuse it (F7 P7 — "cannot make itself un-stoppable"). |
| **Authority boundary crossed** | the grant's **liveness** boundary (F1 §5, RC-5): a grant is a revocable handle, not a one-time token — it stops authorising the moment it is revoked, **including mid-execution**. |
| **Failure point** | step 3 was a non-transactional external effect (a GUI action already taken, a child process already spawned) → cannot be cleanly undone → **`partially completed`, no rollback claimed** (F4 §4, §7). The child process is terminated (F7 P5, P8). |
| **Recovery** | **not auto-retried** (F4 §5 — a revocation-caused stop is never auto-retried). MELFINA reports the exact state: which steps stand, which were undone, which are uncertain. The user decides. |
| **Audit record** | a **PARTIAL / INTERRUPTED Event** (F4 §6, F5 §4.1), appended atomically (F6 §2); an **Audit record of the revocation act itself** (who/what triggered it, when — F4 §6); the terminal outcome; `brings-about` links for the effects that did land. |

**Cross-contract check:** RC-5 ("grants are live revocable handles, invalidate
mid-execution") is stated in **F1 §5** and **F4 §1–§3**, enforced by **F3**
(marks the revocation-id) + **F7 P7** (the sandbox obeys the halt). The **outcome
vocabulary** is defined once in **F4 §4** and F10 §4, F5 §4.1, and this document
all reference it — no divergent vocabulary. ✓

---

## Path E — Malicious reasoner (Ring 2 compromised by prompt injection; tries to exfiltrate + self-authorise)

| Aspect | Detail |
|---|---|
| **Attempt 1: open a socket and POST private data.** | Ring 2 holds **no network capability** (RC-6, L4, F7 §7). There is **no `network-*` effect class** in the vocabulary (F1 §3, F8 §2.3). A socket **cannot be opened** from Ring 2 or Ring 3 (F7 P3). The lethal trifecta cannot form because the outbound leg does not exist (F8 §2.5, INV-1). **Blocked structurally.** |
| **Attempt 2: emit an Intention with `status = authorised`.** | The Monitor **does not trust Intention status as authority evidence** (F3 §5). `authorised` is produced **only** by a recorded AUTHORISE artefact the Monitor itself creates (F5 §4.3, INV-2). A Ring-2-authored `authorised` field is ignored; the Intention is treated as `proposed`. **Blocked.** |
| **Attempt 3: propose a benign-looking action whose canonical form is malicious** (e.g. argv that shell-expands). | There is no shell (F2 §1). `argv` is a vector, never re-split (F2 §4). The grant binds to the exact canonical action (F2 §5). A `shell-exec` is a **distinct high-risk class** requiring its own grant (RC-4, F8 §2.3) that a pre-authorised routine scope does not include. **Blocked.** |
| **Attempt 4: describe a high-risk action as low-risk in the user-facing text.** | The Monitor **classifies from the canonical action + governance floors**, never from the capability's or reasoner's self-declared risk (F3 §5, F8 §2.4). The AI-generated user-visible description is **not trusted** for authorisation (F3 §5). **Blocked.** |
| **Attempt 5: split one high-risk effect into many small requests.** | Aggregate-effect governance (RC-2): the activity chain's **union** of authority + scope + aggregate effect is classified; **no risk laundering** (F8 §2.7). The aggregate budget cuts off hard (F8 §2.7, `MEL-REQ-183`). **Blocked.** |
| **Authoritative component** | the **Reference Monitor** — the single chokepoint (F3 §1 complete mediation). |
| **Trust boundary** | Ring 2 → Ring 0. The Monitor treats **everything** from Ring 2 as an untrusted proposal (F3 §5 distrust table). |
| **Authority boundary** | Ring 2 has cognitive autonomy only — it may reason and propose; it has **no** decision or execution authority (the triad, INV-2). |
| **Failure point** | if a *classification bug* let a malicious action through, VERIFY (F10) is the backstop; a `contradicted` verdict is a stop-condition; the Supervisor halts the capability; the aggregate budget bounds total damage. Defence in depth, not a single wall. |
| **Recovery** | emergency stop; revoke the activity chain's grants (F4); review everything since the last clean verification; the Chronicle is append-only so the record of the attack is intact (F5 §2). |
| **Audit record** | every denied proposal is recorded with its **deny reason code** (F3 §3 closed set); the pattern (many denials) feeds the Supervisor and Self-Evaluation (`MEL-REQ-248`). |

**Cross-contract check:** the "no authority from model output / proposal status /
self-description" invariant is owned by **F3 §5**, and supported by **F1**
(grants come only from the Monitor), **F2** (no shell, canonical actions), **F8**
(risk floors, aggregate, trifecta), **F7** (no network primitive). Five contracts,
consistent, each with a named enforcement point. ✓

---

## Path F — Malicious capability (a Ring-3 component is compromised; tries to exceed its grant)

| Aspect | Detail |
|---|---|
| **Attempt 1: access a file outside its grant.** | F7 P4: the instance sees **only** the canonical path set of its `file-*` grants; it cannot traverse or `open()` outside. **Blocked at the boundary**, not inside. |
| **Attempt 2: spawn a child with MELFINA's authority.** | F2 §2.1: `child-authority = nothing` by default; F7 P5: the child inherits no MELFINA authority. **Blocked.** |
| **Attempt 3: forge or replay a grant.** | F1: grants are unforgeable; F7 P10: the instance cannot mint, guess, or replay one. **Blocked** (mechanism-dependent — `[ID]` — the grant representation must deliver this). |
| **Attempt 4: call another component *as* the Monitor.** | F7 P11: a capability cannot impersonate the Monitor or answer a decision request. **Blocked.** |
| **Attempt 5: lie about what it did.** | VERIFY (F10) reads **actual post-state**, not the capability's account (F10 §6 "authoritative over the capability's claim"). A `contradicted` verdict → stop-condition → capability retired (`MEL-REQ-224, 249`). **Caught after the fact; bounded by least authority + aggregate budget.** |
| **Attempt 6: consume unbounded resources / loop.** | F7 P6: hard resource ceilings as cut-offs + a no-progress detector; the external watchdog terminates it. **Blocked.** |
| **Authoritative component** | the **isolation mechanism** (F7) for confinement; the **verifier** (F10) for detecting a lie; the **Monitor** (F3) for never having over-granted. |
| **Trust boundary** | Ring 3 ↔ everything. MELFINA **distrusts the capability code itself** (INV-8, F7 §8). |
| **Authority boundary** | the grant set — least authority (F1 §1), one grant per authorised Intention (F1 §8), no ambient authority (F7 P1). |
| **Failure point** | a real isolation bug (the capability *does* escape) → F7 §5: stop-condition, revoke the grant family, retire the capability, treat the isolation mechanism as suspect, halt autonomous operation pending review. The **core is never at risk** (INV-11, F7 §5). |
| **Recovery** | contained to the instance's subtree (F7 P8); Supervisor restarts or abandons; Ring 1 + Chronicle + Ring 0 untouched. |
| **Audit record** | boundary-denial events; the verifier's `contradicted` verdict Claim; the lifecycle's trust-downgrade + retirement Events; if an escape occurred, a full incident record. |

**Cross-contract check:** F7 (isolation properties P1–P14) + F1 (least authority,
unforgeable grants) + F10 (verify catches lies) + F4 (revoke the family) + F5/F6
(append-only incident record). Consistent. The seam "what if isolation *actually*
fails" is explicitly owned by **F7 §5** — not hand-waved. ✓

---

## Path G — Governance tampering attempt (something edits the Ring-0 governance file)

| Aspect | Detail |
|---|---|
| **Attempt 1: MELFINA proposes a governance change and tries to apply it.** | There is **no effect class** that writes Ring 0 / governance (F1 §3, F8 §2.3 "permanently absent"). The pipeline has **no path** to author or activate a version (F8 §1, §3). A Ring-2 Intention "governance should change" can only ever reach `proposed`; fulfilment is entirely a human act (F8 §5). **Blocked structurally.** |
| **Attempt 2: a compromised process edits the governance file on disk directly.** | The Ring-0 region is **excluded from every `file-*` grant** (RC-1, F1 §3, F7 P4, F8 §2.3) — no capability can be granted write there. An out-of-band compromise (OS-level) that edits it anyway is caught at the **next startup**: F9 §3 walks the chain, recomputes hashes, verifies signatures against the human-held trust root; a mismatch/bad-signature → **MELFINA refuses to run** (RC-1, F9 §3). |
| **Attempt 3: replace the file with an older, validly-signed version** (downgrade). | F9 C7: the head is checked against a **separately protected current-head marker**; an old version presented as current fails the check → **refuse to run**. |
| **Attempt 4: swap in a verification anchor MELFINA controls, then self-sign.** | F9 T3: the trust anchor is human-installed and integrity-protected; startup verification checks it; MELFINA cannot silently substitute it. And MELFINA **does not possess** the signing authority in any usable form (F9 T1, T4). **Blocked.** |
| **Attempt 5: edit governance in a running MELFINA (hot-swap).** | Governance is read-only at runtime (F8 §4); a runtime hash mismatch is a **stop-condition** → emergency stop (F9 §4). |
| **Authoritative component** | **F9's startup verification + the protected head marker**, jointly authoritative for which governance version is active. Nothing at runtime overrides. |
| **Trust boundary** | the human trust root ↔ MELFINA. MELFINA is entirely on the untrusted side of governance authorship. |
| **Authority boundary** | the meta-invariant (INV-6, F8 §2.1): MELFINA cannot change the rules that constrain how it changes itself, regardless of assessed competence or benefit. |
| **Failure point** | **`[OPEN]` — OQ-19 / `MEL-REQ-239`**: the meta-invariant is **necessary but not proven sufficient** against a highly capable self-modifier. If MELFINA could obtain the offline key (social engineering the human, a supply-chain attack on the authoring machine), the chain would verify a malicious version. This is a documented residual risk, not a closed one. |
| **Recovery** | refuse to run; human re-installs a known-good signed chain from a held backup (F9 §6); clean restart re-verifies. |
| **Audit record** | a failed verification produces a **durable report outside the Chronicle** (since the Chronicle may be suspect in a compromise — F9 §5) and, if available, a Chronicle Event. Every verified startup appends an Event with the chain head hash (F9 §5). |

**Cross-contract check:** RC-1 is threaded through **F1 §3**, **F7 P4**, **F8
§2.3 / §9**, **F9 (whole)**. The integrity mechanism is deliberately
**independent of the Chronicle** (F9 §0, §6) so neither can launder the other's
compromise. Consistent. The residual (`[OPEN]`, OQ-19) is stated identically in
F3 §9, F8 §7, and here. ✓

---

## Path H — Local-only violation attempt (any component tries to send data off the machine)

| Aspect | Detail |
|---|---|
| **Flow** | some component — Ring 2, a capability, a verifier, an automation — attempts to transmit data to a remote host. |
| **Blocked because** | (1) **no `network-*` effect class exists** in the core vocabulary (F1 §3, F8 §2.3); (2) **no component in Ring 1 or Ring 2 holds a network capability** and none can open a socket (RC-6, F7 §7); (3) **Ring 3 has zero network capability by default and the socket primitive is withheld**, not merely unconfigured (F7 P3, L4); (4) the reasoning system **runs locally** (`MEL-REQ-155`); (5) data leaving the machine requires **an explicit, per-instance human action** outside the autonomous path (INV-1, F8 §2.9, `MEL-REQ-164–169`). |
| **Authoritative component** | the **shape of the effect vocabulary** (F1 §3, F8 §2.3) — this is a **structural** guarantee `[SEN]`, the strongest kind. There is no runtime decision to get wrong because there is no capability to invoke. |
| **Trust boundary** | there is no outbound trust boundary to cross — Ring 4 (network) is **not built** and is architecturally separated for the future (SYSTEM_ARCHITECTURE Ring 4). |
| **Authority boundary** | INV-1: local-only is constitutional (F8 §2.9). Even a future Ring-4 capability would be checked against the lethal-trifecta prohibition (F8 §2.5) over the activity chain (RC-2). |
| **Failure point** | an OS-level compromise that opens a socket **outside** MELFINA's mediated paths is outside MELFINA's structural guarantee — it is an OS/host security concern. MELFINA's contribution: the core **never assembles** the {private data + untrusted content + outbound channel} trifecta itself, so even a host compromise finds no pre-staged exfiltration pipeline. Documented residual `[U]`. |
| **Recovery** | n/a within MELFINA (nothing to revoke — no grant was ever issued); a detected host compromise is an emergency-stop + human incident. |
| **Audit record** | any attempt that *reaches* the effect vocabulary is denied with a reason code (F3 §3) and recorded; an attempt to open a socket directly fails at the OS boundary (F7 P3) and, if observable, is logged by the isolation mechanism. |

**Cross-contract check:** INV-1 / local-only is owned by **F8 §2.9**, enforced
structurally by **F1 §3** (no class) + **F7 P3 / §7** (no primitive) + **RC-6**.
No contract anywhere introduces a network effect. The one place it could appear —
a future Ring 4 — is explicitly out of scope and gated by the trifecta rule. ✓

---

## Path I — High-risk action ("delete these 400 files", or an irreversible external action)

| Aspect | Detail |
|---|---|
| **Flow** | Ring 2 PROPOSES a `file-op` with operation `delete` over many paths (or `truncate`/`rename`). → AUTHORISE: CLASSIFY → `file-write` with the **irreversible sub-condition** → **`high` risk floor** (F8 §2.4, F2 §2.4). The capability **cannot classify below the floor** (`MEL-REQ-227`, F1 §10). → CHECK GOVERNANCE: high-risk ⇒ `class → required-checks` demands a VERIFY plan **and ≥ 2 architecturally-independent verifiers** (F8 §4, F10 §3, RC-3); aggregate effect over 400 files is "practically irreversible in aggregate" (F1 §8, F8 §2.7). → because it is **irreversible + high-risk**, the pre-authorised routine scope does **not** cover it; it requires **explicit human confirmation** (`MEL-REQ-157, 181`). → only after the human confirms: grant minted, live (F4). → EXECUTE in the sandbox. → VERIFY: two independent verifiers (e.g. one enumerates the actual filesystem, one checks the Chronicle Events + a pre-deletion manifest) — F10 §3. |
| **Authoritative component** | the **human** (for the go/no-go on an irreversible high-risk action); the **Monitor** (for the floor + required-checks); **both verifiers jointly** (for the outcome — disagreement fails toward caution, F10 §4). |
| **Trust boundary** | Ring 2 → Ring 0 → **human** → Ring 0 → Ring 3. The human confirmation is itself an input the Monitor requires; a forged/assumed confirmation is not accepted (F3 §5 — status/description not trusted; the confirmation is an out-of-band signal). |
| **Authority boundary** | the risk floor is a **ceiling on self-classification** (F1 §10, `MEL-REQ-227`); the human-confirmation requirement is a **hard gate** for irreversible high-risk (F8 §2.4, `MEL-REQ-181`); situational autonomy **cannot** raise past this (`MEL-REQ-245`, F8 §2.6). |
| **Failure point** | a verifier is `inconclusive` and the action is irreversible → F10 §4: MELFINA must **not** have reached autonomous execution here (the floor + human gate prevent it); if it somehow did, that is a stop-condition and an incident. Partial deletion mid-run + revocation → `partially completed`, **no rollback claimed** unless a pre-deletion snapshot exists and is restored (F4 §7 — for `file delete/truncate/rename`, "rollback" means restore-from-copy or it is not rollback). |
| **Recovery** | if a snapshot was taken (part of the VERIFY plan for an irreversible action), restore → `rolled back` + verify the restore. Otherwise the loss is real and reported precisely — MELFINA does not claim it undid what it did not. |
| **Audit record** | the human confirmation (recorded with time + what was confirmed); the classification + floor applied; **both** verifier verdict Claims + both evidence sets (F10 §5); a pre-action manifest; the EXECUTE Event with `brings-about`/`ends` links; one terminal outcome (F4 §4). |

**Cross-contract check:** the high-risk path exercises F2 (irreversible
sub-conditions) + F1 §10 (carried-not-decided risk, self-classification ceiling)
+ F8 §2.4/§4 (floors, required-checks) + F10 §3 (≥2 independent verifiers,
"architecturally independent" ≠ statistical) + F4 §7 (what rollback means per
effect kind) + `MEL-REQ-157/181` (human gate). Every hand-off has a named owner.
The one genuine tension — "what if a strong verification is infeasible for an
irreversible action" — is resolved identically in F10 §3/§4 and F8 §2.4: **it
does not run autonomously.** ✓

---

## Contract-vs-contract consistency matrix

| Shared concept | Defined in (owner) | Referenced/enforced in | Consistent? |
|---|---|---|---|
| Closed effect-class vocabulary | **F8 §2.3** | F1 §3, F2 §2, F3 CLASSIFY, F7 §6 | ✓ one list, F1 defers to F8 |
| "grant for argv A ≠ argv B / shell / chain" | **F2 §5** | F1 §4, F3 CHECK SCOPE, Path B/E | ✓ |
| Ring-0 path exclusion from file grants (RC-1) | **F8 §2.3 / F9** | F1 §3, F7 P4, F3 CHECK GOVERNANCE, Path C/G | ✓ 4 statements, 1 enforcement point |
| Grants are live revocable handles (RC-5) | **F1 §5 + F4 §1** | F3 (marks revocation-id), F7 P7, Path D | ✓ |
| Outcome vocabulary (completed/partial/rolled-back/failed/interrupted) | **F4 §4** | F10 §4, F5 §4.1, F6, Path B/C/D/I | ✓ one closed set everywhere |
| "No authority from status / description / self-report" | **F3 §5** | F1 (grants only from Monitor), F8 §2.9, F10 (verify reads reality), Path E/F | ✓ |
| Aggregate-effect governance / no risk laundering (RC-2) | **F8 §2.7** | F1 §8, F3 CHECK AGGREGATE, Path E/I | ✓ |
| Authoritative read vs view read (M2) | **F6 §3.1 / §3.2** | F3 (Monitor reads authoritative state), F5, Path A | ✓ |
| "Architecturally independent" ≠ statistical independence (RC-3) | **F10 §3** | F8 §4, Path I | ✓ over-claim explicitly avoided |
| Meta-invariant necessary-not-sufficient (OQ-19) | **F8 §2.1 / §7** | F3 §9, F9, Path G | ✓ stated identically |
| No network effect / local-only (INV-1, RC-6, L4) | **F8 §2.9** | F1 §3, F7 P3/§7, Path H | ✓ no contract introduces one |
| Verifier is Ring-3, low-authority, not an agent (RC-3) | **F10 §1–§2** | F7 (runs under isolation), F3 (gates it), Path B/C/F/I | ✓ |
| Governance integrity independent of the Chronicle | **F9 §0 / §6** | F5 (Chronicle only references governance), F6, Path G | ✓ |
| Provenance is structural (attributed-to / generated-by / derived-from) | **F5 §6** | F1 §13, F10 §5/V8, all audit-record rows | ✓ |
| Emergency stop behaviour | **F8 §2.8** | F4 (revokes grants), F7 §5, F9 §4, Path D/G | ✓ |

**No contradictions found.** Every shared concept has exactly one owning
contract; every other mention defers to it.

---

## Open seams (carried forward, not defects)

1. **OQ-19 / `MEL-REQ-239`** — the meta-invariant is necessary, not proven
   sufficient. Appears in Paths E and G. `[OPEN]`.
2. **Grant representation** (F1 §19, F7 P10) — "unforgeable" is `[ID]` until a
   representation is chosen and red-teamed. Paths B, F depend on it.
3. **Isolation mechanism** (F7 §10) — every P1–P14 property that is `[ID]`/`[ETL]`
   waits on the mechanism choice + an adversarial corpus. Path F.
4. **Verifier independence in practice** (F10 §3, `[U]`) — common-mode failure is
   real (Knight & Leveson + 2026 replication). Path I mitigates by "fail toward
   caution", not by assuming independence.
5. **Timing / covert channels** (F7 §3) — not a guaranteed property; mitigated
   only by "no outbound channel to exfiltrate to". Path H residual.
6. **OS-level / host compromise** — outside MELFINA's structural guarantees;
   MELFINA's contribution is to never pre-stage the trifecta. Paths G, H.
7. **OQ-20** (authorisation granularity per tier), **OQ-11** (local reasoning
   ceiling), **OQ-12** (routine/consequential boundary) — the contracts are
   written to *express* each option; the choice is deferred.
8. **OQ-M1 / OQ-M2 / OQ-M3 / OQ-M5** — the Chronicle format (F5 §9) preserves
   both alternatives for each; no path above forces a resolution.

---

## Traceability

| Element | Source |
|---|---|
| the six-question per-path frame (authoritative / trust boundary / authority boundary / failure / recovery / audit) | mission Part 11 |
| THINK → DECIDE → PROPOSE → AUTHORISE → EXECUTE → VERIFY | `MEL-REQ-018`; INV-2 |
| the nine paths A–I | mission Part 11 |
| the three must-be-real boundaries | RC-7 |
| all cross-references | F1–F10 of this mission; `8a40207` design set |
