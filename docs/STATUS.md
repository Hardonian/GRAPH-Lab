# STATUS — authoritative progress ledger

Resume protocol: read this file, then `git log --oneline -20`, then verify
the remote SHA below. Do not trust remembered state.

| Milestone | Gate | Status | Commit (local) | Remote SHA verified | Evidence |
|---|---|---|---|---|---|
| M0 | recon + safe foundation | IN PROGRESS | — | — | docs/HOST_CAPABILITIES.md, results/host-capabilities.json |
| M1 | exact graph engine | NOT STARTED | — | — | — |
| M2 | search + witnesses | NOT STARTED | — | — | — |
| M3 | Lean/mathlib bridge | NOT STARTED | — | — | — |
| M4 | HPC/CPU/GPU | NOT STARTED | — | — | — |
| M5 | research campaign | NOT STARTED | — | — | — |
| M6 | release v0.1.0 | NOT STARTED | — | — | — |

## Decisions log

- 2026-10-16 — Repo lives at `/mnt/ai-storage/ai-workspace/repos/graph-lab`
  (NVMe, 633G free). Root disk is 80% full; avoided.
- 2026-10-16 — No prior GRAPH-Lab repo found on GitHub (`Hardonian` account,
  authenticated). No UNIFORM Lab found on host; commission's read-only audit
  condition documented as not-applicable, GRAPH Lab is independent.
- 2026-10-16 — Canonicalization: prefer a vetted Rust binding to nauty/Traces
  (`nauty-pet`) with an exact brute-force fallback capped at small n;
  fallback is NOT an isomorphism oracle beyond the cap and says so.
- 2026-10-16 — GPU posture: V100/P40/RTX3060 all carry production Ollama or
  ComfyUI lanes. All GPU work is capability- and occupancy-gated; CPU is the
  default and the correctness oracle.

## Remote

- GitHub: private repo `Hardonian/GRAPH-Lab` (to be created at M0 close).
- Latest verified remote SHA: (pending)
