# STATUS — authoritative progress ledger

Resume protocol: read this file, then `git log --oneline -20`, then verify
the remote SHA below. Do not trust remembered state.

| Milestone | Gate | Status | Commit (local) | Remote SHA verified | Evidence |
|---|---|---|---|---|---|
| M0 | recon + safe foundation | PASS | e2a9f8f→f79d8cf (amended) | f79d8cfd9b752ffe2007d71e066ea583821114a8 | docs/HOST_CAPABILITIES.md, results/host-capabilities.json, results/doctor-m0.json, results/milestones/M0.md |
| M1 | exact graph engine | PASS | f4bc65d | f4bc65d0cce40bf3a40e7e5adc5407d3f660bbc0 | results/milestones/M1.md, results/enumeration/crosscheck-m1.json |
| M2 | search + witnesses | PASS | d2d085f | d2d085fe94a742cd66b368af2f4d4512d2543438 | results/milestones/M2.md, results/campaigns/* |
| M3 | Lean/mathlib bridge | PASS | (this commit) | (verify below) | results/milestones/M3.md, lean/GraphLab/Facts.lean |
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

- GitHub: https://github.com/Hardonian/GRAPH-Lab (PRIVATE), default branch main.
- Latest verified remote SHA: f79d8cfd9b752ffe2007d71e066ea583821114a8 (M0).
