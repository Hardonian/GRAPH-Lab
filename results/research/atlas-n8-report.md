# Hadwiger & Woodall Exact Atlas Report (n=8)

**Date:** 2026-10-17  
**Universe:** All 12346 non-isomorphic simple graphs on 8 vertices (OEIS A000088)  
**Execution:** 24 threads parallel, 60.85s total (4.9 ms/graph)  
**Source binary:** `graphlab.exe` (Rust, verified release)  

## 1. Conjectures Verification

| Conjecture | Formula | Violations | Status |
|---|---|---|---|
| Hadwiger's Conjecture | $\chi(G) \le h(G)$ | 0 | **VERIFIED (100%)** |
| Woodall's Weak Hadwiger | $h(G) \cdot \alpha(G) \ge 8$ | 0 | **VERIFIED (100%)** |

## 2. Hadwiger Number Distribution ($n=8$)

| Hadwiger Number $h(G)$ | Count | Percentage |
|---|---|---|
| $h=1$ | 1 | 0.01% |
| $h=2$ | 75 | 0.61% |
| $h=3$ | 1639 | 13.28% |
| $h=4$ | 6036 | 48.89% |
| $h=5$ | 4099 | 33.20% |
| $h=6$ | 479 | 3.88% |
| $h=7$ | 16 | 0.13% |
| $h=8$ | 1 | 0.01% |
| **Total** | **12346** | **100.00%** |

## 3. Woodall Ratio Tightness ($h \cdot \alpha = 8$)

Out of 12,346 graphs, exactly **34 graphs** achieve $h(G) \cdot \alpha(G) = 8$.
Since $8 = 2^3$, composite factorizations $1 \times 8$, $2 \times 4$, $4 \times 2$, and $8 \times 1$ appear:

| graph6 | Edges $m$ | $h(G)$ | $\alpha(G)$ | $\chi(G)$ | Factorization | Description |
|---|---|---|---|---|---|---|
| `G?????` | 0 | 1 | 8 | 1 | 1 x 8 | Empty graph E_8 |
| `G?`@?_` | 4 | 2 | 4 | 2 | 2 x 4 | Composite factor witness |
| `G?`@C_` | 5 | 2 | 4 | 2 | 2 x 4 | Composite factor witness |
| `G?`D@_` | 6 | 2 | 4 | 2 | 2 x 4 | Composite factor witness |
| `G?`DA_` | 6 | 2 | 4 | 2 | 2 x 4 | Composite factor witness |
| `G?`@E_` | 6 | 2 | 4 | 2 | 2 x 4 | Composite factor witness |
| `G?bB@_` | 7 | 2 | 4 | 2 | 2 x 4 | Composite factor witness |
| `G?`DB_` | 7 | 2 | 4 | 2 | 2 x 4 | Composite factor witness |
| `G?`F@_` | 7 | 2 | 4 | 2 | 2 x 4 | Composite factor witness |
| `G?`DE_` | 7 | 2 | 4 | 2 | 2 x 4 | Composite factor witness |
| `G?`@F_` | 7 | 2 | 4 | 2 | 2 x 4 | Composite factor witness |
| `GQhTQg` | 12 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQhTUg` | 13 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQhVUg` | 14 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQhTVg` | 14 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQjVRg` | 15 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQhVVg` | 15 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQhVUk` | 15 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQhTVw` | 15 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQzRtg` | 16 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQhVvg` | 16 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQjVVg` | 16 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQjVUk` | 16 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQjUl[` | 16 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQhVVk` | 16 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQzRtk` | 17 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQjVnW` | 17 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQjVVk` | 17 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQjUn[` | 17 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQhVvk` | 17 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQyuzw` | 18 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQzV]w` | 18 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `GQjVn[` | 18 | 4 | 2 | 4 | 4 x 2 | Composite factor witness |
| `G~~~~{` | 28 | 8 | 1 | 8 | 8 x 1 | Complete graph K_8 |

