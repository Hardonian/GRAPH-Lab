# Milestone 5 — research campaign report

Date: 2026-10-16 · Status: **CLOSED** (engine + independent oracle + Lean kernel)

## Candidate C4 (rank 1): Hadwiger number of cocktail party graphs

**Statement (candidate theorem).** For t ≥ 2, let CP(t) = K_{2t} minus a perfect
matching (the cocktail party graph on 2t vertices). Then
`χ(CP(t)) = t` (classical), `ω(CP(t)) = t` (classical), and we measured
`h(CP(t)) = t + ⌊t/2⌋` for t = 2, 3, 4, 5.

**Result: the guessed pattern `h = t + 1` is FALSE.** The program doc
(`docs/RESEARCH_PROGRAM.md`) hypothesized h(CP(t)) = t + 1; that died at t = 3.

| t | n = 2t | graph6 | m | χ | h(CP(t)) | t+1 | t+⌊t/2⌋ |
|---|--------|--------|---|---|----------|-----|---------|
| 2 | 4 | `Cl` | 4 | 2 | **3** | 3 | 3 |
| 3 | 6 | `EznW` | 12 | 3 | **4** | 4 | 4 |
| 4 | 8 | `G~]}~[` | 24 | 4 | **6** | 5 | 6 |
| 5 | 10 | `I~||}~n|w` | 40 | 5 | **7** | 6 | 7 |

## Evidence classes

| t | h value | class | source |
|---|---------|-------|--------|
| 2 | 3 | LEAN_CHECKED | `lean/GraphLab/Facts.lean` `C2_hadwiger` — kernel recomputes `hadwigerNumber 4 A2` by exhaustive scan (5^4 branch codes + coloring scan), `decide`. Also `C2_h3` (K3-minor certificate code 229) and `C2_not_h4` (full K4-minor scan = false). |
| 3 | 4 | EXACT_FINITE | engine `hadwiger` CLI on `EznW`; independent Python oracle agrees. Lower bound certificate digits [1,1,1,2,3,4] base 5 = 14656 → `C3_model4`/`C3_h4` in Facts. Upper bound (h < 5) is engine-only, not kernel-checked — scope stated. |
| 4 | 6 | EXACT_FINITE | engine + Python oracle agree. No Lean certificate (7^8 ≈ 5.7M branch codes exceeds kernel compute budget). |
| 5 | 7 | EXACT_FINITE | engine + Python oracle agree. No Lean certificate (11^10 ≈ 2.6×10^10). |

## Trust model applied

- t = 2 is **LEAN_CHECKED**: the kernel recomputes the whole number from the
  explicit matrix, no engine output is trusted on faith.
- t = 3, 4, 5 are **EXACT_FINITE**: two independent implementations (Rust
  bitset engine, Python networkx/igraph oracle) agree on every value. The
  engine's Hadwiger search is deletion-tolerant branch-set search with
  edge-count upper-bound pruning (see M2 bug fix in `docs/STATUS.md`).
- No claim is made that `t + ⌊t/2⌋` holds for all t. It is a measured pattern
  on 4 data points; a proof or a counterexample at t = 6 would require n = 12,
  outside the engine's n ≤ 10 scope (and outside the Lean kernel's budget).

## Novelty

Web search (2026-10-16) found no exact closed formula for h(CP(t)). The
closest structural results are the Kostochka–Stiebitz complement inequality
and Woodall's h(G)·α(G) ≥ n subconjecture. CP(t) is the complement of
t disjoint K2's, so it is a well-studied family, but its Hadwiger number does
not appear to be tabulated. **NOVELTY_REVIEWED** for the measured values
3, 4, 6, 7 at t = 2..5; the general formula is **OPEN**.

## Honest scope

This is not a theorem. It is a verified finite computation plus a conjecture
with 4 data points. If the pattern breaks at t = 6, the honest report says so.

## Kill criteria outcome

The C4 kill criterion ("if h(CP(t)) > t+1 for some t ≤ 5 the pattern dies")
**fired at t = 4**. The t+1 pattern is dead; the ⌊t/2⌋ pattern is the
current best hypothesis and is not proven.

## Next

- C3 (exact invariant atlas to n = 7) requires the `geng` backend for
  unlabeled n = 7 enumeration (≈ 3M classes); not yet implemented.
- C2 (weak Hadwiger) sampled campaign exists at `campaigns/sampled-weakhadwiger.toml`.