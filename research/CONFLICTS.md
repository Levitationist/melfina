# MELFINA — CONFLICT CATALOGUE

Working document. Expands `RESEARCH_MASTER.md` §6.3.

**Why this file exists.** The defining research finding of MISSION 001 is that for
the ADHD + autism + OCD profile, many helpful features have a harmful mirror
image. These are not problems to "solve" in research — they are **decisions to
make with the user** in the PERSONAL REQUIREMENTS phase. This file states each
tension precisely, gives the evidence on both sides, and lists candidate
resolution *approaches* (not choices).

**Status of the "resolution approaches":** all **[DI]** or **[H]**. None is
selected. Several may prove wrong. This is input to a conversation, not an answer.

**How to use in the next phase:** for each conflict, the user places themselves on
the axis (lean A / balanced / lean B / "depends, here's when"), and we decide
whether MELFINA exposes it as a setting, picks a safe default, or avoids the
feature entirely.

---

## C1 — Structure vs. novelty

| | |
|---|---|
| **Pull A** | Fixed routine and predictability reduce anxiety and executive load. |
| **Pull B** | Sameness becomes under-stimulating and aversive; novelty drives engagement. |
| **Evidence A** | IU↔anxiety in autism *r* ≈ 0.62 (Jenkinson 2020); predictability as uncertainty-reduction; routine preference well documented. **[E]** |
| **Evidence B** | Delay aversion / reward-system findings (Marx 2021; Sonuga-Barke); novelty effects in gamification (Hamari 2014); AuDHD lived-experience reports of routine → boredom. **[E]** + **[clinical]** |
| **Who** | The classic AuDHD internal conflict. Balance is highly individual and may shift day to day. |
| **Candidate approaches** | (a) **Anchored flexibility [H]** — few fixed anchor points, open unstructured space between. (b) User-set "structure dial." (c) Structure as *scaffolding available on demand* rather than an imposed frame. (d) Let novelty live in *content/presentation* while *system behaviour* stays predictable (see C10). |
| **Do not** | Ship "routine mode" vs "flexible mode" as a diagnostic preset. |

---

## C2 — Externalise memory vs. recording becomes compulsion

| | |
|---|---|
| **Pull A** | Get everything out of the head; write it down; keep rich records. |
| **Pull B** | Repeated recording, re-reading, and re-verifying can *be* a compulsion; logs become checking objects. |
| **Evidence A** | Cognitive offloading improves performance (Risko & Gilbert 2016); a specific written plan removes open-loop intrusion (Masicampo & Baumeister 2011); EF/PM deficits persist in adulthood (Alderson 2013; Demetriou 2018). **[E]** |
| **Evidence B** | Repeated checking *reduces* memory confidence (van den Hout & Kindt 2003); compulsive logging / list-re-reading flagged in clinical app reviews; symmetry/ordering is an OCD dimension. **[E]** + **[G]** |
| **Who** | Everyone benefits from A; the OCD-driven risk in B is individual and situation-specific. |
| **Candidate approaches** | (a) **Record once, treat as settled [DI]** — minimise affordances to re-open/re-confirm. (b) Capture is rich; *review* surfaces are deliberately limited and not habitual. (c) No "verify your entries" / "review everything" ritual as a load-bearing mechanic. (d) The system asserts "I have this" so the user's mind can release it — trust replaces re-checking. |
| **Do not** | Build GTD-style mandatory periodic full review as the linchpin. |

---

## C3 — Reminders/prompts vs. notification overload

| | |
|---|---|
| **Pull A** | Prospective-memory gaps need external prompting to bridge them. |
| **Pull B** | Notifications cause interruption cost, alert fatigue, demand-response, and sensory intrusion. |
| **Evidence A** | Time-based PM reliably impaired in ADHD (Talbot & Kerns); reminders help PM in lab settings. **[E]** |
| **Evidence B** | Resumption lag grows with interruption length (Monk 2008); alert responsiveness drops ~30%/repeat (Ancker 2017); location reminders "still not effective" in the field (Sohn 2005); PDA/demand response (clinical); autistic sensory reactivity (Ben-Sasson 2009). **[E]** + **[G]** |
| **Who** | All three profiles feel B; A is strongest for ADHD PM. |
| **Candidate approaches** | (a) Very few things interrupt unbidden; user controls interruption. (b) Deliver at task boundaries / explicit "available now" states (Iqbal & Bailey 2008). (c) A calm, predictable *pull* surface ("what would have prompted me") instead of push. (d) Event/context anchoring + specific actionable content over bare time alarms. (e) Assume decay; don't rely on a repeated identical alert. |
| **Do not** | Drive behaviour change through push. Escalate/nag. |

---

## C4 — Simplify / reduce decisions vs. loss of information and control

| | |
|---|---|
| **Pull A** | Fewer visible options and decisions reduces EF load and choice paralysis. |
| **Pull B** | Hiding things unpredictably violates the need to know what's there and stay in control. |
| **Evidence A** | Choice overload (Scheibehenne 2010 — real but moderated); COGA "help users focus" / "reduce content" (**[G]**); decision cost under EF load. |
| **Evidence B** | IU in autism and OCD (**[E]**); adaptable ≫ adaptive because hiding/moving is unpredictable (Findlater & McGrenere 2004) (**[E]**). |
| **Who** | A serves EF load (all three); B is sharpest for autism predictability and OCD "need to know." |
| **Candidate approaches** | (a) Show few things *by default*, everything reachable *predictably*. (b) Hiding is always user-reversible and follows a rule the user can state. (c) "Progressive disclosure" where the disclosure control is obvious and stable. (d) Never hide something *because the system decided it wasn't important*. |
| **Do not** | Silent, algorithmic prioritisation that changes what's visible. |

---

## C5 — Measurement/feedback vs. compulsive monitoring & perfectionism

| | |
|---|---|
| **Pull A** | Progress feedback creates near-term salience for non-salient tasks (the ADHD reward gap). |
| **Pull B** | Metrics feed compulsive monitoring, perfectionism, self-judgement, and metric-gaming. |
| **Evidence A** | Delay discounting / weak response to distant rewards (Marx 2021). **[E]** |
| **Evidence B** | Gamification effects small and fragile (Sailer & Homner 2020: behavioural *g* ≈ 0.25); novelty effects (Hamari 2014); over-justification (Deci 1999); streaks = loss aversion; NJRE/incompleteness predict OC symptoms (Sica 2022); emotion dysregulation in ADHD. **[E]** |
| **Who** | A is an ADHD lever; B endangers the OCD/perfectionism side and the emotionally-dysregulated side. The user is also a musician (over-justification risk on practice). |
| **Candidate approaches** | (a) **Default: no scorekeeping** — no streaks, points, badges, comparative or aggregate stats. (b) If any progress view exists: private, non-comparative, non-accumulating, partial-credit, disableable, forgettable. (c) Serve the real need (salience) by shrinking the task and concretising the next step, not by an external point economy. |
| **Do not** | Streaks. Completion percentages. "You're on a roll" / "you're behind." |

---

## C6 — Automation vs. autonomy, predictability, and agency

| | |
|---|---|
| **Pull A** | Automating friction removes initiation cost and executive overhead. |
| **Pull B** | Unwanted automation erodes autonomy (demand response), unpredictable system action violates predictability, and reliance erodes skill/vigilance. |
| **Evidence A** | Initiation cost / EF load (Barkley; §3). **[E]** for the deficit. |
| **Evidence B** | Automation complacency & bias (Parasuraman & Manzey 2010); adaptable ≫ adaptive (Findlater & McGrenere 2004); SDT autonomy; PDA; "AI assistance reduces persistence" (2026). **[E]** + **[clinical]** |
| **Who** | A tempts for ADHD; B is a serious risk for autism/PDA and for anyone the tool is meant to *strengthen*. |
| **Candidate approaches** | (a) **Suggest ≫ act.** Autonomous action only for reversible, trivial, high-certainty, pre-authorised cases (Horvitz 1999 expected-value rule). (b) Every automated action is visible, logged, reversible. (c) The system proposes patterns it notices; the user confirms; nothing self-adapts silently. (d) Full function without any automation enabled. |
| **Do not** | Act when the user's goal or the value of acting is uncertain. |

---

## C7 — Completion/closure vs. "not-just-right" dissonance

| | |
|---|---|
| **Pull A** | The relief of "done" closes open loops and frees the mind. |
| **Pull B** | "Not-just-right" / incompleteness loops feed on both the completion hit and the dissonance of anything unfinished. |
| **Evidence A** | Zeigarnik; Masicampo & Baumeister 2011 (open loops cost focus). **[E]** |
| **Evidence B** | NJRE/incompleteness predict OC symptoms (Coles 2003; Sica 2022); completion rituals; perfectionism. **[E]** |
| **Who** | A is universal; B is the incompleteness-driven OCD subtype specifically. |
| **Candidate approaches** | (a) Allow partial progress that "counts" — not only binary done/not-done. (b) Avoid designed "perfect closure" spectacles (all-green grid, zero-inbox celebration, everything-ticked). (c) Closure via *"handed off to the system"* (Masicampo) rather than *"perfectly finished"*. (d) Items can be closed as "no longer relevant" without being "completed." |
| **Do not** | Make an immaculate all-clear state the emotional centrepiece. |

---

## C8 — Prompting/accountability vs. pressure → avoidance and shame

| | |
|---|---|
| **Pull A** | External accountability and nudges help initiation. |
| **Pull B** | Perceived pressure triggers demand avoidance; visible shortfall triggers shame and feeds burnout. |
| **Evidence A** | Implementation intentions *d* = 0.65 (Gollwitzer & Sheeran 2006); body doubling (**[H]**). |
| **Evidence B** | PDA / "persistent drive for autonomy" (clinical); autistic burnout from expectations–abilities mismatch (Raymaker 2020); emotion dysregulation & rejection-sensitivity in ADHD (Beheshti 2020). **[E]** + **[clinical]** |
| **Who** | A is an ADHD initiation lever; B endangers the autism/PDA side and the emotionally-dysregulated side. |
| **Candidate approaches** | (a) Framing: offer, don't instruct; "here's a starting point" not "you should." (b) No disappointment, no streak-style loss framing, no "overdue" red. (c) Accountability only if the *user* opts into it and configures its shape. (d) Shortfalls are surfaced neutrally and only on request. |
| **Do not** | "You didn't do X." Countdown pressure. Guilt UI. |

---

## C9 — Rich context capture vs. total-capture noise and capture friction

| | |
|---|---|
| **Pull A** | Capturing context (what was I doing, what's next) supports task resumption after ADHD context loss. |
| **Pull B** | Capturing everything yields unusable noise; and heavy capture UI creates friction that kills the habit. |
| **Evidence A** | Memory-for-goals / resumption cues (Altmann & Trafton 2002; Monk 2008). **[E]** |
| **Evidence B** | "Beyond total capture" — data in, little usable out (Sellen & Whittaker 2010); PKM capture-friction as the #1 failure point (practitioner consensus); personal-informatics abandonment (Epstein 2016). **[E]** + **[practitioner]** |
| **Who** | Everyone. |
| **Candidate approaches** | (a) Near-zero-friction capture; defer all categorisation. (b) Capture *cues for reconstruction*, not exhaustive records — design for retrieval. (c) Optional lightweight "resumption note" when leaving a task; replay on return. (d) Let the archive be lossy; not everything must be kept or findable. |
| **Do not** | Mandatory fields at capture. Lifelogging-style total capture. |

---

## C10 — Predictable interface vs. stale/unmotivating interface

| | |
|---|---|
| **Pull A** | An unchanging interface is safe and low-load (autism). |
| **Pull B** | An unchanging interface offers no novelty reward and can feel dead (ADHD). |
| **Evidence A** | IU/predictability (**[E]**); adaptive-UI cost (Findlater & McGrenere 2004). |
| **Evidence B** | Reward/novelty response (**[E]**); gamification novelty effects (Hamari 2014). |
| **Who** | Direct AuDHD tension, but lower stakes than C1–C9. |
| **Candidate approaches** | (a) Keep *structure, layout, and behaviour* stable; allow *the user* to restyle surface appearance (theme, density, colour) whenever they want novelty. (b) Novelty comes from the user's own changing content, not from the system rearranging itself. (c) Any system-initiated visual change is opt-in. |
| **Do not** | Auto-changing layouts, surprise redesigns, "new look" pushes. |

---

## Cross-cutting resolution themes (candidate, [DI]/[H])

1. **The axis is the user's to set.** Most of C1–C10 resolve to "expose as a
   gently-defaulted, reversible user dimension" rather than a fixed design choice
   or a diagnostic preset. (RESEARCH_MASTER §15)
2. **Split "system behaviour" from "content/appearance."** Several conflicts
   dissolve if the *system* is rigidly predictable while the *user's own material
   and the cosmetic layer* are freely changeable. (C1, C10)
3. **Trust replaces verification.** If the user believes the system reliably holds
   and resurfaces things, the need to re-check (C2), the open-loop load (C7, C9),
   and some anxiety (C3) all drop. Reliability is therefore a *psychological*
   feature, not only an engineering one.
4. **Neutral defaults at the hazard surfaces.** Completion, metrics, history,
   confirmation, and prompting (C2, C5, C7, C8) default to the minimal, quiet,
   non-judgemental option; the user can add intensity, never the reverse by
   surprise.
5. **Offer, don't impose.** Autonomy-supportive framing (C6, C8) is a global
   stance, not a per-feature toggle.

---

## What the requirements phase must decide (not decided here)

- Where the user actually sits on each of C1–C10.
- Which conflicts become settings, which get a safe default with no setting, and
  which features are dropped entirely because the risk isn't worth it.
- Whether "capacity-state" (low/normal/high) is a manual user switch that changes
  several axes at once, and if so what it changes.
- How the eventual music/creative-practice use rides on the general resolution
  (it should not get its own conflict set).

---

## SECOND-PASS EVIDENCE UPDATES (RESEARCH MISSION 002, 2026-09-10)

Pass 2 did not add or remove conflicts. It changed the evidence weight on several,
corrected two sub-claims, and added one cross-cutting caution. Full detail:
`RESEARCH_MASTER.md` §§20–32. Tier `[U]` = unknown / insufficient evidence.

### Cross-cutting

- **The combined ADHD + autism + OCD profile is unmeasured on every
  design-relevant variable [U].** All axis reasoning is inferred from
  single-condition or two-way-comorbidity data. Comorbidity **amplifies** adaptive
  impairment (co-occurring conditions worsen functioning and quality of life) —
  which makes **individually-calibrated axes** the safe approach and **diagnostic
  "modes" contraindicated** (now evidence-adjacent, not just inference).
- **Autonomy-support** moved from `[E/clinical]` to **`[E]`** (self-determination
  theory meta-analyses: need support → autonomous motivation → performance +
  wellbeing; need-thwarting → the maladaptive path). Strengthens the "offer, don't
  impose" stance behind **C6** and **C8** and the whole catalogue.

### Per-conflict

- **C1 (structure ↔ novelty).** "Anchored flexibility" remains **[H]** — it recurs
  in AuDHD clinical/lived-experience writing but has no experimental test. New:
  autism EF impairment **attenuates somewhat in adulthood** (Demetriou 2018, g =
  0.48, now primary-verified) but persists; music-dropout evidence ties *controlled*
  motivation (pressure, obligation) to dropout and *autonomous* motivation to
  persistence — relevant if "structure" is ever delivered as pressure.

- **C2 (externalise ↔ compulsion)** and **C9 (rich capture ↔ noise/friction).**
  Strengthened on the "helps" side and the "how" side:
  - Cognitive-offloading **improves** memory-based task performance (2025
    meta-analysis); it is value-based decision-making driven by memory *confidence*
    (low confidence → more offloading) — and OCD checking *lowers* memory
    confidence (§5.3), so the two mechanisms interact.
  - The **PIM literature** now makes it **`[E]`** (not just practitioner consensus)
    that *keeping/filing is the expensive, abandonment-prone part*, and that people
    **prefer navigation + context over search for their own information**.
  - 30+ years of academic prototypes (§30): **retrieval and timing, not capture,
    are the unsolved problems**; **total capture is a known dead end**.
  - Candidate approach reinforced: near-frictionless capture, **defer
    organisation**, support navigation/context for retrieval, record-once.

- **C3 (reminders ↔ overload).** Three refinements:
  - **Batching correction:** Pass 1 said no evidence batching helps stress. **Fitz
    et al. 2019** (RCT, n = 237) shows notifications batched into a few predictable
    daily windows improve attention, mood, and felt control; **full silence raises
    anxiety/FoMO**; email-batching evidence stays mixed. Revised target:
    **predictable, user-controlled delivery timing** — not "batch all," not
    "silence all."
  - **Alert-fatigue magnitude correction:** the "~30% per repeat" figure was
    overstated. Ancker 2017: ~10% lower odds of acting per +5 pp of within-context
    repeat alerts. Direction (desensitisation is real) holds.
  - **User-declared timing beats inferred timing:** JITAI evidence — triggers based
    on *self-reported need* are rated better-timed and more helpful than
    distress-inferred triggers; location reminders remain "not effective" in the
    field. Context inference has never become reliable enough to trust for
    intrusion timing.
  - Strongest reminder-efficacy evidence (**NeuroPage**, RCT, n = 143) is in
    memory-impaired neurological patients with *user-chosen* content and human
    setup — adult ADHD/autism/OCD efficacy is **[U]**.

- **C5 (measurement ↔ compulsive monitoring).** Strengthened **both** directions,
  net still unfavourable-by-default for this user:
  - Benefit side `[E]`: self-tracking has **small** positive effects on behaviour
    change and self-knowledge/"sense of accomplishment."
  - Harm side `[E]`: systematic-review evidence of guilt/anxiety/stress, rumination,
    body-image harm, disordered eating, compulsive exercise; **mood-tracking can
    induce depressive rumination and worsen symptoms** — "explicitly unresolved"
    whether it net helps. Harms concentrate in **perfectionism / OCD-anxiety
    traits / depression**.
  - **Musician-specific:** perfectionistic *concerns* (fear of mistakes, doubt,
    evaluation) — not perfectionistic *strivings* — drive music performance anxiety
    and overlap with OCD's intolerance-of-uncertainty and "not-just-right"
    constructs. Metrics/completion/error-highlighting/comparative history are the
    surfaces that feed *concerns* — at the piano, not just in the abstract.
  - Deliberate-practice **amount** explains only ~21–26% of music-performance
    variance (contested; Ericsson camp disputes) — foregrounding "hours practised"
    optimises a weak variable.

- **C6 (automation ↔ autonomy).** Autonomy side → **`[E]`** (SDT meta-analyses).
  Automation side reinforced: a 2026 study found AI assistance **reduced task
  persistence and independent performance** afterward. "Suggest ≫ act" holds.

- **C7 (completion ↔ "not-just-right").** Reinforced: **attention residue** (Leroy
  2009) shows task-switching cost is *sustained*, and — crucially — **believing you
  will have ample time to finish later removes the residue**. Same shape as
  Masicampo: a *credible completion path*, not actual completion, is what relieves
  the load. "Handed off to the system" > "perfectly finished." Perfectionistic-
  concerns ↔ NJRE overlap now explicit.

- **C8 (prompting ↔ pressure/avoidance).** Autonomy-support `[E]`; **camouflaging /
  masking** meta-analysis: higher masking → worse mental health (anxiety,
  depression, sometimes suicidality) and exhaustion — a system that raises the felt
  demand to "keep up appearances," even privately, works against this. Music-dropout
  evidence: *controlled* pressure to practise predicts dropout.

- **C10 (predictable ↔ stale interface).** Autistic-adult web-user studies (AASPIRE
  Web Accessibility Guidelines; animated-UI-element study): avoid clutter, moving
  images, decorative churn; **irrelevant animation measurably hurts task
  performance** for autistic users — even on short tasks. "Keep system behaviour
  and layout stable; let the user restyle surface appearance for novelty" is
  supported.

### New corrections logged

| Was (Pass 1) | Now (Pass 2) |
|---|---|
| "No evidence notification batching lowers stress" | Batching into predictable windows has RCT support (smartphone); silence backfires; email mixed |
| "~30% drop in responsiveness per repeated alert" | ~10% lower acceptance odds per +5 pp repeat-share (Ancker 2017); direction holds, magnitude overstated |
| Implementation intentions target the adult-ADHD initiation gap | Child-ADHD evidence only; **adult ADHD = [U]**; may need baseline EF |
| Autism EF "stable across lifespan" | Present across the lifespan, **attenuates somewhat in adulthood** |
| AuDHD "~30–40% each direction" | ADHD-in-autism ≈ 22% community / 34% clinical; ADHD+OCD lower & more uncertain, and neurofunctionally opposite |
