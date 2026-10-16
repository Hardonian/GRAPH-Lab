# HOST_CAPABILITIES — measured 2026-10-16 (sanitized; no UUIDs/serials)

Host role: AMD EPYC research/GPU server (also runs production Ollama/ComfyUI
lanes and a storefront). GRAPH Lab must coexist without disturbing them.

## CPU / memory

- AMD EPYC 7452 (32 cores / 64 threads), 1 socket, 1 NUMA node (node0 = all
  64 CPUs, size ~168 GiB). No cross-NUMA tuning needed; SMT present.
- RAM: 168 GiB total, ~156 GiB available at recon. Swap 31 GiB (essentially
  unused).
- Schedulable budget for GRAPH Lab CPU jobs: 48 threads (75%) default,
  capped by campaign config; host load at recon was light.

## Storage

- `/` (LVM): 466G total, 93G free (80% used) — do not place build outputs here.
- `/mnt/ai-storage` (NVMe): 938G total, 633G free — repo + caches live here:
  `/mnt/ai-storage/ai-workspace/repos/graph-lab`.

## GPUs (per-device, with occupancy policy)

| idx | device | VRAM | compute cap | driver | live services |
|---|---|---|---|---|---|
| 0 | Tesla V100-SXM2 | 16 GB | 7.0 (sm_70) | 580.178.04 | ollama-v100 + ollama-default lanes |
| 1 | Tesla P40 | 24 GB | 6.1 (sm_61) | 580.178.04 | ollama-p40 + ComfyUI (P40 lane) |
| 2 | GeForce RTX 3060 | 12 GB | 8.6 (sm_86) | 580.178.04 | ollama-3060 lane + 3 desktop-ish processes |

- CUDA toolkit: nvcc 12.6 at /usr/local/cuda-12.6.
- Notes: P40 (Pascal) is supported by the 580 driver but modern PyTorch/JAX
  wheels dropped sm_61 — GPU experiments needing current wheels must target
  the V100 (sm_70) or RTX 3060 (sm_86). CUDA ordinal ≠ nvidia-smi index on
  this host; device selection must bind by UUID-at-runtime (never committed),
  verified by `nvidia-smi` occupancy immediately before any campaign.
- Policy: no GRAPH Lab GPU campaign while occupancy > ~10% on the chosen
  card, except explicitly-allowed shared use of the V100 for short bounded
  benchmarks. GPU never certifies proofs (Lean is CPU-bound).

## Toolchains at recon

| tool | version | note |
|---|---|---|
| rustc / cargo | 1.99.1 | pinned via rust-toolchain.toml |
| python3 | 3.14.7 | venvs via uv only |
| uv | 0.11.21 | working |
| git | 2.53.0 | |
| gh | 2.102.0 | authenticated as Hardonian |
| gcc / g++ | 15.2.0 | needed for nauty C sources via sys crate |
| cmake | present | |
| make | present | |
| elan / lean / lake | NOT INSTALLED | M0 task: user-space install via elan |
| z3 / cvc5 | NOT INSTALLED | M0: pin z3 via Python venv (uv) |
| docker | 29.1.3 | available but not required |

## OS / kernel

- Ubuntu 26.04.1 LTS, kernel 7.0.0-38-generic (do not touch kernel/drivers).

## Prior repo state

- No existing `Hardonian/GRAPH-Lab` on GitHub (GraphQL: repository does not
  resolve) and no local graph-lab dir — greenfield, private repo to be
  created once M0 build validates.
- No UNIFORM Lab found on this host. Commission clause "inspect read-only,
  continue cleanly if unavailable" — documented as unavailable. GRAPH Lab is
  independent; no cross-repo runtime dependency created.

## Benchmark (bounded, pre-parallelism-selection)

Recorded in results/host-capabilities.json `bench` block: single-thread and
48-thread scratch throughput for a bitset-popcount microkernel used by the
enumeration inner loop.
