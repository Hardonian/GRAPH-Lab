# GRAPH Lab

Graph-theoretic discovery and formal verification laboratory: an exact
enumeration/counterexample engine (Rust), a Python control plane, a SAT/SMT
layer, and a Lean 4 / Mathlib proof pipeline with a witness bridge.

## Status

This repository is under active construction milestone by milestone.
`docs/STATUS.md` is the authoritative progress ledger; the README documents
only functions that actually exist and run today.

Currently supported (Milestone 0):

- `cargo run -p graphlab-cli -- doctor` — environment/toolchain report
- Workspace build, unit tests, deterministic CI design

Not yet delivered (planned, do not use):

- enumeration, canonicalization, invariants, search, Lean bridge, GPU paths

## Layout

```
crates/graphlab-core/   Rust computational core (exact bitset graphs)
crates/graphlab-cli/    graphlab CLI
python/graphlab/        Python control plane (Milestone 1+)
lean/                   Lean 4 / Mathlib project (Milestone 3)
campaigns/              versioned campaign configs
results/                evidence ledger (schema documented in results/README.md)
docs/                   architecture, host capabilities, status, trust model
scripts/                bootstrap / verify / reproduce
```

## Quickstart (current)

```
cargo build --workspace
cargo test --workspace
cargo run -p graphlab-cli -- doctor --json
```

## Mathematical trust policy

Search results and compiled Lean files are *inputs* to a claim, never the
claim itself. Every result in this repo carries an explicit status label
(`EXPERIMENTAL`, `EXACT_FINITE`, `SAT_WITNESS`, `LEAN_CHECKED`,
`NOVELTY_REVIEWED`, `OPEN`/`INCONCLUSIVE`) and an audit of the exact theorem
statement. See `docs/PROOF_TRUST_MODEL.md`.

## Hardware notes

Runs correctly on any x86-64 Linux with a recent Rust toolchain — CPU-only by
default. GPU acceleration is capability-gated and never required for
correctness or verification.

## License

MIT (see LICENSE). External components (nauty/Traces, Mathlib, graph6
corpora) keep their own licenses; provenance is recorded in
`docs/HOST_CAPABILITIES.md` and `results/`.
