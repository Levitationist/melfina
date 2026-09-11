# MELFINA — FOUNDATION 2: STRUCTURED ACTION MODEL

**Phase:** LOW-LEVEL FOUNDATIONS — MISSION 001. First pass, pending review.
**Baseline:** `8a40207`. Read F1 (`CAPABILITY_GRANT_MODEL.md`) first.

**CONTAINMENT-CRITICAL (RC-4 / AU-2).** The containment of MELFINA's terminal and
GUI power lives *entirely* in a grant being checked against a **parsed,
structured action**, never a raw string. A coarse or string-based action model
collapses the boundary. This document fixes the abstract action representation
and the normalisation/canonicalisation it must undergo before authorisation.

**This document does not choose a parser, a grammar, or a serialization.** It
fixes the *shape* an action must have and the *guarantees* the eventual parser
must provide.

Derives from: **RC-4**; **INV-3, INV-7, INV-8**; **MEL-REQ-143, 144, 145, 146,
181, 142**; the exec-vs-shell distinction, path canonicalisation / TOCTOU
literature `[E]`.

---

## 0. Dependencies discovered

- **On F1:** the `scope` and `action-constraints` fields of an effect-bearing
  grant are filled by a structured action from this document. A grant is checked
  against a **normalised structured action** (§4), and the grant itself stores
  the **canonicalised** target identities.
- **On F3:** the Reference Monitor's "action within the authorised proposal"
  check (its interpretation step) is, for terminal/GUI/file effects, a
  **structural match** defined here — not string comparison.
- **On F7 (isolation):** the sandbox must be able to *observe* and *gate* each
  structured effect at its boundary; it cannot do that if the effect reaches it
  as an opaque blob. The structured form crosses the sandbox boundary.
- **New dependency flagged:** F1's `local-shell` effect class and this document's
  "shell interposition is distinct" are the same rule seen from two sides; they
  must stay consistent (they do).

---

## 1. Why not a string

A terminal action expressed as a string (`"git commit -m 'x' && rm -rf build"`)
requires the authoriser to *parse the user's intent out of a shell language* to
know what it will do. Shell languages are Turing-complete, context-sensitive, and
full of implicit behaviour (globbing, brace/tilde/parameter/command/arithmetic
expansion, word splitting, quote removal, redirections, background jobs, subshells,
control operators). A grant that says "permit exactly `git commit -m 'x'`" is
**defeated** by `git commit -m 'x'; <anything>`, by `$(git commit -m 'x')`, by
`g""it commit …`, by `/usr/bin/git commit …`, by a `git` earlier on `PATH`, and by
a dozen other spellings.

**The exec family (`execve` and relatives that take an explicit argv vector and no
shell) is not vulnerable to command injection** because there is no shell to
interpret metacharacters (`[E]`). The structural distinction — *an argv vector
handed to the OS* vs *a string handed to a shell* — is the containment boundary.

**Rule (RC-4):** MELFINA's default and safe form of local execution is a
**direct, structured process invocation with an explicit argv and no shell**.
Running anything through a shell that can expand/chain/substitute is **shell
interposition**, a **distinct, higher-risk capability** (`local-shell`, F1 §3),
never implied by a `local-exec` grant, never treated as equivalent to an
exact-command grant. `[SEN]`.

---

## 2. The structured action — abstract shape

An **action** is a typed record. The type determines the fields. The following
are the effect kinds the foundations must support; the list is extensible **only
by adding a new typed record, never by widening an existing one to accept freer
input**.

### 2.1 `process-exec` (a `local-exec` effect)

| Field | Meaning | Constraint |
|---|---|---|
| `program` | the **resolved, canonicalised absolute path** of the executable (§3) | not a bare name; not `PATH`-resolved at run time; the grant stores this resolved identity |
| `argv` | the **ordered vector of argument strings**, `argv[0]` explicit | a vector, never a single string to be re-split; no element is re-interpreted |
| `env` | an **allow-list** of environment variable names→values the child receives | default: an empty or minimal fixed set; the grant names exactly which vars are passed; nothing inherited implicitly (esp. not `PATH`, `LD_*`, `IFS`, `BASH_ENV`) |
| `cwd` | the **resolved, canonicalised** working directory | within the grant's file scope |
| `stdin` / `stdout` / `stderr` | each is one of: `closed`, `null`, `pipe-to-melfina`, `file(<resolved path within scope>)`, `inherit(<explicitly granted fd>)` | no implicit inheritance of the parent's descriptors |
| `resource-limits` | wall-clock, CPU, memory, output-size, child-count ceilings | hard; from the grant |
| `child-authority` | what the child process may itself do: by default **nothing beyond `resource-limits`** — no ability to spawn, no new fds, no network | a child does **not** inherit MELFINA's authority; if a workflow needs a child to act, that is a separate structured action with its own grant |

**No `shell` field.** A `process-exec` is never routed through a shell.

### 2.2 `shell-exec` (a `local-shell` effect — distinct, higher-risk)

Same fields as `process-exec` plus:

| Field | Meaning |
|---|---|
| `shell` | the resolved shell executable |
| `script` | the shell text to be interpreted |
| `interpretation-note` | a required, recorded acknowledgement that this action can expand / chain / substitute and is therefore high-risk |

A `shell-exec` action can **only** be produced when a `local-shell` grant exists
(F1). Its risk class is **high** by construction (F1 §3, `CAPABILITY_MODEL.md`
§5). The Monitor cannot be "tricked" into treating a `shell-exec` as a
`process-exec` — they are different record types. `[SEN]`.

### 2.3 `gui-op` (a `gui-act` effect)

| Field | Meaning | Constraint |
|---|---|---|
| `target` | a **structured descriptor** of the element/window: application id + a stable element identity (accessibility-tree node id / role+name path / window id) — **not** a raw pixel coordinate where a structured identity is obtainable | the grant names the target application and a bounded set of target descriptors |
| `operation` | one of a **closed set**: `activate`, `set-text(<value>)`, `select`, `invoke(<named command>)`, `read-state`, `scroll`, `key(<named key or chord>)` | not "type this arbitrary keystroke stream"; a text field is filled with `set-text`, not simulated keypresses, where the platform allows |
| `value` | the datum for `set-text` / `select` | bounded; recorded |
| `fallback-coordinate` | **only** permitted when no structured identity exists, explicitly flagged, and recorded as a lower-assurance action | raises the action's risk; the grant must explicitly allow coordinate fallback |

**Rule:** GUI actions are structured **where practical** (RC-4). Where the
platform genuinely offers no structured handle, the coordinate form is an
explicit, recorded, higher-risk fallback — not the default, not silent. `[ID]`
(what "practical" means is platform-specific and a build concern).

### 2.4 `file-op` (a `file-read` / `file-write` effect)

| Field | Meaning | Constraint |
|---|---|---|
| `path` | the **resolved, canonicalised absolute path** (§3) | stored resolved in the grant; re-resolution that differs ⇒ deny (TOCTOU) |
| `operation` | one of: `read`, `read-range`, `write`, `append`, `create`, `truncate`, `rename(<resolved dest in scope>)`, `delete`, `stat`, `list` | enumerated; `delete` / `truncate` / `rename` are irreversible-class and get the extra confirmation (`MEL-REQ-181`) |
| `content-bound` | max bytes for a write | from the grant |

**The Ring-0 governance region is a hard, non-overridable exclusion** on every
`file-op` — the resolved path is rejected if it intersects the Ring-0 region,
regardless of any grant (RC-1). `[SEN]`.

### 2.5 `process-op` (control of an already-running child)

`signal(<named signal>)`, `wait`, `read-output`, `close-input`. Scoped to a
`child-id` produced by a prior `process-exec` under the same activity chain.

### 2.6 `chronicle-op`, `notify-op`, `capability-op`

These effect-bearing actions are also structured records (append these unit
shapes / enqueue this notification within the Gateway rules / mint this
capability with this declared authority). They are less prone to the
string-injection problem but follow the same discipline: **enumerated operations,
bounded parameters, no free-text field that changes what the effect does.**

---

## 3. Canonicalisation

Before an action is authorised, every **identity-bearing field** (`program`,
`cwd`, `path`, `rename` destination, `env` values that are paths) is
**canonicalised** to a stable form. The canonicalisation must (acceptance
criteria — `[ID]`):

1. **Resolve to an absolute path** bound to a fixed, known root — never leave a
   relative component to be resolved later in a different context.
2. **Collapse `.` and `..`** and repeated separators.
3. **Resolve every symlink** in the path (each component), and record the fully
   dereferenced target — the grant is over the *real* object, not a link that can
   be re-pointed.
4. **Apply the platform's name normalisation** (Unicode normalisation form, case
   folding where the filesystem is case-insensitive) so alternate spellings of
   the same object collapse.
5. **Reject** rather than guess: an unresolvable path, a path that escapes the
   fixed root, a path with a component that does not exist where existence is
   required, or a path that resolves into an excluded region ⇒ the action is
   **not authorised** (fail closed).
6. **Bind the resolution result into the grant.** At each guarded effect the
   Monitor re-canonicalises and compares to the stored identity; **any
   difference ⇒ deny** (Time-Of-Check-To-Time-Of-Use defence). The Monitor does
   **not** silently re-resolve to the new target. `[SEN]` for the rule; `[ID]`
   for the atomicity of check-and-use (the mechanism — e.g. operating on an
   already-opened handle rather than re-opening by name — is a build concern).

**`program` resolution specifically:** never `PATH`-searched at run time (that is
`execvp`/`execlp` behaviour, unsafe unless `PATH` is trusted — `[E]`). The grant
names the resolved absolute executable; `env` does not carry `PATH` to the child
unless explicitly granted.

---

## 4. Normalisation of the action itself

Beyond identity fields, the **whole action record** is normalised to a canonical
form before the grant check, so that two representations of the same effect
compare equal and one representation of two effects does not slip through:

| Normalisation | Prevents |
|---|---|
| `argv` is a vector of already-split strings; no element is ever re-split, re-quoted, or expanded | word-splitting / quote-removal / glob smuggling |
| no field may contain an embedded action (no "argv element that is itself a command to run") | nested-execution smuggling |
| `env` is a fixed map, keys sorted, values literal | ordering / duplicate-key ambiguity; `IFS`/`BASH_ENV`/`ENV` injection |
| numeric fields (limits, ranges, bounds) are integers in a fixed unit | unit/parse ambiguity |
| enumerated fields (`operation`, `stdin` kind, signal names) are drawn from a **closed set**; an unknown value is a malformed action, not a pass-through | "unknown operation treated permissively" |
| the record has a **fixed set of fields for its type**; extra fields ⇒ malformed | field-injection |
| a `process-exec` with any field that only `shell-exec` has ⇒ malformed | type confusion |

**The parser's contract:** given a structured action, produce **exactly one**
canonical form, or **reject**. It must be **total** (defined on all inputs — every
input either canonicalises or is rejected) and **deterministic** (same input ⇒
same canonical form). It must **never** "best-effort" a malformed action into a
plausible one. `[ID]` — this is the acceptance criterion the eventual parser is
tested against.

---

## 5. The core invariant (RC-4)

> A grant for **`program X` + `argv A`** authorises **only** the effect
> "invoke the canonical object *X* with the exact canonical vector *A*, with the
> granted `env`/`cwd`/fds/limits". It does **not** authorise:
>
> - `program X` + `argv B` (`B ≠ A`);
> - `shell → program X + argv A` (that needs a `local-shell` grant);
> - `program X + argv A` **followed/preceded by** any other command;
> - `program X` reached via a different `PATH` entry, a symlink, a relative path,
>   or an alternate spelling;
> - a **child** of *X* performing further effects;
>
> **unless the grant explicitly authorises that broader operation** (a wider
> `argv` pattern, a `local-shell` grant, an additional structured action, an
> explicit `child-authority`).

`[SEN]` — this follows from the structured shape + canonicalisation + the "one
grant per guarded effect" rule (F1 §8). `[ETL]` that a specific implementation's
parser + Monitor check actually has no bypass — this is the single most important
red-team target of LOW-LEVEL FOUNDATIONS.

---

## 6. Observation vs execution

Terminal and GUI capabilities separate **observe** (read state: `stat`, `list`,
`read-state`, `read-output`) from **execute** (cause an effect) into distinct
operations with distinct grants (`MEL-REQ-143`, SYSTEM_ARCHITECTURE §12). An
`observe` grant can never cause an effect. `[SEN]`.

---

## 7. Contract Q&A

| Question | Answer |
|---|---|
| **Guarantees** | every effect-bearing action reaching the Monitor is a typed, closed-field, canonicalised record with exactly one canonical form; a `process-exec` grant cannot be exercised as a shell, a different argv, a different binary, or a chain |
| **Requires** | a total, deterministic parser/canonicaliser (F-build); a filesystem/OS able to expose stable object identities; the Monitor's structural-match check (F3) |
| **Trusts** | the OS's resolution of an *already-opened handle*; the canonical form produced by its own parser |
| **Distrusts** | any string as an action; `PATH`/`env`-time resolution; a name re-resolved at use time; a parser's "best guess"; a coordinate when a structured target exists; the child process |
| **Enters** | a proposed structured action (from a Ring-2 proposal or a user-invoked request) |
| **Leaves** | either exactly one canonical action record, or a rejection with a reason (recorded) |
| **Malformed input** | **rejected**, recorded as a malformed-action Event, never coerced; the pipeline stops the action at PROPOSE/AUTHORISE |
| **Failure** (parser crash / resolution error / ambiguous canonicalisation) | **fail closed** — the action is not authorised; recorded |
| **Authoritative** | the canonical action record is authoritative for what the effect *is*; the OS is authoritative for what the object *is* (via an opened handle) |
| **Independently verifiable** | yes — the parser is a pure function testable against a fixture corpus of {input → expected canonical form or expected rejection}, including an adversarial corpus (spellings, expansions, traversal, type-confusion). VERIFY re-reads the *actual* post-state against the *canonical* intended effect (F10) |

---

## 8. Enforceability summary

| Invariant | Class |
|---|---|
| Actions are typed, closed-field records; no free-text field changes the effect | **[SEN]** |
| `process-exec` never routed through a shell; `shell-exec` is a distinct type + grant | **[SEN]** |
| grant for `X+A` ≠ authority for `X+B` / shell / chain / alt-spelling | **[SEN]** rule / **[ETL]** no bypass in the built parser+check |
| identity fields canonicalised (abs path, `..` collapsed, symlinks resolved, name-normalised) before authorisation | **[ID]** |
| grant stores resolved identity; re-resolution difference ⇒ deny (TOCTOU) | **[SEN]** rule / **[ID]** atomic check-and-use mechanism |
| Ring-0 region excluded from every `file-op`, non-overridably | **[SEN]** |
| parser is total + deterministic + never best-effort | **[ID]** (acceptance criterion) |
| GUI structured where practical; coordinate fallback explicit + recorded + higher-risk | **[ID]** (platform-dependent) |
| observe ≠ execute (distinct grants) | **[SEN]** |
| child process inherits no MELFINA authority | **[SEN]** rule / **[ID]** the OS mechanism that drops it |

---

## 9. Deferred (not decided here)

- The concrete grammar / schema for each action type and its serialization.
- The parser implementation and language.
- The canonicalisation library / OS calls used (and the atomic check-and-use
  technique — operate-on-handle vs re-open).
- Which GUI-automation substrate exposes structured targets on the eventual
  platform, and where the "practical" line falls.
- The adversarial fixture corpus (built at LOW-LEVEL FOUNDATIONS + maintained).
- Whether an `argv` **pattern** language (e.g. "`git commit -m <any string>`") is
  offered, and if so its exact expressiveness — it must stay decidable and
  non-Turing-complete. `[OPEN]`.

## 10. Traceability

| Element | Source |
|---|---|
| structured actions not raw shell; shell interposition distinct; AU-2 containment-critical | RC-4; SYSTEM_ARCHITECTURE §12, §27; ARCHITECTURAL_ALTERNATIVES AA-6 |
| computer actions run the full pipeline; narrow reversible pre-authorised scopes only | `MEL-REQ-144` |
| terminal and GUI are separate capabilities/grants | `MEL-REQ-143` |
| no blanket machine authority; scoped, logged, revocable | `MEL-REQ-146` |
| irreversible actions get an extra deliberate confirmation | `MEL-REQ-181` |
| code actions verified and reversible | `MEL-REQ-142` |
| assume adversarial content sometimes wins; contain blast radius | INV-8; `MEL-REQ-185` |
| exec-vs-shell, canonicalisation, TOCTOU | command-injection / path-resolution literature `[E]` |
