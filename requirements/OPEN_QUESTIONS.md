# MELFINA — OPEN QUESTIONS

Companion to `requirements/REQUIREMENTS_MASTER.md`. Questions the requirements
phase **cannot** answer — deferred to a later phase, to prototyping, or to
real-world use. Recorded so they are not silently decided elsewhere.

Each: the question, why it's open, what class it is, which requirements depend on
it, and where it should be resolved.

**Classes:** **C** design hypothesis · **D** unknown needing experimentation ·
**E** deliberately undecided (a real decision, just not now).

**OQ-1 … OQ-15** are from the base + earlier addenda; **OQ-16 … OQ-21** were added
by the dynamic-self-directed correction.

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
