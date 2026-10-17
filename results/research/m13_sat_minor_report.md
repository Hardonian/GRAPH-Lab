# Milestone M13 Research Report: SAT-Based Minor Solver & Exact Hadwiger Classification for $n > 10$

## Executive Summary
Prior to Milestone M13, exact Hadwiger number computations ($h(G)$) via restricted-growth branch set enumeration were capped at $n \le 10$ (`HADWIGER_MAX_N = 10`) due to the combinatorial growth of Bell partitions with deletion.

In Milestone M13, we implemented an exact CDCL SAT-based complete minor engine (`crates/graphlab-core/src/hadwiger_sat.rs`) powered by `varisat` (pure Rust CDCL solver). The solver supports arbitrary graphs on up to 64 vertices, breaking through the previous $n \le 10$ barrier.

We classified four famous benchmark graphs ($n = 11 \dots 16$), finding exact Hadwiger numbers and certified constructive branch-set models, all independently validated by our kernel-grade validator `check_branch_model`:
1. **Grötzsch Graph** ($n = 11, m = 20, \chi = 4$): Triangle-free Mycielskian of $C_5$. Exact Hadwiger number: **$h = 6$**. (Hadwiger's Conjecture holds with margin $+2$).
2. **Chvátal Graph** ($n = 12, m = 24, \chi = 4$): Triangle-free 4-regular Hamiltonian graph. Exact Hadwiger number: **$h = 6$**. (Hadwiger's Conjecture holds with margin $+2$).
3. **Clebsch Graph** ($n = 16, m = 40, \chi = 4$): Triangle-free 5-regular strongly regular graph $\text{srg}(16, 5, 0, 2)$ (folded 5-cube). Exact Hadwiger number: **$h = 8$**. (Hadwiger's Conjecture holds with margin $+4$).
4. **Hypercube $Q_4$** ($n = 16, m = 32, \chi = 2$): 4-dimensional hypercube. Exact Hadwiger number: **$h = 6$**. (Hadwiger's Conjecture holds with margin $+4$).

---

## 1. CNF Formulation of Complete Graph Minors

A graph $G = (V, E)$ contains $K_t$ as a minor if and only if there exist $t$ pairwise disjoint non-empty subsets $B_0, \dots, B_{t-1} \subseteq V$ such that:
1. Each induced subgraph $G[B_k]$ is connected.
2. For each pair $0 \le j < k < t$, there is at least one edge in $G$ with one endpoint in $B_j$ and the other in $B_k$.

### 1.1 Variables
- **Branch Assignment:** $x_{v, k} \in \{0, 1\}$ for each vertex $v \in V$ and branch $k \in \{0, \dots, t-1\}$. $x_{v, k} = 1$ iff $v \in B_k$.
- **Branch Roots:** $root_{v, k} \in \{0, 1\}$ for $v \in V, k \in \{0, \dots, t-1\}$. Designates the unique root of branch $B_k$.
- **DAG Parent Pointers:** $p_{v, u} \in \{0, 1\}$ for directed edge $(v, u)$ where $u \in N(v)$.
- **Distance Order Variables:** $d_{v, l} \in \{0, 1\}$ for $v \in V, l \in \{0, \dots, n - t - 1\}$. Represents $\text{dist}(v) \ge l + 1$ from root.
- **Cross-Edges:** $cross_{e, j, k} \in \{0, 1\}$ for edge $e \in E$ and branch pair $0 \le j < k < t$.

### 1.2 Constraint System
1. **Pairwise Disjoint Branch Sets:**
   $$\forall v \in V, \; 0 \le j < k < t: \quad \neg x_{v, j} \lor \neg x_{v, k}$$
2. **Unique Branch Roots with Symmetry Breaking:**
   $$\forall k: \quad \bigvee_{v \in V} root_{v, k}$$
   $$\forall k, \; u < v: \quad \neg root_{u, k} \lor \neg root_{v, k}$$
   $$\forall v, k: \quad \neg root_{v, k} \lor x_{v, k}$$
   $$\forall k < t - 1, \; v \in V, \; u \le v: \quad \neg root_{v, k} \lor \neg root_{u, k+1}$$
   *(The strict ordering $root_0 < root_1 < \dots < root_{t-1}$ breaks the $t!$ automorphism symmetry of $K_t$.)*
3. **Connectivity via Spanning Trees with Acyclic Order:**
   - Every non-root vertex in $B_k$ must select a parent neighbor:
     $$\forall v \in V, k: \quad \neg x_{v, k} \lor root_{v, k} \lor \bigvee_{u \in N(v)} p_{v, u}$$
   - Parent must be in the same branch:
     $$\forall u \in N(v), k: \quad \neg p_{v, u} \lor \neg x_{v, k} \lor x_{u, k}$$
   - Root has distance 0 ($\neg d_{v, 0}$):
     $$\forall k: \quad \neg root_{v, k} \lor \neg d_{v, 0}$$
   - Strict distance descent along parent edges ($\text{dist}(v) \ge \text{dist}(u) + 1$):
     $$\neg p_{v, u} \lor d_{v, 0}$$
     $$\forall l: \quad \neg p_{v, u} \lor \neg d_{u, l} \lor d_{v, l+1}$$
     $$\neg p_{v, u} \lor \neg d_{u, \max}$$
4. **Pairwise Branch Adjacency:**
   $$\forall 0 \le j < k < t: \quad \bigvee_{e \in E} cross_{e, j, k}$$
   For each $e = \{u, v\} \in E$:
   $$cross_{e, j, k} \implies (x_{u, j} \lor x_{u, k}) \land (x_{v, j} \lor x_{v, k}) \land \neg(x_{u, j} \land x_{v, j}) \land \neg(x_{u, k} \land x_{v, k})$$

### 1.3 Pruning Theorems
- **Edge Bound:** $|E(G)| \ge \binom{t}{2}$.
- **Degree Pigeonhole Pruning:** If $\Delta(G) < t - 1$ and $t > \lfloor n / 2 \rfloor$, $G$ cannot contain a $K_t$ minor.
  *Proof:* If all branch sets had size $\ge 2$, then $n = \sum |B_k| \ge 2t \implies t \le n/2$. If $t > n/2$, at least one branch set must be a singleton $\{v\}$. A singleton has external degree at most $\text{deg}(v) \le \Delta(G) < t - 1$, and therefore cannot connect to all $t - 1$ other branch sets. $\blacksquare$

---

## 2. Benchmark Classification Results

| Graph | Vertices ($n$) | Edges ($m$) | Chromatic Number ($\chi$) | Exact Hadwiger ($h$) | Hadwiger Margin ($h - \chi$) | Certified Branch Model |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Grötzsch** | 11 | 20 | 4 | **6** | $+2$ | `[0, 3, 1, 2, 4, 3, 1, 2, 4, 0, 5]` |
| **Chvátal** | 12 | 24 | 4 | **6** | $+2$ | `[4, 3, 0, 1, 1, 2, 0, 3, 5, 4, 5, 2]` |
| **Clebsch** | 16 | 40 | 4 | **8** | $+4$ | `[7, 0, 2, 0, 5, 1, 2, 1, 3, 3, 4, 5, 6, 6, 4, 7]` |
| **Hypercube $Q_4$** | 16 | 32 | 2 | **6** | $+4$ | `[0, 3, 1, 1, 2, 3, 2, 5, 0, 3, 4, 4, 5, 5, 2, 5]` |

### Detailed Notes on Benchmarks:
1. **Grötzsch Graph:**
   - Smallest triangle-free graph with $\chi = 4$.
   - Has $K_6$ minor (contracting 5 vertices into pairs, leaving apex as singleton).
   - $h = 6 \ge 4$ firmly confirms Hadwiger's Conjecture.
2. **Chvátal Graph:**
   - Smallest 4-regular triangle-free graph with $\chi = 4$.
   - $h = 6 \ge 4$ confirms Hadwiger's Conjecture.
3. **Clebsch Graph:**
   - Folded 5-cube, triangle-free 5-regular graph on 16 vertices.
   - Partitions into exactly 8 connected pairs of vertices!
   - Every pair of the 8 parts shares an edge ($\binom{8}{2} = 28$ cross edges, 8 internal edges; $28 + 8 = 36 \le 40$).
   - Yields an exact $K_8$ minor!
4. **Hypercube $Q_4$:**
   - Bipartite 4-regular graph on 16 vertices.
   - Contains an exact $K_6$ minor ($h = 6$).
   - $h = 6 \ge 2$ satisfies Hadwiger's Conjecture.

---

## 3. CLI Integration & Verification

The CLI command `graphlab hadwiger` now automatically detects graphs with $n > 10$ and seamlessly routes them to the CDCL SAT solver:
```bash
$ target/release/graphlab hadwiger --input "JkLTAQGK?N_" --json
{
  "branch_model": [3, 1, 2, 4, 0, 1, 2, 0, 3, 4, 5],
  "chi": 4,
  "graph6": "JkLTAQGK?N_",
  "hadwiger": 6,
  "m": 20,
  "n": 11,
  "solver": "sat_cdcl"
}
```
All models produced are verified against `check_branch_model` before being output.

---

## 4. Status and Provenance
- **Status:** EXACT_FINITE / SAT_WITNESS
- **Rust Toolchain:** `1.99.1-x86_64-pc-windows-gnullvm`
- **Dependency Added:** `varisat = "0.2.2"` (pure Rust CDCL SAT solver)
- **Gates Passed:** 9/9 in `.\scripts\verify.ps1`
