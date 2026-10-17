# Changelog

All notable changes to GRAPH Lab. Format based on Keep a Changelog.

## [0.1.0] — 2026-10-16

### Added

- Exact graph engine (Rust): bitset adjacency (`MAX_VERTICES = 64`), strict
  graph6 codec with round-trip validation, canonicalization (nauty/Traces
  primary, exhaustive lex-max fallback capped at n ≤ 8), unlabeled
  enumeration to n = 6 (counts 1,1,2,4,11,34,156), and invariant
  computation (components, triangles, chromatic number via DFS,
  independence/clique via DP, degree sequence, edge count).
- Hadwiger number engine: deletion-tolerant branch-set search with
  edge-count upper-bound pruning, exact for n ≤ 10.
- Witness search pipeline with schema `graphlab-witness/1`, domain modes
  (exhaustive-unlabeled, sampled), evidence classes, per-witness
  independent recheck, minimization, and TOML campaign configs.
- Python control plane (`python/graphlab/`): networkx + igraph oracles,
  graph6 encode/decode, independent cross-check scripts.
- Lean 4 / Mathlib bridge (`lean/`): kernel decision procedures for clique,
  coloring, branch-set model, and complement; `SimpleGraph` bridge via
  `ofBoolMatrix`; kernel-checked certificates in `Facts.lean` for K4, P4,
  C5, CP(2), CP(3).
- CLI: `doctor`, `enumerate`, `hadwiger`, `search`, `minimize`, `verify`,
  `benchmark`, `canon`, `g6`, `hash`.
- CI: Rust (fmt/clippy/test/release/doctor/hash), Python
  (ruff/mypy/pytest), and Lean (`lean_audit.py --run`) gates.

### Research

- Measured `h(CP(t)) = t + floor(t/2)` for the cocktail party graph CP(t) =
  K_{2t} minus a perfect matching, at t = 2,3,4,5 (values 3,4,6,7).
  t = 2 is LEAN_CHECKED; t = 3,4,5 are EXACT_FINITE. The earlier guess
  `h = t+1` is false (died at t = 4). The general formula is OPEN.
  See `docs/RESEARCH_PROGRAM.md` and `results/research/M5-report.md`.

### Changed

- CI no longer hides failures: the Python job's `|| true` fallbacks were
  removed (they masked real ruff/mypy/pytest failures), and the Lean job
  is enabled instead of `if: false`.
- `scripts/lean_audit.py` now rejects empty tracked `.lean` source files —
  a silent `lake build` replay of a cached olean was the root cause of a
  false M3 "build passed" report.

## [Unreleased]