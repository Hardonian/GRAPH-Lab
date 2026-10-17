# GRAPH Lab

Graph-theoretic discovery and formal verification laboratory: an exact
enumeration/counterexample engine (Rust), a Python control plane, and a
Lean 4 / Mathlib proof pipeline with a kernel-checked witness bridge.

## Status

`docs/STATUS.md` is the authoritative progress ledger; this README documents
features that actually exist and run in the repository today.

**Milestones M0 through M12 are PASS.**  
- **Authoritative Progress Ledger:** [`docs/STATUS.md`](docs/STATUS.md)
- **Comprehensive Research Paper:** [`papers/hadwiger_classification_and_verification.md`](papers/hadwiger_classification_and_verification.md) ([LaTeX source](papers/paper.tex))
- **Research Reports:** [`results/research/`](results/research/)

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

---

## Computational Atlases ($n \le 8$)

The computational engine scales native unlabeled graph enumeration to $n \le 8$ in under 2 seconds:

| Vertices $n$ | Unlabeled Classes | Atlas File | Hadwiger Violations | Weak Hadwiger Violations | Tight Extremal ($h \cdot \alpha = n$) |
|:---:|:---:|:---:|:---:|:---:|:---:|
| $n \le 6$ | 208 | Memory | 0 | 0 | 18 |
| $n = 7$ | 1,044 | [`results/research/atlas-n7.jsonl`](results/research/atlas-n7.jsonl) | 0 | 0 | 26 |
| $n = 8$ | 12,346 | [`results/research/atlas-n8.jsonl`](results/research/atlas-n8.jsonl) | 0 | 0 | 34 |

- **Zero Violations:** Hadwiger's Conjecture ($h \ge \chi$) and the Weak Hadwiger Conjecture ($h \cdot \alpha \ge n$) hold across 100% of the 13,598 surveyed graphs.
- **Tight Extremal Graphs:** Exactly 34 non-isomorphic graphs at $n=8$ satisfy $h(G) \cdot \alpha(G) = 8$.

---

## Machine-Checked Lean 4 Proof Bridge

Every branch-set minor certificate is encoded as a compact base-$(t+1)$ integer word and validated directly by Lean 4's kernel reduction:
- **Zero-Axiom Guarantee:** Strictly 0 axioms, 0 `sorry`, and 0 `native_decide` across all 10 Lean source files.
- **Continuous Audit:** Enforced by `scripts/lean_audit.py` on every verification gate run.

---

## Delivered CLI & Engine Capabilities

- `cargo run -p graphlab-cli -- doctor --json` — environment and toolchain diagnosis
- `cargo run -p graphlab-cli -- enumerate N [--output file]` — exact unlabeled enumeration up to $n = 8$ (pinned OEIS A000088 sequence: `[1, 1, 2, 4, 11, 34, 156, 1044, 12346]`)
- `cargo run -p graphlab-cli -- hadwiger --input <graph6> [--json]` — exact branch-set Hadwiger number $h(G)$ for $n \le 10$
- `cargo run -p graphlab-cli -- canon --input <graph6>` / `--g6 --decode <s>` — Nauty/Traces canonical labeling
- `cargo run -p graphlab-cli -- search --campaign <toml>` — deterministic witness search campaigns
- `cargo run -p graphlab-cli -- minimize --witness <file> [--output <file>]` — size-decreasing witness reduction
- `cargo run -p graphlab-cli -- verify --witness <file>` — solver-independent witness validation
- `cargo run -p graphlab-cli -- benchmark --threads N [--gpu-if-capable]` — high-throughput CPU/GPU benchmarking
- `python/graphlab/` — Python control plane with independent NetworkX and igraph verification oracles
- `lean/` — Lean 4 / Mathlib formalization with kernel-checked certificates
- `scripts/verify.ps1` / `scripts/verify.sh` — 9-gate verification suite (fmt, clippy, test, release, cli-doctor, py-ruff, py-mypy, py-pytest, lean-audit)

---

## Repository Layout

```
crates/graphlab-core/   Rust computational core (bitset graphs, branch sets, Nauty canonicalizer)
crates/graphlab-cli/    graphlab CLI entry point
papers/                 Professional research papers and LaTeX publications
python/graphlab/        Python control plane (NetworkX & igraph independent oracles)
lean/                   Lean 4 project (kernel-checked decision procedures and certificates)
results/research/       Empirical atlases (atlas-n7.jsonl, atlas-n8.jsonl) and research reports
campaigns/              Versioned campaign configurations
docs/                   Architecture, status, proof trust model, research program
scripts/                Verification gates (verify.ps1, verify.sh, lean_audit.py)
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