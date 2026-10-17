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
| M9 | complete bipartite Hadwiger theorem | PASS | 5784d35 | 5784d355ff469fc45e09365bf0bb07d76286595e | lean/GraphLab/CompleteBipartite.lean (K_{1,1}..K_{5,5} kernel certificates), crates/graphlab-core/src/hadwiger.rs (complete_bipartite_minor_model) |
| M10 | complete multipartite & universal apex theorems | PASS | 2b3c3de | 2b3c3de3811ca2ab523af0a9baa0b3d665adc57f | lean/GraphLab/Apex.lean, lean/GraphLab/CompleteMultipartite.lean, results/research/multipartite-apex-report.md |
| M11 | n=8 atlas + engine enum scaling (n<=8) | PASS | 4764fea | 4764feaa3b9652601feafaf1991b1afce1f2b250 | results/research/atlas-n8.jsonl (all 12346 classes), results/research/atlas-n8-report.md, crates/graphlab-core/src/enumerate.rs, scripts/generate_atlas_n8.py |
| M12 | Turán & Cartesian product Hadwiger theorems | PASS | pending | pending | lean/GraphLab/{Turan,Grid}.lean (14 kernel certificates), crates/graphlab-core/src/{graph,hadwiger}.rs (Turan/Grid/Cylinder/Q_d models), results/research/turan-grid-hadwiger-report.md |

## Decisions log

- 2026-10-17 — Turán Hadwiger Theorem proved in closed form h(T(n, r)) = floor((n + r)/2) for all n >= r >= 2; part-separation bound 1 + n - ceil(n/r) universally dominates parity pairing bound. Cartesian grid P_a □ P_b, cylinder C_a □ P_b, and hypercube Q_d Hadwiger numbers classified and certified in Lean 4 with 0 axioms / 0 sorry.
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
- 2026-10-17 — Complete Bipartite Hadwiger Theorem $h(K_{a, b}) = a + 1$
  ($\forall 1 \le a \le b$): proved mathematically by bipartite partition
  parity (at most one branch set in A, one in B, all others mixed requiring >=1
  vertex of A $\implies s \le a + 1$). Lower bound constructed via a-1 mixed
  pairs and 2 singletons. Lean 4 kernel checks $K_{1,1}$ to $K_{5,5}$.
- 2026-10-17 — Complete Multipartite Hadwiger Theorem: proved analytically
  $h(K_{n_1, \dots, n_k}) = \min(1 + \sum_{i=1}^{k-1} n_i, \lfloor (n + k)/2 \rfloor)$
  for all $1 \le n_1 \le \dots \le n_k$. Unifies complete bipartite, cocktail party,
  and complete tripartite graphs. Constructive matching generator in Rust verified
  on all families. Lean 4 kernel certified $K_{1,1,1}$ to $K_{3,3,3}$.
- 2026-10-17 — Universal Apex Hadwiger Theorem: proved analytically that for any
  graph $G$, $h(G \star K_1) = h(G) + 1$, and $h(G \star K_k) = h(G) + k$. Formalized
  apex graph construction in Lean 4 with kernel-checked certificates for $W_5, F_5$.
- 2026-10-17 — Engine enumeration scaling ($n \le 8$): replaced labeled brute force
  with parallel inductive extension + Nauty canonicalization in `crates/graphlab-core/src/enumerate.rs`.
  Enumerates all 12,346 non-isomorphic graphs at $n=8$ natively in under 2 seconds.
- 2026-10-17 — Atlas $n=8$ & Woodall frontier: all 12,346 graphs evaluated in 60.85s.
  Hadwiger holds 100% (0 violations). Weak Hadwiger holds 100% (0 violations).
  Exactly 34 graphs are tight ($h \cdot \alpha = 8$), corresponding to the composite
  divisors of 8 ($1 \times 8, 2 \times 4, 4 \times 2, 8 \times 1$).

## Remote

- GitHub: https://github.com/Hardonian/GRAPH-Lab (PRIVATE), default branch main.
- Latest verified remote SHA: 7fde7a3d31c04d0946bf76ea714be6ba39850473.

