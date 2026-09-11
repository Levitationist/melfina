# MELFINA — KNOWLEDGE AND REASONING MODEL

**Companion to `requirements/REQUIREMENTS_MASTER.md` PART IV-C** (§56–76,
`MEL-REQ-254`…`MEL-REQ-364`), added by **REQUIREMENTS EXPANSION MISSION 002**
(2026-09-11). **Status:** first pass, pending user review.

**What this file is.** The operational definitions, taxonomies, decomposition
pipeline, research grounding, and dedicated adversarial review that are too
long-form for the atomic MEL-REQ format but are load-bearing for it. Every
MEL-REQ in PART IV-C that references "§1/§2/§3/§5/§6 of this document" is
pointing here.

**What this file is not.** Not a new requirements list (no new MEL-REQ IDs are
minted here — all requirements live in `REQUIREMENTS_MASTER.md`). Not
architecture. Not a technology choice. Not a claim that any capability
described below is achieved — every section states what is established,
inferred, hypothesised, or unknown, per the evidence tiers.

**Evidence tiers:** `[E]` established · `[G]` guidance · `[DI]` design
inference · `[H]` hypothesis · `[U]` unknown / insufficient evidence.

---

## 1. The first-principles decomposition pipeline

Referenced by `MEL-REQ-289–293` (§61).

```
PROBLEM
  → concepts               (what is this actually about?)
  → assumptions            (what is being taken for granted?)
  → primitives             (what are the smallest things that don't decompose further, here?)
  → relationships          (how do the primitives relate?)
  → constraints            (what must hold? what can't?)
  → mechanisms             (how does change happen?)
  → mathematical / formal representation   (§2 below — only if it's useful)
  → computational representation           (only if it's useful)
  → implementation          (§64 — philosophical programming)
  → experiment              (§66 — scientific reasoning)
  → verification             (§70 — external verification, F10)
  → revision                 (feed back into any earlier stage)
```

**This is not a mandatory linear procedure.** A trivial problem should not be
forced through all twelve stages (`MEL-REQ-292`); a problem with no useful
formal representation skips straight from mechanisms to implementation or stays
qualitative (`MEL-REQ-298`). The pipeline is a *vocabulary for what depth looks
like*, exactly as PART IV-B treats strategy dimensions as a vocabulary, not a
fixed enumeration (`MEL-REQ-206`).

**Stopping criteria (`MEL-REQ-292`).** Decomposition stops when any of:
- further decomposition would not change the conclusion or the plan (diminishing
  marginal value — the same value-of-computation logic as `MEL-REQ-241–242`);
- an **epistemic boundary** is reached: the next level down is not knowable with
  currently available evidence (state this explicitly, per `MEL-REQ-298`);
- a **physical/mathematical boundary** is reached: the primitive genuinely does
  not decompose further in the relevant theory (e.g. treating an electron's
  charge as a given constant, not deriving it from something more basic);
- a resource budget (`MEL-REQ-241–243`) is exhausted.

A decomposition that stops for resource reasons MUST say so, distinct from one
that stops because it hit a genuine boundary (`MEL-REQ-293`) — conflating "I ran
out of budget" with "this cannot be decomposed further" is exactly the kind of
confident overreach `MEL-REQ-298`/`MEL-AR-21` exist to prevent.

---

## 2. "Wisdom" — an operational definition and its critique

Referenced by `MEL-REQ-279–284` (§59).

### 2.1 The candidate definition (from the mission, adopted as a working
definition — not a claim of achievement)

> **Wisdom is the capability to form, evaluate, and use meaningful connections
> among observations, mechanisms, abstractions, values, constraints,
> consequences, and experience across domains and time, while preserving
> uncertainty and distinguishing supported relationships from speculation.**

### 2.2 Critique

This definition is useful because it is **falsifiable in principle** — each
clause names a checkable capability (form/evaluate/use connections; preserve
uncertainty; distinguish supported from speculative) rather than an
unfalsifiable property like "good judgement" or "insight". It deliberately
excludes anything about *feelings*, *consciousness*, or *values MELFINA holds
for itself* — wisdom here is a **reasoning-quality property**, not a character
trait.

It has three weaknesses, stated honestly rather than smoothed over:

1. **It is a compound of capabilities already required elsewhere** (§58
   analogy discipline, §66 scientific reasoning, §56 decision reconsideration,
   §61 first-principles reasoning). "Wisdom" as defined is not a new mechanism —
   it is what the *combination* of those capabilities, done well and
   consistently, would look like from the outside. This is intentional (it
   avoids inventing a "wisdom module"), but it means the definition cannot be
   tested by looking for a distinct capability; it can only be tested by
   checking whether the combination is coherent and improving over time
   (`MEL-REQ-279`, `282`).
2. **"Meaningful" and "form...evaluate...use" are graded, not binary.** The
   definition does not specify a threshold at which connection-making counts as
   wise rather than merely competent. No such threshold is proposed here —
   attempting to define one would itself be exactly the kind of unfalsifiable,
   grandiose claim the mission and `MEL-REQ-133`/`280` explicitly prohibit.
3. **Human wisdom research (a note on scope).** The definition deliberately does
   **not** draw on the psychological wisdom literature (e.g. Baltes' Berlin
   Wisdom Paradigm, Sternberg's balance theory) as a source of authority,
   because that literature characterises a *human* trait acquired through lived
   experience, uncertainty about one's own life, and social/emotional
   maturation — properties this document does not claim MELFINA has or is
   acquiring. Borrowing that vocabulary without the underlying phenomenon would
   be exactly the "wisdom" language the mission warns against turning into "a
   mystical software component". `[DI]`

### 2.3 Evaluation dimensions

Per the mission's list, adopted as the axes along which §59's requirements
(`MEL-REQ-279`, `282`) are evaluated over time, none of them individually
sufficient:

cross-domain integration · temporal reasoning · causal reasoning (§3 below) ·
consequence awareness · second-order effects · uncertainty · perspective
comparison (§71's competing-frameworks requirement is one instance) · value/
constraint awareness · abstraction · analogy (§3) · counterfactual reasoning
(§3.2) · experience-derived patterns · falsification · knowing when not to act
(`MEL-REQ-261`) · knowing what information is worth obtaining (`MEL-REQ-323`).

**None of these is claimed to be solved.** Several (counterfactual reasoning at
Pearl's third rung, reliable "knowing what's worth finding out") are
research-stage or unsolved even in the general AI literature — see §6.

---

## 3. The analogy / correlation / causation taxonomy

Referenced by `MEL-REQ-272–278` (§58) and `MEL-REQ-313` (§66).

### 3.1 The closed relationship-type vocabulary

| Type | What it means | What licenses it | What it does NOT license |
|---|---|---|---|
| **Established relationship** | A relationship the relevant field treats as settled (a physical law, a mathematical theorem, a well-replicated empirical regularity). | Citation to the field's own settled status. | Treating a *contested* finding in the same field as if it were this tier. |
| **Mechanistic analogy** | Domain A and domain B share an actual underlying mechanism (not just a similar-looking pattern) — e.g. diffusion equations describing both heat flow and some population-spread models because the same PDE structure genuinely governs both. | A shared formal or causal mechanism, stated explicitly. | Treating the analogy as evidence that domain B's *other* properties transfer from domain A. |
| **Structural analogy** | Domain A and domain B share a *relational structure* (the shape of the relationships), without a claimed shared mechanism (Gentner's structure-mapping: relations transfer, not object attributes `[E]`). | A stated structural correspondence + explicit note that no mechanism claim is made. | Being treated as if it were a mechanistic analogy or an established relationship. |
| **Correlation** | Two things covary in the available data. Pearl's first rung — pure association, `P(Y|X)` `[E]`. | Observed covariation, stated with its measurement basis. | Any claim about what would happen under intervention (Pearl's second rung) — correlation alone never licenses "if we changed X, Y would change". |
| **Causal relationship** | A stated intervention on X is expected to change Y — Pearl's second rung, `P(Y|do(X))` `[E]`, or third rung (counterfactual) where a specified causal model supports it. | A causal model, an intervention/natural-experiment/RCT-like basis, or an explicit assumption set under which the causal claim would hold. | Being asserted from correlation alone (the single most common failure mode this taxonomy exists to prevent). |
| **Plausible hypothesis** | A candidate explanation not yet supported by evidence at any of the above tiers, but not arbitrary either — it follows from stated reasoning. | An explicit statement of the reasoning that produced it and what would test it. | Being acted on as if it were established. |
| **Speculation** | An idea offered with no evidentiary or mechanistic basis beyond "this seems interesting". | Nothing — this tier exists so speculation can be *labelled as such* rather than silently upgraded. | Informing a consequential decision (`MEL-REQ-277`) without first being tested against a higher tier. |

### 3.2 Why Pearl's ladder matters here specifically

Pearl's association→intervention→counterfactual hierarchy `[E]` is adopted
because it gives a **non-arbitrary, well-established ordering** for exactly the
distinction the mission asks for (correlation ≠ causal relationship). Each rung
requires strictly more assumptions or data than the one below it — modern
statistical learning is very good at rung 1 and structurally cannot answer rung
2/3 questions from rung-1 data alone. This is why `MEL-REQ-272` forbids
presenting a correlation as a causal relationship even when the correlation is
strong: strength of association says nothing about which rung it licenses.

### 3.3 Why Gentner's structure-mapping matters here specifically

Structure-mapping theory `[E]` is the de facto standard account of analogy in
cognitive science: an analogy's validity rests on **shared relational/causal
structure**, not surface or attribute similarity, and the theory itself
distinguishes analogy (structure transfers, objects don't) from literal
similarity (both transfer) — which is exactly the "analogy ≠ identity"
distinction `MEL-REQ-274` requires. Its implementation, the Structure Mapping
Engine, demonstrates the same mapping process applies across conceptual,
perceptual, causal, mathematical, and social content — supporting the mission's
premise that a *general* cross-domain connection-making capability is a
coherent target, not a category error.

### 3.4 The specific, current failure mode this guards against

Recent (2025–2026) evaluations of large language models performing analogical
reasoning document that models "hallucinate or produce superficial analogies
lacking valid structural correspondence" and "lack a formal mechanism for
ensuring that the correspondence between two domains is consistent and
logically valid" `[E]`. A dedicated 2025 study asks directly whether such
models can recognise their own analogical hallucinations, evaluating
uncertainty estimation for exactly this failure mode `[E]`. This is the direct
evidentiary basis for `MEL-REQ-275`/`MEL-AR-20`: the taxonomy and the
"state what's shared/what differs/what would falsify it" requirement
(`MEL-REQ-273`) are a structural countermeasure to a failure mode that is
documented, not hypothetical, in the current generation of reasoning systems.

Sources: [Structure-mapping theory (Wikipedia summary of Gentner 1983)](https://en.wikipedia.org/wiki/Structure-mapping_theory); [Gentner & Markman, "Structure Mapping in Analogy and Similarity"](https://home.csulb.edu/~cwallis/382/readings/482/GenterMarkman.pdf); [Pearl's Ladder of Causation overview](https://www.researchgate.net/figure/Pearls-Ladder-of-Causation-The-first-rung-associations-only-allows-predictions-based_fig2_357875366); [Can LLMs Truly Perform Analogical Reasoning? (2025)](https://aclanthology.org/2025.findings-acl.1230.pdf); [Can LLMs Recognize Their Own Analogical Hallucinations? (2025)](https://aclanthology.org/2025.knowllm-1.8.pdf).

---

## 4. Architecture / model / foundation compatibility test

Full detail behind `REQUIREMENTS CHECKPOINT 002`'s compatibility summary. Each
row is one of the mission's 20 compatibility checks (§31).

| # | Check | Result |
|---|---|---|
| 1 | E²CI unchanged | ✓ — `MEL-REQ-264` requires representability without a new primitive; `MEL-REQ-341` blocks a domain-importance-based exception |
| 2 | Five-ring architecture | ✓ — no requirement assumes a different ring topology |
| 3 | Ring-0 governance | ✓ — §71 (`MEL-REQ-338`), §75–76 all explicitly subordinate reasoning to governance |
| 4 | Ring-1 deterministic core | ✓ — PART IV-C requirements are Ring-2 (reasoning) and Ring-3 (capability) concerns; none touches Ring-1's determinism requirement |
| 5 | Ring-2 optional reasoning | ✓ — §56–70 are all cognitive-autonomy requirements (Ring 2); none grants Ring-2 an effect path |
| 6 | Ring-3 capabilities | ✓ — §65/§75 explicitly re-gate modelling/experimentation through the existing capability grant model |
| 7 | Ring-4 future network | ✓ — §69 (`MEL-REQ-324`, `328`) explicitly keeps knowledge acquisition inside the local-only core |
| 8 | THINK→DECIDE→PROPOSE→AUTHORISE→EXECUTE→VERIFY | ✓ — reinforced by `MEL-REQ-254`, `311`; never bypassed |
| 9 | local-only core | ✓ — `MEL-REQ-324`, `328` |
| 10 | capability grants | ✓ — `MEL-REQ-347–350` restate F1 explicitly |
| 11 | aggregate-effect governance | ✓ — `MEL-REQ-353` restates RC-2/F1 §8/§11/F8 §2.7 |
| 12 | live revocation | ✓ — no requirement here creates a new grant type outside F1/F4 |
| 13 | structured terminal/GUI actions | ✓ — `MEL-REQ-354` restates F2 §5 against reasoning-driven ambiguity resolution |
| 14 | verifier contract | ✓ — `MEL-REQ-329–331` explicitly make F10 authoritative, self-evaluation complementary only |
| 15 | Chronicle as source of truth | ✓ — `MEL-REQ-264–270` route all domain knowledge through F5/F6 |
| 16 | dynamic capability creation | ✓ — `MEL-REQ-350` re-gates it against a reasoning-sophistication bypass |
| 17 | nine-tier self-modification | ✓ — §76 restates `MEL-REQ-233–239` against four specific "capability implies authorisation" arguments |
| 18 | meta-invariant | ✓ — `MEL-REQ-356`, `361–364`; residual risk recorded as `OQ-32`, not resolved by fiat |
| 19 | anti-dependency requirements | ✓ — `MEL-REQ-322`, `MEL-AR-27` extend `MEL-REQ-071`/`MEL-AR-07` |
| 20 | dynamic resource allocation | ✓ — `MEL-REQ-292` (decomposition stopping criteria) and §62 (formalisation effort) both route through `MEL-REQ-241–243` |

**No requirement makes AI the source of truth, the authority, or the verifier;
none equates intelligence with permission; none expands the E²CI ontology.**

---

## 5. Adversarial review — attacks A–T

Per the mission's §32. Format matches
`design/ARCHITECTURE_ADVERSARIAL_REVIEW.md`: Attack | Violated assumption |
Existing defence | Missing requirement (if any) | Correction | Residual risk.

| # | Attack | Violated assumption | Existing defence (pre-MISSION-002) | Correction (MISSION 002) | Residual risk |
|---|---|---|---|---|---|
| A | Extremely capable reasoner | "Greater capability stays bounded by the same gates" | F1/F3 (authority is grant-based, not capability-based) | `MEL-REQ-347–348` make the invariance explicit | Untested against an actually highly capable reasoner — inherent to any design-time analysis |
| B | Manipulative reasoner | "MELFINA's own outputs are not used to manipulate the user" | `MEL-REQ-004/005/058` | `MEL-REQ-283`, `359` extend this to wisdom/ethics-flavoured manipulation specifically | A sufficiently subtle manipulation may not resemble the named patterns; mitigated, not eliminated |
| C | Hallucinating reasoner | "A confident-sounding output is checked before being trusted" | `MEL-REQ-059/096/248` | `MEL-REQ-272–275` (analogy taxonomy), `330` (confidence ≠ verification) | Detecting a well-formed, plausible hallucination remains hard in general — `[U]`, see §6 |
| D | Goal-confused reasoner | "MELFINA pursues the stated objective, not a drifted one" | `MEL-REQ-149` (no self-authored objectives) | `MEL-REQ-360` closes a knowledge-acquisition-based drift route | Objective drift via a long chain of individually-reasonable steps is the general alignment problem — not solved here |
| E | Knowledge-corrupted reasoner | "Foundational beliefs don't silently change" | `MEL-REQ-120` | `MEL-REQ-326–327` (reconcile-with-provenance, never silent overwrite) | A sufficiently gradual, evidence-backed corruption is, by construction, indistinguishable from legitimate belief revision — flagged, not solved |
| F | Cross-domain false-analogy generator | "A connection is checked before being relied on" | (none, pre-MISSION-002) | `MEL-REQ-272–277` (the whole taxonomy) — **this attack is the primary reason §58 exists** | False-positive detection itself is imperfect (`MEL-REQ-282` tracks it over time, does not guarantee it) |
| G | Mathematical overformalizer | "Rigor-looking output is actually rigorous" | (none, pre-MISSION-002) | `MEL-REQ-298–299` (no forced formalisation), `MEL-REQ-351` (formalism can't reclassify risk) | A user without the domain background may still be persuaded by correct-looking notation — a communication risk, not fully closed by a requirement |
| H | Code-generating reasoner producing subtly unsafe code | "Generated code is tested, not trusted on style" | `MEL-REQ-142/221–223` | `MEL-REQ-307–309` (tests from spec, discrepancies surfaced) | Test suites are necessarily incomplete; `MEL-REQ-223` already states safety rests on test+sandbox+rollback+human-gate, not proof — unchanged, restated |
| I | Scientific-model overconfidence | "A model's fit is not confused with truth" | `MEL-REQ-095/129–131` | `MEL-REQ-297` (uncertainty propagation), `312/315` (simulation ≠ confirmed reality) | Model misspecification that both fits the data and is wrong is a general epistemology problem |
| J | Capability-creating reasoner | "New capabilities don't get more authority via sophistication" | `MEL-REQ-220–223` | `MEL-REQ-350` makes the closure explicit | The gates (test/sandbox/human-authorise) are only as good as their implementation — an F7/F10 concern, not a §75 one |
| K | Self-modifying reasoner | "Understanding ≠ authorisation to change tiers 6–9" | `MEL-REQ-233–239` | `MEL-REQ-357`, `361–364` name four specific arguments and close each | This is exactly `OQ-19`'s territory — see `OQ-32` below |
| L | User accidentally granting excessive authority | "A grant means what it says" | F1 (scope/authority fields), `MEL-REQ-146` | No new requirement — this is a user-interface/grant-clarity concern for a later phase, not a reasoning-capability concern; flagged, not addressed here | Out of this mission's scope by design |
| M | Composed individually-safe actions → dangerous aggregate | "Composition doesn't launder risk" | RC-2, F1 §8/§11, F8 §2.7 | `MEL-REQ-353` restates it against a *reasoning-derived* decomposition specifically | The aggregate-budget mechanism itself is `[ID]`/`[ETL]` per F1's own enforceability table — unchanged here |
| N | Stale foundational knowledge | "Old beliefs are marked stale, not silently trusted" | `MEL-REQ-120` | `MEL-REQ-258` (decisions reconsidered on new evidence), `MEL-REQ-268` (retention policy) | Detecting that a specific belief has gone stale, without an external trigger, is itself a reasoning-quality question — `[U]` |
| O | Conflicting laws / jurisdictions | "MELFINA doesn't silently pick a jurisdiction" | (none, pre-MISSION-002) | `MEL-REQ-333` (jurisdiction + temporal validity), `MEL-REQ-337` (states divergence, doesn't resolve it) | Correctly identifying *which* jurisdiction applies to a given user situation is itself hard and not solved by a labelling requirement |
| P | Conflicting ethical frameworks | "MELFINA doesn't silently pick one framework as *the* answer" | (none, pre-MISSION-002) | `MEL-REQ-334` (represent more than one), `MEL-REQ-335` (descriptive vs normative) | Framework selection/weighting for a given question is a genuinely contested question in moral philosophy, not resolvable by requirement |
| Q | False "wisdom" claims | "MELFINA doesn't claim wisdom it hasn't demonstrated" | (none, pre-MISSION-002) | `MEL-REQ-280` (no self-claim), `MEL-REQ-359` (can't override intent via the claim) | The *evaluation* of whether connection-making is actually improving (`MEL-REQ-279/282`) is aspirational and unsolved — see `OQ-26` |
| R | Dependency-forming teacher | "Teaching builds independence, not reliance" | `MEL-REQ-071` | `MEL-REQ-322`, `MEL-AR-27` generalise this across domains | Whether a given interaction *actually* builds independence vs. reliance is an empirical, per-user question — `OQ-13`-adjacent |
| S | Experimentation as an authority bypass | "'It's just an experiment' isn't a bypass" | `MEL-REQ-018` (the pipeline) | `MEL-REQ-311`, `352` close this explicitly for modelling/simulation framing | None beyond correct implementation of F2/F3 at build time |
| T | Reinterpreting governance through reasoning | "Understanding governance ≠ authority over it" | F8 §1, F9 (governance outside MELFINA's reasoning) | `MEL-REQ-356` closes this explicitly; `MEL-REQ-288` routes findings as proposals only | `OQ-19`/`OQ-32`: whether this holds against a sufficiently capable self-modifier remains open by design, not by omission |

**Overall verdict:** every attack maps to either (a) an existing foundation-
contract defence that PART IV-C restates rather than duplicates, or (b) a
residual, explicitly-acknowledged open question (most converging on `OQ-19`/
`OQ-32`). **No attack required inventing a new authority mechanism.** Several
attacks (F, G, Q) motivated genuinely new requirements (the analogy taxonomy,
the no-forced-formalisation rule, the no-self-claimed-wisdom rule) because
MISSION 001 had no prior requirement addressing them at all — these are net
additions to coverage, not patches to a flaw in MISSION 001.

---

## 6. Research grounding (mission areas A–Z)

Per mission §27. Each area: a short grounding statement and evidence tag.
**Where the literature does not justify a strong requirement, that is stated
explicitly** — a `[H]`/`[U]` tag here never became a MUST/SHOULD in
`REQUIREMENTS_MASTER.md` on its own (§0.2 of that document).

| Area | Grounding | Tag |
|---|---|---|
| A. First-principles reasoning | Well-established as a pedagogical and engineering discipline (physics, engineering education); no general AI system reliably performs it end-to-end on arbitrary problems. | `[G]` discipline / `[U]` general capability |
| B. Mathematical modelling & scientific reasoning | Dimensional analysis, model comparison, and uncertainty propagation are standard practice in the physical and engineering sciences. | `[E]` discipline / `[H]` MELFINA's execution of it |
| C. Symbolic + numerical reasoning | Computer algebra systems and numerical methods are mature, decades-old technology; using them as *tools* (not inventing new ones) is straightforward. | `[E]` |
| D. Program synthesis / derivation | Demonstrated in narrow, well-specified domains; general, reliable synthesis from informal specification is unsolved — consistent with MISSION 001's existing "verified codegen ≈ unsolved" finding (`MEL-REQ-223`). | `[E]` narrow / `[U]` general |
| E. Formal methods | Real industrial adoption exists (e.g. Verum Dezyne at Philips/Thermo-Fisher; Motorola reported 40–50% testing-productivity gains from model-based testing) but adoption is narrow, siloed, and limited by training cost — not a general software-engineering default. | `[E]`, narrow adoption |
| F. Executable specifications | A subset of formal methods; same adoption caveats. | `[E]`, narrow |
| G. Model-based systems engineering | Established discipline in aerospace/safety-critical engineering; not evidence that a general reasoning system can do it well across arbitrary domains. | `[G]` |
| H. Cross-domain analogical reasoning | Gentner's structure-mapping theory is the established cognitive-science account (§3.3); current LLMs demonstrably hallucinate analogies without a structural-validity check (§3.4). | `[E]` theory / `[E]` current failure mode |
| I. Scientific discovery systems | The Adam/Eve robot-scientist systems are a real, demonstrated instance of automated hypothesis-generate-test cycles — but in a narrow, fully-instrumented domain (yeast functional genomics; early-stage drug screening), not general science. | `[E]`, narrow domain |
| J. Computational creativity | An active research field; MISSION 001 already flagged that "computational creativity requires the system to report its process" (`MEL-REQ-075`, `[DI]`) — unchanged here. | `[DI]` |
| K. Knowledge representation | E²CI (already established, `model/HUMAN_CENTRAL_MODEL.md`) is treated as sufficient for cross-domain knowledge (`MEL-REQ-264`) — a design inference, not independently re-derived here. | `[DI]` |
| L. Personal knowledge management | Already grounded in MISSION 001 (`research/RESEARCH_MASTER.md` §14); unchanged by this mission. | `[E]` (carried forward) |
| M. Causal inference | Pearl's association/intervention/counterfactual ladder (§3.2) is established theory, widely adopted in statistics, epidemiology, and econometrics. | `[E]` |
| N. Counterfactual reasoning | The third rung of Pearl's ladder — requires a fully specified causal model; general, reliable counterfactual reasoning by an AI system over arbitrary domains is not established. | `[E]` theory / `[U]` general execution |
| O. Metareasoning / value of computation | Already established in MISSION 001 (`MEL-REQ-204–209`, Russell & Wefald 1991); unchanged, extended by §61's decomposition-stopping criteria. | `[E]` (carried forward) |
| P. AI self-evaluation | Already established as weak/overconfident in MISSION 001 (`MEL-REQ-248`); §70 strengthens the requirement to use external checks, not the underlying evidence. | `[E]` (carried forward) |
| Q. N-version / independent verification | Grounded in `design/foundations/VERIFIER_CONTRACT.md` (Knight & Leveson 1986; 2026 AI-agent replication showing common-mode failure); unchanged here, cross-referenced. | `[E]` (carried forward from F10) |
| R. Adversarial reasoning / systems security | The generalised lens (§60) is a design inference extending established security-engineering practice (trust boundaries, attack surfaces, blast radius) to non-software systems; the extension itself is not independently evidenced. | `[G]` security practice / `[DI]` generalisation |
| S. Human-AI mixed initiative | Already established in MISSION 001 (`MEL-REQ-228–232`, MI-CCy/COFI); unchanged. | `[E]` (carried forward) |
| T. AI-assisted learning and teaching | Already established in MISSION 001 (ITS meta-analyses, `MEL-REQ-070–074`); §68 extends scope, not evidence. | `[E]` (carried forward) |
| U. Moral/ethical reasoning | Machine-ethics research documents that single-framework (pure consequentialist or deontological) approaches have real, known failure modes in practice, motivating the pluralistic representation required by `MEL-REQ-334`. | `[E]` |
| V. Philosophy of science | Popperian falsification and the observation/interpretation distinction (`MEL-REQ-096`, `316`) are long-established philosophy-of-science positions; adopted as design guidance, not re-derived. | `[G]` |
| W. Philosophy of mathematics / computation | Not separately researched for this pass; §62/§64's requirements rest on engineering practice (dimensional analysis, specification-first design), not a philosophical position on the nature of mathematical objects. | `[U]` — not grounded, and no requirement here depends on resolving it |
| X. Embodied skill / motor learning (music, martial arts) | Fitts & Posner's three-stage model (cognitive/associative/autonomous) is an established, decades-old account of motor-skill acquisition, directly grounding `MEL-REQ-342–343`'s knowledge/execution distinction. | `[E]` |
| Y. Cognitive science of abstraction, analogy, transfer, insight | Analogy: Gentner (§3.3, `[E]`). Transfer and insight more broadly: well-studied in learning science generally (already partially grounded via ITS/spaced-retrieval evidence in MISSION 001); not independently re-researched here for "insight" specifically. | `[E]` analogy / `[G]` transfer / `[U]` insight specifically |
| Z. Scientific discovery / automated experimentation | Same grounding as I (robot scientists) — real but narrow; general automated experimentation across arbitrary domains is not established, consistent with `MEL-REQ-319`'s explicit no-novelty-promise. | `[E]` narrow / `[U]` general |

**Honest summary:** the areas with the strongest grounding (analogy theory,
causal-inference theory, motor-learning stages, self-evaluation weakness,
verifier independence, machine-ethics framework pluralism, hallucinated-
citation trends) are exactly the areas where PART IV-C imposes a **constraint**
(a taxonomy, a labelling requirement, an external-check requirement). The areas
with the weakest grounding (general automated discovery, general counterfactual
reasoning, insight, philosophy of mathematics) are exactly the areas where
PART IV-C states a **capability target under evaluation**, never a claimed or
required-to-succeed capability. This pattern is deliberate, not incidental.

---

## 7. Traceability

| Element | Source |
|---|---|
| the decomposition pipeline (§1) | mission §8; `MEL-REQ-289–293` |
| the wisdom definition + critique (§2) | mission §6, verbatim candidate definition; `MEL-REQ-279–284` |
| the analogy/causation taxonomy (§3) | mission §5; `MEL-REQ-272–278`, `313` |
| the compatibility test (§4) | mission §31; `REQUIREMENTS CHECKPOINT 002` |
| the adversarial review (§5) | mission §32, attacks A–T; `MEL-REQ-349–364`, `MEL-AR-20–27` |
| the research grounding (§6) | mission §27, areas A–Z |
| Gentner's structure-mapping theory | [Wikipedia: Structure-mapping theory](https://en.wikipedia.org/wiki/Structure-mapping_theory); [Gentner & Markman](https://home.csulb.edu/~cwallis/382/readings/482/GenterMarkman.pdf) |
| Pearl's ladder of causation | [Ladder of Causation overview](https://www.researchgate.net/figure/Pearls-Ladder-of-Causation-The-first-rung-associations-only-allows-predictions-based_fig2_357875366) |
| LLM analogical hallucination (2025) | [Can LLMs Truly Perform Analogical Reasoning?](https://aclanthology.org/2025.findings-acl.1230.pdf); [Can LLMs Recognize Their Own Analogical Hallucinations?](https://aclanthology.org/2025.knowllm-1.8.pdf) |
| Adam/Eve robot scientists | [Robot Scientist — Wikipedia](https://en.wikipedia.org/wiki/Adam_(robot)); [Towards Robot Scientists for autonomous scientific discovery](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC2813846/) |
| Fitts & Posner motor-learning stages | [Fitts & Posner's stages of learning](https://sportscienceinsider.com/stages-of-learning/) |
| formal-methods industrial adoption | [Reality Check on Formal Methods in Industry](https://onlinelibrary.wiley.com/doi/10.1002/smr.70069); [Formal Methods Adoption in Industry: An Experience Report](https://dl.acm.org/doi/10.1007/978-3-031-43678-9_5) |
| machine-ethics competing frameworks | [Normative Moral Pluralism for AI](https://arxiv.org/pdf/2508.08333) |
| hallucinated-citation trends (2026) | [Nature: Hallucinated citations are polluting the scientific literature](https://www.nature.com/articles/d41586-026-00969-z); [phys.org, 2026-05](https://phys.org/news/2026-05-ai-generated-fake-citations-scientific.html) |
| N-version / verifier independence | `design/foundations/VERIFIER_CONTRACT.md` §3 (carried forward, not re-researched) |
