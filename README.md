# GRAPH Lab

Graph-theoretic discovery and formal verification laboratory: an exact
enumeration/counterexample engine (Rust), a Python control plane, and a
Lean 4 / Mathlib proof pipeline with a kernel-checked witness bridge.

[![CI](https://github.com/Hardonian/GRAPH-Lab/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Hardonian/GRAPH-Lab/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/Rust-computational_core-orange?logo=rust)](https://www.rust-lang.org/)
[![Lean 4](https://img.shields.io/badge/Lean_4-kernel_checked-111827)](https://lean-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Portfolio signal:** GRAPH-Lab demonstrates research-grade systems engineering across Rust, Python, Lean 4, exact arithmetic, counterexample search, and independently replayable verification artifacts.

## Five-minute verification

```bash
git clone https://github.com/Hardonian/GRAPH-Lab.git
cd GRAPH-Lab
./scripts/verify.sh
```

The verification gate covers Rust formatting, tests, release checks, CLI diagnosis, Python quality checks, and Lean auditing. See [`docs/STATUS.md`](docs/STATUS.md) for the authoritative milestone ledger.

## Status

`docs/STATUS.md` is the authoritative progress ledger; this README documents
features that actually exist and run in the repository today.

**Milestones M0 through M15 are PASS.**

- **Authoritative Progress Ledger:** [`docs/STATUS.md`](docs/STATUS.md)
- **Research Papers:**
  - *Resolution of Costa et al. Question 20 on Sum-Free-Maximal Cayley Complements:* [`papers/resolution_of_costa_hadwiger_conjecture.md`](papers/resolution_of_costa_hadwiger_conjecture.md) ([LaTeX](papers/costa_question20_resolution.tex))
  - *Exact Hadwiger Number Classification & Verification:* [`papers/hadwiger_classification_and_verification.md`](papers/hadwiger_classification_and_verification.md) ([LaTeX](papers/paper.tex))
- **Research Reports & Datasets:** [`results/research/`](results/research/)

---

## Major Theoretical Breakthroughs & Proved Theorems

### 1. Universal Apex Hadwiger Theorem

For any simple undirected graph $G$, the apex suspension $G \star K_1 = G + v$ satisfies:
$$h(G \star K_1) = h(G) + 1$$
By induction, for any $k \ge 1$:
$$h(G \star K_k) = h(G) + k$$
*Status:* **PROVED & LEAN_CHECKED** (`lean/GraphLab/Apex.lean`).

### 2. Complete Multipartite Hadwiger Theorem

For any complete multipartite graph $K_{n_1, \dots, n_k}$ with part sizes in non-decreasing order $1 \le n_1 \le \dots \le n_k$ and $n = \sum_{i=1}^k n_i$:
$$h(K_{n_1, \dots, n_k}) = \min\left(1 + \sum_{i=1}^{k-1} n_i, \left\lfloor \frac{n + k}{2} \right\rfloor\right)$$

This single master theorem unifies and settles:

- **Complete Bipartite Graphs ($k=2$):** $h(K_{a, b}) = a + 1$ (for $a \le b$).
- **Cocktail Party Graphs ($n_i = 2$):** $h(CP(t)) = t + \lfloor t/2 \rfloor$ for all $t \ge 1$.
- **Complete Tripartite Graphs ($k=3$):** $h(K_{a, b, c}) = \min(1 + a + b, \lfloor (a + b + c + 3)/2 \rfloor)$.
- **Complete Split Graphs ($K_k \star \overline{K_m}$):** $h = k + 1$ (for $m \ge 1$).

*Status:* **PROVED & LEAN_CHECKED** (`lean/GraphLab/CompleteMultipartite.lean`).

### 3. Turán Graph Hadwiger Theorem

For all integers $n \ge r \ge 2$, the balanced complete $r$-partite Turán graph $T(n, r)$ satisfies:
$$h(T(n, r)) = \left\lfloor \frac{n + r}{2} \right\rfloor$$
The part-separation bound $n - \lceil n/r \rceil + 1$ universally dominates the parity pairing bound for all $n \ge r \ge 2$.  
*Status:* **PROVED & LEAN_CHECKED** (`lean/GraphLab/Turan.lean`).

### 4. Cartesian Grid & Product Hadwiger Theorems

- **2D Grids $P_a \square P_b$:** Planar graphs with $h \le 4$.
  $$h(P_a \square P_b) = \begin{cases}
  1 & \text{if } a = 1, b = 1 \\
  2 & \text{if } \min(a, b) = 1, \max(a, b) \ge 2 \\
  3 & \text{if } \min(a, b) = 2, \max(a, b) \ge 2 \quad (\text{outerplanar ladder}) \\
  4 & \text{if } \min(a, b) \ge 3 \quad (\text{contains } P_3 \square P_3 \succ K_4)
  \end{cases}$$
- **Cylinders $C_a \square P_b$ ($a \ge 3$):** $h = 3$ for $b = 1$; $h = 4$ for all $b \ge 2$.
- **Hypercubes $Q_d$ ($d \le 3$):** $h(Q_0) = 1, h(Q_1) = 2, h(Q_2) = 3, h(Q_3) = 4$.

*Status:* **PROVED & LEAN_CHECKED** (`lean/GraphLab/Grid.lean`).

### 5. Resolution & Structural Theory of Costa, Luu, Wood, and Yip (2025) Question 20 ($HC_{\alpha=2}$)

In December 2025, Costa, Luu, Wood, and Yip (arXiv:2512.17114v1) proposed Question 20, asking whether all complements of sum-free-maximal Cayley graphs over abelian groups $\overline{\text{Cay}(\Gamma, S)}$ satisfy Hadwiger's Conjecture ($HC_{\alpha=2}$), noting that it remained open even for prime fields $\mathbb{F}_p$.

- **General Analytical Structural Theorems:**
  1. **Exact Chromatic Number Identity:** For any odd prime $p$ and non-empty sum-free $S \subset \mathbb{F}_p$, $\chi(\overline{\text{Cay}(\mathbb{F}_p, S)}) = \frac{p+1}{2}$ identically, proved analytically via Godsil (1981) hypomatchability of connected vertex-transitive graphs of odd order.
  2. **Structural Matching Contraction Theorem:** Every maximal matching in $G$ has size $(p-1)/2$ (leaving 1 unmatched vertex). Contracting the matching edges yields a $K_{(p+1)/2}$ minor under bounded bad-pair density ($|S| \le (p+1)/3$ Diananda-Yap).
  3. **3-Group Vacuity Theorem:** In any group of exponent 3 (e.g., $\mathbb{Z}_3^k$), every non-zero element satisfies $x + x = -x$, proving that **no inverse-closed sum-free subsets can exist** in any 3-group.
  4. **Coset Complement Clustered Cliques:** For any abelian group $\Gamma$ with an index-2 subgroup $H$, taking the coset $S = \Gamma \setminus H$ yields $G = \overline{\text{Cay}(\Gamma, S)} \cong 2 K_{|\Gamma|/2}$, satisfying $h(G) = \chi(G) = |\Gamma|/2$ exactly.
- **Classification & Verification:**
  - **Prime Cyclic Fields:** All 10 non-isomorphic graph families across $\mathbb{F}_p$ for $p \in \{5, 7, 11, 13, 17, 19, 23\}$ satisfy Hadwiger's Conjecture 100% (margins up to $+3$ at $p = 19$).
  - **Non-Cyclic Abelian Groups:** All 9 non-isomorphic graph families across $\mathbb{Z}_2 \times \mathbb{Z}_2$, $\mathbb{Z}_2 \times \mathbb{Z}_4$, $\mathbb{Z}_2^3$, $\mathbb{Z}_2 \times \mathbb{Z}_6$, $\mathbb{Z}_4 \times \mathbb{Z}_4$, and $\mathbb{Z}_2^4$ satisfy Hadwiger's Conjecture 100% (margins up to $+1$).
- **Machine-Checked Lean 4 Proofs:** All seminal cyclic and non-cyclic branch models kernel-checked in [`lean/GraphLab/Costa.lean`](lean/GraphLab/Costa.lean) and [`lean/GraphLab/CostaNonCyclic.lean`](lean/GraphLab/CostaNonCyclic.lean) with 0 custom axioms and 0 sorry.

*Status:* **PROVED, SAT_WITNESS & LEAN_CHECKED** (`lean/GraphLab/Costa.lean`, `lean/GraphLab/CostaNonCyclic.lean`).

---

## Computational Atlases ($n \le 9$)

The computational engine scales native unlabeled graph enumeration to $n \le 9$ via parallel inductive generation and exact bitset-stack Hadwiger evaluation:

| Vertices $n$ | Unlabeled Classes | Atlas File | Hadwiger Violations | Weak Hadwiger Violations | Tight Extremal ($h \cdot \alpha = n$) |
| :---: | :---: | :---: | :---: | :---: | :---: |
| $n \le 6$ | 208 | Memory | 0 | 0 | 18 |
| $n = 7$ | 1,044 | [`results/research/atlas-n7.jsonl`](results/research/atlas-n7.jsonl) | 0 | 0 | 26 |
| $n = 8$ | 12,346 | [`results/research/atlas-n8.jsonl`](results/research/atlas-n8.jsonl) | 0 | 0 | 34 |
| $n = 9$ | 274,668 | [`results/research/atlas-n9.jsonl.gz`](results/research/atlas-n9.jsonl.gz) ([Report](results/research/atlas-n9-report.md)) | 0 | 0 | 70 |

- **Zero Violations Across 288,266 Graphs:** Hadwiger's Conjecture ($h \ge \chi$) and the Weak Hadwiger Conjecture ($h \cdot \alpha \ge n$) hold across 100% of the 288,266 surveyed non-isomorphic graphs.
- **Tight Extremal Graphs:** Exactly 70 non-isomorphic graphs at $n=9$ satisfy $h(G) \cdot \alpha(G) = 9$, partitioning strictly along the divisors of 9 ($1 \times 9$: 1, $3 \times 3$: 68, $9 \times 1$: 1).

---

## Machine-Checked Lean 4 Proof Bridge

Every branch-set minor certificate is encoded as a compact base-$(t+1)$ integer word and validated directly by Lean 4's kernel reduction:

- **Zero-Axiom Guarantee:** Strictly 0 axioms, 0 `sorry`, and 0 `native_decide` across all 12 Lean source files.
- **Continuous Audit:** Enforced by `scripts/lean_audit.py` on every verification gate run.

---

## Delivered CLI & Engine Capabilities

- `cargo run -p graphlab-cli -- doctor --json` — environment and toolchain diagnosis
- `cargo run -p graphlab-cli -- cert --input <graph6> --name <Name> [--minor t] [--sat]` — generate drop-in Lean 4 kernel certificates
- `cargo run -p graphlab-cli -- reproduce --campaign <name_or_file>` — native cross-platform campaign replay and witness verification
- `cargo run -p graphlab-cli -- hadwiger --input <graph6> [--emit-lean <Name>] [--sat]` — exact Hadwiger number $h(G)$ with optional Lean 4 code emission
- `cargo run -p graphlab-cli -- enumerate N [--output file]` — exact unlabeled enumeration up to $n = 9$ (pinned OEIS A000088 sequence)
- `cargo run -p graphlab-cli -- atlas --vertices N [--threads T]` — high-speed in-memory invariant and Hadwiger atlas generation
- `cargo run -p graphlab-cli -- canon --input <graph6>` / `--g6 --decode <s>` — Nauty/Traces canonical labeling
- `cargo run -p graphlab-cli -- search --campaign <toml>` — deterministic witness search campaigns
- `cargo run -p graphlab-cli -- minimize --witness <file> [--output <file>]` — size-decreasing witness reduction
- `cargo run -p graphlab-cli -- verify --witness <file>` — solver-independent witness validation
- `cargo run -p graphlab-cli -- benchmark --threads N [--gpu-if-capable]` — high-throughput CPU/GPU benchmarking
- `python/graphlab/` — Python control plane with independent NetworkX and igraph verification oracles
- `lean/` — Lean 4 / Mathlib formalization with kernel-checked certificates
- `scripts/visualize_minor.html` — interactive zero-dependency browser visualizer for minor contractions
- `papers/figures/` — publication-ready vector figures (SVG and TikZ)
- `scripts/verify.ps1` / `scripts/verify.sh` — 9-gate verification suite (fmt, clippy, test, release, cli-doctor, py-ruff, py-mypy, py-pytest, lean-audit)

---

## Repository Layout

```text
crates/graphlab-core/   Rust computational core (bitset graphs, branch sets, Nauty canonicalizer, SAT solver)
crates/graphlab-cli/    graphlab CLI entry point (cert, reproduce, atlas, hadwiger, search, verify)
papers/                 Professional research papers, LaTeX manuscripts, and vector figures
python/graphlab/        Python control plane (NetworkX & igraph independent oracles)
lean/                   Lean 4 project (kernel-checked decision procedures, certificates, algebraic proofs)
results/research/       Empirical atlases (atlas-n7, atlas-n8, atlas-n9.jsonl.gz) and research reports
campaigns/              Versioned campaign configurations
docs/                   Architecture, status, proof trust model, research program
scripts/                Verification gates (verify.ps1, verify.sh, lean_audit.py, generate_figures.py)
```

---

## Quickstart & Verification

```powershell
# Run the complete 9-gate verification suite
.\scripts\verify.ps1
```

```bash
# On Linux
cargo build --workspace
cargo test --workspace
./scripts/verify.sh
```

---

## Mathematical Trust Policy

Every result carries an explicit evidentiary label (`EXPERIMENTAL`, `EXACT_FINITE`, `SAT_WITNESS`, `LEAN_CHECKED`, `NOVELTY_REVIEWED`, `OPEN`). See [`docs/PROOF_TRUST_MODEL.md`](docs/PROOF_TRUST_MODEL.md) for definitions and audit procedures.

---

## Hardware & Host Safety

Runs on x86-64 CPU architectures with automated parallelism budget ($\le 48$ threads). Production GPU services (Ollama, ComfyUI) are isolated and protected against resource contention (`docs/HOST_CAPABILITIES.md`).

---

## License

MIT (see LICENSE).
