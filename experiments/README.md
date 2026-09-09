# experiments/

Throwaway probes and spikes: measure a syscall cost, try a storage format,
test a parsing approach. One subdirectory per experiment, with a short
`README.md` stating the question and the finding.

**Belongs here:** disposable code written to answer one question.

**Does not belong here:** anything the real system depends on. Nothing graduates
from `experiments/` to `src/` by being copied — findings inform a fresh design.

Build trees under here are git-ignored.
