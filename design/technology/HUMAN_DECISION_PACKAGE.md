# MELFINA — HUMAN DECISION PACKAGE: TECHNOLOGY SELECTION MISSION 001

**Purpose of this file.** `TECHNOLOGY_SELECTION.md` and its ten companion
documents are the *evidence and reasoning*. This file is the **thing to
actually read and decide on** — six decision areas, each with what you'd be
approving, why it matters if it's wrong, the recommendation, the honest
alternative, and a place to record your call. Nothing here is new analysis;
every claim links back to where it was argued in full.

**A correction, made while assembling this:** earlier documents (including
`PROJECT_STATE.md`) described "twelve decisions" requiring sign-off. That
count was imprecise. The accurate picture: **six decision areas**, spanning
roughly sixteen individual technology choices in `TECHNOLOGY_DECISION_LOG.md`
(several choices are bundled under one approval because they stand or fall
together — e.g. the signature scheme and the hash function). `PROJECT_STATE.md`
has been corrected to say "six decision areas" rather than "twelve
decisions."

**How to use this:** for each area, pick one — **Approve** (build against
it as recommended) / **Approve with changes** (say what to change) /
**Reject** (say why, and whether you want a re-evaluation or you're
supplying the answer yourself) / **Need more information** (ask, and I'll
either answer from the existing research or say honestly that it needs a
fresh look). Nothing gets built against any of these until you've marked it.

---

## Decision 1 — Primary implementation language

**What you'd be approving:** Rust as the one language the trusted core
(governance, the Chronicle, the Supervisor) is written in. No second
language is being adopted for the core at this time — isolated capabilities
(math, local AI) may internally use something else, but that's a packaging
detail, not part of this decision.

**Why it matters if it's wrong:** this is the single most expensive thing
to reverse in the entire stack — once real code exists, changing the
primary language means rewriting it.

**Recommendation:** Rust. **Confidence: HIGH.** It's the only candidate that
is both memory-safe by default (ruling out C/C++) and has a type system
strong enough to make the closed vocabularies in the foundation contracts
(effect classes, action types, governance tiers) compiler-checked, not just
documented.

**Honest alternative:** Zig — a genuinely interesting "boring, explicit"
language — but it's pre-1.0 as of this evaluation, which is a real risk for
something you'll maintain solo for years. Recorded as the fallback if Rust
turns out to be a bad fit in practice, not rejected as inferior on the
merits.

**Rejected outright:** C/C++ (fails the memory-safety requirement for
exactly the code that most needs it), Go (weaker fit for the closed-
vocabulary encoding, and its C-interop story is friction-heavy at the
terminal/GUI boundary), OCaml/Haskell (better type systems in theory, worse
fit for a solo maintainer in practice), Swift (weak Linux systems-
programming ecosystem).

**Full detail:** `LANGUAGE_EVALUATION.md`.

**Your call:** ☐ Approve ☐ Approve with changes: _____ ☐ Reject ☐ Need more info

---

## Decision 2 — Chronicle storage substrate

**What you'd be approving:** your entire life record is stored as (a) a
small, purpose-built append-only log file we write ourselves, as the
authoritative copy, plus (b) SQLite as a disposable, rebuildable index/cache
on top for fast queries.

**Why it matters if it's wrong:** this is the format of data you'll
accumulate for years. Getting it wrong is expensive to fix after the fact,
though notably *less* expensive than the language, because the cache half
is disposable by design and the log format is simple enough to migrate if
needed.

**Recommendation:** the two-tier design above. **Confidence: MEDIUM** — the
one piece of genuinely new, unfuzzed code in the whole stack is our own
log-writing logic, so this is conditional on a crash/power-loss test that
hasn't been run yet (see `TECHNOLOGY_EXPERIMENT_PLAN.md` item 1 — this is
the single highest-priority thing to test before trusting real data to it).

**Honest alternative:** LMDB as the authoritative store instead of our own
log, if that crash-testing finds problems with the custom approach. This is
a named, ready fallback, not a hypothetical one.

**Rejected outright:** PostgreSQL (needs a running server, which conflicts
with "runs on your machine, nothing else required"), RocksDB (its whole
design is built around discarding old data during background cleanup, which
directly fights the requirement that nothing in your history ever silently
disappears).

**Full detail:** `CHRONICLE_EVALUATION.md`.

**Your call:** ☐ Approve ☐ Approve with changes: _____ ☐ Reject ☐ Need more info

---

## Decision 3 — Ring-3 isolation (how a capability/skill is contained)

**What you'd be approving:** every time MELFINA runs something with real
effects (a file operation, a terminal command, a GUI action), it happens
inside a locked-down process — unable to see the network, unable to see
files it wasn't given, unable to touch anything outside its box — using
four composed Linux kernel features (Landlock, seccomp, namespaces,
cgroups). For the rare, especially risky action, an even stronger box (a
tiny separate virtual machine) is used instead.

**Why it matters if it's wrong:** this is the actual thing standing between
"a capability misbehaves" and "a capability does damage." It's the most
security-critical technology decision in the whole stack.

**Recommendation:** the composed-kernel-features approach as the everyday
default; a small VM reserved for the rare high-risk case. **Confidence:
HIGH** — these Linux features specifically became solid enough to rely on
for exactly this kind of unprivileged, no-root sandboxing only in the last
kernel cycle or two, which is worth knowing since it means this wasn't
really a viable choice even a couple of years ago.

**Honest, permanent caveat:** no sandbox is unbreakable. A kernel-level bug
could, in principle, defeat any of this. This isn't a flaw specific to what
was chosen — it's true of every option evaluated, including the heavier
ones — and the response plan for if it ever happens is documented, not
glossed over.

**Rejected outright:** using a full container system (Docker-style) as the
default — it would add real complexity for no actual extra protection, since
its isolation comes from the exact same kernel features we'd be using
directly anyway.

**Full detail:** `ISOLATION_EVALUATION.md`.

**Your call:** ☐ Approve ☐ Approve with changes: _____ ☐ Reject ☐ Need more info

---

## Decision 4 — How the "governance never trusts a capability, and reasoning never gets authority" rule is actually built

**What you'd be approving:** the part of MELFINA that decides "is this
allowed?" runs as its own separate program, walled off from everything
else — even from the rest of MELFINA's own core. Similarly, the reasoning
part (wherever an AI model is involved) also runs separately and physically
cannot hand itself permission to do anything; it can only ask.

**Why it matters if it's wrong:** this is the concrete, load-bearing answer
to "how do you actually stop reasoning from becoming authority" — not a
policy on paper, but a wall a program cannot walk through.

**Recommendation:** exactly the separation described above, with the pieces
talking to each other over a narrow, authenticated local channel.
**Confidence: HIGH.**

**Full detail:** `PROCESS_AND_IPC_EVALUATION.md`.

**Your call:** ☐ Approve ☐ Approve with changes: _____ ☐ Reject ☐ Need more info

---

## Decision 5 — Governance integrity: how "MELFINA can't rewrite its own rules" has actual teeth

**What you'd be approving:** the constitutional rules MELFINA operates
under are cryptographically signed by a key that **never touches the
machine MELFINA runs on** — you'd sign a new version of the rules yourself,
offline, with a small separate tool, and MELFINA can only check the
signature, never produce one. Additionally, if your hardware has a TPM
chip (a small security chip most computers from the last several years
have), it's used to stop an old, previously-valid rule set from being
snuck back in later.

**Why it matters if it's wrong:** this is what makes "the AI cannot expand
its own permissions" more than a design intention — it's what makes it
physically true.

**Recommendation:** Ed25519 signatures + SHA-256 hashing (both extremely
standard, boring choices, deliberately not exotic) + an offline key.
**Confidence: HIGH** for that baseline.

**Two things genuinely need your input, not just approval:**
1. **Do you want to also use a hardware security key** (like a YubiKey) for
   signing, on top of the offline-key baseline? This is optional — the
   baseline already keeps the key off the running machine — but a hardware
   key would additionally protect against someone getting hold of the key
   *file* itself.
2. **Does your actual machine have a TPM chip?** If yes, it gets used for
   the rollback-protection piece (recommended, strong). If not, a weaker
   fallback is used instead, and you should know that going in rather than
   discover it later.

**Full detail:** `CRYPTO_GOVERNANCE_EVALUATION.md`.

**Your call:** ☐ Approve baseline ☐ Also want hardware key: _____ ☐ TPM present: yes/no/unsure ☐ Need more info

---

## Decision 6 — Local AI runtime

**What you'd be approving:** the technology used to run a local AI model on
your own machine (`llama.cpp`, a well-established, widely-used piece of
software for exactly this). **Which specific model to run is deliberately
not part of this decision** — that's a separate, later choice.

**Why it matters if it's wrong:** less than the others, honestly — this
piece is explicitly designed to be swappable, and it holds zero authority
regardless of which model runs inside it or how it behaves.

**Recommendation:** `llama.cpp`. **Confidence: HIGH** for the engine.

**Full detail:** `REASONING_COMPUTATION_EVALUATION.md`.

**Your call:** ☐ Approve ☐ Approve with changes: _____ ☐ Reject ☐ Need more info

---

## What happens after you mark these

- Anything marked **Approve** becomes the accepted basis for CORE ENGINE.
- Anything marked **Approve with changes** gets folded back into the
  relevant evaluation document and re-checked against the foundation
  contracts before being treated as accepted.
- Anything marked **Reject** stays open — either I re-run that category's
  evaluation against different constraints you specify, or you tell me
  what to use instead and I check *that* against the foundation contracts
  (F1–F10) for compatibility, the same way every recommendation here was
  checked.
- **CORE ENGINE does not begin, and no dependency gets installed, until
  this package has your marks on it.** The recommended very first piece of
  actual work, once approved, is the crash/power-loss test on the Chronicle
  log (Decision 2) — before any real data ever touches it.

## Everything NOT in this package

The other ~10 technology choices in `TECHNOLOGY_DECISION_LOG.md` (build
system, terminal/GUI mechanism, math/numerical libraries, migration policy,
platform strategy, and so on) are lower-stakes, offered as reviewable
recommendations, and are **not** blocking — CORE ENGINE can proceed on them
without a formal sign-off here, though you're of course welcome to weigh in
on any of them too.
