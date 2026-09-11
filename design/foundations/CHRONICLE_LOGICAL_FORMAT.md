# MELFINA — FOUNDATION 5: CHRONICLE LOGICAL FORMAT

**Phase:** LOW-LEVEL FOUNDATIONS — MISSION 001. First pass, pending review.
**Baseline:** `8a40207`. Read `../DATA_AND_STATE_MODEL.md` and
`model/HUMAN_CENTRAL_MODEL.md` §4–§6, §8, §11–§12 first.

**This document does not choose a storage engine, on-disk format, encoding,
indexing scheme, or query language.** It fixes the **logical structure** of the
units the Chronicle holds — the fields, their meanings, their invariants — such
that the E²CI model is preserved and no application concept becomes an accidental
primitive.

**It does not resolve OQ-M1 (`Relation` a primitive), OQ-M2 (`State` a
primitive), or OQ-M5 (worries / affect).** Where an unresolved model question
touches the logical format, both alternatives are preserved (§9).

Derives from: model §3–§6, §8, §11, §12; **INV-3, INV-4, INV-5**; **AP-4, AP-5**;
**MEL-REQ-093, 096, 131, 160, 161, 162, 170**; bitemporal modelling (Snodgrass,
SQL:2011), event sourcing, W3C PROV `[E]`.

---

## 0. Dependencies discovered

- **On F6 (`CHRONICLE_CONTRACT`):** F5 is the *shape* of a unit; F6 is the
  *operations* on the sequence of units and their durability/ordering. `transaction-time`
  assignment, atomicity, and monotonic ordering are F6.
- **On F1/F3:** a grant's `scope` for `chronicle-read`/`chronicle-append` names
  units and unit shapes using the identity and selector forms defined here.
- **On F8:** governance is **not** a Chronicle unit (model §12.1). F5's unit set
  deliberately excludes it.
- **Discovered constraint:** the model's structural relations `brings-about`,
  `ends`, `supersedes`, `part-of` are **mechanically load-bearing** — Ring-1
  projection and bitemporal query consume them — even though the model represents
  relations as Claims (adversarial review L2, OQ-M1). F5 records this special
  mechanical status **without** promoting `Relation` to a primitive (§4.4, §9).

---

## 1. The unit set

The Chronicle holds **exactly three kinds of unit** — `Event`, `Claim`,
`Intention` — plus a thin **Entity Registry** of referents. Nothing else is a
stored unit type. In particular the following are **NOT** units and have **no
storage type**: `Context`, `State`, `Relation` (see §9), `Task`, `Goal`,
`Routine`, `Project`, `Memory`, `Skill`, `Workflow`, `Calendar`, `Dashboard`,
`Streak`, `Strategy`. Each is a **derived view or a pattern** over the three unit
kinds (model §13). `[SEN]` — the logical format has three unit shapes; an
implementation that adds a fourth stored unit shape for one of these concepts is
not conforming.

**MELFINA's own state uses the same three unit kinds**, distinguished only by
`holder`/`owner`/`attributed-to = melfina` (model §12, P-10). There is no
separate store for MELFINA's beliefs, proposals, self-evaluation, or learned
adaptation. `[SEN]`.

---

## 2. Fields common to all units

| Field | Meaning | Invariant |
|---|---|---|
| `unit-id` | a stable, opaque, unique identifier for this unit instance | assigned once; never reused; not derived from content (so identical content ⇒ distinct units unless deliberately deduplicated at a higher layer) |
| `unit-kind` | `event` \| `claim` \| `intention` | fixed at creation |
| `transaction-time` | the coordinate at which this unit was **recorded** (F6 assigns it) | **monotonic, immutable, append-only** across the whole Chronicle (INV-5, P-5) |
| `provenance` | see §6 | every unit carries it; there is no path to a unit without provenance |
| `supersedes` | zero or more `unit-id`s of prior units this one **revises or replaces** | the prior units are **retained** and remain queryable by `transaction-time` (AGM, `[E]`) |
| `redaction-marker` | present iff this unit's *content* has been removed by an explicit user redaction (§8) | the unit's existence, `transaction-time`, and "redacted by the user at T" remain; the content does not |

`transaction-time` and `unit-id` are **write-once**. There is **no field on any
unit that a normal operation may update in place** (INV-5). `[SEN]`.

---

## 3. ENTITY (the registry — not a Chronicle unit)

The Entity Registry is **append-versioned**, not append-only-log: an entity's
`id` is stable; its `names`/`aliases`/`kind` accrue versions over time.

| Field | Meaning | Notes |
|---|---|---|
| `entity-id` | opaque, stable handle | referenced by `unit-id`s in participant/holder/target/about positions |
| `kind` | an **open** classifier: `person` \| `place` \| `thing`/`resource` \| `work`/`artifact` \| `concept`/`topic` \| `body-of-knowledge` \| `collective` \| `capability` \| `workflow`/`system` \| `source` \| `self` \| `melfina` \| … | **open** (P-11) — new kinds add no logical-format change; `kind` is an attribute, not a type |
| `names` / `aliases` | how the user and others refer to it, over versions | each carries a validity range |
| `same-as` | claims (in the Chronicle) that two `entity-id`s are the same referent | a Claim, `status` possibly `hypothesised` — identity uncertainty is representable |

- **Entities carry no truth value and no epistemic status** — they are hooks
  (model §4.1). Everything *claimed about* them is a `Claim`. `[SEN]`.
- `self` (the user) and `melfina` are entities of special kinds **for now** —
  **OQ-M9 (are they ordinary entities or the two fixed perspectives?) is not
  resolved here.** The logical format works either way: they are `entity-id`s
  used in `holder`/`owner`/`attributed-to`; whether the implementation privileges
  them is deferred. `[OPEN]`.

---

## 4. The three unit kinds

### 4.1 EVENT

**Something that occurs at a time-point or over an interval, possibly with
participants in roles, possibly bringing about or ending conditions** (model §4.2).

| Field | Meaning | Notes |
|---|---|---|
| `event-kind` | **open**: `observation` \| `action` \| `session`/`activity` \| `communication` \| `decision` \| `state-change` \| `performance` \| `milestone` \| `redaction` \| … | open; `redaction` is a defined kind (§8) |
| `time` | a point or interval on the **single actual timeline** | may be **`expected`** (future) or **`occurred`**; an `expected` Event that occurs is superseded by an `occurred` version, possibly with a corrected time |
| `participants` | `entity-id`s in **roles**: `agent` \| `object` \| `instrument` \| `beneficiary` \| `witness` \| `location` | PROV `wasAssociatedWith`/`used`/`wasAttributedTo` `[E]` |
| `on-behalf-of` | `entity-id` of the agent this action was performed *for* (PROV `actedOnBehalfOf`) | central to MELFINA acting for the user; a MELFINA action has `attributed-to = melfina`, `on-behalf-of = self` |
| `brings-about` / `ends` | `unit-id`s of the **descriptive Claims** (conditions) this Event initiates / terminates | event-calculus `Initiates`/`Terminates` `[E]`; **mechanically load-bearing** (§4.4) |
| `derived-from` / `informed-by` | `unit-id`s of Events/Claims this Event used or responded to | PROV `wasDerivedFrom`/`wasInformedBy` |

### 4.2 CLAIM

**A statement held by an agent, about the world (including about entities,
events, other claims, and intentions), with an origin and an epistemic status.
All of MELFINA's knowledge is claims** (model §4.3, P-4).

| Field | Meaning | Notes |
|---|---|---|
| `content` | one of: **descriptive** ("condition C holds" over a valid-time interval — this is the "state" content, §9); **relational** ("X ρ Y", §4.4); **about-a-claim** ("K1 contradicts K2"); **about-an-entity** ("E has name N") | the content *representation* is deferred; the **kinds** of content are fixed |
| `holder` | whose claim it is: `self` \| `melfina` \| a `source` `entity-id` \| another person `entity-id` | the structural basis for "MELFINA-generated vs user-provided vs inferred" (`MEL-REQ-093`, `096`) |
| `status` | the epistemic mode, **not collapsed**: `observed` \| `reported` \| `inferred` \| `hypothesised` \| `open` \| `stipulated` \| `decided` \| `recalled` | a fixed, closed set (model §4.3); there is **no** `fact` status |
| `confidence` | a **coarse** degree of belief | **OQ-M4 (should confidence be structured — imprecision / ambiguity / ignorance / disputed?) is not resolved here.** The format reserves `confidence` as a field whose internal structure may later gain sub-fields without changing the unit set. `[OPEN]` |
| `provenance` | §6 | `attributed-to` / `generated-by` / `derived-from` |
| `valid-time` | the interval over which the content is asserted **true in the world** | any range (past / present / future); **correctable** (a new Claim with a corrected `valid-time` supersedes) |

**A "fact" is not representable.** The closest is *a descriptive Claim held by
`self`, status `observed` or `stipulated`, high confidence, not superseded, not
contradicted* — and even that is a Claim (model §4.3). `[SEN]`.

### 4.3 INTENTION

**An agent's directedness toward a future condition or action — the
world-to-mind stance** (model §4.4, P-7). Distinct from a Claim by **direction of
fit**; failure of an Intention **falsifies nothing** (model §4.4).

| Field | Meaning | Notes |
|---|---|---|
| `owner` | `self` \| `melfina` \| another agent `entity-id` (as the owner's claim about them) | |
| `target` | a **condition** to bring about (descriptive content, like a Claim's) and/or a specific **Event** to perform / see happen | goals = condition targets; tasks = Event targets + often a plan |
| `creditor` | optional `entity-id` — if present, the agent to whom the owner is **committed** (social commitment, Castelfranchi `[E]`) | absent ⇒ internal intention; present ⇒ the creditor holds an expectation the owner knows about |
| `status` | `active` \| `proposed` \| `authorised` \| `suspended` \| `fulfilled` \| `abandoned` \| `blocked` | a fixed, closed set; **`proposed` is how MELFINA suggests; `authorised` requires an AUTHORISE grant (F3) — reaching `proposed` never produces `authorised`** (INV-2) `[SEN]` |
| `plan` | **optional** ordered set of sub-`Intention`s and/or `expected` Events with temporal/`depends-on` relations | absence is normal (an "open loop" — `MEL-REQ-026`) |
| `standing` | `one-off` \| `recurring` \| `maintenance` | "habit"/"routine" lives here **with no streak or count** (`MEL-AR-02`); **OQ-M3 (recurrence identity — one Intention or a series?) is not resolved here** — see §9 `[OPEN]` |
| `provenance` / `valid-time` | §6 | changed goals are **superseded, not deleted** (P-5) |

**The `status` transition `proposed → authorised` is only ever caused by a
recorded AUTHORISE artefact + a Governance-Store grant (F3).** No reasoning
output, no projection, no automatic rule produces it. This is the autonomy triad,
in the data. `[SEN]`.

### 4.4 Structural relations — mechanically load-bearing, not a primitive

Relationships are **Claims of relational content** (model §5, P-9): each carries
its own `holder`/`status`/`confidence`/`provenance`/`valid-time`/`transaction-time`.
**The logical format does not add a `Relation` unit** (OQ-M1 preserved).

**But** a small set of structural relations is **mechanically consumed by Ring 1**
(the Projection Engine and the bitemporal query logic) and is therefore **special
in enforcement even though modelled as Claims** (adversarial review L2):

| Relation | Consumed by | For |
|---|---|---|
| `brings-about` / `ends` | Projection Engine | deriving the current-state cache ("what holds now") from Events (event-calculus initiate/terminate) |
| `supersedes` | Query + Projection Engine | version chains; "what MELFINA believes now" vs "believed at T" |
| `part-of` | Query + projections | mereological rollup (movement→work, file→repo, concept→subject) |
| `same-as` | Entity Registry + Query | identity resolution across `entity-id`s |

The logical format **records that these relations have this mechanical status**
and requires that:

- an implementation **may** materialise them in an index for projection/query
  performance, **but the index is a rebuildable derived view** (AP-5), never a
  second source of truth; `[SEN]`
- adding a relation to this "mechanically load-bearing" set is a **model-level
  change** (touches OQ-M1), not a foundation change; `[SEN]`
- **all other relations** are ordinary Claims with no special enforcement —
  including open user/MELFINA-defined verb-phrase relations (neutrality, P-2).

**OQ-M1 stays open.** If the model later promotes `Relation` to a primitive, the
logical format gains a fourth unit kind whose fields are those a relational Claim
already carries — a *model* decision, made then, not here. `[OPEN]`.

---

## 5. Time (bitemporal) — the logical semantics

Time is a **dimension, not a unit** (model §6). Every unit is *situated in* two
axes:

- **`valid-time`** — the interval the content is asserted true **in the world**
  (Claims), or the Event occurred / is expected, or the Intention's target window
  is. Any range; correctable by a superseding unit. (Snodgrass "valid time",
  SQL:2011.) `[E]`
- **`transaction-time`** — assigned by the Chronicle on append (F6). **Monotonic,
  immutable, append-only.** (Snodgrass "transaction time", SQL:2011.) `[E]`

The four historical-truth semantics the format must support (as *query
capabilities*, F6 — not stored differently):

| Query | Meaning |
|---|---|
| `at(valid = V, tx = now)` | what MELFINA **now believes** was true at V |
| `at(valid = V, tx = T)` | what MELFINA **believed at time T** about V |
| `history-of(unit-id)` | the `supersedes` chain — every version retained |
| `as-of(tx = T)` | the whole knowledge state as it was at `transaction-time` T |

**"Change the past"** = append a new Claim with an **earlier `valid-time`** and a
**`now` `transaction-time`**, `supersedes` the old; the old **stays** (model §6.1,
INV-5). There is **no** edit-in-place path for `valid-time`. `[SEN]`.

Qualitative temporal structure uses **Allen's interval algebra** (13 relations)
as relational Claims where the user/MELFINA asserts them — not as a stored index
requirement. `[E]` / `[DI]`.

---

## 6. Provenance (structural, not metadata)

Every unit carries, structurally (AP-4, PROV `[E]`):

- **`attributed-to`** — the `holder`/`owner`/agent responsible (PROV `wasAttributedTo`).
- **`generated-by`** — the `unit-id` of the `observation` / `communication` /
  reasoning `Event` (an `action` Event, `agent = melfina`, for an `inferred`
  Claim) that produced this unit.
- **`derived-from`** — the `unit-id`s of the Claims and Events this unit **used**
  (PROV `wasDerivedFrom` / `used`).

**MELFINA can always answer "how do I know this?"** by walking `generated-by` /
`derived-from` to an `observation` Event, a `source` entity, or a reasoning trace
(`MEL-REQ-097`, `131`, `093`). A unit with no reachable grounding is itself a
signal (a bare `stipulated` claim, or a broken chain to be flagged). `[SEN]` for
the requirement to carry it; `[ETL]` for chain completeness on real data.

**Storage ≠ memory** (model §12): a file the user drops in is an `Entity` (kind
`source`/`artifact`); what MELFINA *knows from it* is `Claims` (`status =
reported`, `provenance → that Entity + the ingest observation Event`). The bytes
are an artefact; the knowledge is Claims. `[SEN]`.

---

## 7. Supersession, versioning, belief change

- **Supersession** (`supersedes`) is the versioning mechanism for Claims and
  Intentions. Every version is **retained** and queryable by `transaction-time`.
- **Belief change** (AGM, `[E]`; model §9) — all three operations are **appends**;
  nothing is erased:
  - *expansion* = append a consistent Claim;
  - *revision* = append a Claim + `supersedes` the conflicting ones;
  - *contraction* = append a **retraction Claim**; mark units that
    `derived-from` the retracted one for re-evaluation (they are not
    auto-deleted).
- **Conflict** = two Claims connected by a `contradicts` Claim, **both retained**
  (P-4). The format does **not** force resolution; resolving is a `decision`
  Event that `supersedes` one. `[SEN]`.

---

## 8. Redaction (the only content-removal path)

- **Deletion is a separate, explicit user act** (`MEL-REQ-162`), never a normal
  operation. It is modelled as a **`redaction` Event** (`agent = self`) that
  **tombstones** the target unit(s): the unit's `content` is removed and a
  `redaction-marker` is set; the unit's **`unit-id`, `unit-kind`,
  `transaction-time`, and "redacted by the user at T"** remain.
- **Why the shell remains:** the `transaction-time` sequence must have **no
  hole** — a downstream integrity check (F6) verifies the append chain, and a
  missing unit would break it. A minimal record "a unit was redacted here, by the
  user, at T" preserves chain integrity while removing the content (model §11.2).
- Redaction **cascades to every derived view** (the current-state cache,
  on-demand views) — the tombstone propagates (F6 PROJECT). `[SEN]` for the
  semantics; `[ID]` for cascade completeness.
- **Full deletion is possible** — any unit, any category, or everything
  (`MEL-REQ-162`) — via redaction Events. "Everything" is a bulk redaction that
  leaves an empty-but-integrity-valid chain. `[SEN]`.

---

## 9. Unresolved model questions — both alternatives preserved

| Question | The format's stance |
|---|---|
| **OQ-M1 — is `Relation` a primitive?** | **Preserved.** Relations are Claims of relational content (§4.4). A small set is mechanically load-bearing (§4.4) — recorded as such, *not* promoted. If the model later makes `Relation` primitive, the format gains a fourth unit kind carrying the fields a relational Claim already has. The APPEND/QUERY contract (F6) is written so that adding a unit kind is additive. |
| **OQ-M2 — is `State` a primitive?** | **Preserved.** "What holds now" is **the set of currently-valid descriptive Claims** (model §3.3); it is a **derived view** (the current-state cache), never a stored authoritative unit. If the model later makes `State` primitive, it becomes a **checkpointed projection**, not a new unit kind (`ARCHITECTURAL_ALTERNATIVES.md` AA-8, AU-4). The format does not depend on the answer. |
| **OQ-M3 — recurrence identity (one Intention or a series)?** | **Preserved.** A `recurring` Intention (§4.3) is a generative pattern; each occurrence is its own `Event`. Whether "I skipped Tuesday" attaches to the standing Intention, a per-day instance, or nothing (lapses unpunished, `MEL-REQ-064`) is **not decided**. The format supports both: the standing Intention has `standing = recurring`; per-occurrence structure, if the model adopts it, is sub-Intentions in the `plan` — no new unit kind. **The format must not make streak-like accumulation the path of least resistance** (OQ-M12): there is no `occurrence-count`, `last-done`, or `streak` field. `[SEN]` |
| **OQ-M5 — worries / intrusive thoughts / ambivalence / affect?** | **Preserved.** Currently: a worry is a `Claim` (`status = open`) or an `Intention` with no plan; affect is a `Claim` "self felt X". Whether a light **"concern" / "salient-but-unresolved"** notion is needed is **not decided** and is flagged for **user review**. If adopted, it is most likely a `status` value or a Claim sub-kind, **not** a new primitive (model open questions §"may be primitives"). The format reserves the possibility without committing. `[OPEN]` |
| **OQ-M4 — confidence structure?** | **Preserved.** `confidence` is a field; its internal structure may gain sub-fields later without changing the unit set. `[OPEN]` |
| **OQ-M11 — norms / permissions as claims?** | Permissions/authorisations are **descriptive Claims about a normative state** ("melfina is authorised re: K") — represented, **not enforced** (model §12.1). Whether they need a distinct `status = normative` to avoid confusion with factual claims is a **small semantic clarification deferred to model + this phase's F8/F9**; the format currently uses `status = decided` or `stipulated` for them and flags the question. `[OPEN]` |

**The meta-invariant rules and the permission-enforcement mechanism are NOT
Chronicle units** (model §12.1) — if they were, MELFINA could reason about
editing them (INV-6). They live in Ring-0 governance (F8, F9). `[SEN]`.

---

## 10. Contract Q&A

| Question | Answer |
|---|---|
| **Guarantees** | exactly three unit kinds + a thin registry; every unit carries `unit-id`, `unit-kind`, `transaction-time` (write-once), provenance, and (Claims/Intentions) epistemic status + bitemporal `valid-time`; no field is updatable in place; no "fact" shape; no application concept is a stored unit; MELFINA's state uses the same three kinds |
| **Requires** | F6 to assign `transaction-time` monotonically and append atomically; the Entity Registry for referent identity; the model's structural-relation vocabulary (provisional, OQ-M10) |
| **Trusts** | the model's four-primitive reduction (E²CI); PROV / bitemporal / AGM semantics `[E]` |
| **Distrusts** | any pressure to add a stored type for `Context`/`State`/`Relation`/`Task`/`Goal`/`Routine`/…; any implementation that treats a projection as authoritative |
| **Enters** | a proposed unit (from a capture, an observation, an action result, a Ring-2 `inferred` Claim / `proposed` Intention, a user decision) |
| **Leaves** | a well-formed unit with all common fields populated, ready for APPEND (F6) — or a rejection (malformed) |
| **Malformed input** | a unit missing a required field, using a `status`/`event-kind` outside the closed sets where they are closed, or carrying an update-in-place field ⇒ **rejected**, recorded as a malformed-unit event; never coerced |
| **Failure** | a unit that cannot be well-formed is not appended; capture is not blocked on classification (`MEL-REQ-020`) — the minimal Event lands, enrichment is later Claims |
| **Authoritative** | the Chronicle sequence of units is **the single source of truth** (AP-5); every view is derived and disposable |
| **Independently verifiable** | yes — a schema-conformance check (does every unit carry the common fields; are closed-set fields in range; is there any update-in-place field); a provenance-reachability check; a "no stored type outside the three" check over the whole store |

---

## 11. Enforceability summary

| Invariant | Class |
|---|---|
| exactly three unit kinds + registry; no fourth stored type for a derived concept | **[SEN]** |
| every unit carries provenance + (Claim/Intention) status + bitemporal coords | **[SEN]** |
| `transaction-time` and `unit-id` write-once; no update-in-place field | **[SEN]** |
| no "fact" shape; `status` a closed set; `Intention.status` a closed set | **[SEN]** |
| `proposed → authorised` only via an AUTHORISE artefact (autonomy triad in data) | **[SEN]** |
| "change the past" = append with earlier valid-time; old retained | **[SEN]** |
| deletion = redaction Event; shell + "redacted by user at T" retained | **[SEN]** rule / **[ID]** cascade completeness |
| structural relations (`brings-about`/`ends`/`supersedes`/`part-of`/`same-as`) are load-bearing but modelled as Claims; any index over them is a rebuildable view | **[SEN]** |
| governance is NOT a Chronicle unit | **[SEN]** |
| OQ-M1 / OQ-M2 / OQ-M3 / OQ-M5 / OQ-M4 / OQ-M11 | **[OPEN]** — both alternatives preserved; format is additive |
| provenance chains are complete on real data | **[ETL]** |

---

## 12. Deferred (STORAGE / model)

- The storage engine, on-disk format, encoding, indexing, query language.
- Whether the current-state cache is in-memory / on-disk / hybrid; eager vs lazy
  (AU-1).
- The exact structural-relation vocabulary (OQ-M10 — provisional).
- The `content` representation for descriptive / relational / about-a-claim
  content.
- Resolution of OQ-M1 / OQ-M2 / OQ-M3 / OQ-M4 / OQ-M5 / OQ-M9 / OQ-M11 (model
  phase, informed by prototyping — not this mission).

## 13. Traceability

| Element | Source |
|---|---|
| four primitives; `Context` derived; `State`/`Relation` folded (open) | model §3, §4, §5, §7; INV-4 |
| everything is a Claim with provenance + epistemic status + bitemporal coords | INV-4; AP-4; `MEL-REQ-093`, `096`, `131` |
| append-only history; nothing overwritten; deletion is a separate act | INV-5; AP-5; `MEL-REQ-160`, `162`, `170`; model §11 |
| bitemporal valid-time / transaction-time; the four historical-truth queries | model §6; Snodgrass / SQL:2011 `[E]` |
| MELFINA's state = same primitives, `holder = melfina` | model §12; P-10 |
| governance is not in the model | model §12.1; INV-6 |
| structural relations mechanically load-bearing (L2) | adversarial review §5 L2, §8 |
| streak-like structure must not be the easy path | OQ-M12; `MEL-AR-02` |
| data open, documented, outlives the software, fully deletable | `MEL-REQ-160`, `161`, `162` |
