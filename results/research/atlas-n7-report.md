# Atlas of Invariants and Minor Structure for n = 7 Graphs

Date: 2026-10-17 · Classes: **1044** (all non-isomorphic simple graphs on 7 vertices)
Evidence class: **EXACT_FINITE** (Rust bitset engine + nauty canonicalization + NetworkX oracle)

## 1. Conjectures Verification

- **Hadwiger's Conjecture ($\chi \le h$):** 100% verified (1044/1044 pass, **0 violations**).
- **Woodall's Weak Hadwiger Conjecture ($h \cdot \alpha \ge 7$):** 100% verified (1044/1044 pass, **0 violations**).
- **Tight Woodall Graphs ($h \cdot \alpha = 7$):** exactly **2** graphs achieve the minimum ratio $1.0$.

## 2. Hadwiger Number Distribution at n = 7

| Hadwiger Number h | Graph Count | Percentage |
|:---:|:---:|:---:|
| 1 | 1 | 0.1% |
| 2 | 36 | 3.4% |
| 3 | 323 | 30.9% |
| 4 | 509 | 48.8% |
| 5 | 162 | 15.5% |
| 6 | 12 | 1.1% |
| 7 | 1 | 0.1% |

## 3. Tight Woodall Graphs ($h \cdot \alpha = 7$)

For these graphs, the Woodall product achieves the absolute minimum bound $n = 7$:

| graph6 | m | deg | conn | $\chi$ | $\omega$ | $\alpha$ | $h$ | $h \cdot \alpha$ |
|:---|:---:|:---|:---:|:---:|:---:|:---:|:---:|:---:|
| `F????` | 0 | `[0, 0, 0, 0, 0, 0, 0]` | False | 1 | 1 | 7 | 1 | 7 |
| `F~~~w` | 21 | `[6, 6, 6, 6, 6, 6, 6]` | True | 7 | 7 | 1 | 7 | 7 |
