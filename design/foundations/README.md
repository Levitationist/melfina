# MELFINA — LOW-LEVEL FOUNDATIONS

**Phase:** LOW-LEVEL FOUNDATIONS — MISSION 001. **Status:** first pass, pending
review. **Date:** 2026-09-11. **Baseline:** commit `8a40207` (architecture +
adversarial review + revision 1, on `origin/main`).

**This is still design/research.** No implementation code. No `src/`. No language,
storage engine, sandbox mechanism, cryptographic primitive, IPC model, GUI
toolkit, or local model is chosen. This mission defines the **smallest, precise,
technology-independent contracts every future implementation must obey**, so that
later technology choices can be *evaluated against* them.

---

## What a "foundation contract" is

Each document below fixes the **required properties** of one load-bearing
component and **defers the mechanism**. Every contract answers ten questions:

1. What does this component **guarantee**?
2. What does it **require** (its preconditions / dependencies)?
3. What may it **trust**?
4. What must it **distrust**?
5. What **enters** it?
6. What **leaves** it?
7. What happens on **malformed input**?
8. What happens on **failure**?
9. What is **authoritative**?
10. What can be **verified independently**?

Every MUST-level invariant is tagged with an **enforceability class**:

| Tag | Meaning |
|---|---|
| **[SEN]** structurally enforceable now | the property follows from the *shape* of the contract; an implementation that violates it is not conforming. |
| **[ID]** implementation-dependent | the property holds only if the eventual mechanism is chosen and built correctly; the contract states the acceptance criterion. |
| **[ETL]** empirically testable later | the property cannot be argued from the design; it needs a test suite / red-team / measurement at LOW-LEVEL FOUNDATIONS build time or later. |
| **[OPEN]** open / unknown | genuinely unresolved; carried forward, not closed. |

**Evidence tiers** (carried from the project): `[E]` established · `[G]` guidance ·
`[DI]` design inference · `[H]` hypothesis · `[U]` unknown.

---

## The twelve foundations, in dependency order

Work item *N* may refine item *N−k* only by adding constraint, never by
loosening it. Discovered dependencies are stated explicitly in each document's
§0.

The documents refer to each other by number as **F1 … F12** in the order of this
table (F1 = `CAPABILITY_GRANT_MODEL.md`, … , F12 =
`TECHNOLOGY_SELECTION_CRITERIA.md`).

| # | Document | Fixes | Derives from |
|---|---|---|---|
| 1 | **`CAPABILITY_GRANT_MODEL.md`** | what a grant *is*; its fields; attenuation, delegation, combination, staleness, conflict | RC-2, RC-4, RC-5, RC-6, RC-7; INV-2, INV-3, INV-7, INV-8; MEL-REQ-124, 126, 179–184, 222, 227 |
| 2 | **`STRUCTURED_ACTION_MODEL.md`** | the abstract action representation; normalisation + canonicalisation; the "grant for `X argv A` ≠ authority for `X argv B`" invariant | RC-4; INV-7, INV-8; MEL-REQ-143–146, 181 |
| 3 | **`REFERENCE_MONITOR_CONTRACT.md`** | complete mediation; the request→…→verify decision procedure; what the Monitor must never trust | AP-6, AP-7; INV-2, INV-6, INV-7; MEL-REQ-018, 153, 179–187, 227, 235 |
| 4 | **`REVOCATION_MODEL.md`** | the grant lifecycle; revoke-before / -during / -after / -on-completion; the outcome vocabulary; how partial effect becomes a recorded Event | RC-5; INV-2, INV-11; MEL-REQ-145, 172, 173, 177, 181, 187 |
| 5 | **`CHRONICLE_LOGICAL_FORMAT.md`** | the technology-independent logical structure of Event / Claim / Intention / Entity-reference / bitemporal Time / provenance / supersession / identity; OQ-M1/M2/M3/M4/M5/M11 preserved with both alternatives | model §4–§6, §8, §11, §12; INV-3, INV-4, INV-5; MEL-REQ-160–162, 170 |
| 6 | **`CHRONICLE_CONTRACT.md`** | APPEND / READ / QUERY / REPLAY / VERIFY / PROJECT; atomicity, ordering, idempotency, corruption detection, crash recovery, concurrency, the authoritative-read path | INV-3, INV-5; AP-3, AP-5; RC/M2; MEL-REQ-170, 173, 174 |
| 7 | **`ISOLATION_CONTRACT.md`** | the abstract isolation properties any Ring-3 mechanism must provide; what happens if isolation fails; which properties are structural vs testable | AP-7, AP-8; RC-6, RC-7; L4; MEL-REQ-124, 125, 184, 185 |
| 8 | **`GOVERNANCE_FORMAT.md`** | the logical structure of Ring-0 governance configuration; the governance object; why it is not ordinary MELFINA knowledge | AP-6; INV-6; MEL-REQ-233–236; model §12.1 |
| 9 | **`GOVERNANCE_INTEGRITY.md`** | RC-1's abstract integrity contract; human-held trust root; version chain; startup verification; invalid ⇒ refuse to run; the security properties required of whatever mechanism is chosen | RC-1; AP-6; INV-6; MEL-REQ-235, 239 |
| 10 | **`VERIFIER_CONTRACT.md`** | RC-3's verifier contract; "architecturally independent"; the enumerated single-verifier fallback; disagreement / timeout / unavailable / impossible / irreversible | RC-3; INV-2, INV-8, INV-10, INV-12; MEL-REQ-018, 157, 174, 181, 226, 248, 250 |
| 11 | **`FOUNDATION_CROSS_CONTRACT_ANALYSIS.md`** | the nine end-to-end path traces (A–I); per path: authoritative component, trust boundary, authority boundary, failure point, recovery, audit record; contract-vs-contract consistency | all of 1–10 |
| 12 | **`TECHNOLOGY_SELECTION_CRITERIA.md`** | MUST-HAVE / SHOULD-HAVE / MUST-NOT criteria per technology area, to be applied *after* these foundations are accepted | all of 1–11; `ARCHITECTURAL_PRINCIPLES.md` §3 |

---

## What this mission does NOT do

- choose a language, storage engine, on-disk format, sandbox primitive,
  cryptographic algorithm, key-custody mechanism, IPC model, concurrency model,
  GUI framework, or local model;
- design a concrete API, schema, wire format, or grant syntax;
- write a parser, a serializer, or any executable artefact;
- resolve model open questions **OQ-M1** (`Relation` a primitive), **OQ-M2**
  (`State` a primitive), **OQ-M5** (worries / affect), or any other explicitly
  unresolved model / requirements question;
- alter the architecture. Where a foundation makes an architectural statement
  more precise, it *adds* precision within the architecture's boundaries; any
  genuine contradiction is reported, not silently resolved.

## Reading order

Read `../ARCHITECTURAL_PRINCIPLES.md` and `../SYSTEM_ARCHITECTURE.md` first (the
invariants INV-1…14, principles AP-1…14, decisions AD-1…15, and corrections
RC-1…7 are used throughout and not re-derived here). Then read the twelve in the
order above. `FOUNDATION_CROSS_CONTRACT_ANALYSIS.md` is the integration test;
read it after 1–10.
