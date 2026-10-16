# ARCHITECTURE

Four layers with explicit boundaries. Each is independently testable; each
higher layer treats the lower one as an oracle only after that layer's own
tests pass.

```
graphlab-cli (Rust)            single binary, subcommands
  └─ graphlab-core (Rust)      exact combinatorics: bitset graphs, graph6,
                               canonicalization (nauty/Traces via nauty-pet
                               with capped exact fallback), invariants,
                               deterministic parallel search, checkpoint/
                               resume, witness schemas
Python control plane           typed orchestrator: campaigns (TOML, pinned
  └─ stable JSONL process        seeds), exact int/rational cross-checks,
     boundary (no PyO3 until      NetworkX/igraph independent oracles on
     justified)                   small instances, reporting
SAT/SMT layer                  pinned Z3 (cvc5 optional): finite exact
                               encodings; positive witnesses rechecked
                               solver-independently; UNSAT claims carry
                               scope + proof artifacts where supported
Lean 4 / Mathlib               pinned toolchain + cached mathlib; re-evaluates
                               imported witnesses inside the kernel via a
                               decision procedure on concrete finite graphs;
                               strict source audit (sorry/admit/axiom/unsafe/
                               native_decide) is part of the pipeline, not a
                               suggestion
```

## Data flow for a counterexample claim

formal statement → executable predicate (Rust) → generator+search →
candidate witness (JSONL, hashed) → independent recheck (Python, second
implementation) → minimizer (size-decreasing, violation-preserving, every
step logged) → serialized witness → Lean file negating the original claim
evaluated on the witness → literature check → write-up with status label.

## Determinism rules

- All search paths take explicit seeds (ChaCha8RNG from a u64 seed).
- Iteration order over graphs is canonical (integer bit enumeration order,
  or nauty output order for unlabeled enumeration — never hash-map order).
- Output serialization is sorted; JSON keys are fixed-order structs.
- Every artifact directory contains an evidence ledger entry: command line,
  input SHA256s, seed, tool versions, CPU flags, start/end timestamps, exit
  codes.

## Failure behavior

Invalid graphs are rejected with typed errors (no silent coercion). Missing
optional backends (nauty, z3, lean) degrade to documented capped fallbacks
or clear "backend unavailable" exits — never to fabricated results.
