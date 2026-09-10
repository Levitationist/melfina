# MELFINA — MODEL OPEN QUESTIONS

Companion to `model/HUMAN_CENTRAL_MODEL.md` and `model/MODEL_ALTERNATIVES.md`.
Recorded so no later phase treats them as settled.

**Classes:** **S** unresolved semantic question · **P?** may be a primitive, not
proven · **D→** intentionally left derived (with the risk noted) · **proto**
needs a prototype to answer · **use** needs real-world use to answer ·
**research** needs a future research pass.

IDs: `OQ-M1 … OQ-M12`. Cross-referenced to `HUMAN_CENTRAL_MODEL.md` sections and to
`requirements/` where relevant.

---

## OQ-M1 — Is `Relation` a fifth (or sixth) primitive?
- **Class:** S / P?
- **Question:** The leading model folds relationships into `Claim`s of relational
  content ("X ρ Y", carrying their own time/provenance/confidence — P-9). Model B
  (`MODEL_ALTERNATIVES.md`) argues relationships are as fundamental as objects
  (mission §15) and deserve primitive status. Does "relation = a kind of claim"
  hold up for heavily relational structures (a full musical score; a large
  dependency graph; a concept map), or does it force awkward claim-clusters?
- **What's at stake:** primitive count (4 → 5/6); whether the model reads as
  "graph-shaped" (which risks implying a graph DB — mission §23).
- **Resolve by:** `proto` — build the music and software test cases at data level
  and see whether claim-of-relational-content is natural or strained.
- **Refs:** HCM §5, §14.5; MODEL_ALTERNATIVES Model B.

## OQ-M2 — Is `State` a fifth primitive?
- **Class:** S / P?
- **Question:** The leading model has no first-class "current world state" object;
  "what holds now" = the set of currently-valid descriptive `Claim`s (§3.3, §13).
  Model C argues `State` should be primitive (it is where "what needs attention",
  planning preconditions/effects, and "intentions are desired states" live most
  naturally). Is the derived-query approach expressive and cheap enough in
  practice — especially for the personal-assistant "what requires attention" read
  (mission §9) and for autonomous planning (MEL-REQ-100)?
- **What's at stake:** primitive count; how naturally planning and
  "attention" surfacing fall out; whether states-with-no-known-cause are graceful.
- **Resolve by:** `proto` — implement "what holds now" and "what needs attention"
  as claim-queries; measure conceptual friction.
- **Refs:** HCM §3.3, §13.5, §14.4; MODEL_ALTERNATIVES Model C.

## OQ-M3 — Recurrence identity: one `Intention` or a series?
- **Class:** S
- **Question:** A routine ("practise scales daily") is modelled as a `recurring`
  `Intention` producing a series of `Event`s (HCM §4.4, §6.3). Is the recurring
  intention *one* persistent thing that spans years, or a *series* of freshly
  instantiated intentions? Does "I skipped Tuesday" attach to the standing
  intention, to a per-day instance, or to nothing (per MEL-REQ-064, lapses are
  normal and unpunished)?
- **What's at stake:** how history of a routine is queried; whether "streak-like"
  structure can even be *represented* (it must not be *surfaced* — MEL-AR-02 — but
  can the model accidentally make it easy?).
- **Resolve by:** `proto` + `use`.
- **Refs:** HCM §6.3, §13; MEL-REQ-064, MEL-AR-02.

## OQ-M4 — How should `confidence` / uncertainty be structured?
- **Class:** S
- **Question:** The model separates *degree of belief* (`confidence` scalar),
  *epistemic mode* (`status`), *conflict* (`contradicts`), and *temporal/identity
  uncertainty* (§9). But `confidence` itself is a coarse scalar. Is a scalar
  enough, or does the model need to distinguish *imprecision*, *ambiguity*,
  *ignorance* (no basis either way), and *disputed* (conflicting bases)? Note
  MEL-REQ-059: "I don't know" must be first-class — is that adequately captured as
  "no sufficiently-confident non-contradicted claim exists"?
- **What's at stake:** the honesty of MELFINA's uncertainty reporting; the
  scientific-reasoning discipline (MEL-REQ-129).
- **Resolve by:** `research` (uncertainty representation literature — imprecise
  probability, Dempster-Shafer, possibility theory) + `proto`.
- **Refs:** HCM §9, §14.2; MEL-REQ-059, 129.

## OQ-M5 — How are worries, intrusive thoughts, ambivalence, and affect represented?
- **Class:** S / P?
- **Question:** Direction of fit (P-7) is binary: `Claim` (mind-to-world) vs
  `Intention` (world-to-mind). Real mental states are mixed — a worry is neither a
  confident belief nor a well-formed intention; ambivalence holds opposing
  intentions; emotion colours everything. Given the user profile (research:
  emotion dysregulation, OCD intrusive thoughts, "not-just-right" experiences —
  `[E]`), is "self felt X" as a `Claim` (HCM §14.6) sufficient? Should there be a
  light **"concern" / "salient-but-unresolved"** notion distinct from `Intention`?
  This connects to MEL-REQ-026 (open loops) and MEL-REQ-058 (reassurance
  patterns) — an intrusive worry must be *holdable* without becoming a task or a
  tracked item.
- **What's at stake:** whether MELFINA can serve the user's actual inner life
  without medicalising it (P-3 forbids reifying diagnostic categories) or
  gamifying it.
- **Resolve by:** `research` (affect representation; clinical models of worry that
  don't pathologise) + **user review** — this is exactly the kind of thing the
  user should weigh in on.
- **Refs:** HCM §14.3, §14.6; research §5, §7.2; MEL-REQ-008, 026, 058.

## OQ-M6 — What computes relevance / context, and how?
- **Class:** D→ (mechanism, deferred) / research
- **Question:** The model says context is a relevance-ranked derivation (§7,
  Sperber & Wilson `[E]`) but says **nothing** about how "cognitive effect" and
  "processing effort" are scored, how far relation paths are followed, or how the
  active `Intention` weights the ranking. This is deliberately out of scope
  (P-12) — but it is the crux of the dynamic self-directed MELFINA
  (MEL-REQ-210–213).
- **What's at stake:** whether dynamic context selection is auditable and
  predictable (MEL-REQ-213, 253) depends on this being a *legible* computation.
- **Resolve by:** SYSTEM DESIGN / CORE ENGINE + `research` (retrieval, salience,
  spreading activation, the ACT-R activation equation as a reference).
- **Refs:** HCM §7, §12.1; requirements PART IV-B; OQ-11, OQ-16 in
  `requirements/OPEN_QUESTIONS.md`.

## OQ-M7 — Does "what matters" need representation beyond `Intention` + relevance?
- **Class:** S
- **Question:** The model's handle on "what matters to the user" (mission §9) is
  `Intention`s plus relevance weighting. But things matter without an intention
  attached — a relationship, a value, an identity, a place. Is a **"value" /
  "standing concern"** notion needed, or is it adequately covered by
  `stipulated` `Claim`s ("self cares about honesty") + long-lived relations +
  `maintenance` `Intention`s?
- **What's at stake:** whether MELFINA's sense of the user's priorities is rich
  enough, or flattens everything into goals.
- **Resolve by:** **user review** + `use`.
- **Refs:** HCM §14.9; MODEL_ALTERNATIVES Model D weaknesses.

## OQ-M8 — Is four genuinely *minimum sufficient*?
- **Class:** proto / use
- **Question:** The model argues 3 is insufficient (D) and 5/6 is not clearly
  necessary (OQ-M1, OQ-M2). But "sufficient" is an empirical claim about whether
  the four primitives + emergent patterns actually carry real life without
  contortion. Only building against real data answers it.
- **Resolve by:** `proto` (implement all test cases + the user's actual current
  situation at data level) then `use`.
- **Refs:** HCM §3.3, §15; mission §4, §26.

## OQ-M9 — Are `self` and `melfina` ordinary `Entity`s, or special?
- **Class:** S
- **Question:** The model treats the user (`self`) and `melfina` as `Entity`s of
  special kinds, and uses `holder`/`owner`/`attributed-to` = `self`/`melfina`
  everywhere. Should they instead be *the two fixed perspectives* of the model
  (like the two agents in a two-agent epistemic logic), with everything else
  entities? Does anything break if a third "agent perspective" is ever needed
  (a co-owner? a future multi-user variant — explicitly out of scope, MEL-REQ-003,
  but worth not foreclosing conceptually)?
- **Resolve by:** SYSTEM DESIGN; low urgency.
- **Refs:** HCM §4.1, §10, §12; MEL-REQ-003.

## OQ-M10 — The structural relation vocabulary (HCM §5) is provisional.
- **Class:** S
- **Question:** The ~15 structural relations (`part-of`, `participates-in`,
  `derived-from`, `evidence-for`, `prerequisite-of`, `contributes-to`,
  `depends-on`, `supersedes`, `contradicts`, …) were chosen by walking the test
  cases. Is the set complete? Minimal? Are any of them decomposable into others
  (e.g. is `evidence-for` just `derived-from` + `about`)? Where exactly is the
  line between "structural" (model-defined, carries semantics) and "open"
  (user/MELFINA-defined verb phrases)?
- **Resolve by:** `proto` (does any test case need a structural relation not on
  the list?) + review.
- **Refs:** HCM §5.

## OQ-M11 — How does the model represent *norms* and *permissions* it must not enforce?
- **Class:** S
- **Question:** Permissions/authorisations are descriptive `Claim`s about a
  normative state ("melfina is authorised re: K") — *represented, not enforced*
  (HCM §10, §12.1). The enforcement mechanism and the meta-invariant rules
  (MEL-REQ-235) are deliberately **outside** the model. But: does the model need a
  distinct `status` (e.g. `normative`) for these claims to keep them from being
  confused with factual claims? And how does the model reference the external
  permission gate without containing it?
- **Resolve by:** SYSTEM DESIGN + LOW-LEVEL FOUNDATIONS (the external monitor);
  the model side is a small semantic clarification.
- **Refs:** HCM §10, §12.1; MEL-REQ-235, MEL-AR-18.

## OQ-M12 — Does the model over- or under-serve the "graceful degradation" and "lapses are normal" requirements?
- **Class:** use
- **Question:** MEL-REQ-015, 047, 064 require that gaps, lapses, abandoned
  intentions, and low-capacity periods carry no penalty and no guilt surface. The
  model represents an abandoned `Intention` as `status abandoned` (neutral), a
  gap as simply an absence of `Event`s, and a lapsed routine as a series that
  paused. Is there any way the *structure* of the model makes shortfalls
  computationally salient in a way that would tempt a future interface to surface
  them? (The model must not make "you missed 4 days" the path of least
  resistance.)
- **Resolve by:** `use` + INTERFACE-phase vigilance.
- **Refs:** HCM §4.4, §11; MEL-REQ-015, 047, 064; MEL-AR-02, MEL-AR-07.

---

## Concepts that may be primitives but are not (yet) proven

| Concept | Currently modelled as | Promotion trigger |
|---|---|---|
| `State` (what holds now) | set of currently-valid descriptive `Claim`s | OQ-M2 — if the derived query is too central/awkward |
| `Relation` | `Claim` of relational content | OQ-M1 — if relational domains strain it |
| `Concern` / worry / salient-unresolved | a `Claim` (`status open`) or an `Intention` with no plan | OQ-M5 — if the user's inner life needs it |
| `Value` / standing concern | `stipulated` `Claim`s + long relations | OQ-M7 — if "what matters" flattens to goals |
| `Norm` / permission | descriptive `Claim` about a normative state | OQ-M11 — likely just a `status`, not a primitive |

## Concepts intentionally left derived (and the risk)

| Concept | Risk of leaving it derived |
|---|---|
| context | none — must be derived (relevance theory `[E]`; MEL-REQ-210) |
| history | none — the append-only sequence is the definition |
| process | low — a "process pattern" is a genuine shape, not a lost primitive |
| task / project / habit / goal / calendar / dashboard | **the point** — these must NOT be primitives (MEL-REQ-007); risk is that an interface reintroduces them as if they were, drifting the model |
| strategy / reasoning approach | low, and deliberate — keeping it a `Claim` is what stops the model freezing today's agent tech (mission §18) |
| duration estimate | none — it is a `Claim` with reference-class provenance (MEL-REQ-033) |

## Questions requiring prototypes (`proto`)

OQ-M1, OQ-M2, OQ-M3, OQ-M8, OQ-M10 — all resolvable by building the test cases
(and the user's real current situation) at the data level and observing friction.
**No prototype is built in this phase** (mission §27) — these are queued for
whenever prototyping is authorised (LOW-LEVEL FOUNDATIONS / EXPERIMENTS).

## Questions requiring real-world use (`use`)

OQ-M3, OQ-M7, OQ-M8, OQ-M12 — and the meta-question: **does E²CI actually hold
this user's life well?** No amount of modelling answers this; only REAL-WORLD USE
+ ITERATION does (mission §27).

## Questions requiring a future research pass (`research`)

- OQ-M4 — uncertainty representation (imprecise probability, Dempster-Shafer,
  possibility theory, and how each fits a *personal* system).
- OQ-M5 — non-pathologising representations of worry / rumination / ambivalence /
  affect; clinical models that describe without diagnosing.
- OQ-M6 — relevance / salience computation (spreading activation, ACT-R
  activation, learned retrieval) — but this is a CORE ENGINE research question,
  not a model one.

## Deliberately undecided — implementation (unchanged from `requirements`)

Language · storage substrate · file format · relational vs graph vs document vs
triple vs hypergraph · indexing · the relevance engine · the permission-gate
mechanism · interaction medium · reasoning components. **None decided. None
implied. The model must survive any choice** (mission §23).
