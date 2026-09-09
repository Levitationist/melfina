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

*End of RESEARCH MASTER. Bibliography: `research/BIBLIOGRAPHY.md`. Supporting
detail on the conflict catalogue: `research/CONFLICTS.md`.*
