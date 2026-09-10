# PROJECT_STATE

Single source of truth for where this project is and the rules it operates under.
Update this file whenever the phase changes or a principle is added, removed, or revised.

Last updated: 2026-09-10 (HUMAN / CENTRAL MODEL MISSION 001)

---

## 1. What this is

**MELFINA** — a long-term effort to build a **lightweight, local-first personal
life operating system** designed around the actual cognitive, behavioral,
environmental, and practical needs of one user, rather than around a generic
productivity methodology.

MELFINA is the official project name (set 2026-09-09). Earlier notes may say
"Personal OS"; that was a placeholder.

## 2. Current phase

**Phase: HUMAN / CENTRAL MODEL — MISSION 001 complete (first pass), awaiting user
review. Next phase is SYSTEM DESIGN / ARCHITECTURE, and it does not start until the
human/central model is reviewed and explicitly accepted.**

Research Missions 001 + 002 and PERSONAL REQUIREMENTS MISSION 001 (+ its
dynamic-self-directed correction) are done and pushed (commit `120567a`).

We are NOT coding the application. No architecture chosen. No language (C vs C++)
chosen. No storage substrate chosen — and the model explicitly must survive any
choice (relational / graph / document / triple / hypergraph — none implied). No
framework, model, or UI toolkit chosen. `src/` untouched.

### HUMAN / CENTRAL MODEL MISSION 001 — summary of outcome (2026-09-10)

Determined the smallest coherent conceptual model of a human life from which
MELFINA's capabilities emerge. Not built around application categories (tasks,
habits, notes, calendars, projects, goals, reminders, journals, dashboards, music
practice) — those emerge as views.

Output in `model/`:
- `HUMAN_CENTRAL_MODEL.md` — the model (18 sections + `MODEL CHECKPOINT 001`).
- `MODEL_ALTERNATIVES.md` — 4 candidate models (A event-centric · B entity/
  relationship-centric · C state/transition-centric · D three-primitive) compared
  rigorously; conclusion: no single one is sufficient, a synthesis is necessary.
- `MODEL_OPEN_QUESTIONS.md` — `OQ-M1 … OQ-M12`.
- `README.md` — index.

**Leading model — E²CI. Four primitives:**
- **ENTITY** — anything that persists and can be referred to (`kind` is an open
  attribute: person / place / thing / work / concept / capability / workflow /
  source / `self` / `melfina` / …). Grounded in BFO continuant; personal
  knowledge graphs.
- **EVENT** — something that occurs at/over time, with participants in roles,
  possibly bringing about or ending conditions; includes observations, actions,
  sessions, decisions, and *expected* future events. Grounded in BFO occurrent;
  event calculus; W3C PROV.
- **CLAIM** — a statement held by an agent, with an origin and an epistemic
  **status** (observed / reported / inferred / hypothesised / open / stipulated /
  decided / recalled), a **confidence**, full **provenance**, and **bitemporal**
  coordinates (valid-time / transaction-time). *All of MELFINA's knowledge is
  claims.* Relationships and "what holds now" (state) are claims. Grounded in
  epistemic/doxastic logic; PROV; AGM belief revision; bitemporal modelling.
- **INTENTION** — an agent's directedness toward a future condition or action —
  the world-to-mind stance, distinct from a belief by *direction of fit*. Goals,
  tasks, plans, commitments (an intention with a creditor), routines, reminders
  are cases. Grounded in Anscombe/Searle; BDI; Castelfranchi social commitment.

Plus: **TIME** is a bitemporal *dimension* (not a primitive — "calendar"
dissolves into a view). **CONTEXT** is *derived per situation* by a relevance
trade-off (Sperber & Wilson) — never stored.

**Key decisions (open to challenge):** `State` folded into `Claim` (OQ-M2);
`Relation` folded into `Claim` — carrying its own time/provenance/confidence
(OQ-M1); `Intention` kept (direction of fit) — folding it is rejected;
four primitives — three is insufficient, five/six not clearly necessary (OQ-M8).

**Strongest test-case results:** music needs **zero** music-specific primitives
(work/movement/section/passage = `part-of`-linked entities; practice session = a
`session` event; technical problem = a user-worded descriptive claim;
interpretation = `stipulated` claims MELFINA never overwrites; progression = a
`supersedes` chain — no streak, no hours headline). Scientific reasoning maps
cleanly (hypothesis/evidence/experiment are *labels + a discipline*, not model
extensions). The agency pipeline (USER intends → MELFINA reasons → proposes →
USER authorises → MELFINA acts → world changes → MELFINA observes → updates) maps
exactly, and the autonomy triad is naturally separated. The dynamic self-directed
MELFINA is supported without freezing today's agent tech (strategies and skills
are claims/entities, never enums).

**What the model deliberately does NOT contain:** the self-modification
meta-invariant rules (external to MELFINA's reasoning by design); any enforcement
mechanism (permissions are represented, not guarded); any evaluative verdict the
user did not enter; any diagnostic category (ADHD/autism/OCD not reified); any
implementation choice.

**Top unresolved semantic questions:** is `State` a fifth primitive (OQ-M2); is
`Relation` (OQ-M1); is four genuinely minimum-sufficient (OQ-M8, a prototyping
question); recurrence identity (OQ-M3); structure of `confidence` (OQ-M4); how
worries / intrusive thoughts / affect are represented (OQ-M5) — flagged for user
input; "what matters" beyond `Intention` + relevance (OQ-M7).

**Confidence:** Moderate–High that the four *kinds* of thing (referents /
happenings / claims / directedness) are right — converges across BFO, event
calculus, epistemic logic, direction-of-fit, PROV. High that claims carry
provenance + status + bitemporal coordinates, and that context is derived not
stored. Moderate on `State`- and `Relation`-folding and on whether four is
genuinely minimal. Unknown whether E²CI holds *this user's* life well — only
real-world use tells.

`requirements/` was **not edited** — issues with requirements are logged as model
open questions, not changed (research and requirements kept as immutable prior
checkpoints).

### PERSONAL REQUIREMENTS MISSION 001 — summary of outcome (2026-09-10)

Mission = base brief (30 requirement areas, MUST/SHOULD/MAY/MUST NOT, IDs,
conflicts, anti-requirements, musician requirements, AI requirements) +
autonomous-reasoning addendum (cognitive / decision / execution autonomy) +
local-only / network-isolation addendum + autonomous-personal-intelligence
extension (assistant, autonomous automation, adaptation, skills, scientific
thinking, teaching, world understanding, deep reasoning, emotional understanding,
software engineering, computer agency, controlled self-improvement, general
capability, extreme-capability-+-lightweightness, capability model) +
**dynamic-self-directed correction** (do not model MELFINA as a static capability
catalogue; it dynamically determines what capabilities / context / reasoning /
tools a problem needs and, where authorised, constructs new ones).

Output in `requirements/`:
- `REQUIREMENTS_MASTER.md` — **253 requirements** (`MEL-REQ-001…253`) across 55
  areas + **19 anti-requirements** (`MEL-AR-01…19`) + the **MELFINA Capability
  Model** (15 capabilities — a *vocabulary, not a fixed catalogue*) + the
  **autonomy triad** (cognitive / decision / execution, never collapsed) + the
  **action pipeline** (THINK→DECIDE→PROPOSE→AUTHORISE→EXECUTE→VERIFY) + **PART
  IV-B — THE DYNAMIC SELF-DIRECTED SYSTEM** (metareasoning; dynamic context/skill
  selection; capability-gap recognition + capability creation; human–MELFINA
  co-creation; a graded **nine-tier self-modification model** with **THE
  META-INVARIANT** — MELFINA cannot redefine the rules governing its own
  self-modification; dynamic resource allocation + dynamic autonomy;
  self-evaluation; "best is situational, dynamic ≠ unpredictable") + a feasibility
  classification + `REQUIREMENTS CHECKPOINT 001`.
- `CONFLICTS.md` — 10 research-level (RC1–RC10) + 7 vision-level (VC1–VC7) + 5
  dynamic-system (VC8–VC12) tensions as configurable dimensions / constraints /
  safeguards / open questions. Not resolved by fiat.
- `OPEN_QUESTIONS.md` — OQ-1…OQ-21 + the deliberately-undecided list.
- `README.md` — index.

Priority mix: ~120 MUST · ~90 SHOULD · ~20 MAY · ~23 MUST NOT (incl. the 19 AR).

**Highest-priority MUSTs:** local-only core (stronger than "local-first" — the core
has no notion of a remote; MEL-REQ-014, 164–166); user is the authority /
scaffold-don't-steer / more-than-a-tracker (001–005); no diagnostic modes,
descriptive-not-judgemental data model (006, 008); predictable-deterministic core,
quiet by default, no scorekeeping by default, restraint at the hazard surfaces,
graceful degradation (010–013, 015); OCD checking/reassurance safeguards, "I don't
know" is first-class, no compelled engagement (055–060); AI operates under the
permission model not as owner, fully usable with AI disabled, no self-authored
objectives / no core self-modification / no self-replication (149–150, 153–154);
least-authority + capability-based permissions + audit + autonomous-action budgets
+ assume-prompt-injection + emergency stop (179–187); data is local/open/
exportable/deletable/encrypted/durable (160–170); **THE META-INVARIANT — MELFINA
cannot redefine (silently, autonomously, or by persuasion) the rules governing its
own self-modification; those rules are external to its reasoning, monotonic
(authority only narrows without an explicit human act), versioned, audited (235)**;
self-modification tiers 6–9 never autonomous (236); dynamic choices bounded by the
grounding factors, MELFINA does not optimise one fixed notion of "best", dynamic ≠
unpredictable (207, 251–253); autonomy ceiling not self-raisable (245);
self-evaluation not resting on the model's own metacognitive report (248).

**Anti-requirements (19):** compulsive tracking · gamification/streaks ·
notification spam · rigid methodology · endless configuration · reassurance machine
· AI dependency/deskilling · opaque adaptive drift · surveillance · maintenance-
heavy PKM · feature-heavy dashboard · cloud dependency/phone-home · unrestricted
autonomous agent · manipulation/false-intimacy/relationship-replacement · grandiose
claims · self-authored goals / core self-modification · ungrounded/unpredictable
"dynamism" · silent self-privilege / self-modification-rights escalation · a
self-improvement loop that trusts its own judgement.

**Rejected interpretations (recorded):** autonomy = unrestricted execution;
adapt-with = collect-all-data; general intelligence = one giant model;
self-improvement = self-modification/replication (it is instead a *graded* capability,
tiers 6–9 never autonomous); dynamic/self-directed = free to do anything /
unpredictable; MELFINA can eventually govern its own modification permissions;
immutable invariants are a *sufficient* guarantee for a capable self-modifier
(necessary, not proven sufficient); the system can judge for itself that a
self-improvement worked; the 15 capabilities are the complete feature set;
emotional understanding = the system has feelings; "local-first is enough" (→
local-only core); more capability always better / optimise one fixed notion of
"best"; diagnostic research → diagnostic product modes; explanations fix
over-reliance; recursive self-improvement toward superintelligence is the
trajectory (explicitly not a goal).

**Evidence discipline:** every requirement tagged `[E]/[G]/[DI]/[H]/[U]` and
provenance-classed `A` (evidence) / `B` (user goal) / `C` (hypothesis) / `D`
(unknown) / `E` (deliberately undecided). No `[U]`/`[H]` finding was promoted to a
MUST/SHOULD.

**Local-first vs local-only distinction (documented, per the addendum):**
local-first = works offline, syncs when online, cloud assumed present.
**Local-only core (MELFINA's requirement)** = the core has no notion of a remote at
all; removing the network changes nothing about its ability to understand, reason,
remember, retrieve, plan, decide, teach, and act locally; no cloud component it
defers to or degrades toward. Any future network capability is a separate,
isolated, off-by-default add-on that can never form the "lethal trifecta" with
core data and is never a core dependency (MEL-REQ-166, 167).

**Dynamic self-directed system (PART IV-B):** the 15 capabilities are a vocabulary,
not a ceiling; MELFINA reasons per-situation about how deep to reason, how fast,
what context, which skills, whether existing skills suffice, whether to compose,
whether to build a new capability, whether to experiment, whether to act at all.
Grounded in: rational metareasoning / value-of-computation (Russell & Wefald 1991),
adaptive test-time compute (demonstrated), anytime algorithms, adaptive/self-RAG
(dynamic context), skill libraries (Voyager) + LLM tool-making (CREATOR/LATM)
demonstrated in constrained settings, human-AI co-creativity frameworks, safe
dynamic software update, monotonic confinement / external guardrails, the Gödel
machine as the (intractable) safety ideal. Feasibility is mixed and stated per
requirement: adaptive depth/context/resource allocation = demonstrated; capability
creation = demonstrated only in games/benchmarks, open-world unproven; verified
self-generated code ≈ unsolved (→ safety via test+sandbox+rollback+human-gate);
tiers 6–9 self-modification never autonomous; recursive self-improvement toward
superintelligence explicitly not a goal.

**Confidence:** high for local-only / no-scorekeeping / predictability+autonomy /
the autonomy triad + least-authority security / external-structure-is-worth-
building / the dynamic-system *bounds* (grounding factors, the meta-invariant,
"dynamic ≠ unpredictable"). Moderate for metareasoning + dynamic context/resource
allocation, capability creation with gating, and self-modification tiers 1–5. Low
for the neutral-primitives life model (hypothesis), specific assistive-feature
efficacy for this adult, the emotional-understanding *capability*, and the
deep-reasoning/rediscovery aspiration (narrow formal case only). `[U]`: whether
the meta-invariant is *sufficient* against a highly capable self-modifier
(necessary, not proven sufficient — high tiers kept rare + human-driven + audited).
Unknowable now: whether MELFINA actually improves this user's life — only
REAL-WORLD USE answers that.

### RESEARCH MISSION 002 — summary of outcome (2026-09-10)

Deeper pass targeting Mission 001's evidence gaps. Output: `RESEARCH_MASTER.md`

### RESEARCH MISSION 002 — summary of outcome (2026-09-10)

Deeper pass targeting Mission 001's evidence gaps. Output: `RESEARCH_MASTER.md`
**§§20–32 + `RESEARCH CHECKPOINT 002`**; **Second-Pass Evidence Updates** in
`CONFLICTS.md`; **Second Pass Additions** in `BIBLIOGRAPHY.md`.

- **Evidence strengthened:** autism EF meta-analysis primary-verified (g = 0.48,
  235 studies; attenuates but persists into adulthood; informant-report tracks
  real-world difficulty better than lab tests); adult autism life outcomes (EF +
  daily-living skills predict independent living/employment/mental health;
  ~20% "good outcome"); autonomy-support upgraded to **[E]** (SDT meta-analyses);
  cognitive-offloading benefits (2025 meta-analysis); PIM literature (keeping/
  filing is the costly, abandonment-prone part; people prefer navigation+context
  over search for their own data); attention residue (Leroy 2009 — a *credible*
  completion path removes the load, same shape as Masicampo); notification
  batching (Fitz 2019 RCT); AI reassurance risk (IOCDF guidance + MIT–OpenAI
  preprint RCT); self-tracking as genuinely two-sided; reminder efficacy
  RCT-level only in memory-impaired neurological patients (NeuroPage).
- **Claims corrected (none disproven):** notification batching *can* help
  wellbeing (Pass 1 said no evidence); alert-fatigue magnitude was overstated
  (real: ~10% lower acceptance per +5pp repeat-share, Ancker 2017); autism EF
  "stable across lifespan" → attenuates somewhat in adulthood; AuDHD
  co-occurrence figures refined (ADHD-in-autism ≈ 22–34%); ADHD+OCD co-occurrence
  lower/more uncertain than implied, and neurofunctionally opposite; implementation
  intentions for **adults with ADHD** downgraded to **[U]** (child-only evidence).
- **Musician/pianist context (new):** deliberate-practice *amount* explains only
  ~21–26% of music-performance variance and is contested (Ericsson vs
  Macnamara/Hambrick — contradiction preserved); the evidenced leverage is
  **session structure, planning, reflective self-evaluation, and autonomy**, not
  hour-logging; perfectionistic **concerns** (not strivings) drive music
  performance anxiety and overlap with OCD constructs; autonomous motivation
  protects against practice dropout, controlled motivation predicts it.
- **New tier introduced:** **[U] — unknown / insufficient evidence.**
- **Contradictions preserved** (7 listed): deliberate practice; offloading
  long-term effect; mood/self-tracking; notification batching (smartphone vs
  email); "problematic AI use" as a construct; prospective memory in autism;
  adaptive UIs.
- **Largest evidence gap:** the specific **ADHD + autism + OCD combination** is
  unmeasured on every design-relevant variable; controlled assistive-tech
  efficacy in adults with these conditions is sparse to absent.

Confidence levels for the major conclusions are tabulated in `RESEARCH CHECKPOINT
002`. High confidence: EF/PM/time differences warrant external scaffolding; a
trusted capture/resurfacing system frees attention; predictability + autonomy
support as defaults. Low confidence: specific assistive features help adults;
foregrounding practice-hours for a musician; anything specific to the 3-way
combination.

### RESEARCH MISSION 001 — summary of outcome

- **Question:** how to design a personal computing system that reduces cognitive
  friction and supports daily functioning for a person with lived experience of
  ADHD + autism + OCD, without reinforcing harmful patterns. Not a medical
  project; no diagnosis; no treatment.
- **Method:** scoping literature synthesis (web search, Sept 2026), prioritising
  systematic reviews / meta-analyses / clinical & standards-body guidance, then
  peer-reviewed primary research and HCI/assistive-tech venues. Not a systematic
  review.
- **Files created:** `research/RESEARCH_MASTER.md` (19-section report),
  `research/CONFLICTS.md` (the 10 core design tensions), `research/BIBLIOGRAPHY.md`
  (references with provenance tags), `research/README.md` (updated index).

### Most important findings (see RESEARCH_MASTER §1)

1. Executive-function / working-memory / prospective-memory differences persist
   into adulthood (ADHD and autism, medium effects) — strong rationale for
   *external* structure. **[evidence]**
2. Externalising intentions works; a *specific written plan* removes the
   attention cost of an open loop even before the task is done (Masicampo &
   Baumeister 2011). **[evidence]**
3. ADHD time perception is measurably atypical — "externalise time." **[evidence
   for the deficit; clinical guidance for the remedy]**
4. Predictability reduces anxiety in autism; intolerance of uncertainty is
   central to both autism-anxiety and OCD. **[evidence]**
5. Checking, reassurance, logging, and "always-available AI" can become
   compulsions; repeated checking *worsens* memory confidence. **[evidence /
   clinical]**
6. Notifications degrade with volume; deliver at task boundaries; user controls
   interruption. **[evidence]**
7. Gamification effects are small and fragile; streaks convert intrinsic goals
   into loss-avoidance — contraindicated by default for this profile. **[evidence
   / inference]**
8. Users prefer *adaptable* (user-directed) over *adaptive* (system-directed)
   systems; autonomy-supportive framing reduces demand-avoidance. **[evidence /
   clinical]**
9. The defining challenge is **conflicting needs**: the same feature helpful for
   one profile can harm another (10 tensions catalogued in `CONFLICTS.md`). The
   research does not resolve these — they are decisions for the requirements
   phase, and most resolve to *user-set dimensions, not diagnostic presets*.
10. Local-first (Kleppmann 2019) is a coherent architecture philosophy aligned
    with the project's existing principles.

### Provisional research-informed principles (RESEARCH_MASTER §18)

Scaffold don't steer · user is the authority · predictable over clever · external
memory you can trust · time made visible · quiet by default · no scorekeeping ·
restraint at the compulsion surfaces · graceful degradation · personalisation
instead of diagnosis · AI optional and restrained · local/open/durable · neutral
primitives. **Provisional — subject to user review and later phases.**

### Evidence gaps / caveats (RESEARCH_MASTER §2, §17; BIBLIOGRAPHY "known gaps")

- Scoping synthesis, not systematic; English-language, US-indexed search.
- Several primary sources reached only as abstracts (paywalls).
- Most ADHD/autism cognitive research is on children; adult data thinner.
- Little/no controlled evidence for specific assistive features in *adults*
  (visual timers, body doubling, location reminders).
- Essentially no research on an *integrated* life-OS for this specific overlap.
- Long-term effects of heavy cognitive offloading on this population: unknown.
- Not yet searched: music-practice organisation / deliberate practice / flow;
  CSCW work on task managers as a genre; PKM longitudinal outcomes.

### Open questions carried forward (RESEARCH_MASTER §17)

Conflict-resolution design; offloading dependency; adult assistive-feature
efficacy; reminder design for reminder-fatigued users; AI reassurance safeguards;
whether any safe progress representation exists; the life-model primitives;
capacity-state adaptation; music mapping; what sustains long-term engagement.

### Next phase (as recommended at the close of Mission 002; since completed)

**PERSONAL REQUIREMENTS** — done (see the PERSONAL REQUIREMENTS MISSION 001 summary
above). A further (third) research pass remains possible for the `[U]` gaps but
was judged unnecessary to proceed.

### Pipeline

```
DEEP RESEARCH            <-- MISSION 001 + 002 done, pushed
  -> PERSONAL REQUIREMENTS   <-- MISSION 001 (+ correction) done, pushed (120567a)
  -> HUMAN / CENTRAL MODEL   <-- MISSION 001 done (1st pass); awaiting review
  -> SYSTEM DESIGN           <-- next, only after the model is explicitly accepted
  -> ARCHITECTURE
  -> LOW-LEVEL FOUNDATIONS
  -> CORE ENGINE
  -> STORAGE
  -> INTERFACE
  -> AI / ASSISTANT LAYER
  -> AUTOMATION
  -> TESTING / EVALUATION
  -> REAL-WORLD USE
  -> ITERATION
```

Each stage produces written artifacts before the next begins. The directory
layout in section 6 mirrors this pipeline.

## 3. Principles

These constrain every decision. If a choice violates one, it needs an explicit,
recorded justification in `decisions/`.

- **Local-first, privacy-first.** Data lives on the user's machine. The system
  works fully offline.
- **Minimal resource usage.** Small memory and CPU footprint is a feature.
- **High reliability.** Predictable, durable, hard to corrupt. Data outlives the
  software.
- **Excellent functionality.** Minimalism is not an excuse for a weak tool.
- **Long-term maintainability.** One person should be able to hold the whole
  system in their head.
- **Strong user control.** The user can inspect, export, and override everything.
- **No unnecessary cloud dependency.**
- **No unnecessary frameworks or abstraction layers.**
- **Low-level implementation where technically justified** — not everywhere for
  its own sake. C / C++ are the eventual core implementation candidates.
- **CLI / native interfaces before heavy web abstractions.**
- **Extensibility without architectural bloat.**
- **Understand before assembling.** The user wants to understand the system
  fundamentally, not wire libraries together blindly.

## 4. Domain note: music is not a special case (yet)

The user is a classical pianist. Practice sessions, repertoire, musical ideas,
creative projects, and performances must eventually fit **naturally** into the
same underlying life model as everything else.

Do **not** design a bespoke "music feature." First determine the general
abstractions (e.g. what a "session", an "artifact", a "commitment", a "thread of
work" is). Music should fall out of those as a case, not bolt on beside them.

## 5. Working rules for the agent

- Explain briefly what is being done and why.
- Keep every change small and reversible.
- Distinguish **facts**, **recommendations**, and **assumptions** explicitly.
- Preserve a clear checkpoint of project state (this file + git history).
- Ask before any irreversible change.
- Do not install large frameworks or dependencies.
- Do not generate documentation that no one asked for.
- Do not choose a final architecture.
- Stop when the current task is done; do not run ahead into the next phase.

## 6. Repository layout

| Path            | Holds                                                        |
|-----------------|-------------------------------------------------------------|
| `research/`     | Deep-research findings (Missions 001 + 002). Complete for now. |
| `requirements/` | Requirements spec (PERSONAL REQUIREMENTS MISSION 001). First pass done. |
| `model/`        | Human / central life model (HUMAN / CENTRAL MODEL MISSION 001). First pass done. |
| `design/`       | System design and architecture work. **Next phase**, on model acceptance. |
| `decisions/`    | Dated, lightweight decision records (one file per decision).|
| `experiments/`  | Throwaway probes and spikes. Never the real system.         |
| `src/`          | The eventual implementation. Placeholder only for now.      |

## 7. Environment snapshot (2026-09-09)

Facts, for reference. Not commitments.

- Ubuntu 24.04, kernel 6.14, x86_64, 12 cores, 16 GB RAM.
- Disk `/`: ~4 GB free at setup time — **low; clear headroom before building.**
- Present: gcc 14.3, g++ 14.3, clang 18.1.3, make, cmake 4.4.2, ninja,
  pkg-config, gdb, git 2.43, python 3.12 (+venv), node 20, rustc/cargo, go,
  jq, ripgrep, tmux.
- Not installed (not needed yet): sqlite3 CLI, valgrind, lldb, clang-format,
  clang-tidy, cppcheck, ctags, pandoc.

## 8. Open decisions (not yet made)

Deferred by design. Requirements are implementation-independent; these are for
SYSTEM DESIGN / ARCHITECTURE and later, and only after HUMAN / CENTRAL MODEL.

- The neutral primitive set / life model (`requirements/OPEN_QUESTIONS.md` OQ-1).
- Core language(s) and their boundaries.
- Storage substrate (flat files, embedded DB, custom format, ...).
- Interface medium(s) — requirements say only "multi-modal, user-choosable,
  consistent with the CLI-first leaning".
- Which local reasoning components / model sizes (OQ-11).
- The permission-grant mechanism (object-capability *style* is required by
  `MEL-REQ-180`; the mechanism is not chosen).
- The default consequential/routine action boundary (OQ-12).
- Whether an isolated network add-on is ever built (OQ-11; `MEL-REQ-167`).
- Whether any optional progress representation ships (OQ-6).
- Licensing (deferred).

The AI/assistant layer's *role* is now specified at requirements level (Parts II,
IV, V of `REQUIREMENTS_MASTER.md`): under the permission model, not owner; the
autonomy triad; optional; local reasoning for the core. *How* it is built is not
decided.

## 9. Checkpoints

- **2026-09-09** — Repository initialised. Branch `main`. Research-first scaffold
  created: directory structure, this file, README, `.gitignore`, `.editorconfig`.
  No dependencies. No code. Awaiting research mission.
- **2026-09-09** — Project renamed **Personal OS → MELFINA**.
- **2026-09-09** — **RESEARCH MISSION 001 complete (first pass).** Created
  `research/RESEARCH_MASTER.md`, `research/CONFLICTS.md`, `research/BIBLIOGRAPHY.md`;
  updated `research/README.md`. Scoping literature synthesis on designing for the
  ADHD + autism + OCD overlap. No requirements, no design, no architecture, no
  dependencies, no code. `src/` untouched.
- **2026-09-09** — Repo connected to GitHub remote
  `https://github.com/Levitationist/melfina` as `origin`; history pushed to
  `origin/main`. Permanent MELFINA git/GitHub workflow rule adopted (task → work →
  verify → update state → checkpoint → review diff → commit → push → verify clean
  tree). Mission 001 accepted by the user as a checkpoint.
- **2026-09-10** — **RESEARCH MISSION 002 complete (second, deeper pass).**
  Appended `RESEARCH_MASTER.md` §§20–32 + `RESEARCH CHECKPOINT 002`; added
  Second-Pass sections to `CONFLICTS.md` and `BIBLIOGRAPHY.md`; updated
  `research/README.md`. Adult-specific evidence, the 3-way overlap, musician/
  pianist context, CSCW/PIM, PKM/offloading trade-offs, controlled adult
  assistive-tech evidence, notification/reminder evidence, AI compulsion risk,
  self-tracking, academic prior art, and an evidence-quality audit of Mission 001
  (2 claims corrected, several refined, 1 partly downgraded to `[U]`; none
  disproven). New tier `[U]`. Contradictions preserved. No requirements, no
  design, no architecture, no language/storage choice, no dependencies, no code.
  `src/` untouched.
- **2026-09-10** — Research Missions 001 + 002 accepted as a checkpoint by the
  user and pushed (`d3160f0` on `origin/main`).
- **2026-09-10** — **PERSONAL REQUIREMENTS MISSION 001 complete (first pass).**
  Created `requirements/REQUIREMENTS_MASTER.md` (203 requirements `MEL-REQ-001…203`
  + 16 anti-requirements `MEL-AR-01…16` + MELFINA Capability Model + autonomy triad
  + action pipeline + feasibility classification + `REQUIREMENTS CHECKPOINT 001`),
  `requirements/CONFLICTS.md` (RC1–RC10 + VC1–VC7 as configurable dimensions /
  constraints / safeguards / open questions), `requirements/OPEN_QUESTIONS.md`
  (OQ-1…OQ-15 + deliberately-undecided list); updated `requirements/README.md` and
  this file. Includes a §18 deep research pass (~40 sources on autonomous agents,
  cognitive architectures, computer-use/coding agents, capability-based security,
  corrigibility, adjustable autonomy, ITS/learning science, local AI, continual
  learning, affective computing, computational creativity, automated discovery,
  agent memory, deskilling, XAI, prompt-injection/lethal-trifecta, BDI, abstention,
  self-improvement). **No architecture. No language (C/C++/Rust/…). No storage
  (SQLite/…). No framework, model, or UI toolkit. No dependencies. No code. `src/`
  untouched.**
- **2026-09-10** — **PERSONAL REQUIREMENTS MISSION 001 — dynamic-self-directed
  correction.** Reframed the capability model as a *vocabulary, not a fixed
  catalogue* (`MEL-REQ-016`); added **PART IV-B — THE DYNAMIC SELF-DIRECTED
  SYSTEM** (`MEL-REQ-204…253`): metareasoning / dynamic strategy selection;
  dynamic context selection as active reasoning; dynamic skill selection +
  capability-gap recognition + capability creation/evolution; human–MELFINA
  co-creation; a graded **nine-tier self-modification model** with **THE
  META-INVARIANT** (`MEL-REQ-235` — MELFINA cannot redefine the rules governing
  its own self-modification; external, monotonic, versioned, audited); dynamic
  resource allocation + dynamic autonomy; dynamic self-evaluation; "best is
  situational, dynamic ≠ unpredictable". Added anti-requirements `MEL-AR-17…19`,
  conflicts `VC8…VC12`, open questions `OQ-16…OQ-21`; extended the feasibility
  classification and `REQUIREMENTS CHECKPOINT 001`. Total now **253 requirements +
  19 anti-requirements**. Includes a dedicated research pass (~28 sources: rational
  metareasoning, adaptive test-time compute, anytime algorithms, LLM metacognition,
  skill-library learning, LLM tool-making, HTN planning, automatic curriculum,
  program synthesis / verified codegen, Gödel machine / Darwin Gödel Machine, safe
  dynamic software update, computational reflection, human-AI co-creativity,
  self-generated workflow orchestration, adaptive/self-RAG, self-improving-agent
  evaluation, monotonic confinement / external guardrails). **Still no
  architecture, no language, no storage, no framework/model/UI, no dependencies,
  no code. `src/` untouched.**
- **2026-09-10** — Research Missions 001+002 and PERSONAL REQUIREMENTS MISSION 001
  (+ correction) accepted as a checkpoint by the user and pushed (`120567a` on
  `origin/main`).
- **2026-09-10** — **HUMAN / CENTRAL MODEL MISSION 001 complete (first pass).**
  Created `model/HUMAN_CENTRAL_MODEL.md` (the model, 18 sections + `MODEL
  CHECKPOINT 001`), `model/MODEL_ALTERNATIVES.md` (4 candidate models compared —
  A event-centric · B entity/relationship-centric · C state/transition-centric ·
  D three-primitive — concluding a synthesis is necessary), `model/
  MODEL_OPEN_QUESTIONS.md` (`OQ-M1…OQ-M12`); updated `model/README.md` and this
  file. **Leading model E²CI: four primitives — ENTITY · EVENT · CLAIM ·
  INTENTION — in a bitemporal TIME dimension, with CONTEXT derived per situation
  (relevance theory), never stored.** All application categories (task / habit /
  note / calendar / project / goal / reminder / journal / dashboard / practice
  session / experiment / decision / hypothesis / capability / automation /
  strategy) emerge as *views or patterns*, not primitives. Grounded in BFO
  continuant/occurrent, event calculus, W3C PROV, epistemic/doxastic logic, AGM
  belief revision, bitemporal modelling, Anscombe/Searle direction of fit,
  Castelfranchi social commitment, activity theory, Sperber & Wilson relevance
  theory, Allen's interval algebra, Conway's self-memory system (grounding pass,
  ~15 sources). Music needs **zero** music-specific primitives; the agency
  pipeline and the dynamic self-directed MELFINA are both supported without
  freezing today's agent tech. The model deliberately does **not** contain the
  self-modification meta-invariant rules, any enforcement mechanism, any
  evaluative verdict, or any diagnostic category. Top open questions: is `State` a
  fifth primitive (OQ-M2); is `Relation` (OQ-M1); is four genuinely
  minimum-sufficient (OQ-M8); how are worries/affect represented (OQ-M5).
  `requirements/` and `research/` **not edited**. **No architecture, no
  technology, no schema, no format, no code. `src/` untouched.** Awaiting user
  review and explicit acceptance before SYSTEM DESIGN / ARCHITECTURE.
