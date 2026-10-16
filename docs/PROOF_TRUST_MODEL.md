# PROOF TRUST MODEL

What each label means, and exactly what you are trusting when you see it.

## Labels

- `EXPERIMENTAL` — numeric/computational clue. No theorem. Floating point
  may be involved; tolerances documented at the call site.
- `EXACT_FINITE` — a claim restricted to an explicit finite domain (e.g.
  "all simple graphs on ≤6 vertices") verified by exhaustive enumeration
  whose coverage is independently checked against known counts and a second
  implementation.
- `SAT_WITNESS` — a solver assignment was DECODED and re-evaluated by an
  independent exact checker (Rust and/or Python). The SAT solver is not
  trusted for the claim; it is trusted only to propose candidates.
- `LEAN_CHECKED` — the exact statement (audited against the English
  statement in the same file) compiles under a pinned toolchain from a fresh
  checkout, `lake build` exits 0, and a source scan proves the proof term
  contains no `sorry`, `admit`, unjustified `axiom`, `unsafe`, or
  `native_decide` escape hatch. You trust: Lean's kernel + Mathlib + the
  statement audit performed by a named reviewer in the evidence ledger.
- `NOVELTY_REVIEWED` — literature/conjecture-tracker cross-check completed
  and dated. Subject to independent expert review; not a priority claim.
- `OPEN` / `INCONCLUSIVE` — nothing established. A bounded search that found
  no counterexample rules out exactly what its scope says it rules out.

## The gap between English and formal

A proof checker validates the formal statement supplied, not the intended
conjecture. For every `LEAN_CHECKED` artifact this repo requires a
side-by-side semantic audit: English definition, Mathlib definition,
quantifier/domain comparison, and an explicit note on any definitional
divergence (e.g. whether "graph" allows n=0, whether coloring counts
colorability vs a specific coloring). The audit lives next to the .lean file.

## Witnesses crossing into Lean

The witness bridge does not "trust" the Rust engine. It re-encodes the
concrete graph inside Lean (as an explicit adjacency matrix / Fin-valued
SimpleGraph) and re-evaluates the violated property by kernel computation on
that concrete instance. If Rust were systematically wrong, the Lean
re-evaluation would disagree — disagreement is a hard pipeline failure, not
a warning.

## What is never claimed here

- SMT `sat` alone, timeouts, or `unknown` are not results.
- AI model confidence is not evidence.
- A compiled Lean file is not a proof of the open problem it resembles.
