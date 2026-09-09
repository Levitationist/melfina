# PROJECT_STATE

Single source of truth for where this project is and the rules it operates under.
Update this file whenever the phase changes or a principle is added, removed, or revised.

Last updated: 2026-09-09

---

## 1. What this is

**MELFINA** — a long-term effort to build a **lightweight, local-first personal
life operating system** designed around the actual cognitive, behavioral,
environmental, and practical needs of one user, rather than around a generic
productivity methodology.

MELFINA is the official project name (set 2026-09-09). Earlier notes may say
"Personal OS"; that was a placeholder.

## 2. Current phase

**Phase: DEEP RESEARCH — RESEARCH MISSION 001 complete (first pass), awaiting
user review. No further research until the user gives the next mission.**

We are NOT coding the application. No architecture has been chosen. No framework
or runtime dependency has been adopted. Nothing in the research output is a
requirement or a design commitment.

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

### Next phase

**PERSONAL REQUIREMENTS** — with the user: elicit real personal frictions and
needs in the user's own terms; place the user on each `CONFLICTS.md` axis; decide
which implications become requirements and at what priority. Optionally a second
research pass first (library-database access) to close the gaps above.
**Do not start until the user gives the requirements mission.**

### Pipeline

```
DEEP RESEARCH            <-- MISSION 001 done (1st pass); awaiting review
  -> PERSONAL REQUIREMENTS   <-- next, on user's go
  -> HUMAN / CENTRAL MODEL
  -> SYSTEM DESIGN
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
| `research/`     | Deep-research findings, surveys, source notes, comparisons. |
| `requirements/` | The user's real, personal requirements once elicited.       |
| `model/`        | The human / central life model — core abstractions.         |
| `design/`       | System design and architecture work.                        |
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

- Core language(s) and their boundaries.
- Storage substrate (flat files, embedded DB, custom format, ...).
- Interface shape (pure CLI, TUI, local daemon + clients, ...).
- Whether/how an AI assistant layer participates.
- Licensing (deferred).

## 9. Checkpoints

- **2026-09-09** — Repository initialised. Branch `main`. Research-first scaffold
  created: directory structure, this file, README, `.gitignore`, `.editorconfig`.
  No dependencies. No code. Awaiting research mission.
- **2026-09-09** — Project renamed **Personal OS → MELFINA**.
- **2026-09-09** — **RESEARCH MISSION 001 complete (first pass).** Created
  `research/RESEARCH_MASTER.md`, `research/CONFLICTS.md`, `research/BIBLIOGRAPHY.md`;
  updated `research/README.md`. Scoping literature synthesis on designing for the
  ADHD + autism + OCD overlap. No requirements, no design, no architecture, no
  dependencies, no code. `src/` untouched. Awaiting user review before the
  PERSONAL REQUIREMENTS mission.
