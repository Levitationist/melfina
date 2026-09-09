# MELFINA — RESEARCH MASTER

**Mission:** RESEARCH MISSION 001 — Deep medical / clinical / cognitive-science / HCI research.
**Status:** Complete (first pass), pending review.
**Date:** 2026-09-09.
**Scope discipline:** This document is *research*. It stops at *possible design implications*.
It does **not** define requirements, architecture, or features. See §16 and the
`RESEARCH FINDING → POSSIBLE DESIGN IMPLICATION → [STOP]` boundary in §19.

---

## Reading guide and evidence tiers

Every substantive claim below is tagged with one of:

| Tag | Meaning |
|-----|---------|
| **[E]** | **Evidence.** Directly supported by peer-reviewed primary research, systematic review, or meta-analysis. Effect sizes given where available. |
| **[G]** | **Clinical / professional guidance.** Recommendation from a recognised clinical or standards body (NICE, APA, IOCDF, W3C/WAI, etc.) or well-established clinical practice. Not the same as a proven mechanism. |
| **[DI]** | **Design inference.** A reasonable engineering implication *derived* from [E] or [G]. Not itself tested. |
| **[H]** | **Hypothesis.** Plausibly useful, not established. Flagged for later testing. |

Where the literature conflicts, this is stated explicitly (look for **Conflicting evidence:**).
Where evidence is thin or absent, this is stated (**Gap:**).

**Important disclaimers.**
- This is not a medical document. It does not diagnose anyone, does not describe the
  user's individual condition, and must not be used to select or reject treatment.
- ADHD, autism, and OCD are heterogeneous. Group-level findings (even large ones)
  do not describe any individual. A meta-analytic effect size of *g* = 0.5 means
  substantial overlap between groups.
- Most cognitive research on ADHD and autism is on children. Adult data is thinner.
- Almost no research exists on integrated "life operating system" software for the
  specific ADHD + autism + OCD overlap. Much of §15's design space is uncharted.
- Correlations in this literature are mostly cross-sectional. Causal language is
  avoided unless an experimental or longitudinal design supports it.

Full citations: `research/BIBLIOGRAPHY.md`. Inline references use `Author year`.

---

## 1. Executive summary

**The core tension.** MELFINA's user population profile (ADHD + autism + OCD, by
self-description) sits at an intersection where the *same* software feature can
help one need and actively harm another. The research does not resolve this
tension; it maps it. The central finding of this mission is that **for this
profile, the design risk is not "too few features" — it is features that
inadvertently become engines for compulsion, rigidity, notification fatigue, or
demand-avoidance.**

**What the evidence supports reasonably well:**

1. **Executive function and working-memory differences are real and persist into
   adulthood** in ADHD (working memory *g* ≈ 0.5, Alderson 2013) and autism
   (domain-general EF *g* ≈ 0.5, Demetriou 2018). This is a strong rationale for
   *external* structure — offloading memory, plans, and context out of the head. **[E]**
2. **Externalising intentions works.** Cognitive offloading reliably improves task
   performance (Risko & Gilbert 2016; Gilbert 2022). Writing a *specific plan* for
   an unfinished task removes the intrusive-thought and focus cost of leaving it
   undone, even though the task is still not done (Masicampo & Baumeister 2011). **[E]**
3. **Time perception is measurably atypical in ADHD** (children/adolescents:
   accuracy *g* > 0.4, precision *g* = 0.66; Zheng 2022). "Externalise time" is the
   consistent clinical recommendation. **[E] for the deficit; [G] for the remedy.**
4. **Predictability reduces anxiety in autism; unpredictability drives it.**
   Intolerance of uncertainty is tightly linked to anxiety in autism (*r* ≈ 0.62;
   Jenkinson 2020) and is a candidate cognitive vulnerability in OCD (Gillett 2018;
   Shihata review). **[E]**
5. **Checking, reassurance, and logging can become compulsions.** Repeated checking
   *reduces* memory confidence (van den Hout & Kindt 2003). Reassurance-seeking and
   family accommodation predict *worse* OCD outcomes. Always-available AI is now
   documented by clinicians as a novel reassurance vector (npj Digital Medicine
   2026; IOCDF 2026). **[E] / [G]**
6. **Notifications degrade with volume.** Interruptions carry a resumption cost that
   grows with interruption length (Altmann & Trafton 2002; Monk 2008). Deferring
   alerts to task boundaries reduces cost (Iqbal & Bailey 2008). Repeated alerts
   lose their power (alert-fatigue literature: ~30% drop in responsiveness per
   repeat). **[E]**
7. **Gamification effects are small and fragile.** Meta-analytic effects are
   modest (behavioural *g* ≈ 0.25; Sailer & Homner 2020), often attributed to
   novelty, and streak mechanics specifically convert intrinsic goals into
   loss-avoidance. For a perfectionism/compulsion-prone user this is a
   contraindicated pattern by default. **[E] / [DI]**
8. **Users prefer control over automation.** Adaptable (user-directed) interfaces
   are consistently preferred over adaptive (system-directed) ones, even when
   slightly slower (Findlater & McGrenere 2004). Autonomy-supportive framing
   reduces demand avoidance (self-determination theory; PDA literature). **[E] / [G]**
9. **Local-first is a coherent, documented architecture philosophy** (Kleppmann
   2019) that aligns with privacy, longevity, and user-control goals. **[E] as a
   design paradigm.**

**What the evidence does *not* settle:** whether specific assistive features
(visual timers, body doubling, location reminders) work for adults; how the three
profiles' needs should be *prioritised* when they conflict; the long-term effect
of heavy cognitive offloading on this population; and essentially the entire
question of an *integrated* life-model system rather than point tools.

**Preliminary orientation for MELFINA (full list in §18):** a system that is
*external structure without enforcement* — it holds memory, time, plans, and
context so the user does not have to, while refusing the roles of nag, scorekeeper,
reassurance oracle, and behavioural enforcer. Structure should be *offered and
adaptable*, not imposed. Completion, metrics, history, and confirmation are
**high-risk surfaces** requiring deliberate design restraint.

---

## 2. Research methodology

**Approach.** Web-based literature search (September 2026), prioritising the source
hierarchy in the mission brief: systematic reviews and meta-analyses first, then
clinical guidelines and professional bodies, then peer-reviewed primary research,
then HCI / assistive-technology venues (CHI, UIST, ASSETS, TOCHI, IMWUT), then
reputable institutions. Productivity blogs, SEO content, and vendor marketing were
treated as *signals of what exists*, not as evidence, and are labelled as such.

**Databases / venues reached:** Nature / Molecular Psychiatry / npj Digital
Medicine (abstracts; several full texts paywalled), PubMed / PMC, ScienceDirect
(Behaviour Research and Therapy, JOCRD, Journal of Attention Disorders), SAGE
(Autism), Wiley (European Journal of Social Psychology, Autism Research), ACM DL
(CHI, IMWUT), W3C/WAI, Ink & Switch, Cambridge repository.

**Synthesis method.** Findings were grouped by mechanism rather than by diagnosis
where possible, then cross-checked for the three-way overlap (§6). Each finding was
assigned an evidence tier. Design implications were derived separately and marked
`[DI]`/`[H]` so they cannot be mistaken for findings.

**Limitations of this pass.**
- English-language sources only; US-centric search index.
- Several key papers (Demetriou 2018 full text; npj Digital Medicine 2026 full
  text) were accessible only as abstracts / secondary summaries. Effect sizes taken
  from abstracts or from independent summaries are noted as such.
- No systematic search protocol (PRISMA); this is a scoping synthesis, not a
  systematic review. A second pass with database access and citation-chaining is
  warranted before requirements (see §17).
- Recency bias: search favoured 2015–2026. Foundational older work (Barkley,
  Rachman, Zeigarnik, Gollwitzer, Horvitz) was included deliberately.
- No primary contact with clinicians or with the user's own history (correctly, per
  mission scope — that belongs to the PERSONAL REQUIREMENTS phase).

---

## 3. ADHD findings

ADHD in adults is best understood not as "attention deficit" but as a disorder of
**self-regulation over time** — the deployment of attention, effort, and action
toward goals that are not immediately rewarding.

### 3.1 Executive function and the self-regulation model

- **[E]** Working-memory differences persist into adulthood. Meta-analysis of 38
  studies (Alderson 2013, *Neuropsychology*): verbal WM *g* ≈ 0.55, visuospatial WM
  *g* ≈ 0.49 (medium). Deficits are larger when central-executive demands are high
  (manipulation, not just storage).
- **[E/G]** Barkley's Executive Function–Self-Regulation model (Barkley 1997, and
  later work) frames ADHD as impaired behavioural inhibition undermining four
  self-directed executive functions: nonverbal working memory (holding the past and
  future in mind), internalised speech (self-instruction), self-regulation of
  affect/motivation/arousal, and reconstitution (planning/problem-solving). The
  model's practical claim, repeatedly emphasised: **interventions must act at the
  "point of performance" — the actual place and time where the behaviour fails —
  not through knowledge or insight delivered elsewhere.**
- **[E]** Emotional dysregulation is now regarded as a *core* feature of adult
  ADHD, not a comorbidity (systematic review, Beheshti 2020). It predicts
  psychosocial impairment (employment, relationships) more strongly than
  inattention or hyperactivity. Rejection-sensitivity experiences (strong dysphoric
  reactions to perceived criticism/rejection) are reported as highly functionally
  disruptive in qualitative work (PLOS One 2024), though "rejection-sensitive
  dysphoria" is not a formal diagnostic construct.

### 3.2 Time

- **[E]** Time-perception impairments are consistent in ADHD. Meta-analysis of 27
  studies, ~1,620 ADHD vs ~1,250 controls (Zheng 2022, *J Atten Disord*):
  perception less accurate (*g* > 0.40) and less precise (*g* = 0.66), across
  visual and auditory stimuli. A separate meta-analysis (25 studies, 1,633
  participants) found a medium-effect time-*discrimination* deficit.
- **[E]** Time *reproduction* / *production* errors are elevated (smaller effect).
  The lay term "time blindness" is not a technical term but corresponds to a real,
  replicated finding.
- **[G]** Clinical consensus: externalise time. Make duration and elapsed/remaining
  time continuously *visible* rather than requiring internal estimation. Visual
  analog representations (shrinking disc, filled bar) are the standard clinical
  recommendation.
- **Gap:** high-quality *efficacy* trials of specific external-time tools (Time
  Timer–style devices, time-blocking apps) in adults are sparse; the recommendation
  rests on the mechanism plus clinical practice, not RCTs.

### 3.3 Prospective memory (remembering to do things later)

- **[E]** Time-based prospective memory (do X at 3pm) is reliably impaired in ADHD.
  Event-based PM (do X when Y happens) shows inconsistent results — cueing helps
  (review, Talbot & Kerns 2014/2017).
- **[DI]** Event-based, context-cued reminders may be more robust than pure
  time-based alarms for this population — but see §10 on reminder fatigue and §8
  on the mixed field evidence for location-based reminders.

### 3.4 Motivation, reward, and delay

- **[E]** Delay discounting is steeper in ADHD: people choose smaller-sooner over
  larger-later rewards more often. Meta-analysis of monetary delay discounting
  (Jackson & MacKillop 2016, *Biol Psychiatry CNNI*) and a comparative
  meta-analysis of 37 group comparisons / 3,763 participants (Marx 2021) both
  confirm small-to-medium effects. Real (vs hypothetical) rewards and actual
  waiting sharpen the effect.
- **[E/theory]** Sonuga-Barke's dual-pathway model: ADHD involves *both* an
  executive-function pathway *and* a distinct motivational pathway ("delay
  aversion" — waiting itself is aversive). These are partly independent; an
  individual may have one more than the other.
- **[DI]** Tasks without near-term salience are neurologically harder to *start*
  and *sustain*. Design that shortens the gap between action and feedback, or that
  makes a distant goal locally concrete, is working with the grain of this finding.
  (This is *also* the mechanism gamification exploits — see §11 for why that is not
  a free lunch.)

### 3.5 Task initiation

- **[G/H]** "Activation energy" for starting non-salient tasks is a widely reported
  clinical and lived-experience phenomenon; it plausibly follows from 3.1 + 3.4 but
  is not itself a cleanly measured construct.
- **[E, indirect]** Implementation intentions ("when situation X arises, I will do
  Y") have a medium-large effect on goal initiation and shielding: *d* = 0.65
  across 94 studies / 8,000+ participants (Gollwitzer & Sheeran 2006). Effects in
  ADHD-specific samples are less studied but the technique targets exactly the
  initiation gap.
- **[E]** Task decomposition into concrete next actions is a core mechanism in both
  GTD (Heylighen & Vidal 2008) and CBT for adult ADHD.
- **[E/H]** "Body doubling" (working alongside another person, physically or
  virtually) is heavily reported in ADHD communities and is an active HCI research
  topic (e.g. CHI work framing it as a space/time/mutuality continuum), but robust
  outcome evidence is limited. Treat as **[H]** with a plausible social-facilitation
  / external-cue mechanism.

### 3.6 What actually helps adults (intervention evidence)

- **[E/G]** CBT for adult ADHD (often including organisational-skills training,
  time-management, and anti-procrastination modules) produces medium-to-large
  *within*-group and small-to-medium *between*-group symptom reductions, plus
  functional gains in time management and organisation (multiple meta-analyses;
  see adhdevidence.org syntheses).
- **[G]** NICE NG87 recommends, for adults, a combination of psychoeducation, CBT,
  environmental/organisational strategies (planners, task apps, breaking work into
  steps), and — where appropriate — medication. Medication (stimulants first-line)
  has the largest short-term symptom effect but is outside MELFINA's scope; the
  point for design is that **environmental scaffolding is explicitly part of the
  standard of care**, not a workaround.

---

## 4. Autism findings

Framed here in functional and neurodiversity-informed terms: autism as a different
cognitive and perceptual style with characteristic strengths (sustained deep
focus, systematic thinking, pattern detection, honesty of communication) and
characteristic costs under environments built for a different style.

### 4.1 Attention style: monotropism

- **[E/theory]** Monotropism (Murray, Lesser & Lawson 2005) proposes that autistic
  attention tends to be concentrated deeply on a small number of "interests" (in
  the technical sense of attention tunnels) at once, versus distributed attention
  ("polytropism"). Consequences: depth and expertise; difficulty task-switching,
  processing parallel inputs, and recovering from interruption; "flow" states that
  are costly to enter and exit. There is broad agreement that atypical attention
  distribution is central to autism; monotropism is one influential framing.
- **[E, emerging]** The Monotropism Questionnaire (Garau 2023) shows monotropic
  traits correlate with autistic and ADHD traits; convergence with lab/ERP measures
  of hyperfocus is so far *limited* (2025 study) — the construct is better
  supported at the experiential/questionnaire level than the neurophysiological one.
- **[DI]** Interruptions are not merely annoying in this frame — they can collapse a
  costly-to-rebuild attention state. Protecting focus and supporting deliberate,
  low-cost context switches matters more than for a general-population tool.

### 4.2 Executive function

- **[E]** Meta-analysis of EF in autism spanning 1980–2016 (Demetriou 2018,
  *Molecular Psychiatry*): a **medium, domain-general** effect (overall Hedges
  *g* ≈ 0.5 per the paper's abstract and secondary summaries) that did **not
  fractionate cleanly** into specific subdomains and remained **relatively stable
  across the lifespan** (did not simply "improve with age" or disappear in
  adulthood). EF task performance had **limited value as a diagnostic marker**
  (large individual variation).
- **[E]** Cognitive flexibility / set-shifting: difficulty is most pronounced in
  *maintaining* a new set after switching, and in *real-world* flexibility.
  Notably, **real-world flexibility (parent/self-report) predicts anxiety, whereas
  lab task-switching performance often does not** (Frontiers 2025) — a warning that
  lab EF measures and lived executive difficulty are not the same thing.
- **[E]** Autistic-trait studies in the general population find autistic traits
  associate with *lower perceived* EF but **not reliably poorer EF task
  performance** (Molecular Autism 2025) — reinforcing that self-experienced
  executive load is the more design-relevant variable.

### 4.3 Predictability, uncertainty, transitions, routine

- **[E]** Intolerance of uncertainty (IU) is strongly associated with anxiety in
  autistic people: meta-analysis (Jenkinson 2020, *Autism*) *r* ≈ 0.62 (large); a
  2024 meta-analysis (33 papers, 8,347 participants) links IU, restrictive/
  repetitive behaviours, and anxiety in a mutually reinforcing triad.
- **[E/G]** Preference for predictability and routine is well documented and
  functions partly as *uncertainty reduction*. Transitions between activities/
  contexts are a recognised friction point; advance notice and previewing what
  comes next are standard supports.
- **[DI]** A system that surfaces "what is coming," changes state predictably, and
  never silently rearranges the user's world is working with this grain. Sudden
  UI changes, unannounced automation, and non-deterministic behaviour are
  correspondingly costly. (See §6.4 — this collides with ADHD novelty needs.)

### 4.4 Sensory considerations

- **[E]** Meta-analysis of sensory modulation symptoms (Ben-Sasson 2009, *JADD*):
  large differences between autistic and comparison groups; ordering roughly
  under-responsivity > over-responsivity > sensation-seeking; moderated by age and
  by comparison-group type. Sensory phenomena are common across the lifespan.
- **[G]** Standard accommodations: reduce unnecessary sensory load, avoid abrupt or
  unexpected sensory events (sound, motion, flashing), give the user control over
  intensity. Overlaps directly with W3C COGA "help users focus" (§8).
- **[DI]** Interface implications: motion/animation restraint, no attention-grabbing
  visual churn, sound off by default and user-configurable, calm default palette,
  predictable layout. These are cheap to honour and hard to retrofit.

### 4.5 Communication, autonomy, and burnout

- **[E/theory]** Double empathy problem (Milton 2012): autistic–non-autistic
  communication difficulty is *mutual and bidirectional*, not a one-way autistic
  deficit. Supported by studies showing information transfers as efficiently in
  autistic–autistic chains as in non-autistic ones, and less efficiently in mixed
  chains (Crompton 2020). **[DI]** implication for an AI layer: do not assume the
  user's phrasing, priorities, or emotional expression map onto neurotypical
  defaults; do not "correct" communication style.
- **[E/G]** Autonomy: self-determination theory distinguishes autonomous from
  controlled motivation; autonomy-supportive framing (genuine choices,
  self-direction, respect for competence) sustains motivation. The "pathological
  demand avoidance" / "persistent drive for autonomy" literature describes a
  profile where *any* externally-imposed demand — including self-imposed ones, and
  including demands from an app — can trigger a threat response and avoidance.
  **[DI]** A system that issues directives, applies pressure, or frames tasks as
  obligations may provoke avoidance in exactly the users it aims to help. Offering,
  suggesting, and leaving control with the user is safer.
- **[E]** Autistic burnout (Raymaker 2020, qualitative; AASPIRE): a syndrome of
  prolonged (3+ months) exhaustion, loss of function (including executive/daily-
  living function), and reduced tolerance to stimuli, attributed to chronic
  life-stress and a **mismatch between expectations and abilities without adequate
  supports** — often driven by sustained masking/camouflaging. **[DI]** A "life OS"
  that ratchets up expectations, surfaces every shortfall, or demands constant
  interaction could contribute to this load rather than relieving it. The system
  should degrade gracefully during low-capacity periods.

### 4.6 Stereotype cautions

- Autistic people are highly heterogeneous. Not all have sensory hyper-reactivity,
  not all value routine equally, not all share communication preferences, and
  strengths are not universal either. Verbal ability, support needs, and
  co-occurring conditions vary enormously. Any MELFINA abstraction must be
  *configurable to the individual*, not keyed to "autism" as a monolith (§15).

---

## 5. OCD findings

**Mission priority section.** OCD is where well-intentioned productivity features
carry the clearest risk of harm.

### 5.1 Core mechanism

- **[E/G]** OCD is maintained by a cycle: intrusive thought / doubt / "not-right"
  feeling → anxiety or distress → compulsion (overt or mental) → short-term relief
  (negative reinforcement) → strengthened urge to repeat. The compulsion *prevents*
  the corrective learning that the feared outcome is unlikely or tolerable.
- **[E/G]** First-line psychological treatment is Exposure and Response Prevention
  (ERP). Mechanistically reframed via **inhibitory learning theory**: exposure does
  not erase the fear association but builds a competing "safety" association;
  **safety behaviours and avoidance undermine this** by making the new learning
  context-dependent and non-generalising (Blakey & Abramowitz 2016; IOCDF).
- **[E]** Intolerance of uncertainty is a candidate *cognitive vulnerability* for
  OCD (qualitative review, Gillett 2018; Shihata et al.). Network analyses link the
  "I can't function when uncertain" facet of IU to the "can't control my thoughts"
  facet of OCD. Prospective IU (need for predictability) and inhibitory IU
  (paralysis under uncertainty) both feature.
- **[E]** Inflated responsibility (Salkovskis) and, for checking specifically,
  Rachman's cognitive theory (Rachman 2002): compulsive checking arises when
  someone who believes they carry special responsibility for preventing harm is
  *unsure* the threat is neutralised. Intensity scales with perceived
  responsibility × probability of harm × severity of harm. The behaviour is
  self-perpetuating (see 5.3).

### 5.2 "Not just right" experiences and incompleteness

- **[E]** A subset of OCD is driven less by feared catastrophe and more by
  **incompleteness / "not-just-right experiences" (NJREs)** — a sensory-affective
  sense that something is not yet correct, driving repetition until "rightness" is
  achieved (Coles et al. 2003). Meta-analysis (Sica et al. 2022, *JOCRD*): NJREs /
  incompleteness predict OC symptoms in clinical and community samples.
- **[DI]** Features that create a satisfying "click into place" — perfect
  completion states, everything-ticked, zero-inbox, a fully green grid — may
  provide exactly the transient "just right" hit that an incompleteness-driven
  loop feeds on, and the corresponding *dissonance* when something is unchecked.
  This is a specific, named risk, not a vague worry.

### 5.3 Checking and memory — the paradox

- **[E]** Repeated checking **worsens** memory confidence. Controlled experiments
  (van den Hout & Kindt 2003, and replications): repeatedly checking a virtual
  stove left recollections *less* vivid, *less* detailed, and confidence *lower*,
  despite unchanged accuracy. Mechanism: repetition → familiarity → conceptual
  (not perceptual) processing → less vivid memory → more doubt → more checking.
  Effects on memory confidence/vividness/detail are large.
- **[E]** Later work: repeated checking induces distrust specifically in *memory*
  (not perception/attention), and induces uncertainty about *future* threat.
- **[DI]** Any MELFINA feature that invites the user to *re-verify* something
  already recorded — re-open, re-read, re-confirm, "review your entries" — is
  pushing on this exact mechanism. Recording something **once** and having the
  system treat it as settled is protective; affordances for endless re-checking
  are not.

### 5.4 Reassurance seeking and accommodation

- **[E]** Providing reassurance gives large short-term anxiety relief but drives a
  cycle of escalating reassurance-seeking. **Family accommodation** (others
  adjusting behaviour to reduce the person's distress — answering the same
  question, participating in rituals) is **one of the strongest predictors of OCD
  severity and of poor treatment outcome**, in children and adults (Lebowitz; Wu
  et al.; PMC 2024 lifespan review).
- **[E, emerging]** **AI chatbots are a documented new reassurance vector.** A
  transdiagnostic model (npj Digital Medicine 2026) and clinician commentary
  (IOCDF "Digital Reassurance Seeking in OCD" 2026; ADAA; CHI 2026 "Reassurance
  Robots") describe why general-purpose chatbots are unusually potent here:
  always available, never socially fatigued, infinitely patient, generate *endless
  fresh variations* of an answer, and remove the natural social friction that would
  otherwise interrupt the loop. Users report hours/day of compulsive querying.
- **[DI/H]** An AI assistant inside a personal life system, with memory of the
  user's worries, is close to a worst-case configuration for this mechanism unless
  deliberately designed against it (rate/scope limits, refusal to re-answer
  settled questions, naming the pattern, deferring to the user's own prior
  decision). See §13.

### 5.5 Other patterns relevant to software

- **[E/G]** Compulsive **logging / tracking / list-making / ordering / arranging**
  can themselves be compulsions (symmetry/ordering is a recognised OCD dimension).
  Clinical app reviews explicitly warn that re-reading logs and monitoring anxiety
  scores can *become* the compulsion.
- **[G]** Avoidance is central: a person may avoid whole categories of task,
  decision, or content to prevent triggering an obsession. A system that forces
  engagement (mandatory fields, "you must categorise this", blocking progress until
  resolved) can collide with avoidance in a distressing way.
- **[G]** Perfectionism is clinically relevant in a subset (especially with NJRE /
  OCPD traits): "if it's not done perfectly it doesn't count." Metrics and
  completion states interact badly with this.

### 5.6 What this section is *not*

This is not a plan to treat OCD, screen for it, or push ERP through software.
MELFINA is not a clinical tool. The purpose here is purely to **identify
software-induced failure modes** so the design can avoid *manufacturing*
compulsions. Any actual therapeutic use is out of scope and would require clinical
involvement.

---

## 6. Overlap / conflict findings

The three profiles are **not** independent, and the overlap is where design gets
hard.

### 6.1 Co-occurrence is common

- **[E]** ADHD + autism ("AuDHD"): large co-occurrence; often-cited estimates
  around 30–40%+ in each direction (exact figures vary by sample and criteria).
  DSM-5 now permits the dual diagnosis. Combined EF burden tends to be additive.
- **[E]** Autism + OCD: reported comorbidity ranges widely (5–35%; ~17–25% in
  several samples) and **shrinks when raters carefully distinguish
  autistic repetitive behaviour from OCD compulsions**. The OCD+autism subgroup has
  earlier onset and a distinct symptom profile (more symmetry/ordering, checking,
  somatic/religious obsessions).
- **[E]** ADHD + OCD: co-occur more than chance despite being in some ways
  phenomenologically opposite (impulsivity vs. compulsivity; novelty-seeking vs.
  harm-avoidance). When comorbid, executive load and distress are higher.

### 6.2 Distinguishing similar-looking behaviours (matters for the data model)

- **[E/G]** Autistic routine vs. OCD compulsion: autistic routines are often
  **self-affirming, calming, ego-syntonic**, tied to sensory regulation or interest;
  OCD compulsions are usually **ego-dystonic, distressing, anxiety-driven**, done to
  prevent a feared outcome or discharge a "not-right" feeling. The *same outward
  action* (ordering objects, following a fixed sequence) can be either.
- **[E]** Repetitive behaviours in autism vs OCD differ in their associations with
  anxiety, executive function, and sensory processing — but not reliably in
  topography.
- **[DI]** MELFINA cannot and should not classify a user's behaviour as
  "compulsion" vs "routine." But it should avoid a data model that *forces* one
  interpretation — e.g. labelling all recurring actions "habits" to be "kept up,"
  or all sequences "checklists" to be "completed." The meaning is the user's to
  assign.

### 6.3 Conflicting support needs — the catalogue

This is the practical heart of the mission. Documented or strongly inferable
tensions:

| # | Need A | Need B (pulls opposite) | Source basis |
|---|--------|-------------------------|--------------|
| 1 | **Structure / routine / predictability** (autism, OCD-adjacent) | **Novelty / variation / flexibility**; sameness becomes aversive and under-stimulating (ADHD) | AuDHD literature [E/clinical]; delay-aversion & reward [E] |
| 2 | **Externalise memory & intentions** — write everything down (ADHD EF, PM) | **Excessive recording / re-checking becomes compulsion**; logs become checking objects (OCD) | Cognitive offloading [E]; checking→memory-distrust [E] |
| 3 | **Reminders & prompts** to bridge prospective-memory gaps (ADHD) | **Notification overload / alert fatigue / demand-avoidance / sensory intrusion** (all three) | PM deficits [E]; alert fatigue [E]; PDA [clinical]; sensory [E] |
| 4 | **Reduce decisions / simplify / hide complexity** (EF load, choice overload) | **Loss of information, loss of control, unpredictable hiding** (autism predictability, OCD need to know) | Choice overload [E]; COGA [G]; IU [E] |
| 5 | **Measurement & feedback** to create salience & progress (ADHD reward) | **Compulsive monitoring, perfectionism, self-judgement, gaming the metric** (OCD, emotional dysregulation) | Delay discounting [E]; gamification caveats [E]; NJRE [E] |
| 6 | **Automation** to remove friction and initiation cost (ADHD) | **Unwanted automation, loss of autonomy, unpredictable system behaviour, skill/agency erosion** (autism, PDA, automation-bias) | SDT/PDA [E/clinical]; automation complacency [E]; adaptable > adaptive [E] |
| 7 | **Completion states / closure** — the relief of "done" (Zeigarnik; ADHD) | **"Not-just-right" dissonance when incomplete; completion rituals** (OCD incompleteness) | Zeigarnik / Masicampo [E]; NJRE [E] |
| 8 | **Prompt engagement / accountability / nudging** (ADHD initiation) | **Pressure → demand avoidance; shortfall visibility → shame, burnout** (autism, emotional dysregulation) | Implementation intentions [E]; PDA [clinical]; autistic burnout [E] |
| 9 | **Rich context capture** for task resumption (ADHD context loss) | **Total capture → total noise; retrieval burden; capture friction kills the system** (general) | Memory-for-goals [E]; lifelogging critique [E]; PKM capture-friction [practitioner] |
| 10 | **Predictable, unchanging interface** (autism) | **Stale, boring, unmotivating; no novelty reward** (ADHD) | IU/predictability [E]; reward/novelty [E] |

- **[H]** A recurring resolution pattern in the AuDHD clinical/lived-experience
  literature is **"anchored flexibility"**: a small number of fixed anchor points
  (predictable) with free, low-structure space between them (flexible). Not
  established experimentally, but it recurs and is a reasonable design hypothesis.
- **[DI]** Because the conflicts are *individual* in balance — one user leans
  routine, another leans novelty; one finds metrics motivating, another finds them
  toxic — **the resolution cannot be a fixed design decision. It has to be a
  user-controlled dimension** (§15). This is possibly the single most important
  structural implication in this document.

### 6.4 Where the profiles *reinforce* each other (design can serve all three at once)

- **Predictable, deterministic system behaviour** serves autism (uncertainty),
  OCD (reduces doubt), and ADHD (trust that the system caught it, so the loop
  closes — Masicampo).
- **External structure held outside the head** serves ADHD (EF/PM), autism (EF),
  and OCD (less need to mentally rehearse).
- **User control / no surprise automation** serves autism (predictability), PDA/
  autonomy, OCD (control), and the automation-bias literature.
- **Low sensory / low interruption** serves autism (sensory, monotropism), ADHD
  (distractibility, resumption cost), and OCD (fewer triggers).
- **Calm, non-judgemental framing** serves emotional dysregulation / RSD (ADHD),
  burnout / masking (autism), and shame-perfectionism (OCD).

These convergences are where MELFINA can be unambiguously good.

---

## 7. Cognitive science findings (memory, externalisation, time, attention, motivation)

### 7.1 Cognitive offloading and external memory

- **[E]** Cognitive offloading — using physical action / the environment to reduce
  internal information-processing demand — is a normal, pervasive strategy (Risko &
  Gilbert 2016, *Trends Cogn Sci*). Three interacting components: (1) metacognitive
  self-assessment drives the *decision* to offload; (2) offloading then *reshapes*
  those metacognitive judgements; (3) offloading directly changes what the
  cognitive system can do.
- **[E]** *Intention* offloading (external reminders for delayed intentions):
  reviewed in Gilbert 2022. Key points: people who offload perform better on the
  target task; the decision to offload is biased (people sometimes under-offload
  relative to optimal, sometimes over-offload); offloading interacts with
  metacognitive confidence; populations with EF/memory challenges (ADHD, older
  adults, brain injury) benefit but their *calibration* about when to offload can
  be off.
- **[E]** Trust in the external tool is a moderator: people offload more to tools
  they trust, and mis-calibrated trust (either direction) degrades outcomes.
- **[H/DI]** Concern raised in the literature (and in the "memory paradox" / AI
  discourse): heavy offloading *may* reduce unaided memory or the metacognitive
  skill of knowing what you know. Not established for this population; flagged as an
  open question (§17). Design implication: the tool should be *reliable enough to
  trust* (so offloading is rational) while not *engineering learned helplessness*.

### 7.2 Unfinished tasks (Zeigarnik / Masicampo)

- **[E]** Zeigarnik (1927): interrupted/unfinished tasks stay more active in memory
  than completed ones. Modern replications are mixed on the memory-advantage claim
  but consistent on the *intrusion* claim.
- **[E]** Masicampo & Baumeister (2011, *J Pers Soc Psychol*): unfulfilled goals
  produce intrusive thoughts and measurably impair performance on a *subsequent
  unrelated* task requiring focus. Critically — **making a specific plan for the
  unfinished goal eliminates both the intrusions and the performance cost**, even
  though the goal remains undone. The mind appears to treat a *credible, specific
  plan* as a hand-off to a trusted process.
- **[DI]** This is the strongest single cognitive-science warrant for a capture /
  planning system: the value is not "productivity," it is **freeing working memory
  and attention from the background load of open loops** — *if* the user believes
  the system will surface the item at the right time (trust, again). A vague list
  the user doesn't trust provides little of this benefit.

### 7.3 Interruption, task resumption, memory-for-goals

- **[E]** Memory-for-goals model (Altmann & Trafton 2002): the currently-active
  goal must be *retrieved from memory*; its activation decays over time and
  suffers interference from other goals. Hence a **resumption lag** after
  interruption.
- **[E]** Longer and more cognitively demanding interruptions produce longer
  resumption lags (Monk, Trafton & Wagner 2008). **Cues at the point of
  interruption** and a brief opportunity to *rehearse* the suspended goal reduce
  the cost.
- **[DI]** For a monotropic / ADHD user the resumption cost is plausibly higher.
  Design leverage: (a) reduce unnecessary interruptions; (b) when the user *does*
  leave a task, help them capture a resumption cue ("what was I about to do next")
  cheaply; (c) on return, replay that cue rather than dumping them back cold.

### 7.4 Prospective memory

- **[E]** PM = forming, retaining, and executing a delayed intention at the right
  moment. Event-based cues (tied to a situation) generally outperform time-based
  cues; salient, specific cues outperform vague ones.
- **[E]** Reminders improve PM in lab and field — but see §8 for the field-study
  nuance and §10 for degradation with overuse.

### 7.5 Planning fallacy

- **[E]** People systematically underestimate how long *their own* tasks will take
  (Buehler, Griffin & Ross 1994: predicted 33.9 days vs actual 55.5 for theses).
  The bias is specific to one's own tasks (observers over-estimate). Cause: people
  build a best-case plan-based scenario and neglect their own base-rate history.
- **[E]** Partial remedies with empirical support: "reference-class forecasting"
  (explicitly recall how long *similar past* tasks took), unpacking a task into
  components, and having others estimate.
- **[DI]** If MELFINA ever helps with time estimates, its useful contribution is
  **feeding back the user's own history** ("last 3 times you scheduled 'practice
  session' you used 95 min, not 60"), not asking for a fresh guess.

### 7.6 Implementation intentions

- **[E]** If-then plans ("when X, then Y") — *d* = 0.65 for goal attainment; also
  protect against derailment (*d* = 0.77) (Gollwitzer & Sheeran 2006). They work by
  pre-delegating action initiation to an environmental cue, reducing reliance on
  in-the-moment executive control.
- **[DI]** A capture format that nudges toward *when/where* an action will happen,
  not just *what*, is aligned with this. But note tension #8: this must be an
  offered scaffold, not a mandatory field.

### 7.7 Choice overload / decision cost

- **[E]** Larger option sets can reduce the likelihood of *any* choice and lower
  satisfaction with the choice made (the effect is real but moderated — it's
  strongest with hard-to-compare options, no clear preference, and no default).
- **[DI]** For an EF-loaded user, *presenting* a task list of 40 items is itself a
  decision-cost event. Showing few things at once, offering a sensible default
  "next," and making "not now" frictionless are aligned with this — but the *hiding*
  must be predictable and reversible (tension #4).

---

## 8. HCI findings

### 8.1 Cognitive accessibility — W3C COGA "Making Content Usable"

The most directly applicable standards-body guidance. Eight design objectives
(W3C/WAI Working Group Note, *Making Content Usable for People with Cognitive and
Learning Disabilities*):

1. **Help users understand what things are and how to use them** — familiar
   patterns, consistent design, clear labels, recognisable icons.
2. **Help users find what they need** — clear navigation, search, important things
   prominent, short paths to key functions.
3. **Use clear and understandable content** — plain language, short blocks,
   literal (not metaphorical) phrasing, no double negatives, summaries.
4. **Help users avoid mistakes and know how to correct them** — prevent
   unintended actions, easy undo, forgiving forms, clear feedback, accept multiple
   input formats.
5. **Help users focus** — limit interruptions, keep the critical path short,
   reduce content volume, provide re-orientation cues when attention lapses.
6. **Ensure processes do not rely on memory** — no reliance on recall (e.g.
   passwordless auth, no "enter the 3rd character"), carry context forward through
   multi-step flows.
7. **Provide help and support** — easy access to real help, context-sensitive
   help, alternative formats, reminders and task-management support.
8. **Support adaptation and personalisation** — allow the interface to be
   simplified, allow extensions, respect user configuration, don't block
   assistive add-ons.

- **[G]** Cognitive functions COGA addresses explicitly: memory (short-term/
  working), attention & executive function, language processing, processing speed.
- **[DI]** Objectives 4, 5, 6, 8 are near-perfect matches for the MELFINA
  population. Objective 8 (adaptation/personalisation) reinforces §6.3 and §15:
  *personalisation is an accessibility requirement, not a nice-to-have.*

### 8.2 Notification and interruption design

- **[E]** Deferring notifications to task **breakpoints** (natural sub-task
  boundaries) rather than delivering immediately reduces frustration, resumption
  time, and errors; **coarser** breakpoints (bigger boundaries) → lower cost
  (Iqbal & Bailey 2008, CHI).
- **[E]** No consistent evidence that email/notification *batching* lowers stress
  (it may raise perceived productivity). So "batch everything" is not a guaranteed
  win — the mechanism that matters is *timing relative to the user's task state*,
  not batching per se.
- **[E]** A day without push notifications leaves people feeling *both* less
  distracted *and* more anxious / disconnected (Kushlev; "Productive, Anxious,
  Lonely" 2016) — removing prompts entirely has its own cost.
- **[DI]** Implication: user-controlled interruption, delivery aligned to task
  boundaries or explicit "I'm available now" states, a visible and predictable
  place to see "what would have interrupted me," and very few things allowed to
  break through unbidden.

### 8.3 Memory externalisation in practice / GTD

- **[E/theory]** Heylighen & Vidal (2008): GTD's effectiveness is explained by
  situated/embodied/distributed cognition — the brain offloads to the environment
  as external memory, action trigger, and source of affordances. GTD's core moves:
  capture everything out of the head; define the concrete *next action*; organise
  by context; review regularly to keep trust in the system.
- **Conflicting evidence / Gap:** GTD has essentially **no controlled outcome
  evidence** — the authors themselves note it resists measurement (no explicit
  priorities to optimise). It is a *theoretically well-motivated* practice, not an
  evidence-based one. Its "review everything regularly" step is also a potential
  checking surface for OCD (tension #2).

### 8.4 Reminder systems — the field-study nuance

- **[E]** Lab studies: reminders reliably help PM. Field studies are more sobering:
  location-based reminders (Sohn 2005 "Place-Its"; later work) are used but
  **"still not an effective tool"** in daily life — triggering is imprecise,
  context is hard to detect, and reminders fire at the right place but wrong moment.
- **[E]** Older-adult field data: reminders help *because* users compensate by
  checking them more under high cognitive load — i.e. the benefit depends on a
  user behaviour, not just the reminder firing.
- **[DI]** Don't assume "add a reminder" solves prospective memory. Reminder
  *content* (specific, actionable), *timing* (task boundary, right context), and
  *trust* matter more than the trigger type.

### 8.5 Adaptive vs adaptable interfaces

- **[E]** Findlater & McGrenere (2004) and follow-ups: static menus fastest,
  adaptable (user-arranged) next, adaptive (system-rearranged) slowest — but users
  **prefer adaptable** and dislike the unpredictability of adaptive. "Ephemeral"
  adaptation (highlighting rather than moving) mitigates some cost.
- **[E]** Personality/cognitive-load moderates: adaptive UIs help some users under
  high load and frustrate others.
- **[DI]** Strong support for §15's direction: MELFINA should let the *user*
  reshape the system; the system should not silently reshape itself. If the system
  ever adapts, it should *suggest* ("you always do X on Mondays — pin it?") and let
  the user accept, not just act.

### 8.6 Neurodivergent-led and participatory design

- **[G/E]** "Nothing about us without us." Scoping reviews (CHI 2025) find autistic
  adults are still under-involved in designing the tech built for them, and that
  researcher-defined outcomes have historically dominated over lived experience.
- **[DI]** For MELFINA this is straightforward: the user *is* the design authority.
  The requirements phase should privilege the user's own account of their frictions
  over any template derived from diagnostic categories (this is also §15's point).
- **[H]** "Scaffolding, not instructing" (neurodiverse participatory-sensemaking
  work): technology that *invites* interpretation rather than *imposing* it. A good
  north-star phrasing for the AI layer.

---

## 9. Assistive-technology findings

- **[G]** Mainstream AT patterns relevant here: external time displays; visual
  schedules / timelines with previewing of "what's next"; task-breakdown tools;
  checklists with carried-forward context; prompting/cueing systems; reduced-
  stimulation "focus" modes; text simplification.
- **[E, weak]** Evidence quality for most cognitive/neurodevelopmental AT is low:
  small samples, short duration, few RCTs, heavy reliance on child populations and
  on proxy report. The direction is generally positive; the magnitude and
  durability are uncertain.
- **[E]** Visual/graphical schedules have the strongest evidence base (largely in
  autistic children, for transitions and independent task completion). Extending to
  self-directed adult use is reasonable **[DI]** but not directly evidenced.
- **[practitioner signal, not evidence]** Tools *marketed* for this population and
  what their design bets are:
  - **Tiimo** — visual timeline, pictograms, colour, on-screen time
    representation; bet: concreteness reduces time-blindness and transition
    friction.
  - **Goblin Tools ("Magic ToDo")** — AI breaks a vague task into micro-steps,
    adjustable "spiciness"; bet: the initiation barrier is *task ambiguity*.
  - **Amazing Marvin** — heavily configurable task manager explicitly targeting
    ADHD; bet: no single workflow fits, so expose everything as toggles.
  - **Habitica** — full gamification (XP, avatars, party accountability); bet:
    external reward scaffolding. Note §11's caveats and community reports of
    burnout / obligation.
  - **Routine, Sunsama, Akiflow** — "calendar + tasks + daily planning ritual";
    bet: a structured daily planning/shutdown ritual externalises prioritisation.
- **[DI]** Convergent design bets across the credible tools: **visual/concrete
  representation of time; task decomposition; heavy configurability; a lightweight
  recurring "planning" touchpoint.** Divergent / contested bet: **gamification.**

---

## 10. Productivity-system failure modes

Why conventional systems fail — especially for this population.

### 10.1 Capture friction

- **[E/practitioner]** If getting an item *into* the system costs more than a few
  seconds or one decision ("which project? which tag?"), capture is abandoned, and
  once capture is unreliable the whole system loses trust (and with it the
  Masicampo benefit). This is the most commonly cited PKM/task-system failure
  point. **[DI]** Capture must be near-zero-friction and *defer* categorisation.

### 10.2 Over-structuring / maintenance collapse

- **[practitioner, strong consensus]** Elaborate hierarchies, taxonomies, and
  templates built up front become a maintenance burden that collapses. "Emergent
  organisation" (structure grows from use) is the widely-recommended alternative.
- **[DI]** For an OCD-adjacent user, an elaborate structure is *also* a large
  surface for "is everything filed correctly?" checking. Minimal imposed structure
  is protective on two fronts.

### 10.3 The metric becomes the goal

- **[E]** Streak/points systems convert an intrinsic goal ("learn," "practise
  piano") into a loss-avoidance behaviour ("don't break the streak"). Engagement
  can stay high while the original purpose hollows out; "streak creep" and
  gamification burnout are documented in user populations (Duolingo studies,
  Decision Lab analysis). **[DI]** Contraindicated as a default for this user.

### 10.4 Self-tracking abandonment ("the device in the drawer")

- **[E]** Personal-informatics research (Li 2010 stage model; Epstein 2015 "lived
  informatics"; Epstein 2016 "Beyond Abandonment"; Clawson 2015): people lapse by
  forgetting to track, upkeep burden, deliberately skipping, or suspending;
  lapses usually start at the *collection* stage; abandonment is normal and not
  necessarily failure. Designs should **treat lapses as expected**, make re-entry
  painless, and not punish gaps.
- **[DI]** No guilt UI for gaps. No "you haven't logged in 6 days." Re-entry after
  a lapse should feel identical to normal use.

### 10.5 Notification-driven systems

- **[E]** See §8.2. Systems that lean on push to drive behaviour hit alert fatigue:
  responsiveness to a repeated alert can drop ~30% per repetition (clinical
  decision-support literature, e.g. Ancker 2017). The alerts that matter get buried
  with the ones that don't.

### 10.6 Rigid methodology imposition

- **[G/DI]** GTD, time-blocking, Pomodoro, bullet-journaling etc. each encode
  assumptions (stable context, predictable energy, willingness to follow a
  ritual). For fluctuating capacity (ADHD energy variability, autistic burnout
  cycles) a system that *requires* the methodology to be followed to stay coherent
  will break during exactly the periods the user most needs support.
- **[E, tangential]** Pomodoro's fixed 25-minute interval has no special empirical
  status; the useful ingredients (a defined start, a bounded commitment, planned
  breaks) are generic. Forcing a break mid-flow can be costly for a monotropic
  user (§7.3).

### 10.7 Planning that ignores the user's own history

- **[E]** Planning fallacy (§7.5). Systems that ask "how long will this take?" and
  take the answer at face value bake in the underestimate, producing chronically
  over-stuffed days → shortfall → shame (emotional dysregulation) → abandonment.

### 10.8 All-or-nothing completion models

- **[E/DI]** A binary done/not-done model plus visible aggregate completion (a
  progress bar, "3/10 today") interacts badly with perfectionism and incompleteness
  (§5.2, §5.5). Partial progress that "doesn't count" is demotivating for the ADHD
  reward system *and* dissonant for the OCD incompleteness loop.

---

## 11. Gamification and metrics — dedicated analysis

Because the mission flags this and because it's where "help ADHD salience" most
directly collides with "harm OCD/perfectionism."

- **[E]** Meta-analysis (Sailer & Homner 2020, *Educational Psychology Review*):
  gamification effects — cognitive *g* ≈ 0.49, motivational *g* ≈ 0.36,
  behavioural *g* ≈ 0.25. **The cognitive effect survived high-rigour subgroups;
  the motivational and behavioural effects were less robust.**
- **[E]** Hamari, Koivisto & Sarsa (2014) review: effects "generally positive but
  highly dependent on context and users," with frequent **novelty effects** (gains
  fade as the mechanic becomes familiar) and methodological weakness.
- **[E]** Meta-analysis on motivation specifically (2023): gamification raised
  intrinsic motivation and perceived autonomy/relatedness but had **minimal effect
  on competence** — i.e. it can make an activity feel better without making the
  person better at it.
- **[E]** Over-justification risk: extrinsic reward layered onto an
  already-intrinsically-motivating activity (piano practice, learning) can
  *reduce* intrinsic motivation once the reward is salient. Directly relevant given
  the user is a musician.
- **[E]** Streaks specifically operate via **loss aversion** (losing a streak hurts
  ~2× more than an equivalent gain feels good) and convert long-horizon goals into
  short-horizon loss-avoidance. Documented downsides: anxiety about breaking the
  streak, "streak vacation" gaming, continuing the behaviour after the underlying
  goal is gone, burnout.
- **[E/§5]** Metrics + perfectionism + incompleteness: a visible score, grid, or
  completion percentage provides both the "just right" hit and the dissonance it
  feeds on; historical stats become a re-checking / rumination surface.

**Synthesis [DI]:**
- Gamification is **not** an appropriate default for this user profile.
- If any progress representation exists, safer forms are: *private*, *non-
  comparative*, *non-streak*, *non-punitive*, *forgettable* (no aggregate history
  demanding maintenance), and *user-disableable*.
- The legitimate underlying need — near-term salience for non-salient tasks
  (§3.4) — is better served by making the *task itself* smaller and its *next
  step* concrete than by attaching an external point economy.

---

## 12. Privacy / local-first findings

- **[E, as a design paradigm]** "Local-first software" (Kleppmann, McGranaghan,
  Nicholas, Warren; Ink & Switch / Cambridge, 2019). Seven ideals: **fast**
  (no network round-trip), **multi-device**, **offline**, **collaboration**,
  **longevity** (data outlives the vendor), **privacy** (end-to-end by default),
  **user control** (vendor can't restrict access/use). CRDTs are offered as *one*
  technical route to multi-device sync; they are not required for a
  single-user-primary system.
- **[G]** Data-minimisation and privacy-by-design are codified principles (e.g.
  GDPR Art. 5 & 25): collect only what's needed for a stated purpose, retain only
  as long as needed, default to the most privacy-protective configuration.
- **[G/DI]** For a system holding a person's whole inner life, relevant properties:
  data stored in open, inspectable, documented formats; full export; local
  encryption at rest; explicit and revocable access for any component (especially
  an AI layer or any sync); an audit trail of what was read/changed/sent; the
  ability to run fully offline forever; no telemetry by default.
- **[E, related]** MyLifeBits / total-capture critique (Sellen & Whittaker 2010,
  *CACM*, "Beyond Total Capture"): capturing everything produced data that was
  "in, very little usable out." Retrieval, not capture, is the bottleneck. Design
  should be grounded in how human memory actually cues and reconstructs, not in
  exhaustive recording.
- **[DI]** Privacy here is not only an external-threat question. Given §5, the
  *user's own* future access to exhaustive logs of their worries, checks, and
  shortfalls is a design consideration: what is retained, how long, how visible,
  and how easily re-surfaced are OCD-relevant choices as much as security choices.

---

## 13. AI-related findings

### 13.1 Automation, trust, and human factors (pre-LLM, still applies)

- **[E]** Levels/stages of automation (Parasuraman, Sheridan & Wickens 2000):
  automation is a continuum across four stages (information acquisition, analysis,
  decision selection, action) and multiple levels within each. Higher automation
  buys speed/effort but costs situation awareness, and creates **complacency** and
  **automation bias**.
- **[E]** Automation bias / complacency (Parasuraman & Manzey 2010): with reliable
  automation, people attend less to raw data and get **slower and less accurate at
  catching automation failures over time**, especially under multitask load. Errors
  of *omission* (missing what the automation missed) and *commission* (following
  the automation against contrary evidence).
- **[E]** Higher degree of automation also drives **skill/agency erosion** — a
  particular concern for a tool meant to *support* rather than *replace* the user's
  own executive function (see §7.1 offloading concern).

### 13.2 Mixed-initiative interaction

- **[E/G]** Horvitz (1999, CHI), "Principles of Mixed-Initiative User Interfaces" —
  12 principles. Most load-bearing for MELFINA:
  - Consider *uncertainty* about the user's goal; don't act as if you know.
  - Consider the *cost/benefit* of acting, including the cost of a wrong guess and
    of interrupting.
  - Only take *automated action when the expected value clearly exceeds* that of
    doing nothing or merely asking.
  - Make it easy to *invoke* and *dismiss* the agent; maintain a memory of recent
    interactions; allow *efficient correction*.
  - Do not *scope-creep* the automation beyond what it can do well.
- **[DI]** This maps cleanly onto §5.4 (AI reassurance), §4.5 (autonomy/PDA), and
  §8.5 (adaptable > adaptive): **suggest, don't act, when goal or value is
  uncertain; make acting rare and always reversible.**

### 13.3 LLM-specific: overreliance, calibration, hallucination

- **[E]** LLM fluency and confident tone **lower users' vigilance** and drive
  overreliance, even when the model is wrong; anthropomorphism and explanations
  further increase (possibly unwarranted) reliance (multiple CHI 2024–2025
  studies).
- **[E]** LLMs are often **poorly calibrated** — even strong models can be ~30% off
  target on confidence in specialist domains. They will state wrong things with the
  same fluency as right things.
- **[E]** Well-calibrated *linguistic* uncertainty ("I'm not certain, but…") can
  improve appropriate reliance — *if* the expressed uncertainty tracks actual
  correctness. Poorly-calibrated hedging just adds noise.
- **[E, recent]** "AI assistance reduces persistence and hurts independent
  performance" (2026 arXiv): offloading problem-solving to an AI reduced users'
  own subsequent performance and willingness to persist — an empirical instance of
  the §7.1 concern.

### 13.4 AI in a personal support system — synthesis

- **When AI should DO** (take an autonomous action): only for reversible,
  low-stakes, high-certainty, explicitly pre-authorised actions where the cost of a
  wrong guess is trivial and the user has opted in. Example class: reformatting a
  captured note, attaching an obvious timestamp. **[DI]**
- **When AI should SUGGEST:** when it has a useful pattern-based guess but the goal
  or value is uncertain, or the action is not trivially reversible. Default mode for
  anything touching planning, prioritisation, or the user's data model. **[DI]**
- **When AI should ASK:** when it needs information only the user has, or when it
  detects a possible conflict (e.g. the user is asking the same worry-question
  repeatedly — §5.4). **[DI]**
- **When AI should DO NOTHING:** when expected value doesn't clearly beat silence;
  when the user is in a low-capacity state; when the request pattern looks like
  reassurance-seeking or rumination and answering again would likely feed a loop;
  when it is uncertain and the stakes are real. **[DI/H]**
- **OCD-specific safeguards [H, from clinician commentary not controlled trials]:**
  the assistant should be able to recognise and *name* a reassurance-seeking
  pattern rather than satisfy it; decline to re-litigate a decision the user
  already recorded ("you decided this on Tuesday — do you want to re-open it, or
  trust past-you?"); avoid generating endless reworded reassurance; not keep the
  user in conversation.
- **Autonomy safeguards [DI]:** the assistant frames outputs as options; never
  issues directives; never expresses disappointment; is fully optional (the system
  must be completely usable with the AI layer disabled).
- **Transparency [G/DI]:** every AI action is logged and attributable; the user can
  see what data the AI read; uncertainty is expressed honestly and calibratedly;
  the AI does not impersonate certainty it lacks.

---

## 14. Prior art

### 14.1 Methodologies

| System | Problem it targets | Core assumptions | What works | What fails / risks for this user |
|---|---|---|---|---|
| **GTD** (Allen) | Open loops occupying the mind | Stable contexts; willingness to do weekly review; no explicit priorities | Capture + next-action + trust-the-system is cognitively well-founded (Heylighen & Vidal 2008; Masicampo 2011) | No outcome evidence; weekly review is a checking surface; collapses if review lapses |
| **Bullet Journal** (Carroll) | Rapid capture, migration forces triage | Analog; daily/monthly ritual; manual migration | Low-tech, flexible, "migration" naturally sheds dead tasks | Manual upkeep burden; can become an aesthetic/perfectionism sink |
| **PARA / "Second Brain"** (Forte) | Where to file digital notes | Actionability-based hierarchy | A consistent filing heuristic | Up-front taxonomy; filing decisions = friction + checking |
| **Time-blocking** | Making intentions concrete in time | Predictable energy/attention; accurate duration estimates | Externalises when, not just what; implementation-intention-like | Planning fallacy → overstuffed days; rigid vs ADHD variability; guilt on slippage |
| **Pomodoro** | Task initiation, sustained attention | 25-min unit is meaningful; breaks are safe to take | Bounded commitment lowers initiation barrier | Fixed interval interrupts monotropic flow; arbitrary |

### 14.2 Tools (design bets — see §9 for detail)

- **PKM (Obsidian, Logseq, Roam):** local Markdown / block model, bidirectional
  links, plugin ecosystems. *Works:* longevity (plain files), flexibility,
  emergent structure. *Fails:* capture still requires a "where does this go"
  decision in practice; link-graph tending can become compulsive; power = config
  burden.
- **Neurodivergent-first (Tiimo, Goblin Tools):** visual time, pictograms, AI task
  breakdown, adjustable detail. *Works:* concreteness, decomposition, low reading
  load. *Fails/limits:* cloud-dependent; narrow (planning *or* breakdown, not a
  life model); AI breakdown quality varies.
- **ADHD task managers (Amazing Marvin):** maximal configurability. *Works:* fits
  many workflows. *Fails:* the configuration itself is an EF task; paradox of
  choice.
- **Gamified (Habitica):** full external reward economy. *Works:* initial novelty
  boost. *Fails:* §11 — novelty fades, obligation/anxiety, streak burnout,
  over-justification.
- **Lifelogging / Quantified Self (MyLifeBits, wearables):** capture everything,
  analyse later. *Works:* as a research probe. *Fails:* retrieval bottleneck
  (Sellen & Whittaker 2010); most data never used; capture burden; for this user,
  exhaustive self-data is an OCD-relevant liability.

### 14.3 What MELFINA can learn / should avoid

- **Learn:** capture-first with deferred organisation; concrete external
  representation of time; task decomposition to a concrete next step; plain files
  / open formats for longevity; a light recurring planning touchpoint; emergent
  rather than imposed structure; heavy user configurability *of behaviour*.
- **Avoid:** up-front taxonomies; mandatory categorisation; weekly-review-as-
  linchpin; streaks / points / badges / comparative metrics; rigid time units;
  total capture; guilt/shortfall UI; cloud dependency; any single imposed
  methodology; automation that acts without asking.

---

## 15. Personalisation findings

- **[E]** Adaptable (user-controlled) beats adaptive (system-controlled) for
  preference and predictability, even at a small efficiency cost (§8.5; Findlater &
  McGrenere 2004).
- **[G]** W3C COGA objective 8 makes personalisation/adaptation an explicit
  cognitive-accessibility requirement.
- **[E/G]** Self-determination theory + PDA literature: autonomy support (choice,
  self-direction) sustains motivation and reduces demand-avoidance; system-imposed
  behaviour change invites resistance.
- **[E]** Autistic-trait and general-population EF work shows *perceived* /
  *self-experienced* executive difficulty diverges from task-measured difficulty —
  so personalisation should be driven by **the user's report of their own
  friction**, not by inferred category membership.
- **[practitioner]** Amazing Marvin's existence and positioning is a market signal
  that "one workflow per person" is real; its main criticism is that exposing
  *everything* as a toggle makes setup itself an executive task.

**Synthesis — "designed for a category" vs "adapts to the individual":**

- **[DI]** MELFINA should **not** ship an "ADHD mode / autism mode / OCD mode."
  The three profiles conflict (§6.3); a category preset would bake in a
  conflict-resolution the individual didn't choose, and would also mis-serve the
  many people who don't match the stereotype.
- **[DI]** Instead, the conflict axes from §6.3 become **explicit, user-set
  dimensions** — e.g. how much structure vs openness; whether progress is shown at
  all; how (and whether) the system prompts; how much the AI initiates; how much
  history is retained and surfaced. Sensible defaults, everything adjustable, no
  dark patterns pushing one way.
- **[DI/H]** Personalisation should be **gradual and low-stakes**: start minimal,
  let the user turn things on as they discover a need, and make every setting
  reversible without penalty. A big up-front configuration wizard is itself an EF
  barrier and a "did I set this up right?" checking surface.
- **[DI]** If the system *learns* patterns, it should surface them as
  **suggestions the user confirms**, never silent adaptation (§8.5, §13.2). And it
  should be able to *forget* / be corrected easily.
- **[E/DI]** "Scaffolding, not instructing" (§8.6) as the governing stance: the
  system holds structure *for* the user to use as they see fit, rather than
  prescribing how they should run their life. This is also how the eventual music /
  creative-practice use should be approached — as the user's own structures, not a
  built-in "practice methodology."

---

## 16. Design implications (consolidated) — RESEARCH-DERIVED, NOT REQUIREMENTS

> Everything in this section is **[DI]** or **[H]**. It is what the evidence
> *suggests a designer should consider*. It is **not** a feature list, **not** a
> spec, and **not** a commitment. The PERSONAL REQUIREMENTS phase decides what, if
> any, of this MELFINA actually does.

### 16.1 Foundational stance

1. **External structure without enforcement.** Hold memory, time, plans, and
   context so the user doesn't have to; do not nag, score, or enforce.
   (§3.1, §7.1, §7.2, §4.5)
2. **Suggest, don't impose.** Autonomy-supportive framing throughout; the user is
   the authority; the AI and any automation default to offering. (§4.5, §8.5,
   §13.2, §15)
3. **Predictable and deterministic.** The system never silently rearranges,
   hides, or acts. State changes are visible and previewable. (§4.3, §5.1, §13)
4. **Personalisation is an accessibility requirement**, and it takes the form of
   explicit user-set dimensions on the §6.3 conflict axes — not diagnostic
   presets. (§6.3, §8.1-obj8, §15)
5. **Degrade gracefully.** The system must stay coherent and non-punishing during
   low-capacity periods; lapses are expected, re-entry is frictionless. (§4.5,
   §10.4)

### 16.2 Capture and memory

6. Near-zero-friction capture; categorisation deferred or optional. (§10.1)
7. Record once; treat as settled. Minimise affordances to re-verify recorded
   items. (§5.3)
8. Support capturing a *resumption cue* when leaving a task; replay it on return.
   (§7.3)
9. A captured item should be able to carry a concrete *next action* and optionally
   a *when/where* — offered, never mandatory. (§3.5, §7.6)
10. Retrieval, not capture, is the hard problem — design for how the user will
    *find and reconstruct*, not for completeness. (§12)

### 16.3 Time

11. Represent time concretely and visibly (duration, elapsed, remaining) rather
    than requiring internal estimation. (§3.2)
12. Preview "what's next"; make transitions gentle and announced. (§4.3)
13. For any duration estimate, feed back the user's *own history* for similar
    work; don't just accept a fresh guess. (§7.5, §10.7)

### 16.4 Prompts / notifications

14. Very few things may interrupt unbidden; the user controls interruption.
    (§8.2, §4.4)
15. Prefer delivery at task boundaries or explicit "available now" states.
    (§8.2)
16. A visible, predictable place to see "what would have prompted me." (§8.2)
17. Vary/decay-resist prompts; assume repeated identical alerts lose force.
    (§10.5)
18. Prompts are event/context-anchored and specific where possible, not just
    time-based. (§3.3, §8.4)

### 16.5 Progress, completion, metrics — RESTRAINT SURFACE

19. No streaks, points, badges, comparative stats, or aggregate completion
    percentages by default. (§11)
20. Avoid binary all-or-nothing completion as the only model; allow partial
    progress that "counts." (§10.8)
21. Any progress representation: private, non-comparative, non-punitive,
    non-accumulating, user-disableable. (§11)
22. Be cautious with "perfect" completion states (all-clear, zero-inbox,
    all-green) — they can feed incompleteness/"just-right" loops. (§5.2)
23. Historical logs of the user's own activity are a re-checking / rumination
    surface — retention, visibility, and re-surfacing are deliberate choices.
    (§5.3, §5.5, §12)

### 16.6 AI layer

24. Fully optional; system completely usable with AI disabled. (§13.4)
25. DO only reversible/trivial/pre-authorised actions; SUGGEST for anything
    touching planning, priorities, or the data model; ASK for user-only info or on
    detected conflict; DO NOTHING when value doesn't beat silence or capacity is
    low. (§13.2, §13.4)
26. Recognise and *name* reassurance-seeking / rumination patterns instead of
    feeding them; defer to the user's own recorded prior decisions; don't generate
    endless reworded reassurance; don't keep the user in conversation. (§5.4,
    §13.4)
27. Express calibrated uncertainty; log every action and every data access; never
    impersonate certainty. (§13.3)
28. Never express disappointment, pressure, or judgement. (§4.5, §3.1-emotion)

### 16.7 Data / architecture posture (philosophy only — no tech choices)

29. Local-first: offline-forever, fast, user owns the data, outlives any vendor.
    (§12)
30. Open, inspectable, documented storage format; full export. (§12)
31. Local encryption at rest; explicit, revocable, audited access for every
    component including AI and any sync. (§12)
32. No telemetry by default; data minimisation and privacy-by-design as defaults.
    (§12)

### 16.8 The life model (most important, least evidenced)

33. **[H]** Do not model the user's life as "tasks + habits + projects" by
    default — those framings carry the failure modes above (habits→streaks,
    tasks→binary completion, projects→taxonomy). Seek a smaller, more neutral set
    of primitives (candidates to *investigate*, not adopt: a "thing on my mind" /
    open loop; a "session" of engaged activity; a "note/artifact"; a "thread"
    connecting them over time; a "commitment" to someone). Music practice,
    repertoire, ideas, and performances should fall out of these as *cases*, per
    the project brief. (§6.2, §10, §14, §15)
34. **[H]** The data model must not force "compulsion vs routine," "obligation vs
    choice," or "success vs failure" interpretations onto the user's entries. Keep
    primitives descriptive; let meaning be user-assigned. (§6.2)

---

## 17. Open research questions

Carried forward. Roughly prioritised.

1. **Conflict resolution:** For the §6.3 tensions, is "anchored flexibility" (few
   fixed points + open space) actually better than alternatives, and how should the
   balance be exposed and set? Essentially unstudied for this overlap.
2. **Offloading dependency:** Does heavy, sustained cognitive offloading help or
   harm this population over months/years — unaided memory, metacognition, agency?
   Current evidence is short-term and general-population.
3. **Assistive-feature efficacy in adults:** Do visual timers, task decomposition,
   context reminders, body doubling, and visual schedules actually improve outcomes
   for *self-directed adults* with this profile (not children, not proxy-report)?
4. **Reminder design for reminder-fatigued users:** What content/timing/modality
   keeps prospective-memory support effective without habituation or demand
   response?
5. **AI reassurance safeguards:** Do the §13.4 safeguards (naming the pattern,
   deferring to prior decisions, refusal to re-answer) actually reduce compulsive
   querying, or do they frustrate legitimate use? Needs testing with real users.
6. **Progress representation:** Is there *any* form of progress feedback that
   delivers ADHD salience benefit without OCD/perfectionism cost, or is the safe
   answer simply "none"?
7. **The life-model primitives:** Does a small neutral primitive set actually
   accommodate real neurodivergent life (including music) better than
   task/habit/project? This is answerable partly by the requirements phase and
   partly by prototyping much later.
8. **Capacity-state adaptation:** Can/should the system detect low-capacity periods
   (and how, without surveillance) to shift its own behaviour — or should that only
   ever be a manual user switch?
9. **Music / creative practice:** How do deliberate-practice structure, repertoire
   tracking, and creative ideation map onto general life primitives without a
   bespoke module — and what does the (thin) research on musicians' practice
   organisation and on flow say here? (Not investigated this pass.)
10. **Longevity of engagement:** What predicts whether *this* kind of user keeps
    using *this* kind of system past the novelty period?

---

## 18. Preliminary principles for MELFINA (research-informed, provisional)

> Provisional. Subject to revision by the user and by later phases. These are
> *design principles*, distinct from the project's existing *operating principles*
> in `PROJECT_STATE.md`.

- **P1 — Scaffold, don't steer.** MELFINA holds structure for the user to use;
  it does not prescribe how they should live, work, or feel.
- **P2 — The user is the authority.** Every interpretation, priority, category,
  and setting is the user's. The system offers; the user disposes.
- **P3 — Predictable over clever.** Deterministic, previewable, no silent action
  or rearrangement. Surprise is a cost, not a delight.
- **P4 — External memory you can trust.** Capture is effortless; recorded things
  stay recorded and settled; the system reliably resurfaces things at the right
  time so the user's mind can let go.
- **P5 — Time made visible.** Duration and "what's next" live outside the head, in
  concrete form.
- **P6 — Quiet by default.** Minimal interruption; the user controls what breaks
  through; no notification-driven behaviour change.
- **P7 — No scorekeeping.** No streaks, points, comparative metrics, or guilt.
  Progress, if shown at all, is private, gentle, partial-credit, and optional.
- **P8 — Restraint at the compulsion surfaces.** Completion, confirmation,
  history, metrics, and re-checking are treated as hazards and designed with
  deliberate minimalism.
- **P9 — Graceful degradation.** Works during low-capacity periods; lapses are
  normal; re-entry is free of friction and free of judgement.
- **P10 — Personalisation instead of diagnosis.** No category modes. The conflict
  axes are explicit, gently-defaulted, fully adjustable user dimensions.
- **P11 — The AI is optional and restrained.** Suggests far more than it acts,
  refuses to be a reassurance oracle, never judges, logs everything, and can be
  switched off entirely with no loss of core function.
- **P12 — Local, open, durable.** The user's whole life-data is local-first,
  encrypted, in open formats, fully exportable, and outlives any software.
- **P13 — Neutral primitives.** The underlying model avoids task/habit/project
  framings that carry known failure modes; it seeks a small descriptive set from
  which specific uses (including music) emerge as cases.

---

## 19. The boundary — RESEARCH FINDING → POSSIBLE DESIGN IMPLICATION → [STOP]

```
   RESEARCH FINDING                          POSSIBLE DESIGN IMPLICATION
   (§3–§15, tagged [E]/[G])                  (§16, tagged [DI]/[H])

   ADHD working memory & PM deficits    →    hold memory/plans/context externally
   persist into adulthood [E]

   Writing a specific plan removes the  →    a trusted capture+resurfacing system
   intrusion of an open loop [E]             frees attention even before doing

   Repeated checking worsens memory     →    record once; minimise re-verify
   confidence [E]                            affordances

   Reassurance/accommodation predict    →    AI must not be a reassurance oracle;
   worse OCD outcomes; AI is a new           name the pattern, defer to prior
   vector [E]/[G]                            decisions

   IU ↔ anxiety in autism (r≈.62) [E]   →    deterministic, previewable, no
                                            surprise; predictability as a feature

   Streaks convert intrinsic goals to  →    no streaks/points/comparative metrics
   loss-avoidance; gamification              by default
   effects small & fragile [E]

   Users prefer adaptable to adaptive  →    user reshapes system; system suggests,
   [E]; autonomy support reduces            never silently adapts or directs
   demand avoidance [E/G]

   Conflicting needs across the three  →    conflict axes = explicit user-set
   profiles [E/clinical]                     dimensions, not diagnostic presets

   Local-first is a coherent paradigm  →    offline-forever, open formats, user
   [E]                                       owns data, encrypted, no telemetry

                          ┌─────────────────┐
                          │     [STOP]       │
                          └─────────────────┘

   The next phase (PERSONAL REQUIREMENTS) decides — WITH THE USER — which of
   these implications become actual requirements, in what priority, and how the
   user's individual balance sits on each conflict axis. Nothing in §16/§18 is a
   commitment. No architecture, storage, interface, or feature is chosen here.
```

---

<a id="second-research-pass"></a>

# SECOND RESEARCH PASS

**RESEARCH MISSION 002 — deeper pass on the evidence gaps of Pass 1.**
Date: 2026-09-10. Status: complete, pending review.

**Purpose:** strengthen the evidence base, not redesign anything. No requirements,
no architecture, no code. Sections 20–32 below supplement §§1–19; where a Pass-1
claim is corrected, the change is stated explicitly and also logged in §31 and in
`RESEARCH CHECKPOINT 002`.

**New evidence tier used from here on:** **[U] — Unknown / insufficient evidence.**
A question where credible sources are absent, too weak, or too conflicting to
support any tier. `[DI]`/`[H]` are never promoted to `[E]`.

**Governing principle for this pass:** *preserve contradictions.* Where two
credible sources disagree, both are recorded and the disagreement is named.

---

## 20. Method and scope of this pass

- Second scoping search (Sept 2026), same source hierarchy. ~55 additional
  queries; ~15 full texts or systematic summaries read; several primary sources
  (Nature family) still abstract-only.
- Two Pass-1 primary sources were retrieved in full this time: **Demetriou 2018**
  (autism EF meta-analysis, PMC5984099) and **Macnamara & Maitra 2019** (deliberate
  practice replication, PMC6731745). Effect sizes below are now primary-verified.
- Still not a systematic review. Still English-language, US-indexed. Still no
  clinician or user contact (correct for this phase).
- New domains searched that Pass 1 did not cover: music/deliberate practice,
  academic personal-information-management (PIM), academic memory-augmentation
  prototypes, problematic-AI-use literature, self-tracking outcome meta-analyses,
  camouflaging/masking, adult autism life outcomes, JITAI (just-in-time adaptive
  intervention) evidence.

---

## 21. Adult-specific evidence (ADHD, autism)

Pass 1 flagged that most cognitive research is on children. This pass looked
specifically for adult data.

### 21.1 Autism — executive function (Demetriou 2018, now primary-verified)

- **[E]** 235 studies, 14,081 participants (6,816 autistic, 7,265 control). Overall
  **Hedges g = 0.48 (95% CI 0.43–0.53)** — moderate. EF impairment did **not
  fractionate**: concept formation/set-shifting, flexibility, fluency, planning,
  response inhibition, and working memory all showed small-to-moderate effects of
  similar size → "broad executive dysfunction," not a specific deficit.
- **[E] — correction to Pass 1 §4.2.** Pass 1 said EF impairment is "relatively
  stable across the lifespan." More precisely: **adults with autism show *smaller*
  EF effect sizes than children** ("adults with ASD perform better in EF than
  younger age groups"), with a notable adolescent dip for working memory. The
  impairment **attenuates with age but does not disappear** — it persists into
  adulthood at a reduced magnitude. Revised claim: *EF differences are present
  across the lifespan and diminish somewhat in adulthood.*
- **[E]** EF task performance had **poor diagnostic utility**; the one measure with
  clinical promise was the **BRIEF**, an *informant-report* questionnaire. The
  authors suggest **ecologically valid, report-based EF measures track real-world
  difficulty better than lab tests.** This converges with the Pass-1 findings that
  *self-perceived* EF (not lab EF) predicts anxiety and that autistic traits track
  perceived-but-not-measured EF. **[DI] unchanged and strengthened:** design should
  respond to the user's *reported* friction, not to any inferred or tested profile.
- IQ differences between groups did not moderate the effect. Studies using a
  single diagnostic instrument found smaller effects (broader inclusion → milder
  apparent dysfunction).

### 21.2 Autism — adult life outcomes

- **[E]** Adult outcomes are, on average, poor. Meta-analytic and review data
  (Howlin & Magiati; Steward): roughly **~20% of autistic adults meet a "good
  outcome"** definition (independent living + employment + meaningful
  relationships); ~50% of "high-functioning" autistic adults are employed;
  ~50% live with parents; many need substantial daily-living support.
- **[E]** **Daily-living skills and executive function independently predict adult
  outcomes** (living independently, employment, education, mental health). Adaptive
  function often lags well behind IQ ("the adaptive–cognitive gap").
- **[DI]** This is a substantive strengthening of the rationale for MELFINA:
  external executive/daily-living scaffolding for autistic adults is targeting a
  variable with demonstrated predictive weight on life outcomes — not a
  convenience.

### 21.3 Autism — camouflaging / masking

- **[E]** Meta-analysis and systematic reviews: **higher self-reported camouflaging
  is associated with worse mental health** — anxiety, depression, and (in several
  studies) suicidality; it is also linked to exhaustion and to delayed diagnosis.
  Findings for specific components (masking vs compensation vs assimilation) are
  **mixed**. Camouflaging has short-term social benefits (stigma avoidance,
  connection) and long-term costs. Cross-sectional; causal direction not
  established.
- **[DI]** Reinforces Pass-1 §4.5 (burnout) and P9 (graceful degradation): a system
  that increases the felt demand to "perform" or keep up appearances — even
  privately — works against this. A private, low-demand tool that reduces the
  executive cost of daily life is, on this evidence, plausibly protective; a
  demanding one is plausibly harmful. Not directly tested.

### 21.4 ADHD — adults

- **[E]** Working-memory deficits in adults confirmed again (Alderson 2013: verbal
  g ≈ 0.55, visuospatial g ≈ 0.49). Emotion dysregulation confirmed as a core
  adult feature (Beheshti 2020).
- **[E]** **CBT / organisational-skills training for adult ADHD** has repeated
  meta-analytic support: medium-to-large within-group and small-to-medium
  between-group symptom reduction, with functional gains specifically in **time
  management and organisation**. Environmental scaffolding is part of the standard
  of care (NICE NG87), not a workaround.
- **[E, children only] / [U, adults]** — **correction to Pass 1 §3.4/§7.6.**
  Implementation intentions were cited as *d* = 0.65 (general population) and
  presented as targeting the ADHD initiation gap. Direct evidence in ADHD exists
  **only in children** (Gawrilow & Gollwitzer 2008: implementation intentions
  normalised Go/No-Go response inhibition in children with ADHD; replicated in
  small studies). **Not studied in adults with ADHD.** Older-adult data indicates
  **executive-function prerequisites** — if EF is too impaired, forming and
  retrieving the if-then plan itself fails. Revised: *if-then planning is a
  promising scaffold with strong general-population evidence and child-ADHD
  evidence, but adult-ADHD efficacy is [U], and it may require baseline EF that
  fluctuates.*

### 21.5 Prospective memory — adult, condition-specific

- **[E, with a caveat]** Autism, adults: meta-analysis (Landsiedel 2017) — **large
  time-based PM impairment, small event-based impairment.** **Contradiction:** a
  2026 study found **no autistic/neurotypical PM difference after controlling for
  verbal ability** — i.e. some of the apparent PM deficit may be explained by other
  cognitive factors, not PM per se. Both stand.
- **[E]** ADHD: time-based PM impaired, event-based inconsistent (consistent with
  Pass 1).
- **[DI] unchanged:** event/context-anchored, specific cues are more robust than
  bare time alarms — but see §27 for the reminder-fatigue and field-efficacy
  caveats.

---

## 22. The combined profile — comorbidity and conflict, revisited

Pass 1 §6 is broadly upheld; two figures are refined and one caution is added.

### 22.1 Co-occurrence figures — refined

- **[E] — refinement of Pass 1 §6.1.** ADHD + autism: systematic-review pooled
  prevalence of **ADHD in autistic samples ≈ 22% (community) to ≈ 34% (clinical)**;
  some child samples run to 40–50%. Pass 1's "~30–40% each direction" was at the
  upper end and sample-dependent. **Robust finding: when ADHD and autism co-occur,
  adaptive functioning, quality of life, and symptom burden are all worse than
  either alone** (co-occurring conditions "amplify autistic symptoms" and impair
  adaptive function). Comorbidity in autism generally is **the norm** — ~70% of
  autistic people have ≥1 co-occurring psychiatric condition.
- **[E] — refinement of Pass 1 §6.1.** ADHD + OCD: co-occurrence is **lower and
  more uncertain than Pass 1 implied** — OCD in ADHD samples ≈ 1–13%; ADHD in OCD
  samples ≈ 0–23%; wide variance, partly because **inattention symptoms in OCD are
  mistaken for ADHD**. The disorders show **opposite fronto-striatal activation**
  (ADHD hypo-, OCD hyper-frontostriatal). They *can* co-occur, and when they do,
  treatment is genuinely complicated (may require SSRI + stimulant together;
  literature is sparse). Revised: *ADHD+OCD co-occurrence is real but less common
  and less well characterised than ADHD+autism or autism+OCD; the two are in some
  respects neurofunctional opposites.*
- **[E]** Autism + OCD (Pass 1 figure holds): ~17–25% in several samples, lower
  with careful differential rating; distinct subgroup (earlier onset,
  symmetry/ordering/checking).

### 22.2 The "don't assume single-condition findings transfer" caution — now evidenced

- **[E/DI]** The mission asked us not to assume findings from one condition apply
  to the combined profile. The comorbidity-amplification evidence (22.1) supports
  this: **the combined profile is not the union of three feature lists — it is
  generally *more* impaired in adaptive function, with interacting demands.**
  Nothing found in this pass measures the *specific* ADHD+autism+OCD combination on
  any design-relevant variable. That remains **[U]** and is the single largest
  evidence gap (see checkpoint).
- **[H]** The Pass-1 conflict catalogue (`CONFLICTS.md`) is therefore best treated
  as a set of *individually-calibrated axes*, because the interaction is
  unmeasured and the profile is heterogeneous even within itself.

### 22.3 Autonomy — now with meta-analytic support

- **[E] — upgrade from Pass 1 [E/clinical].** Self-determination theory autonomy
  support has **multilevel meta-analytic support** (e.g. 192-study MASEM; workplace
  and education meta-analyses): perceived need support → need satisfaction →
  autonomous motivation → better performance *and* wellbeing; need-thwarting
  predicts the maladaptive path. This moves "autonomy-supportive framing" (P2, P10,
  §16.1) from mostly-clinical-inference toward **evidence-supported** as a general
  principle. (Still not tested on this specific population or on a "life OS.")

---

## 23. Musician / classical-pianist context

New area. **Caution: musician-specific findings do not automatically generalise to
this user, and the deliberate-practice literature is actively contested.**

### 23.1 Deliberate practice — contested

- **[E]** Meta-analysis (Macnamara, Hambrick & Oswald 2014, *Psychological
  Science*): deliberate practice explained **~21% of performance variance in music**
  (26% games, 18% sports, <4% education/professions). Most variance is **not**
  explained by practice amount.
- **[E, primary-verified]** Replication of the foundational Ericsson 1993 violin
  study (Macnamara & Maitra 2019, *Royal Society Open Science*, n = 39,
  double-blind): deliberate practice accounted for **26% of the ability difference
  between skill groups, vs 48% in the original**; among the *most accomplished*
  players, practice amount did **not** distinguish the best from the merely good.
- **Contradiction — preserved.** Ericsson and colleagues dispute both the
  operationalisation of "deliberate practice" in these meta-analyses and the
  analytic choices (see the 2021 exchange in *Psychological Research*). The
  Ericsson camp maintains DP is dominant when strictly defined; the
  Macnamara/Hambrick camp maintains it is important but far from sufficient. **Both
  positions are live in the literature.**
- **[DI]** Whatever the exact share, **practice *quality*/structure matters and
  practice *amount* alone is a weak predictor.** A system that foregrounds *hours
  logged* would be optimising a weak variable (and see §11 over-justification, §29
  self-tracking). A system that supports *how* a session is structured (goals,
  problem identification, strategy, self-evaluation) is aligned with the stronger
  part of the evidence — **but this is a design inference, not a demonstrated
  intervention effect for this user.**

### 23.2 Self-regulated practice, planning, goal-setting

- **[E]** PRISMA review of self-regulated learning in advanced musicians (dos
  Santos Silva & Marinho 2025) and intervention studies: expert practice involves
  **planning, problem identification, strategy selection, progress monitoring,
  self-evaluation, and task decomposition**. Many students arrive with **little or
  no experience of goal-setting or planning** for practice.
- **[E]** Psychological-skills / SRL interventions with musicians produced: more
  realistic and hierarchical goals, more structured and goal-directed practice,
  greater practice efficiency and focus, more proactive performance preparation.
  Goal-setting correlated with self-efficacy, time management, self-evaluation,
  coping. (Small studies, mostly pre/post, some without controls.)
- **[E]** Students with **lower baseline self-regulation gain the most** from
  self-monitoring support.
- **[DI]** If MELFINA ever touches practice, the evidenced leverage points are
  **planning and structuring a session** and **reflective self-evaluation** — not
  time-logging. Offered, not required (autonomy, §22.3; PDA, §4.5).

### 23.3 Practice logging / self-monitoring in music

- **[E, mixed]** Practice diaries and audio/video self-review are the most-used
  tools in music-SRL research and can foster self-monitoring and strategy
  awareness. **But** the same literature notes the risk of turning "visible
  external evidence" into "invisible internal questioning" (i.e. self-surveillance
  and doubt) — a music-domain echo of the OCD checking concern (§5.3). Evidence is
  from intervention studies with students, not from a general "logging is good"
  claim.
- **[DI]** Consistent with Pass-1 restraint on history/metrics surfaces (§16.5):
  reflective review *chosen by the user* may help; a standing, always-visible
  practice log inviting comparison and re-inspection carries the self-monitoring
  risk.

### 23.4 Flow

- **[E]** Flow-condition–experience model: the antecedents are **skill–challenge
  balance, clear goals, and clear feedback**. In performing musicians these
  antecedents explain a large share of flow-state variance (one study: 54% in
  performing). Flow correlates **negatively with music performance anxiety** and
  positively with perceived performance quality and creativity.
- **[E]** Flow is **disrupted by negative/self-evaluative thoughts** and by
  interruption; a 2026 study distinguishes **boredom** (robustly linked to *poorer*
  practice outcomes) and **flow** (linked to *better* outcomes), while **trait/state
  mind-wandering during practice was *not* associated with outcomes** — a
  distinction worth keeping.
- **[DI]** For a monotropic and/or ADHD musician, this converges with Pass-1 §7.3
  (protect focus, minimise interruption). A tool that interjects during a practice
  or creative session — notifications, prompts, check-ins — is acting against a
  state that is hard to enter and valuable when entered. "Clear goals + clear
  feedback" are also the two flow antecedents a support tool could plausibly help
  set *before* a session (not during).

### 23.5 Perfectionism and music performance anxiety

- **[E]** Music performance anxiety (MPA) is common: **~15–60% of musicians report
  it; roughly a third have severe problems.** Classical musicians may be *more*
  perfectionistic than other performers (score fidelity, "no mistakes").
- **[E]** The **perfectionistic strivings vs perfectionistic concerns**
  distinction matters: *concerns* (fear of mistakes, doubts, others' evaluation)
  are consistently linked to MPA, negative affect, and cognitive anxiety;
  *strivings* (high personal standards) are more mixed and sometimes adaptive.
  Perfectionistic concerns overlap conceptually with OCD's intolerance-of-
  uncertainty and "not-just-right" constructs (§5).
- **[DI]** This is direct support for Pass-1 §16.5 in the user's own creative
  domain: metrics, completion states, error-highlighting, and comparative history
  are exactly the surfaces that feed perfectionistic *concerns*. The design
  restraint at those surfaces is not just about OCD in the abstract — it plausibly
  matters at the piano.

### 23.6 ADHD musicians (small qualitative base)

- **[E, qualitative, small]** Phenomenological work with graduate music students
  with ADHD: **structured solo practice is a hard context** (initiation, sustained
  attention, planning); **interest-based motivation** is the reported workaround;
  **hyperfocus** is described as an asset for long immersive practice; notably,
  during **active music-making and performance, ADHD symptoms were not evident in
  behaviour** — context-dependence is strong (Wilde & Welch 2022).
- **[DI/H]** The friction to support is in *practice organisation and initiation*,
  not in playing itself. Consistent with the whole document's direction: scaffold
  the approach to the work, then get out of the way.

### 23.7 Practice adherence / dropout

- **[E]** ~50% of music students stop lessons/activities by age 17. **Autonomous
  motivation protects against dropout; controlled motivation predicts it**
  (SDT again). Unwillingness to practise and loss of motivation are the internal
  drivers; need-thwarting (autonomy/competence/relatedness) precedes decline.
- **[DI]** A tool that adds *controlled* pressure to practise (obligation framing,
  streaks, guilt) is pushing the lever that predicts *dropout*. Supporting
  autonomous motivation (connection to the user's own goals, competence feedback,
  choice) is the evidenced direction — and matches P2/P7/§22.3.

---

## 24. Task management, CSCW, personal information management

### 24.1 Task lists (foundational CSCW)

- **[E]** Bellotti et al. 2004, "What a to-do: studies of task management towards
  the design of a personal task list manager" (CHI): to-do items are heterogeneous
  and constantly re-prioritised; **many tasks are never recorded** (kept in the
  head or in the world); lists are abandoned when they drift out of sync with
  reality; "the list" competes with email, paper, and memory as task media.
- **[E]** Later CSCW/PIM work is consistent: task and information tools fail at the
  **keeping/capture** boundary and at **staying current**; ~60% of people use
  to-do lists in some form, but few use one system consistently.

### 24.2 Personal Information Management (Bergman & Whittaker)

- **[E]** *The Science of Managing Our Digital Stuff* (Bergman & Whittaker 2016)
  and Whittaker's "information curation" framework: PIM has three activities —
  **keeping** (decide what to retain and where), **organising**, and **retrieving**
  — and the *same person* does all three, which makes it different from library/
  web information management.
- **[E]** Empirically, people **prefer navigation (folders) over search** for their
  *own* information, even though search is objectively often faster, because
  navigation provides context, incremental decisions, and a sense of control.
  Tag-based and pure-search approaches that work for public information
  **underperform for personal information**.
- **[E]** "Keeping" is cognitively costly and error-prone: people both over-keep
  (hoard) and under-keep (lose things); filing decisions at capture time are a
  known friction and abandonment point.
- **[DI]** Converges hard with Pass-1 §10.1–§10.2 and `CONFLICTS.md` C9: **defer
  organisation, don't force filing at capture, support navigation/context for
  retrieval, don't assume search solves personal retrieval.** This is now backed by
  a substantial PIM literature, not just practitioner consensus.

### 24.3 Interruption, resumption, attention residue

- **[E]** Attention residue (Leroy 2009, *OBHDP*): after switching from Task A to
  Task B, cognition about A persists and **degrades B performance (accuracy, speed,
  depth) for a sustained period**, not just briefly. **Key moderator:** people who
  believed they would have **ample time to finish A later** showed **no residue**.
- **[E]** Memory-for-goals / resumption-lag findings (Pass 1 §7.3) replicated and
  extended: longer, more demanding, more frequent interruptions → larger resumption
  cost; cues and a rehearsal opportunity at the breakpoint help.
- **[DI] strengthened:** the Leroy moderator is the same shape as Masicampo
  (§7.2) — **a credible path to completion neutralises the ongoing cost of an
  unfinished thing.** A capture system's real job is to *be that credible path*.
  And: minimise forced switches; when the user switches, help them leave a cue and
  believe the thing is safely held.

---

## 25. Personal knowledge management & cognitive offloading — when it helps vs. becomes overhead

The mission asked specifically: when does externalisation help, and when does
maintaining the system become the cognitive load?

### 25.1 Offloading helps — strengthened

- **[E]** Meta-analysis (2025, *Memory & Cognition*): cognitive offloading
  **improves memory-based task performance** and reduces interindividual
  variability. Offloading is "value-based decision-making" — people weigh expected
  cognitive effort against expected benefit (Boldt & Gilbert 2024).
- **[E]** The decision to offload is driven by **confidence in one's own memory**,
  and this holds **even when confidence is unrelated to actual ability** — i.e.
  people with low memory confidence (common in ADHD; *induced* by checking in OCD,
  §5.3) will offload more, sometimes more than optimal, sometimes less.
- **[E]** Brief **metacognitive training** (as few as ~5 practice trials) improved
  how well people calibrated their offloading decisions.

### 25.2 …but the *system* can become the load — the maintenance-burden question

- **[E, weak / practitioner-heavy]** There is **little rigorous longitudinal
  research** on PKM ("second brain") systems specifically. What exists:
  - The PIM literature (24.2): keeping/filing/upkeep is costly and a known
    abandonment point.
  - Personal-informatics abandonment research (Pass 1 §10.4): lapses usually start
    at collection; abandonment is normal.
  - Note-taking research: **studies rarely include delayed retention tests**, so
    which capture strategies produce *durable* benefit is largely **[U]**.
  - Strong, consistent **practitioner** signal (multiple independent sources,
    2024–2026): elaborate PKM systems collapse under upkeep; "second brain fatigue"
    / "the maintenance tax" / "the archive rots"; the recommended fix is **emergent
    structure** (organise from use, not up front) and **minimal capture friction**.
- **[DI]** Best-supported synthesis: **externalisation helps when (a) capture is
  near-frictionless, (b) the user trusts retrieval will work, and (c) upkeep is not
  required to keep the system coherent.** It becomes overhead when it demands
  filing decisions at capture, periodic full reviews/reorganisation, or an
  elaborate structure that must be maintained. This matches Pass-1 §10 and
  `CONFLICTS.md` C2/C9; the new contribution is that the PIM literature makes the
  "keeping is the expensive part" claim [E], while the "big systems collapse" claim
  stays **[practitioner-strong, E-weak]**.

### 25.3 The offloading-dependency concern (Pass 1 open question #2) — still open

- **[E, emerging, mixed]** Some recent work: heavy AI assistance **reduced task
  persistence and independent performance** afterward (2026); "the memory paradox"
  discourse warns of skill/knowledge erosion.
- **[E, counter]** Other work: deliberate, planful integration of digital tools is
  associated with **better** planning behaviour and metacognition, functioning as a
  "regulatory aid."
- **[U]** Net effect over months/years, **for this population**, on unaided memory
  / metacognition / agency: **unknown.** Contradictory short-term general-population
  findings. This stays a top open question.

---

## 26. Digital assistive technology — controlled evidence in adults

The mission asked for controlled studies, adults, technology-supported EF /
planning / memory.

- **[E] — the strongest evidence is not in ADHD/autism.** **NeuroPage** (Wilson et
  al.; RCT-level, n = 143, acquired brain injury and other neurological
  conditions): a pager sending reminders for user-chosen target behaviours at
  agreed times **significantly increased achievement of those behaviours vs
  baseline**, with benefit fading somewhat after withdrawal. This is the most
  robust "reminders work" result — but the population is memory-impaired
  neurological patients, **not adults with ADHD/autism/OCD**.
- **[E, pilot]** **AppReminders** (2023, pilot feasibility RCT, n = 29, ABI):
  Google Calendar vs a purpose-built app; most participants (19/21) learned an app
  from a 30-min tutorial; retention 66%, adherence 74%. Design features (clear
  workflow, structure) plausibly improve uptake. Feasibility, not efficacy.
- **[E, single-case]** Multiple **single-case experimental designs** show Google
  Calendar and similar mainstream tools support prospective remembering in
  individual memory-impaired adults.
- **[E, older adults with MCI/dementia]** RCT (n = 52, 4 weeks): those who used a
  reminder app **more frequently** showed better prospective memory and better
  instrumental activities of daily living — benefit **depended on the user's
  checking behaviour**, not just on the reminder firing.
- **[E]** **Smartphone-assisted psychoeducation for adult ADHD** (RCT, n = 60):
  both smartphone- and brochure-assisted formats reduced core symptoms; the
  smartphone did not clearly add benefit over the brochure.
- **[U] — the direct question.** *Controlled* evidence that a task/planning/memory
  app improves outcomes **for adults with ADHD, autism, or OCD specifically** is
  **sparse to absent.** Extrapolation from ABI/MCI populations is reasonable **[DI]**
  but not evidence.
- **[DI]** Cross-population pattern that does hold up: reminder/prompt aids help
  when (a) the user is trained to use them, (b) content is specific and
  user-chosen, (c) delivery is at an agreed/appropriate time, and (d) the user
  actually engages with the aid. Bare, generic, unengaged reminders do little.

---

## 27. Notification and reminder design — strengthened

### 27.1 Interruption cost — solid

- **[E]** Reduced notification-interruption frequency → higher task performance and
  **lower strain**; the mechanism is task-switching effort + resumption lag +
  attention residue (§24.3). Notification-disabling interventions increase
  *intentional* phone use and reduce felt "checking habit strength."
- **[E]** Moderators: fear-of-missing-out and "telepressure" (felt social pressure
  to respond quickly) worsen the impact and change what interventions work.

### 27.2 Batching — CORRECTION to Pass 1 §8.2

- **Pass 1 said:** "No consistent evidence that email/notification batching lowers
  stress." **This was too strong.**
- **[E]** **Fitz et al. 2019** (*Computers in Human Behavior*, randomised field
  experiment, n = 237, 2 weeks, 4 arms): notifications **batched 3×/day** produced
  **lower inattention, higher concentration, better mood, more felt control, and
  higher perceived productivity** vs as-they-arrive. **Hourly** batching was less
  beneficial. **Switching notifications off entirely produced *more* anxiety and
  FoMO.**
- **[E]** Email-specific batching studies remain **mixed** — some find no stress
  benefit, and effects depend on the person (e.g. FoMO level) and context ("for
  whom and under what circumstances," 2022).
- **Revised claim:** *Batching notifications into a few predictable daily windows
  has at least one good RCT showing wellbeing and attention benefits for smartphone
  notifications; email-batching evidence is mixed and moderator-dependent;
  eliminating notifications entirely has its own cost (anxiety, FoMO).* The robust
  design point is **predictable, user-controlled delivery timing** — not "batch
  everything" and not "silence everything."

### 27.3 Alert fatigue — CORRECTION to Pass 1 §10.5 / §16.4-17

- **Pass 1 said:** "responsiveness to a repeated alert can drop ~30% per repetition."
  **The specific number was an overstatement** from a loose secondary source.
- **[E]** **Ancker et al. 2017** (*BMC Med Inform Decis Mak*; 112 clinicians,
  retrospective cohort): for every **5-percentage-point increase in the proportion
  of a clinician's alerts that were within-patient repeats**, the odds of accepting
  an alert dropped by **~10%**. Higher workload and more low-value alerts also
  lowered acceptance.
- **Revised claim:** *Repeated and low-information alerts measurably reduce
  responsiveness (desensitisation is real and quantified in the clinical-alert
  literature); the exact magnitude in a personal-tool context is unknown, but the
  direction is well established.* The design implication (don't rely on repeated
  identical alerts; keep prompts high-value) is unchanged.

### 27.4 Context-sensitive / just-in-time reminders — modest and mixed

- **[E]** Location-based reminders: used, liked in principle, but **"still not an
  effective tool"** in daily life (imprecise triggering, right place/wrong moment)
  — Pass 1 finding holds.
- **[E, mixed]** JITAIs (sensor/EMA-triggered support at moments of need): **mixed
  evidence on behaviour change; few studies adequately powered.** Notably,
  **triggers based on self-reported need are rated as better-timed and more helpful
  than triggers based on inferred distress**, despite firing less often.
- **[DI]** For MELFINA: **user-declared "now is a good time / I need this now"
  beats system-inferred timing** on the current evidence. Inference-heavy
  context-awareness is not a solved problem and risks wrong-moment intrusion
  (which, for this population, is costly — §4.4, §24.3).

### 27.5 User control of notifications

- **[E]** "Alert Now or Never" (Mehrotra et al., *ACM TOCHI* 2021) and related:
  notification acceptance is highly individual and context-dependent; **when users
  lack *granular* control they resort to switching a whole channel off.** Granular,
  per-source, per-context control raises long-term opt-in and makes the alerts that
  do arrive feel intentional.
- **[DI]** Strong support for Pass-1 §16.4: few things interrupt; the user sets
  what, per source, predictably; a calm pull surface for everything else.

---

## 28. AI and compulsive-behaviour risk — strengthened, with the state of evidence

### 28.1 What is now [E]

- **[E]** **MIT–OpenAI RCT** (2025; ~1,000 participants; 4 weeks; **preprint, not
  peer-reviewed**): higher daily chatbot use correlated with **higher loneliness,
  higher emotional dependence, more problematic use, and lower real-world
  socialisation.** Users with stronger attachment tendencies and higher trust in
  the bot showed more dependence. (Within-trial associations; the RCT manipulated
  some features, but the dose–response is correlational.)
- **[E]** Narrative reviews (2025–2026, e.g. *Frontiers in Public Health*): heavier
  generative-AI-chatbot use is associated with elevated loneliness and emotional
  dependence and reduced offline social engagement; proposed mechanisms —
  **emotional attachment, anthropomorphism, instant reinforcement, parasocial
  dynamics, infinite availability, endless response variation.**
- **[E]** LLM **overreliance** driven by fluency/confident tone; LLMs frequently
  **poorly calibrated**; calibrated linguistic uncertainty can help appropriate
  reliance (Pass 1 §13.3, holds).

### 28.2 What is [G] (professional guidance, not trial evidence)

- **[G]** **IOCDF** (2026) and multiple OCD-specialist clinics: AI chatbots have
  the properties that make reassurance-seeking maximally reinforcing (always
  available, never frustrated, instant, will re-answer the same question in
  different words). Recommendation: clinicians should **ask patients about AI use**;
  patients with OCD generally prefer AI as an **adjunct with human oversight**, not
  a standalone.
- **[G]** Systematic review "Artificial Intelligence in OCD" (2025): potential AI
  applications must **actively prevent** supporting reassurance/ritualising;
  context-specific clinician guidelines do not yet exist.

### 28.3 What is contested / [U]

- **Contradiction — preserved.** A "problematic AI use" / "AI addiction" construct
  is **emerging but disputed.** ≥4 measurement scales exist (mostly modelled on
  substance-use criteria); critics ("People are not becoming 'AIholic'") argue the
  addiction framing is premature and pathologises normal tool use. The clinical
  entity is **not established**; the *phenomenon* of compulsive/dependent use in
  vulnerable individuals has converging support.
- **[U]** Whether specific design safeguards (naming the pattern; refusing to
  re-answer settled questions; deferring to the user's own prior decision; rate
  limits) actually reduce compulsive querying **without frustrating legitimate
  use** — untested. Pass 1 §13.4 safeguards remain **[H]/[DI]**.

### 28.4 Net for MELFINA

- **[DI]** The direction of Pass 1 §13 is reinforced, now with an RCT (preprint)
  and professional guidance behind the concern: **an AI layer with memory of the
  user's worries, always available, is a high-risk configuration for this profile.**
  Optional, restrained, suggest-don't-act, non-anthropomorphic, honest about
  uncertainty, does not keep the user in conversation, and can name a
  reassurance-seeking pattern. All of that is defensible on current evidence; none
  of it is *proven* to help.

---

## 29. Quantified self / self-tracking — the double-edged evidence

Pass 1 leaned on the harms. This pass found the benefit evidence too; the honest
picture is genuinely two-sided.

### 29.1 Benefits — [E], generally small

- **[E]** Meta-analyses: wearable activity trackers produce a **small but
  significant** positive effect on physical activity and on "sense of
  accomplishment"/perceived health; self-monitoring is one of the more effective
  behaviour-change techniques for diet and some weight outcomes.
- **[E]** Self-quantification can **increase self-awareness and self-knowledge**;
  in *clinically depressed* samples, mood self-monitoring has increased emotional
  self-awareness and **reduced** depressive/anxious symptoms in some studies.

### 29.2 Harms — [E], concentrated in vulnerable users

- **[E]** Systematic-review evidence of self-tracking harms: **negative emotional
  reactions** (guilt, anxiety, stress, pressure, frustration) and **maladaptive
  cognition** (rumination, body-image dissatisfaction); **disordered eating and
  compulsive exercise** as behavioural outcomes; social isolation in some cases.
- **[E]** Mood self-monitoring can **induce depressive rumination and worsen
  symptoms** — the same act that helps some users harms others. Whether electronic
  mood-tracking net helps or harms clinically **is explicitly unresolved** in the
  literature.
- **[E]** **Orthorexia** (rigid "healthy eating" preoccupation) sits on the
  OCD/anxiety spectrum and is associated with **perfectionism and
  obsessive-compulsive traits**; self-tracking of food is implicated as an
  aggravator.
- **[E]** RCT: giving people a self-tracking device did **not** reliably improve
  health outcomes over a year — the device is not self-justifying.

### 29.3 Synthesis

- **Contradiction — preserved.** Self-tracking is **beneficial on average by small
  margins for behaviour change and self-knowledge, and harmful for a minority —
  disproportionately people with perfectionism, OCD/anxiety traits, or depression**,
  via rumination and compulsive monitoring.
- **[DI]** For a user who describes OCD and is a (perfectionism-prone) classical
  musician, the risk/benefit runs unfavourable **by default**. Pass-1 §16.5 stands
  and is now better evidenced: no default metrics; any tracking is user-initiated,
  private, non-comparative, interruptible, forgettable, and easy to stop. Mood
  logging in particular should be treated as a **known-double-edged** feature, not
  a safe default.

---

## 30. Prior art — academic prototypes and their design logic

Deeper than Pass 1 §14. Analysed for underlying design bet.

| System (year) | Bet | What it got right | What it got wrong / limits |
|---|---|---|---|
| **Memex** (Bush 1945, concept) | Associative trails mirror how mind links ideas | Linking > rigid hierarchy; personal augmentation as a goal | Purely conceptual; assumed capture is free |
| **Forget-me-not** (Lamming & Flynn 1994) | Log *context* (who/where/what device) as episodic retrieval keys | Context, not content, is the retrieval cue; wearable/ambient capture | Retrieval UI weak; privacy of continuous logging unaddressed |
| **Remembrance Agent / JITIR** (Rhodes 1996–2000) | Continuously surface notes relevant to *current* context, unprompted | Proactive, just-in-time, zero-query retrieval; ranked by relevance | Relevance/noise trade-off unsolved; constant suggestion can distract |
| **CybreMinder** (Dey & Abowd 2000) | Reminders triggered by rich context (location, time, social, activity), not just time | Multi-context triggers; user-defined situations | Context detection unreliable → wrong-moment firing (still true, §27.4) |
| **Memory Glasses** (DeVaul 2003) | Context-aware, *interruption-sensitive* reminder delivery | Recognised that *when* to deliver is the hard part; sub-threshold cues | Hardware-bound; lab-stage |
| **MyLifeBits** (Gemmell/Bell 2001–2007) | Capture everything, sort later | Proved feasibility of lifetime personal store | "Data in, little usable out"; retrieval bottleneck; total capture = total noise (Sellen & Whittaker 2010) |
| **NeuroPage** (Wilson 1990s–2000s) | Dead-simple paged reminders, user-chosen targets, agreed times | **The one RCT-level success** (§26); simplicity + user-chosen content + human setup | Requires a person to help set targets; benefit fades after withdrawal |
| **Memoro** (CHI 2024) | LLM turns rambling voice capture into concise notes + retrieves on demand | Lowers *capture* friction (speak, don't type/file); query-time synthesis | Depends on LLM reliability; new failure modes (hallucinated recall) |
| **ProMemAssist** (UIST 2025) | Model the user's *working memory* from multimodal sensing; assist proactively at the right moment | Explicit working-memory model; timing-aware proactivity | Heavy sensing; proactivity/intrusion trade-off; early-stage |
| **"Executive Dysfunction by Design"** (ASSETS 2025) | Analyses how mainstream productivity tools *cause* executive dysfunction for neurodivergent users; contrasts AI support with healthcare-access barriers | Names the anti-pattern: tools that assume the executive function they're supposed to support | Analysis, not a system |

**Cross-cutting lessons [DI]:**
1. **Retrieval and timing are the unsolved hard problems**, repeatedly, for 30+
   years. Capture and storage are comparatively easy. A new system should invest
   its cleverness in *resurfacing the right thing at the right moment* and in
   *not* firing at the wrong moment.
2. **The one clear success (NeuroPage) is the simplest system**, with user-chosen
   content and a human in the setup loop. Complexity has a poor track record here.
3. **Context inference has never become reliable enough** to trust for
   intrusion-timing. User-declared context/availability is the safer bet on
   current evidence (§27.4).
4. **Total capture is a known dead end.** Design from how memory cues
   reconstruction, not from completeness.
5. **LLM-era prototypes** genuinely reduce capture and synthesis friction but
   introduce **hallucinated-recall** risk — a personal-memory tool that
   *confabulates your past* is a serious failure mode, especially against OCD
   memory-distrust (§5.3).

---

## 31. Evidence-quality audit of Pass 1

Reviewed the load-bearing claims of §§1–19. Outcome per claim: **upheld**,
**refined**, **corrected**, or **downgraded**.

| Pass-1 claim | Audit outcome |
|---|---|
| ADHD working memory persists into adulthood (g≈0.5) | **Upheld.** Alderson 2013 figures stand. |
| Autism EF: medium, domain-general, ~stable across lifespan | **Refined.** g = 0.48 (CI 0.43–0.53) now primary-verified (Demetriou 2018, 235 studies / 14,081 pp). Domain-general: upheld. "Stable across lifespan" → **corrected** to "present across the lifespan, **attenuates somewhat in adulthood**." Added: lab EF has poor diagnostic utility; informant-report (BRIEF) tracks real-world difficulty better. |
| ADHD time perception atypical (accuracy g>0.4, precision g=0.66) | **Upheld** (child/adolescent data; adult data still thinner). |
| Writing a specific plan removes open-loop intrusion (Masicampo & Baumeister 2011) | **Upheld and reinforced** by the Leroy 2009 attention-residue moderator (ample-time-to-finish belief removes residue). |
| Repeated checking worsens memory confidence (van den Hout & Kindt) | **Upheld.** |
| Reassurance / family accommodation predict worse OCD outcomes | **Upheld.** |
| AI as a novel reassurance vector | **Upheld and strengthened** — now IOCDF guidance + an RCT (preprint) + reviews. Still: harm mechanism [E], "AI addiction" as a clinical entity [contested/U]. |
| IU ↔ anxiety in autism (r≈0.62) | **Upheld** (Jenkinson 2020). |
| Gamification effects small/fragile; streaks = loss-avoidance | **Upheld.** Sailer & Homner figures stand; over-justification risk now cross-referenced to musician over-justification evidence. |
| Users prefer adaptable > adaptive; autonomy support reduces demand avoidance | **Upheld; autonomy-support half upgraded** from [E/clinical] to **[E]** via SDT meta-analyses (192-study MASEM etc.). Adaptable>adaptive: Findlater & McGrenere stands; note some newer studies find adaptive UIs help *some* users under high load. |
| Notifications: deliver at breakpoints; user controls interruption | **Upheld** (Iqbal & Bailey; Mehrotra TOCHI 2021). |
| "No consistent evidence batching lowers stress" (§8.2) | **CORRECTED.** Fitz et al. 2019 RCT shows batching 3×/day improves attention/mood/control for smartphone notifications; email evidence mixed; full silence raises anxiety/FoMO. Revised to "predictable, user-controlled timing; neither 'batch all' nor 'silence all'." |
| "~30% drop in responsiveness per repeated alert" (§10.5) | **CORRECTED / downgraded.** Real figure (Ancker 2017): ~10% lower odds of acceptance per +5pp within-patient-repeat share. Direction upheld, magnitude was overstated. |
| Implementation intentions target the ADHD initiation gap (d=0.65) | **Refined / partly downgraded.** d=0.65 is general-population. ADHD evidence is **children only** (response-inhibition normalisation); **adults with ADHD: [U]**; may need baseline EF to work. |
| GTD is theoretically motivated but has ~no outcome evidence | **Upheld** (Heylighen & Vidal; no RCTs). |
| Local-first as a coherent paradigm (Kleppmann 2019) | **Upheld** (paradigm-level, not an efficacy claim). |
| Cognitive offloading improves task performance (Risko & Gilbert) | **Upheld and strengthened** — 2025 meta-analysis; "value-based decision-making" model; metacognitive training calibrates it. |
| "Total capture" is a dead end (Sellen & Whittaker) | **Upheld and reinforced** by 30 years of prototype history (§30). |
| Conflicting needs across the three profiles | **Upheld; caution added** — comorbidity *amplifies* impairment, and the specific 3-way combination is unmeasured on any design variable ([U]). |
| Prior-art "design bets" table (§14) | **Expanded** (§30) with academic prototypes; core lessons unchanged, "retrieval & timing are the hard problems" now explicit. |

**No Pass-1 claim was disproven.** Two were corrected (batching, alert-fatigue
magnitude), several refined, one partly downgraded to [U] for the adult-ADHD case
(implementation intentions).

---

## 32. Updated notes to the conflict catalogue

Full detail in `CONFLICTS.md` (updated). Summary of what Pass 2 changes there:

- **C2 (externalise vs compulsion)** and **C9 (rich capture vs noise/friction):**
  strengthened — the **PIM literature** now makes "keeping/filing is the expensive
  part" **[E]**, and 30 years of prototypes make "retrieval & timing are the
  bottleneck" **[E/DI]**.
- **C3 (reminders vs overload):** refined — batching correction (§27.2); alert-
  fatigue magnitude correction (§27.3); **user-declared timing beats inferred
  timing** (§27.4); the strongest reminder-efficacy evidence (NeuroPage) is in a
  different population.
- **C5 (measurement vs compulsive monitoring):** strengthened both directions —
  self-tracking has **small real benefits** *and* **real harms concentrated in
  perfectionism/OCD/depression**; mood-tracking is explicitly double-edged;
  musician perfectionistic-concerns evidence makes the harm side concrete for the
  user's creative domain.
- **C6 (automation vs autonomy):** autonomy side upgraded to **[E]** (SDT
  meta-analyses); automation side reinforced (AI-assistance-reduces-persistence
  finding).
- **C7 (completion vs "not-just-right"):** reinforced — Leroy moderator + Masicampo
  both say a *credible completion path*, not actual completion, is what relieves
  the load; perfectionistic-concerns ↔ NJRE overlap now explicit.
- **New cross-cutting note:** because the 3-way combined profile is unmeasured,
  every axis should be individually user-calibrated; category presets are
  contraindicated (now [E]-adjacent via comorbidity-amplification, not just [DI]).

---

# RESEARCH CHECKPOINT 002

### What evidence was strengthened

1. **Autism EF impairment** — now primary-verified: g = 0.48 (CI 0.43–0.53), 235
   studies / 14,081 participants, broad (non-fractionated), attenuating but
   persistent into adulthood; **informant-report EF measures track lived
   difficulty better than lab tests.**
2. **Adult autism life outcomes** — EF and daily-living skills are **evidenced
   predictors** of independent living, employment, and mental health; ~20% "good
   outcome" rate. External executive scaffolding targets a high-stakes variable.
3. **Autonomy support** — upgraded from clinical inference to **[E]** via SDT
   meta-analyses (need support → autonomous motivation → performance + wellbeing;
   need-thwarting → the maladaptive path).
4. **Cognitive offloading** — 2025 meta-analysis confirms it improves memory-based
   performance; framed as value-based decision-making; metacognitive training can
   calibrate it.
5. **PIM as a field** — "keeping/filing is the costly, abandonment-prone part";
   "people prefer navigation+context over search for their own information";
   defer-organisation is now [E]-backed, not just practitioner consensus.
6. **Attention residue (Leroy 2009)** — task-switching cost is sustained, not
   brief; **belief in ample time to finish later removes it** — same shape as
   Masicampo. Strong warrant for "be a credible completion path."
7. **Notification batching** — one good RCT (Fitz 2019) shows wellbeing/attention
   benefit from predictable batched delivery; full silence backfires.
8. **AI reassurance risk** — professional guidance (IOCDF) + a large preprint RCT
   (MIT–OpenAI: use ↔ loneliness/dependence/problematic use) + review literature.
9. **Self-tracking** — now evidenced as genuinely two-sided: small average
   benefits; real harms concentrated in perfectionism/OCD/depression.
10. **Musician-domain support** — the leverage is **session structure, planning,
    reflective self-evaluation, and autonomy**, not practice-hour logging;
    perfectionistic *concerns* (not strivings) drive performance anxiety and
    overlap with OCD constructs.
11. **Reminder efficacy** — RCT-level support exists (NeuroPage, n=143) but in
    memory-impaired neurological patients, with user-chosen content and human
    setup.

### What claims were weakened / corrected (none disproven)

- **CORRECTED:** "No evidence batching lowers stress" → batching into predictable
  windows has RCT support for smartphone notifications; email evidence mixed;
  silence has its own cost.
- **CORRECTED / downgraded:** "~30% drop per repeated alert" → Ancker 2017's actual
  figure is ~10% lower acceptance odds per +5pp repeat-share; direction holds,
  magnitude was overstated.
- **REFINED:** "Autism EF stable across lifespan" → present across the lifespan,
  **attenuates somewhat in adulthood.**
- **PARTLY DOWNGRADED to [U]:** implementation intentions for **adults with ADHD**
  — direct evidence is children-only; adult-ADHD efficacy unknown; may require
  baseline EF.
- **REFINED:** AuDHD co-occurrence "~30–40% each way" → ADHD-in-autism ≈ 22%
  (community) to 34% (clinical), sample-dependent; ADHD+OCD co-occurrence is
  **lower and more uncertain** than Pass 1 implied (and the two show opposite
  fronto-striatal activation).
- **CONTESTED, flagged:** deliberate practice as the dominant driver of musical
  expertise — meta-analyses put it at ~21–26% of variance; the Ericsson camp
  disputes the methodology. Both positions live.

### Important contradictions (preserved, not resolved)

1. **Deliberate practice:** dominant (Ericsson) vs important-but-far-from-
   sufficient (Macnamara/Hambrick). ~21–26% of music-performance variance in the
   meta-analytic camp.
2. **Cognitive offloading long-term:** "reduces persistence / erodes independent
   skill" vs "planful use improves metacognition and planning." Net effect for this
   population: **[U].**
3. **Mood / self-tracking:** increases self-awareness and can reduce symptoms
   (some depressed samples) vs induces rumination and worsens symptoms (others).
   Net clinical effect "explicitly unresolved" in the source literature.
4. **Notification batching:** wellbeing benefit (Fitz, smartphone) vs no stress
   benefit (several email studies). Moderator-dependent.
5. **"Problematic AI use":** a real emerging phenomenon in vulnerable users vs an
   over-pathologising construct ("not becoming AIholic"). The clinical entity is
   not established.
6. **Prospective memory in autism:** large time-based deficit (2017 meta-analysis)
   vs no deficit after controlling for verbal ability (2026 study).
7. **Adaptive UIs:** disliked / slower / unpredictable (Findlater & McGrenere) vs
   helpful for some users under high cognitive load (later IUI work).

### Remaining evidence gaps ([U])

1. **The specific ADHD + autism + OCD combination** — not measured on *any*
   design-relevant variable. Everything about the combined profile is inferred from
   single-condition or two-way-comorbidity data. **Largest gap.**
2. **Controlled efficacy of task/planning/memory software for adults with ADHD,
   autism, or OCD** — sparse to absent; extrapolated from ABI/MCI populations.
3. **Long-term (months–years) effect of heavy cognitive offloading** on unaided
   memory, metacognition, and agency — for anyone, and especially this population.
4. **Whether the Pass-1 §13.4 AI safeguards** (name the pattern, refuse to
   re-answer, defer to prior decisions, rate-limit) reduce compulsive querying
   without frustrating legitimate use — untested.
5. **Reminder design for reminder-fatigued / demand-avoidant users** — what
   content/timing/modality keeps prospective-memory support effective.
6. **Whether any progress representation** delivers ADHD salience benefit without
   OCD/perfectionism cost — still no evidence it exists; "none" remains the safe
   answer.
7. **The neutral-primitives hypothesis** (§16.8 P13) — untested; partly a
   requirements/prototyping question.
8. **"Anchored flexibility"** (structure ↔ novelty resolution, C1) — recurs in
   clinical/lived-experience writing; no experimental test.
9. **Capacity-state adaptation** — whether a manual low/normal/high switch that
   changes several axes at once helps; and whether *detecting* capacity is possible
   without surveillance (probably not, on §27.4 evidence).
10. **Longitudinal PKM outcomes** — almost no rigorous data; note-taking studies
    rarely test delayed retention.
11. **Music practice organisation for *this* user** — musician findings are
    group-level and must not be assumed to transfer; the ADHD-musician base is tiny
    and qualitative.

### Implications now sufficiently supported to carry into requirements ([DI], evidence-backed)

- **External structure that holds memory, plans, time, and context out of the
  head** — supported by EF/PM evidence (ADHD + autism, adult), offloading
  meta-analysis, PIM literature, Masicampo, Leroy, and (for autistic adults) the
  outcome-prediction evidence for EF/daily-living skills.
- **Near-frictionless capture, deferred organisation, navigation+context for
  retrieval** — PIM literature + prototype history + practitioner consensus.
- **Be a credible completion path** (so open loops stop costing attention) —
  Masicampo + Leroy.
- **Predictable, deterministic, no-surprise system behaviour** — IU evidence
  (autism + OCD), adaptive-UI evidence, context-inference-unreliability history.
- **Autonomy-supportive framing; suggest don't impose; no obligation/pressure
  framing** — SDT meta-analyses (now [E]), PDA/clinical, music-dropout evidence.
- **Quiet by default; few interruptions; predictable, user-controlled,
  granular delivery timing; user-declared "now" over inferred timing** —
  interruption/residue evidence, Fitz RCT, Mehrotra TOCHI, JITAI evidence,
  alert-fatigue evidence.
- **No default metrics/streaks/scores/comparative history; restraint at
  completion, confirmation, history surfaces** — gamification meta-analysis,
  self-tracking harm evidence, musician perfectionistic-concerns evidence, NJRE.
- **AI layer optional, restrained, non-anthropomorphic, honest about uncertainty,
  not a reassurance oracle, does not keep the user in conversation** — AI-reliance
  evidence, MIT–OpenAI preprint, IOCDF guidance.
- **Local-first, open formats, encrypted, exportable, no telemetry, data
  minimisation** — local-first paradigm, privacy-by-design, and the OCD-relevant
  point that the user's own future access to exhaustive self-logs is a design
  choice.
- **Personalisation via individually-calibrated axes, not diagnostic presets** —
  adaptable>adaptive, COGA objective 8, comorbidity-amplification (the combined
  profile is not a union of presets), heterogeneity within each condition.
- **Graceful degradation during low-capacity periods; lapses are normal** —
  personal-informatics abandonment research, autistic burnout, camouflaging-harm
  evidence, emotion dysregulation.

### Remaining hypotheses ([H] — plausible, not established)

- Neutral primitives (open loop / session / note / thread / commitment) accommodate
  real neurodivergent life, including music, better than task/habit/project.
- "Anchored flexibility" resolves the structure↔novelty tension.
- The specific AI safeguards reduce compulsion without frustrating use.
- A manual capacity-state switch is useful and sufficient.
- Supporting *session structure and reflection* (not logging) is the right way to
  touch music practice, if at all.
- Body doubling / co-presence features help (mechanism plausible; outcome evidence
  thin).

### Confidence level of major conclusions

| Conclusion | Confidence | Basis |
|---|---|---|
| EF/PM/time differences are real, persist into adulthood, and warrant external scaffolding | **High** | Multiple meta-analyses, both conditions, adult data, outcome-prediction evidence |
| For this profile, the key design risk is features that become compulsion / rigidity / overload engines | **Moder–High** | Strong mechanism evidence per condition; the *combination* is inferred, not measured |
| A trusted capture/resurfacing system frees attention (even before doing) | **High** | Masicampo + Leroy + offloading meta-analysis, converging |
| Predictability and autonomy-support should be design defaults | **High** | IU evidence + SDT meta-analyses + adaptive-UI evidence |
| No streaks / metrics / scorekeeping by default | **Moderate–High** | Gamification meta-analysis + self-tracking harm evidence + perfectionism evidence; "is there a safe form?" unresolved |
| Quiet-by-default, user-controlled, predictably-timed prompts | **Moderate–High** | Interruption/residue + Fitz RCT + Mehrotra + alert-fatigue; exact parameters unknown |
| AI layer is high-risk and must be optional/restrained | **Moderate** | Professional guidance + preprint RCT + reliance studies; "AI addiction" contested; safeguard efficacy untested |
| Reminders help | **Moderate** | RCT-level only in memory-impaired neurological patients; adult ADHD/autism/OCD [U]; field efficacy of context-triggers is weak |
| Specific assistive features (visual timers, body doubling, location reminders) help adults | **Low** | Mechanism plausible; controlled adult evidence sparse to absent |
| Deliberate-practice-amount is worth foregrounding for a musician | **Low / negative** | Meta-analyses: ~21–26% of variance; amount alone is a weak lever; over-justification risk |
| The neutral-primitives life model | **Not assessed (design hypothesis)** | No evidence either way; a requirements/prototyping question |
| Any conclusion specific to the ADHD+autism+OCD *combination* | **Low (inferred)** | No direct evidence exists |

---

## Boundary reaffirmed

```
   RESEARCH FINDING  →  POSSIBLE DESIGN IMPLICATION  →  [STOP]
```

Pass 2 strengthened the evidence under many Pass-1 implications and corrected two
claims. It did **not** produce requirements, choose an architecture, choose a
language or storage substrate, or design MELFINA. The next phase (PERSONAL
REQUIREMENTS) remains gated on the user's explicit authorisation.

---

*End of RESEARCH MASTER (Pass 2). Bibliography: `research/BIBLIOGRAPHY.md`.
Conflict catalogue: `research/CONFLICTS.md`.*
