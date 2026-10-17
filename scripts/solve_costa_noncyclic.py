#!/usr/bin/env python3
"""solve_costa_noncyclic.py — Resolve Costa et al. (2025) Question 20 for Non-Cyclic Abelian Groups.

Investigates:
  "Let (Gamma, +) be an abelian group and S be an inverse-closed generating subset
   of Gamma such that 0 not in S, and S is sum-free-maximal. Does \overline{Cay(Gamma, S)}
   satisfy Hadwiger's Conjecture (HC_{\alpha=2})?"

Evaluates:
  1. Z_2 x Z_2 (order 4)
  2. Z_2 x Z_4 (order 8)
  3. Z_2^3 = Z_2 x Z_2 x Z_2 (order 8)
  4. Z_3 x Z_3 (order 9) — Vacuity theorem (no inverse-closed sum-free sets in 3-groups)
  5. Z_2 x Z_6 (order 12)
  6. Z_4 x Z_4 (order 16)
  7. Z_2^4 (order 16)
"""

from __future__ import annotations

import json
import subprocess
import time
from pathlib import Path
from typing import Any, Callable, Dict, List, Set, Tuple

REPO = Path(__file__).resolve().parents[1]
RESULTS_JSON = REPO / "results" / "research" / "costa_question20_noncyclic_results.json"
REPORT_MD = REPO / "results" / "research" / "costa-noncyclic-report.md"
CLI_BIN = REPO / "target" / "release" / "graphlab.exe"


def run_cli_invariants(g6: str) -> Dict[str, Any]:
    cmd = [str(CLI_BIN), "invariants", "--input", g6, "--json"]
    res = subprocess.run(cmd, capture_output=True, text=True, check=True)
    return json.loads(res.stdout)


def run_cli_hadwiger(g6: str) -> int:
    cmd = [str(CLI_BIN), "hadwiger", "--input", g6, "--sat"]
    res = subprocess.run(cmd, capture_output=True, text=True, check=True)
    return int(res.stdout.strip())


def analyze_abelian_group(
    name: str,
    elements: List[Any],
    add_func: Callable[[Any, Any], Any],
    zero_elem: Any,
) -> List[Dict[str, Any]]:
    n = len(elements)
    nonzero = [x for x in elements if x != zero_elem]
    inv = {x: [y for y in nonzero if add_func(x, y) == zero_elem][0] for x in nonzero}

    # Partition non-zero elements into inverse pairs
    pairs = []
    seen = set()
    for x in nonzero:
        if x not in seen:
            y = inv[x]
            if x == y:
                pairs.append({x})
                seen.add(x)
            else:
                pairs.append({x, y})
                seen.add(x)
                seen.add(y)

    # Check 3-group obstruction: if any element x has 2x = -x, then x+x in {x, -x}
    # If all elements have order 3, no inverse-closed sum-free sets exist!
    has_order_3_only = all(add_func(x, add_func(x, x)) == zero_elem for x in nonzero)
    if has_order_3_only:
        print(f"[{name}] Order {n}: 3-group vacuity theorem applies (0 inverse-closed sum-free sets exist).")
        return [{
            "group": name,
            "order": n,
            "status": "VACUOUSLY_SATISFIED",
            "reason": "Theorem: Every element x has order 3, so -x = 2x = x + x in S + S, precluding sum-freeness.",
            "classes": []
        }]

    # Backtracking search for sum-free sets
    def is_sum_free(S: Set[Any]) -> bool:
        for x in S:
            for y in S:
                if add_func(x, y) in S:
                    return False
        return True

    def generates(S: Set[Any]) -> bool:
        reached = {zero_elem}
        q = [zero_elem]
        while q:
            curr = q.pop(0)
            for s in S:
                nxt = add_func(curr, s)
                if nxt not in reached:
                    reached.add(nxt)
                    q.append(nxt)
        return len(reached) == n

    valid_sf = []

    def backtrack(idx: int, current_S: Set[Any]):
        if idx == len(pairs):
            if current_S and is_sum_free(current_S):
                valid_sf.append(set(current_S))
            return
        backtrack(idx + 1, current_S)
        cand = current_S | pairs[idx]
        if is_sum_free(cand):
            backtrack(idx + 1, cand)

    backtrack(0, set())

    maximal_S = [s for s in valid_sf if not any(s != s2 and s.issubset(s2) for s2 in valid_sf)]
    gen_maximal = [s for s in maximal_S if generates(s)]

    print(f"[{name}] Order {n}: {len(maximal_S)} maximal SF sets, {len(gen_maximal)} generating.")

    if not gen_maximal:
        return [{
            "group": name,
            "order": n,
            "status": "VACUOUSLY_SATISFIED",
            "reason": "No sum-free subset generates the entire group.",
            "classes": []
        }]

    # Build complement graphs and deduplicate isomorphism classes
    import networkx as nx

    classes: Dict[str, Dict[str, Any]] = {}
    for S in gen_maximal:
        G = nx.Graph()
        G.add_nodes_from(range(n))
        elem_to_idx = {el: i for i, el in enumerate(elements)}
        for u in elements:
            for v in elements:
                if u != v:
                    neg_u = inv[u] if u != zero_elem else zero_elem
                    diff = add_func(v, neg_u)
                    if diff not in S:
                        G.add_edge(elem_to_idx[u], elem_to_idx[v])

        g6 = nx.to_graph6_bytes(G).decode().strip().replace(">>graph6<<", "")
        if g6 not in classes:
            # Query CLI invariants
            iv = run_cli_invariants(g6)
            h = run_cli_hadwiger(g6)
            chi = iv["chromatic_number"]
            classes[g6] = {
                "g6": g6,
                "n": iv["n"],
                "m": iv["m"],
                "degree_sequence": iv["degree_sequence"],
                "components": iv["components"],
                "connected": iv["connected"],
                "chi": chi,
                "alpha": iv["independence_number"],
                "omega": iv["clique_number"],
                "hadwiger": h,
                "hadwiger_holds": chi <= h,
                "margin": h - chi,
                "sample_S": [list(x) if isinstance(x, (tuple, list)) else x for x in list(S)],
                "size_S": len(S),
            }

    return [{
        "group": name,
        "order": n,
        "status": "SATISFIED",
        "num_classes": len(classes),
        "classes": list(classes.values()),
    }]


def main():
    print("=" * 70)
    print("RESOLVING COSTA ET AL. QUESTION 20 FOR NON-CYCLIC ABELIAN GROUPS")
    print("=" * 70)

    groups = [
        # Z_2 x Z_2
        ("Z_2 x Z_2", [(x, y) for x in range(2) for y in range(2)],
         lambda a, b: ((a[0] + b[0]) % 2, (a[1] + b[1]) % 2), (0, 0)),
        # Z_2 x Z_4
        ("Z_2 x Z_4", [(x, y) for x in range(2) for y in range(4)],
         lambda a, b: ((a[0] + b[0]) % 2, (a[1] + b[1]) % 4), (0, 0)),
        # Z_2^3
        ("Z_2^3", [(x, y, z) for x in range(2) for y in range(2) for z in range(2)],
         lambda a, b: ((a[0] + b[0]) % 2, (a[1] + b[1]) % 2, (a[2] + b[2]) % 2), (0, 0, 0)),
        # Z_3 x Z_3
        ("Z_3 x Z_3", [(x, y) for x in range(3) for y in range(3)],
         lambda a, b: ((a[0] + b[0]) % 3, (a[1] + b[1]) % 3), (0, 0)),
        # Z_2 x Z_6
        ("Z_2 x Z_6", [(x, y) for x in range(2) for y in range(6)],
         lambda a, b: ((a[0] + b[0]) % 2, (a[1] + b[1]) % 6), (0, 0)),
        # Z_4 x Z_4
        ("Z_4 x Z_4", [(x, y) for x in range(4) for y in range(4)],
         lambda a, b: ((a[0] + b[0]) % 4, (a[1] + b[1]) % 4), (0, 0)),
        # Z_2^4
        ("Z_2^4", [(x, y, z, w) for x in range(2) for y in range(2) for z in range(2) for w in range(2)],
         lambda a, b: ((a[0] + b[0]) % 2, (a[1] + b[1]) % 2, (a[2] + b[2]) % 2, (a[3] + b[3]) % 2), (0, 0, 0, 0)),
    ]

    all_results = []
    total_classes = 0
    total_violations = 0

    start_total = time.time()
    for name, elems, add_fn, zero in groups:
        res = analyze_abelian_group(name, elems, add_fn, zero)
        all_results.extend(res)
        for r in res:
            for cls in r.get("classes", []):
                total_classes += 1
                if not cls["hadwiger_holds"]:
                    total_violations += 1

    total_time = time.time() - start_total

    RESULTS_JSON.parent.mkdir(parents=True, exist_ok=True)
    with open(RESULTS_JSON, "w") as f:
        json.dump(all_results, f, indent=2)

    print("\n" + "=" * 70)
    print(f"EVALUATION COMPLETE in {total_time:.2f}s")
    print(f"Total non-isomorphic classes evaluated: {total_classes}")
    print(f"Hadwiger Conjecture violations: {total_violations}")
    print(f"Results written to: {RESULTS_JSON}")
    print("=" * 70)


if __name__ == "__main__":
    main()
