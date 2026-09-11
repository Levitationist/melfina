# MELFINA — OPEN QUESTIONS

Companion to `requirements/REQUIREMENTS_MASTER.md`. Questions the requirements
phase **cannot** answer — deferred to a later phase, to prototyping, or to
real-world use. Recorded so they are not silently decided elsewhere.

Each: the question, why it's open, what class it is, which requirements depend on
it, and where it should be resolved.

**Classes:** **C** design hypothesis · **D** unknown needing experimentation ·
**E** deliberately undecided (a real decision, just not now).

**OQ-1 … OQ-15** are from the base + earlier addenda; **OQ-16 … OQ-21** were added
by the dynamic-self-directed correction; **OQ-22 … OQ-32** were added by
**REQUIREMENTS EXPANSION MISSION 002** (cross-domain knowledge, first-principles
reasoning, wisdom).

---

## OQ-1 — What are MELFINA's primitives? (the life model)
- **Question:** what small set of neutral primitives should the user's life be
  modelled with, such that tasks, open loops, sessions, notes, threads,
  commitments, learning, projects, and music practice all emerge as *cases*?
- **Why open:** this is the HUMAN / CENTRAL MODEL phase's core job; it needs the
  user's own account of how they think, not a literature answer.
- **Class:** C / E.
- **Depends:** MEL-REQ-007, 008, 026, 069, 078, 085, 151 (and ~10 more SHOULDs).
- **Resolve in:** HUMAN / CENTRAL MODEL.

## OQ-2 — How is music practice modelled on the primitives?
- **Question:** how do repertoire, pieces, sections/passages, practice sessions,
  goals, performance preparation, and reflection map onto the OQ-1 primitives
  without a bespoke "music" type?
- **Why open:** depends on OQ-1; also needs the user's own practice workflow.
- **Class:** C.
- **Depends:** MEL-REQ-077–084.
- **Resolve in:** HUMAN / CENTRAL MODEL, then validated in REAL-WORLD USE.

## OQ-3 — Do specific external supports actually help *this adult*?
- **Question:** visual/continuous time representation, if-then (implementation-
  intention) capture, event-anchored reminders, resumption cues, body-doubling-
  style co-presence — do any measurably help self-directed adults with this
  profile? (Research: mechanism plausible; controlled adult evidence sparse to
  absent; if-then has child-ADHD evidence only.)
- **Class:** D.
- **Depends:** MEL-REQ-023, 024, 035, 037.
- **Resolve in:** REAL-WORLD USE + ITERATION (single-user n-of-1 observation).

## OQ-4 — Does "anchored flexibility" resolve the structure↔novelty tension?
- **Question:** is "a few fixed anchor points + free unstructured space between"
  actually better for this user than alternatives, and how should the balance be
  exposed?
- **Why open:** recurs in AuDHD clinical/lived-experience writing; no experimental
  test; individual.
- **Class:** C.
- **Depends:** RC1; MEL-REQ-175.
- **Resolve in:** REAL-WORLD USE.

## OQ-5 — Can a very capable system feel simple to this user?
- **Question:** does "large capability surface behind on-demand invocation + small
  visible surface + capability composition" actually keep MELFINA feeling light and
  low-load, or does complexity just relocate? Does composition reduce *total*
  system complexity or move it?
- **Class:** C / D.
- **Depends:** VC1, VC6; MEL-REQ-128, 194, 199, and the anti-requirement MEL-AR-11.
- **Resolve in:** ARCHITECTURE (complexity analysis) + REAL-WORLD USE (felt load).

## OQ-6 — Is there any safe progress representation for this user?
- **Question:** does *any* form of progress feedback deliver the ADHD near-term-
  salience benefit without the OCD/perfectionism/self-judgement cost? Current
  answer: none demonstrated; "none" is the safe default.
- **Class:** D.
- **Depends:** RC5; MEL-REQ-012, 046.
- **Resolve in:** REAL-WORLD USE, cautiously, only if the user wants to try.

## OQ-7 — What default proactivity level per area?
- **Question:** how proactive should MELFINA be by default in each area (capture
  follow-up, surfacing next actions, commitment follow-through, preparing
  information ahead of need)? Too low = not useful; too high = intrusive /
  surveillance-feeling / demand-provoking.
- **Class:** D.
- **Depends:** VC2, VC5; MEL-REQ-105–109, 178.
- **Resolve in:** REAL-WORLD USE + ITERATION.

## OQ-8 — How is "comparable" judged for reference-class time estimates?
- **Question:** when MELFINA feeds back "similar past activities took X", how does
  it decide two activities are similar enough to compare?
- **Class:** D.
- **Depends:** MEL-REQ-033.
- **Resolve in:** SYSTEM DESIGN + REAL-WORLD USE tuning.

## OQ-9 — Do the AI compulsion-safeguards help without frustrating legitimate use?
- **Question:** does "name the reassurance-seeking pattern + defer to the user's
  own prior recorded decision + refuse to generate another reworded answer + don't
  keep the user in conversation" actually reduce compulsive querying — or does it
  frustrate the user when they have a genuine new question? How is a genuine
  new question distinguished from a re-ask?
- **Class:** D. *(A [H]-derived safeguard that is nonetheless a MUST because the
  downside risk is serious — but its efficacy is untested.)*
- **Depends:** MEL-REQ-058, 104, 186.
- **Resolve in:** REAL-WORLD USE, with the ability to tune sensitivity.

## OQ-10 — How can MELFINA reliably know what it doesn't know?
- **Question:** calibrated abstention ("I don't know / not enough information") is
  a MUST (MEL-REQ-059), but research shows abstention is an *unsolved* problem and
  reasoning-tuning can *degrade* it. What structural mechanisms (verification
  layers, self-consistency, conformal methods, forcing provenance) make it
  acceptable in practice?
- **Class:** D.
- **Depends:** MEL-REQ-059, 098; the whole action pipeline leans on it.
- **Resolve in:** ARCHITECTURE + AI LAYER; mitigated meanwhile by permission +
  reversibility gates (not by trusting the model's confidence).

## OQ-11 — Where is the local reasoning ceiling, and is a remote add-on ever worth it?
- **Question:** which reasoning tasks MELFINA needs can run locally at acceptable
  quality (small models can do multi-step reasoning; frontier general reasoning
  cannot yet run fully local)? For tasks above the ceiling, is a
  separately-installed, isolated, off-by-default remote capability (MEL-REQ-167)
  ever worth its security/privacy cost — or does MELFINA simply decline those
  tasks forever?
- **Class:** D / E.
- **Depends:** VC3; MEL-REQ-014, 155, 167, 198.
- **Resolve in:** ARCHITECTURE + AI LAYER + ITERATION. Default until then: local or
  decline.

## OQ-12 — What is the default consequential/routine action boundary?
- **Question:** which actions are safe to run at HOTL (act-then-show) by default,
  vs which must be HITL (propose-then-wait)? The set of "routine, reversible,
  low-stakes, pre-authorised" actions.
- **Class:** E (a real decision) + D (needs use to calibrate).
- **Depends:** MEL-REQ-018, 019, 102, 144.
- **Resolve in:** SYSTEM DESIGN (initial conservative list) + REAL-WORLD USE
  (adjustment). Default: err toward HITL.

## OQ-13 — How reliable is LLM-based teaching?
- **Question:** ITS evidence (g≈0.6–0.7) is for structured, hand-authored tutoring
  systems. LLM-based teaching is more flexible but unevenly reliable (can
  hallucinate, can misjudge level). Is it good enough to be a SHOULD, and with what
  guardrails (checkable worked examples, "verify this yourself", staying in the
  user's known domains)?
- **Class:** D.
- **Depends:** MEL-REQ-070–076.
- **Resolve in:** REAL-WORLD USE + ITERATION.

## OQ-14 — Where is the line between helpful adaptation and drift?
- **Question:** MELFINA must adapt with the user (MEL-REQ-117) but not drift
  (MEL-REQ-122). These are opposite pressures and the boundary is not sharply
  definable in advance. What observable criteria distinguish them? How is an
  "improvement" (MEL-REQ-147) validated as actually better *for this user* before
  adoption?
- **Class:** D.
- **Depends:** VC4, VC7; MEL-REQ-117–122, 147–150.
- **Resolve in:** REAL-WORLD USE + ITERATION, with versioning + easy rollback as
  the safety net.

## OQ-15 — Does MELFINA actually improve this user's life?
- **Question:** the whole project's success criterion. No amount of research or
  requirements work can answer it.
- **Class:** D.
- **Depends:** everything.
- **Resolve in:** REAL-WORLD USE + ITERATION. This is the point of the pipeline's
  last two stages.

---

## Added by the dynamic-self-directed correction (OQ-16 … OQ-21)

## OQ-16 — Can dynamic strategy selection be made predictable enough for this user?
- **Question:** MELFINA is required to choose its own approach (reasoning depth,
  speed, context breadth, decomposition, verification effort, autonomy-within-
  bounds) per situation (`MEL-REQ-204–209`), *and* to remain predictable and
  non-surprising (`MEL-REQ-253`). How much strategic variability can this user
  tolerate before it reads as erratic / untrustworthy? What makes a strategic
  choice feel "explained" vs "arbitrary" to them?
- **Why open:** individual; "surprise is a cost" for this profile (IU evidence);
  the balance is a lived-experience question.
- **Class:** D.
- **Depends:** VC8, VC11, VC12; `MEL-REQ-204–213`, `251–253`.
- **Resolve in:** REAL-WORLD USE + ITERATION, with a user-set bound on dynamism.

## OQ-17 — How is a genuine capability gap distinguished from the model just preferring to build something?
- **Question:** `MEL-REQ-217` requires recognising when existing capabilities are
  insufficient. LLM self-knowledge is weak and models exhibit "over-helpfulness"
  (building/acting rather than acknowledging limits). What signals reliably
  indicate a real gap (repeated failure on a task class, explicit user need,
  verifiable inadequacy) vs the model's inclination to create?
- **Class:** D.
- **Depends:** VC9; `MEL-REQ-217`, `219`, `220`.
- **Resolve in:** SYSTEM DESIGN (gating rules) + REAL-WORLD USE (calibration).
  Meanwhile: compose-before-code + test + human-authorise for risk.

## OQ-18 — Can a system that grows its own capabilities stay within one-maintainer comprehension?
- **Question:** `MEL-REQ-192` requires that one person can hold the whole system
  in their head; `MEL-REQ-218–224` let MELFINA construct and evolve capabilities.
  Do compose-before-code, retire-the-obsolete, no-duplication, and on-demand
  loading actually keep the system comprehensible — or does self-generated
  capability inevitably erode that?
- **Class:** H / D.
- **Depends:** VC9; `MEL-REQ-192–195`, `218–224`, `128`.
- **Resolve in:** ARCHITECTURE (complexity budget) + REAL-WORLD USE.

## OQ-19 — Does the self-modification meta-invariant actually hold against a highly capable MELFINA?
- **Question:** `MEL-REQ-235` requires that MELFINA cannot redefine the rules
  governing its own self-modification (external, monotonic, versioned, audited).
  Research shows external immutable rules *may* be circumventable over time — "a
  self-modifying system can preserve constitutional behaviour on familiar tests
  while altering internal abstractions so principles cease to generalise"
  (`MEL-REQ-239`). Is the meta-invariant + grading + audit a *sufficient* safety
  envelope, or only a *necessary* one? Possibly unknowable in advance.
- **Class:** D / U.
- **Depends:** VC10; `MEL-REQ-233–239`.
- **Resolve in:** cannot be fully resolved by design. Mitigation: keep tiers 6–9
  non-autonomous, rare, human-driven, heavily audited; treat any evidence of
  circumvention attempts as a stop condition.

## OQ-20 — What is the right authorisation granularity for the nine self-change tiers?
- **Question:** `MEL-REQ-233` defines nine tiers with escalating authorisation.
  What exactly requires a click, a typed confirmation, a review period, a
  human-authored diff? How are tiers 3–5 (skill/workflow creation and
  modification) gated without making the system unusable?
- **Class:** E (a real decision) + D (needs use).
- **Depends:** VC10; `MEL-REQ-233`, `234`.
- **Resolve in:** SYSTEM DESIGN (initial scheme) + ITERATION.

## OQ-21 — How much of the dynamic self-direction is feasible with *local* reasoning components?
- **Question:** metareasoning, dynamic context selection, capability-gap
  recognition, and self-evaluation are demonstrated mostly with large models.
  Small local models can do multi-step reasoning but are weaker. Which of
  `MEL-REQ-204–253` degrade acceptably on local components, and which effectively
  require capability MELFINA won't have locally (and should therefore decline,
  per `MEL-REQ-155`)?
- **Class:** D.
- **Depends:** VC3, VC8; `MEL-REQ-155`, `204–253`.
- **Resolve in:** ARCHITECTURE + AI LAYER + ITERATION. Overlaps OQ-11.

---

---

## Added by REQUIREMENTS EXPANSION MISSION 002 (OQ-22 … OQ-32)

## OQ-22 — What is the minimum viable foundational-knowledge representation per
domain, and how is domain scope bounded?
- **Question:** `MEL-REQ-264–270` require foundational knowledge to be
  representable within E²CI and acquired incrementally, but do not fix how
  large a domain's "foundational structure" needs to be before it is useful, or
  how domain scope avoids becoming an unbounded, ever-expanding commitment.
- **Class:** C / E.
- **Depends:** VC13; `MEL-REQ-264–270`.
- **Resolve in:** CORE ENGINE / real use — likely per-domain, driven by what the
  user actually engages.

## OQ-23 — Is a learned-vs-authored foundational-knowledge distinction
meaningful once stored?
- **Question:** `MEL-REQ-265`/`327` distinguish user-provided from
  externally-acquired knowledge in provenance, but once a piece of knowledge is
  stored as a Claim with provenance, does the *learned vs. authored* framing add
  anything beyond what provenance already captures, or does it collapse into it?
- **Class:** C.
- **Depends:** `MEL-REQ-265`, `269`, `327`; F5 §6 (provenance).
- **Resolve in:** CORE ENGINE (data-model design pass).

## OQ-24 — How should analogy/causal-link representation structure inside the
Chronicle, without expanding E²CI?
- **Question:** `MEL-REQ-278` requires connections to be ordinary, provenance-
  carrying Claims — but the exact shape (a relational Claim per F5 §4.4's
  "structural relations recorded as Claims" pattern, or something else) is not
  fixed. This directly extends `OQ-M1` (is Relation a primitive?) into the
  cross-domain-connection case specifically.
- **Class:** C.
- **Depends:** `OQ-M1`; `MEL-REQ-272–278`; F5 §4.4, §9.
- **Resolve in:** CORE ENGINE, only after `OQ-M1` itself is resolved or
  deliberately deferred further.

## OQ-25 — What are the practical stopping criteria for decomposition and
formalisation under a resource budget?
- **Question:** `MEL-REQ-292` requires stopping decomposition at diminishing
  value or an epistemic/physical boundary, and `MEL-REQ-241–243` bound resource
  use generally — but no concrete rule for *when* a specific decomposition or
  formalisation attempt should stop is given.
- **Class:** D.
- **Depends:** VC16; `MEL-REQ-241–243`, `289–300`.
- **Resolve in:** CORE ENGINE + REAL-WORLD USE (calibration).

## OQ-26 — How is "wisdom" actually evaluated empirically, and by whom?
- **Question:** `MEL-REQ-279`/`282` require MELFINA's connection-making to be
  evaluated against the operational definition (`KNOWLEDGE_AND_REASONING_MODEL.md`
  §2) over time, but no evaluation method, cadence, or evaluator (the user, a
  fixed test suite, both) is specified. What would falsify a claim that
  MELFINA's connection-making is improving?
- **Class:** D.
- **Depends:** VC14; `MEL-REQ-279–284`.
- **Resolve in:** REAL-WORLD USE + ITERATION — likely cannot be resolved by
  design alone.

## OQ-27 — How is cross-domain hallucination (false-positive connections)
detected in practice?
- **Question:** genuine cross-domain insights and false-positive "pattern
  hallucinations" (`MEL-REQ-275`) can look similarly surprising at first
  encounter. What observable signal distinguishes them before a connection is
  relied upon?
- **Class:** D.
- **Depends:** VC14; `MEL-REQ-275`, `277`, `282`.
- **Resolve in:** CORE ENGINE (verification tooling) + REAL-WORLD USE.

## OQ-28 — Does an adversarial-lens default create the hypervigilance pattern
`MEL-REQ-058` exists to prevent, for this specific user?
- **Question:** §60's generalised adversarial-systems lens is meant for
  analysing systems, not the user's own life — but no requirement restricts it
  from being applied there, and this user's profile includes documented
  checking/reassurance/hypervigilance risk (`MEL-REQ-013`, `058`).
- **Class:** D.
- **Depends:** VC15; `MEL-REQ-285–288`.
- **Resolve in:** REAL-WORLD USE — this is an individual, lived-experience
  question, not a design one.

## OQ-29 — What authorisation granularity applies to a MELFINA-authored
ethical/legal analysis informing a consequential decision?
- **Question:** `MEL-REQ-332–339` establish a structured ethics/law/civics
  capability, but do not specify whether such an analysis, when it feeds a
  consequential decision, needs review beyond the ordinary pipeline
  (`MEL-REQ-018`), or is treated exactly like any other proposal input.
- **Class:** E (a real decision) + D (needs use).
- **Depends:** VC17; `MEL-REQ-332–339`.
- **Resolve in:** SYSTEM DESIGN / next architecture pass.

## OQ-30 — Which domains have practical local independent-verification tooling,
and what happens where none exists?
- **Question:** `MEL-REQ-329–331` require external checks for important
  reasoning (symbolic/numerical/unit checkers, alternative derivations,
  designated verifiers). Some domains have mature, automatable local checkers
  (algebra, unit consistency, some statistical tests); others (much of ethics,
  history, qualitative social science) do not. What is the fallback where no
  local automatable check exists?
- **Class:** D.
- **Depends:** F10 (verifier contract); `MEL-REQ-329–331`.
- **Resolve in:** CORE ENGINE + TECHNOLOGY SELECTION (which checkers to build
  or integrate).

## OQ-31 — How much of PART IV-C is feasible with local reasoning components at
all?
- **Question:** extends `OQ-21` to the whole of PART IV-C: metareasoning,
  cross-domain connection-making, first-principles decomposition, mathematical
  abstraction, and ethical-framework comparison are demonstrated mostly with
  large, often non-local models. Which of `MEL-REQ-254–364` degrade acceptably
  on local components, and which should MELFINA simply decline, per
  `MEL-REQ-155`?
- **Class:** D.
- **Depends:** OQ-11, OQ-21; `MEL-REQ-155`, `254–364`.
- **Resolve in:** ARCHITECTURE + AI LAYER + ITERATION.

## OQ-32 — Does PART IV-C change the answer to OQ-19 (meta-invariant
sufficiency)?
- **Question:** `OQ-19` asks whether the meta-invariant (`MEL-REQ-235`) holds
  against a highly capable self-modifier. PART IV-C explicitly builds MELFINA
  toward first-principles reasoning about mathematics, formal systems, and (in
  §71/§75–76) its own governance. Does this materially change the answer to
  `OQ-19` — or only make the question more urgent without changing it?
- **Class:** D / U.
- **Depends:** VC18, OQ-19; `MEL-REQ-356`, `361–364`.
- **Resolve in:** cannot be fully resolved by design, per `OQ-19`'s own
  resolution note. Mitigation: unchanged from `OQ-19` — keep tiers 6–9
  non-autonomous, rare, human-driven, heavily audited; treat this question as a
  standing item for every future review of MELFINA's reasoning capability, not
  a one-time check.

---

## Deliberately-undecided list (E-class, consolidated)

Recorded so no later document treats these as already-decided:

1. The primitive set / life model (OQ-1).
2. Storage substrate, file format, on-disk encoding.
3. Programming language(s) and their boundaries.
4. Interaction medium(s) beyond "multi-modal, user-choosable, consistent with the
   project's CLI-first leaning".
5. Which local reasoning components / model sizes / whether any are neural at all
   for a given function.
6. The permission-grant representation (object-capability *style* is required;
   the mechanism is not chosen).
7. The default consequential/routine boundary (OQ-12) — initial list at SYSTEM
   DESIGN, not now.
8. Whether an isolated network add-on is ever built (OQ-11).
9. Whether any optional progress representation ships (OQ-6).
10. Sync between the user's own devices, if ever — not a core concern
    (local-only core); would be an isolated capability like any network feature.
11. The dynamic-dispatch / metareasoning mechanism (`MEL-REQ-204–209`).
12. The capability lifecycle mechanism — create / test / promote / version /
    retire (`MEL-REQ-219–227`).
13. The external permission-monitor design that enforces the meta-invariant
    (`MEL-REQ-235`) — required to be external, deterministic, monotonic; the
    mechanism is not chosen.
14. Authorisation granularity for the nine self-change tiers (OQ-20).
15. Whether MELFINA ever performs tier 6+ (code / subsystem / architecture)
    self-modification at all, or those stay human-only indefinitely
    (`MEL-REQ-236` keeps them non-autonomous regardless).
16. The fixed regression / verification harness for capabilities
    (`MEL-REQ-221`, `226`).
17. The exact domain scope and depth of cross-domain foundational knowledge
    (OQ-22) — bounded only by "incremental, as the user's problems engage it"
    (`MEL-REQ-270`), not by a fixed list.
18. The local-checker/verification tooling per domain (OQ-30) — which domains
    get an automatable local check and which do not.
