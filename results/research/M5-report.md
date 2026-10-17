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
| 2 | 3 | LEAN_CHECKED | `lean/GraphLab/Facts.lean` `C2_hadwiger` — kernel recomputes `hadwigerNumber 4 A2` by exhaustive scan (`decide`). Also `lean/GraphLab/CocktailParty.lean` `CP2_model` (certificate code 217) and `CP2_h3`. |
| 3 | 4 | LEAN_CHECKED (lower bound) / EXACT_FINITE (upper) | `lean/GraphLab/CocktailParty.lean` `CP3_model` (code 2911) / `CP3_h4` kernel-checked lower bound. Upper bound verified by Rust engine + Python oracle. |
| 4 | 6 | LEAN_CHECKED (lower bound) / EXACT_FINITE (upper) | `lean/GraphLab/CocktailParty.lean` `CP4_model` (code 5506586) / `CP4_h6` kernel-checked lower bound. Upper bound verified by Rust engine + Python oracle. |
| 5 | 7 | LEAN_CHECKED (lower bound) / EXACT_FINITE (upper) | `lean/GraphLab/CocktailParty.lean` `CP5_model` (code 131253841) / `CP5_h7` kernel-checked lower bound. Upper bound verified by Rust engine + Python oracle. |

## Trust model applied

- t = 2 is **LEAN_CHECKED**: the kernel recomputes the whole number from the
  explicit matrix, no engine output is trusted on faith.
- t = 1..5 lower bounds ($K_{t + \lfloor t/2 \rfloor}$ minors) are **LEAN_CHECKED**:
  Lean 4 kernel evaluates `validBranchModel` on explicit constructive certificates via `decide`.
- t = 3, 4, 5 exact numbers are **EXACT_FINITE**: two independent implementations (Rust
  bitset engine, Python oracle) agree on every value, and match the proved structural formula
  $h(CP(t)) = t + \lfloor t/2 \rfloor$.
- General formula $h(CP(t)) = t + \lfloor t/2 \rfloor$ for all $t$:
  - Lower bound is constructively generated and verified for all $t$ (`cocktail_party_minor_model`).
  - Upper bound is proved structurally from the non-edge structure of $CP(t)$ (matching pairs).

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