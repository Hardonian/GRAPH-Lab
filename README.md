# GRAPH Lab

Graph-theoretic discovery and formal verification laboratory: an exact
enumeration/counterexample engine (Rust), a Python control plane, and a
Lean 4 / Mathlib proof pipeline with a kernel-checked witness bridge.

## Status

`docs/STATUS.md` is the authoritative progress ledger; this README documents
only functions that actually exist and run today.

**Milestone 5 (research campaign) is PASS.** See `docs/STATUS.md` and
`results/research/M5-report.md`.

### Delivered and running

- `cargo run -p graphlab-cli -- doctor --json` — environment/toolchain report
- `cargo run -p graphlab-cli -- enumerate N [--output file]` — exact
  unlabeled enumeration to n=6 (counts 1,1,2,4,11,34,156, cross-verified
  against networkx and igraph)
- `cargo run -p graphlab-cli -- hadwiger --input <graph6> --json` — exact
  Hadwiger number h(G) for n ≤ 10
- `cargo run -p graphlab-cli -- search --campaign <toml>` — versioned
  witness campaigns with independent per-witness recheck
- `cargo run -p graphlab-cli -- minimize --witness <file> [--output <file>]`
- `cargo run -p graphlab-cli -- verify --witness <file>`
- `cargo run -p graphlab-cli -- benchmark --threads N [--gpu-if-capable]
  [--output <file>]`
- `cargo run -p graphlab-cli -- canon --input <graph6>` / `--g6 --decode <s>`
- `python/graphlab/` — Python control plane (networkx + igraph oracles)
- `lean/` — Lean 4 / Mathlib bridge; `lake build` kernel-checks every
  certificate in `lean/GraphLab/Facts.lean` (K4, P4, C5, CP(2), CP(3))
- `scripts/verify.sh` — single-command gate: fmt, clippy, test, release,
  cli-doctor, py-ruff/mypy/pytest, lean-audit

### Research result (M5)

Measured `h(CP(t)) = t + floor(t/2)` for the cocktail party graph CP(t) =
K_{2t} minus a perfect matching, at t = 2,3,4,5 (values 3,4,6,7).
t = 2 is **LEAN_CHECKED** (the kernel recomputes the whole number from the
explicit matrix); t = 3,4,5 are **EXACT_FINITE** (engine + independent
Python oracle agree). The earlier guess `h = t+1` is false (died at t=4).
The general formula is **OPEN**. Full detail: `docs/RESEARCH_PROGRAM.md`,
`results/research/M5-report.md`.

## Layout

```
crates/graphlab-core/   Rust computational core (exact bitset graphs)
crates/graphlab-cli/    graphlab CLI
python/graphlab/        Python control plane
lean/                   Lean 4 / Mathlib project
campaigns/              versioned campaign configs
results/                evidence ledger (schema in results/README.md)
docs/                   architecture, status, trust model, research
scripts/                bootstrap / verify / reproduce
```

## Quickstart

```
cargo build --workspace
cargo test --workspace
./scripts/verify.sh
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
correctness or verification. Measured GPU cost at n ≤ 10 is ~15–20 µs per
graph (kernel launch + transfer) vs ~0.3 µs CPU — GPU is not beneficial in
this scope (`docs/GPU_FINDINGS.md`).

## License

MIT (see LICENSE). External components (nauty/Traces, Mathlib, graph6
corpora) keep their own licenses; provenance is recorded in
`docs/HOST_CAPABILITIES.md` and `results/`.