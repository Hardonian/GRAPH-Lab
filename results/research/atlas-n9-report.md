# Empirical Research Report: Complete Hadwiger and Invariant Atlas for $n = 9$ (274,668 Graphs)

**Author:** GRAPH-Lab Computational Discovery Engine  
**Date:** October 2026  
**Status:** EXACT_FINITE  
**Dataset:** `results/research/atlas-n9.jsonl` (81.4 MB, 274,668 records)

---

## 1. Executive Summary

Using the parallel inductive enumeration engine and exact stack-bitset minor solver in `graphlab-cli`, we generated and evaluated the complete topological and minor invariant atlas for all **274,668 non-isomorphic simple graphs** on 9 vertices (OEIS A000088).

Every graph was evaluated for:
- Exact order $n$, size $m$, degree sequence, connected components, triangle count, 4-cycle count.
- Clique number $\omega(G)$, independence number $\alpha(G)$, chromatic number $\chi(G)$.
- Hadwiger number $h(G)$ (maximum complete minor $K_t \preccurlyeq G$).
- Hadwiger condition ($\chi(G) \le h(G)$) and Woodall's Weak Hadwiger condition ($h(G) \cdot \alpha(G) \ge 9$).

### Key Findings
1. **Hadwiger's Conjecture holds for 100% of $n = 9$ graphs:** Zero violations out of 274,668 graphs.
2. **Woodall's Weak Hadwiger Conjecture holds for 100% of $n = 9$ graphs:** Zero violations out of 274,668 graphs.
3. **Modal Hadwiger Peak:** Over 51.4% of all graphs at $n = 9$ have $h(G) = 5$.
4. **Surplus Margins:** 77.7% of graphs have $h(G) > \chi(G)$ strictly, with 1,248 graphs beating the chromatic number by $+3$.
5. **Exact Factorization of Tight Extremal Graphs:** Exactly **70 graphs** satisfy $h(G) \cdot \alpha(G) = 9$, partitioning strictly along the divisors of 9 ($1 \times 9$, $3 \times 3$, $9 \times 1$).

---

## 2. Invariant Distributions at $n = 9$

### 2.1 Hadwiger Number Distribution

| Hadwiger Number $h(G)$ | Graph Count | Percentage | Cumulative |
|:---:|:---:|:---:|:---:|
| 1 | 1 | 0.0004% | 0.0004% |
| 2 | 152 | 0.0553% | 0.0557% |
| 3 | 9,203 | 3.3506% | 3.4063% |
| 4 | 85,215 | 31.0247% | 34.4310% |
| 5 | 141,381 | 51.4734% | 85.9044% |
| 6 | 37,381 | 13.6095% | 99.5140% |
| 7 | 1,314 | 0.4784% | 99.9924% |
| 8 | 20 | 0.0073% | 99.9996% |
| 9 | 1 | 0.0004% | 100.0000% |
| **Total** | **274,668** | **100.0%** | — |

### 2.2 Chromatic Number Distribution

| Chromatic Number $\chi(G)$ | Graph Count | Percentage |
|:---:|:---:|:---:|
| 1 | 1 | 0.0004% |
| 2 | 1,118 | 0.4070% |
| 3 | 87,381 | 31.8133% |
| 4 | 155,291 | 56.5377% |
| 5 | 28,722 | 10.4570% |
| 6 | 2,028 | 0.7383% |
| 7 | 118 | 0.0430% |
| 8 | 8 | 0.0029% |
| 9 | 1 | 0.0004% |
| **Total** | **274,668** | **100.0%** |

### 2.3 Hadwiger Margin $(h(G) - \chi(G))$

| Margin $(h - \chi)$ | Graph Count | Percentage | Interpretation |
|:---:|:---:|:---:|:---|
| $< 0$ | 0 | 0.0% | **Zero Hadwiger Violations** |
| $+0$ | 61,287 | 22.3131% | Tight ($h = \chi$) |
| $+1$ | 160,899 | 58.5795% | Surplus $+1$ |
| $+2$ | 51,234 | 18.6531% | Surplus $+2$ |
| $+3$ | 1,248 | 0.4544% | Surplus $+3$ |

---

## 3. Woodall's Weak Hadwiger Conjecture ($h(G) \cdot \alpha(G) \ge 9$)

Woodall (1989) conjectured that for every graph $G$, $h(G) \cdot \alpha(G) \ge |V(G)|$.
Across all 274,668 graphs:
- **Violations:** 0 (100% adherence).
- **Tight Extremal Graphs ($h(G) \cdot \alpha(G) = 9$):** Exactly 70 graphs.

### Classification of Tight Extremal Graphs ($h \cdot \alpha = 9$)

| Factorization $(h, \alpha)$ | Count | Structural Characterization |
|:---:|:---:|:---|
| $(9, 1)$ | 1 | Complete graph $K_9$. |
| $(3, 3)$ | 68 | Triangle-free or near-triangle-free graphs of chromatic number 3 with independence number 3 contracting to $K_3$ but containing no $K_4$ minor. |
| $(1, 9)$ | 1 | Empty graph $E_9$. |
| **Total** | **70** | Divisor partition: $\sum_{d \mid 9} N(d, 9/d) = 1 + 68 + 1 = 70$. |

---

## 4. Performance & Verification Benchmark

- **Platform:** AMD 64-thread host running MinGW-UCRT with Rayon parallelism (24 threads budgeted).
- **Enumeration Throughput:** 274,668 graphs generated and canonicalized in **2.56 seconds** (107,292 graphs/sec).
- **Evaluation Throughput:** All invariants and exact Hadwiger numbers computed in **61.14 seconds** (4,492.3 graphs/sec).
- **Total Pipeline Execution:** 63.70 seconds.
