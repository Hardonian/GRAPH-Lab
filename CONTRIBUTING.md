# Contributing

This is a research engineering repository with strict verification gates.

- One PR per milestone scope; conventional commit messages
  (`feat(core):`, `perf(hpc):`, `research:`, `chore:`).
- `scripts/verify.sh` must pass before review.
- Any new mathematical claim needs: exact statement, evidence ledger entry
  (command, input hash, RNG seed, exit code), status label, and a
  novelty/literature note.
- Tests must cover failure behavior (invalid graph6, malformed witnesses,
  solver unknown, missing backend), not only happy paths.
