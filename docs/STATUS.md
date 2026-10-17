# STATUS — authoritative progress ledger

Resume protocol: read this file, then `git log --oneline -20`, then verify
the remote SHA below. Do not trust remembered state.

| Milestone | Gate | Status | Commit (local) | Remote SHA verified | Evidence |
|---|---|---|---|---|---|
| M0 | recon + safe foundation | PASS | e2a9f8f→f79d8cf (amended) | f79d8cfd9b752ffe2007d71e066ea583821114a8 | docs/HOST_CAPABILITIES.md, results/host-capabilities.json, results/doctor-m0.json, results/milestones/M0.md |
| M1 | exact graph engine | PASS | f4bc65d | f4bc65d0cce40bf3a40e7e5adc5407d3f660bbc0 | results/milestones/M1.md, results/enumeration/crosscheck-m1.json |
| M2 | search + witnesses | PASS | d2d085f | d2d085fe94a742cd66b368af2f4d4512d2543438 | results/milestones/M2.md, results/campaigns/* |
| M3 | Lean/mathlib bridge | PASS | 6f4f778 | 6f4f778b873937ea9c4341a314ab95a26b3b3353 | results/milestones/M3.md, lean/GraphLab/{Decide,MathlibBridge,Facts}.lean |
| M4 | HPC/CPU/GPU | PASS | 2a7937d | 2a7937d67c1f1f7426a7f132296b665f0205a27b | results/milestones/M4.md, results/bench/*, docs/GPU_FINDINGS.md |
| M5 | research campaign | PASS | c0bcfee | c0bcfee8a60796d213c4f7c53cdb87c356f6a3e8 | results/research/M5-report.md, docs/RESEARCH_PROGRAM.md, lean/GraphLab/Facts.lean (CP(2..3) certificates) |
| M6 | release v0.1.0 | PASS | e885060 | e88506093cae18ef17c2f0d6118d0d5718df28ea | README.md, CHANGELOG.md, distribution artifacts |
| M7 | theorem expansion + optimization | PASS | 921e4b1 | 921e4b18d385e5f4e99dc223a88330d6bf9987eb | lean/GraphLab/CocktailParty.lean (CP(1..5) kernel certificates), crates/graphlab-core/src/hadwiger.rs (bitset optimization), scripts/verify.ps1 |
| M8 | n=7 atlas + parameterized CP proof | PASS | 785975b | 785975b851d3b2b695d0b8b59dcedccd2915cdf7 | results/research/atlas-n7.jsonl (all 1044 classes), results/research/atlas-n7-report.md, lean/GraphLab/CocktailParty.lean (cpBranchCode + CP6) |

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
- 2026-10-17 — Windows toolchain: MartinStorsjo LLVM-MinGW UCRT with
  `scripts/nauty_windows_shim.h` (`AVOID_SYS_WAIT_H`, `putc_unlocked`) and
  target `x86_64-pc-windows-gnullvm` builds `nauty-Traces-sys` cleanly.
- 2026-10-17 — Hadwiger optimization: stack bitsets (`[u64; 16]`) replace
  heap allocations; bitwise neighbor masks eliminate nested vertex iterations.
- 2026-10-17 — Cocktail party theorem $h(CP(t)) = t + \lfloor t/2 \rfloor$:
  constructive minor generator verified for all $t$; $CP(1)$ to $CP(5)$ branch
  models kernel-checked in Lean 4 with 0 axioms / 0 sorry / 0 native_decide.
- 2026-10-17 — Atlas n=7 & Woodall frontier: all 1,044 non-isomorphic graphs
  on 7 vertices evaluated. Hadwiger holds 100% (0 violations). Woodall's Weak
  Hadwiger ($h \cdot \alpha \ge 7$) holds 100% (0 violations); exactly 2 graphs
  are tight ($h \cdot \alpha = 7$: $E_7$ and $K_7$).
- 2026-10-17 — Parameterized CP branch generator: `cpBranch` and `cpBranchCode`
  formalized in Lean 4; $CP(6)$ on 12 vertices ($K_9$ minor) kernel-checked.

## Remote

- GitHub: https://github.com/Hardonian/GRAPH-Lab (PRIVATE), default branch main.
- Latest verified remote SHA: 785975b851d3b2b695d0b8b59dcedccd2915cdf7.

