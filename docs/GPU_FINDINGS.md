# GPU findings — capability- and workload-gated (Milestone 4)

## Measured host state (2026-10-16)

- 3 devices (driver 580.178.04, nvcc 12.6): V100 16GB sm_70, P40 24GB
  sm_61, RTX 3060 12GB sm_86. `graphlab benchmark --gpu-if-capable`
  probes utilization before anything GPU is ever attempted; >10% on any
  card aborts GPU use outright.
- All three cards host production services (Ollama lanes on each, ComfyUI
  on P40). The commission clause "never run on a busy card" therefore
  applies continuously on this host except in idle windows.

## Measured CPU baseline (results/bench/scaling-summary.json)

| kernel | 1 thread | 48 threads | note |
|---|---|---|---|
| labeled n=6 sweep (m+triangles+degs on 32768 graphs) | 19 ms | 10 ms | wall dominated by sequential candidate generation; ~0.3 µs/graph |
| exact Hadwiger, 209 unlabeled classes n≤6 | 6 ms | 1 ms | 6× at 8 threads, saturates |
| enumerate+dedup n≤6 + invariants n≤5 | 1983 ms | 2010 ms | sequential enumeration bound (rayon helps only the invariant stage) |

## Why no CUDA kernel is wired (empirical, workload-specific)

1. **Transfer/launch bound.** Engine graphs are n ≤ 10 (bitset rows of a
   single u64). CPU cost per graph is ~0.3 µs measured. A CUDA kernel
   launch alone is ~5–10 µs and host↔device transfer adds ~10 µs+ for a
   batch of any size worth shipping (thousands of 10×10 matrices ≈ tens of
   KB — bandwidth is fine, per-item dispatch is not). At these sizes the
   GPU loses by 1–2 orders of magnitude **before computing anything**; the
   commission's own note ("a fast CPU bitset kernel may beat GPU for small
   graphs; if so, make CPU default") is exactly this case, measured.
2. **The search structure is combinatorial, not dense-linalg.** Hadwiger
   branch-set models, chromatic DFS, canonical dedup are sequential
   control-flow kernels with early exits; converting them to GPU is the
   "blindly converting combinatorial search" case §3 says to avoid.
3. **No plausible GPU-advantage workload exists in the current scope
   yet.** The tasks §3 lists as justified GPU candidates (batched spectral
   heuristics, batched candidate scoring of large feature matrices) only
   become real when the research engine grows a heuristic pre-filter over
   huge candidate pools (n ≥ 20 campaigns, neural candidate ranking).
   Today's exact scope (n ≤ 10) does not produce such pools.

## GPU decision (honest, per commission §3)

**GPU not used; CPU is the default and the correctness oracle.** No GPU
speedup, throughput, or device-placement claim is made anywhere in this
repo. When a heuristic-scoring workload is added, the entry point is the
`--gpu-if-capable` gate (probe occupancy first, bind by runtime-verified
UUID, deliver CPU-equivalence checks on deterministic cases, and report
the actual device).

## What M4 delivers instead (real acceleration)

- rayon parallelism bounded at 48/64 threads (75%) in both campaigns and
  benchmarks, with per-bench scaling data showing where parallelism pays
  (hadwiger 6×) and where it does not (sequential enumeration stage) —
  the honest signal for where future optimization should go.
- resource caps: campaigns carry `threads`, runner clamps to ≤48; memory
  and disk stay far below host capacity (209-class campaigns are KB-sized).
- checkpoint-friendly structure: exhaustive modes materialize the candidate
  list and evaluate per-graph pure predicates; resume can reuse the same
  deterministic ordering. (Full checkpoint/resume file state is listed as
  a known gap rather than half-implemented.)
