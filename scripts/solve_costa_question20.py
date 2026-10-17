#!/usr/bin/env python3
r"""
solve_costa_question20.py — Computational verification of Hadwiger's Conjecture
for Complements of Sum-Free-Maximal Cayley Graphs over Finite Fields F_p.

Addresses:
Question 20 (Costa, Luu, Wood, Yip, arXiv:2512.17114v1, Dec 2025):
"Let (Gamma, +) be an abelian group and S be an inverse-closed generating subset of Gamma
such that 0 not in S, and S is sum-free-maximal. Does \overline{Cay(Gamma, S)} satisfy
Hadwiger's Conjecture (HC_{\alpha=2})?
Conjecture HC_{\alpha=2} is open even for \overline{Cay(F_p, S)}, where p is an odd prime
and S is an inverse-closed generating sum-free-maximal subset of F_p."
"""

import sys
import json
import itertools
import subprocess
from pathlib import Path
from typing import List, Set, Tuple, Dict, Any

REPO_ROOT = Path(__file__).resolve().parent.parent
GRAPHLAB_BIN = REPO_ROOT / "target" / "release" / "graphlab.exe"
if not GRAPHLAB_BIN.exists():
    GRAPHLAB_BIN = REPO_ROOT / "target" / "release" / "graphlab"

def is_sum_free(S: Set[int], p: int) -> bool:
    """Check if (S + S) cap S == empty in F_p."""
    for x in S:
        for y in S:
            if (x + y) % p in S:
                return False
    return True

def find_maximal_sum_free_sets(p: int) -> List[Set[int]]:
    r"""
    Find all inverse-closed sum-free-maximal subsets S of F_p \ {0}.
    Since S is inverse-closed, S is a union of pairs {x, p - x} for 1 <= x <= p // 2.
    """
    pairs = [(x, p - x) for x in range(1, p // 2 + 1)]
    m = len(pairs)
    
    # We want maximal sum-free sets. We can search subsets of pairs.
    # For small m (up to 15, corresponding to p <= 31), 2^m subsets is at most 32768.
    valid_subsets = []
    
    for k in range(1, m + 1):
        for combo in itertools.combinations(pairs, k):
            S = set()
            for u, v in combo:
                S.add(u)
                S.add(v)
            if is_sum_free(S, p):
                valid_subsets.append(S)
                
    # Filter for maximal: S is not a strict subset of any other sum-free set
    maximal_sets = []
    for s in valid_subsets:
        # Check if s can be extended by any other pair
        can_extend = False
        for u, v in pairs:
            if u not in s:
                candidate = s | {u, v}
                if is_sum_free(candidate, p):
                    can_extend = True
                    break
        if not can_extend:
            maximal_sets.append(s)
            
    return maximal_sets

def to_graph6(n: int, adj: List[List[bool]]) -> str:
    """Encode an adjacency matrix to graph6 string format."""
    bit_vec = []
    for j in range(1, n):
        for i in range(j):
            bit_vec.append(1 if adj[i][j] else 0)
            
    # n header
    chars = [chr(n + 63)]
    # 6-bit chunks
    for i in range(0, len(bit_vec), 6):
        chunk = bit_vec[i:i+6]
        val = 0
        for b in chunk:
            val = (val << 1) | b
        val <<= (6 - len(chunk))
        chars.append(chr(val + 63))
    return "".join(chars)

def build_cayley_complement(p: int, S: Set[int]) -> Tuple[str, int]:
    r"""
    Construct G = \overline{Cay(F_p, S)}.
    Vertices: 0, 1, ..., p-1.
    Edges in Cayley: (u, v) iff (v - u) % p in S.
    Edges in G: (u, v) iff u != v and (v - u) % p not in S.
    """
    adj = [[False] * p for _ in range(p)]
    m_count = 0
    for u in range(p):
        for v in range(u + 1, p):
            diff = (v - u) % p
            if diff not in S:
                adj[u][v] = True
                adj[v][u] = True
                m_count += 1
    g6 = to_graph6(p, adj)
    return g6, m_count

def get_canonical_hex(g6: str) -> str:
    res = subprocess.run(
        [str(GRAPHLAB_BIN), "canon", "--input", g6],
        capture_output=True,
        text=True,
        check=True
    )
    return res.stdout.strip()

def get_invariants(g6: str) -> Dict[str, Any]:
    res = subprocess.run(
        [str(GRAPHLAB_BIN), "invariants", "--input", g6, "--json"],
        capture_output=True,
        text=True,
        check=True
    )
    return json.loads(res.stdout)

def test_minor(g6: str, t: int, timeout_sec: int = 60) -> Tuple[bool, Any]:
    try:
        res = subprocess.run(
            [str(GRAPHLAB_BIN), "hadwiger", "--input", g6, "--sat", "--minor", str(t), "--json"],
            capture_output=True,
            text=True,
            timeout=timeout_sec,
            check=True
        )
        data = json.loads(res.stdout)
        return data.get("has_minor", False), data.get("branch_model")
    except subprocess.TimeoutExpired:
        return False, None

def compute_hadwiger_full(g6: str) -> Tuple[int, Any]:
    res = subprocess.run(
        [str(GRAPHLAB_BIN), "hadwiger", "--input", g6, "--sat", "--json"],
        capture_output=True,
        text=True,
        check=True
    )
    data = json.loads(res.stdout)
    return data["hadwiger"], data.get("branch_model")

def main():
    primes = [5, 7, 11, 13, 17, 19, 23]
    if len(sys.argv) > 1:
        primes = [int(x) for x in sys.argv[1:]]
        
    print(f"=== Resolution of Costa et al. (2025) Question 20 for Primes: {primes} ===", flush=True)
    
    results = []
    
    for p in primes:
        print(f"\n--- Investigating F_{p} (order {p}) ---", flush=True)
        max_sets = find_maximal_sum_free_sets(p)
        print(f"Found {len(max_sets)} maximal sum-free subsets in F_{p}.", flush=True)
        
        # Group into isomorphism classes of complement graphs
        iso_classes = {}
        for idx, S in enumerate(max_sets):
            g6, m_count = build_cayley_complement(p, S)
            c_hex = get_canonical_hex(g6)
            if c_hex not in iso_classes:
                iso_classes[c_hex] = {
                    "p": p,
                    "representative_S": sorted(list(S)),
                    "all_S_count": 1,
                    "g6": g6,
                    "m": m_count,
                    "c_hex": c_hex
                }
            else:
                iso_classes[c_hex]["all_S_count"] += 1
                
        print(f"Grouped into {len(iso_classes)} distinct isomorphism classes.", flush=True)
        
        for class_idx, (c_hex, info) in enumerate(iso_classes.items(), 1):
            g6 = info["g6"]
            inv = get_invariants(g6)
            chi = inv["chromatic_number"]
            alpha = inv["independence_number"]
            tri = inv["triangles"]
            clique = inv["clique_number"]
            
            target_t = (p + 1) // 2
            print(f"  Class {class_idx}/{len(iso_classes)}: S={info['representative_S']} (size |S|={len(info['representative_S'])})", flush=True)
            print(f"    Graph: n={p}, m={info['m']}, chi={chi}, alpha={alpha}, omega={clique}", flush=True)
            assert alpha <= 2, f"Expected alpha <= 2, got {alpha}"
            
            # 1. Test target minor t = (p + 1) // 2
            has_half_minor, model_half = test_minor(g6, target_t, timeout_sec=60)
            print(f"    Target minor K_{target_t} (ceil(n/2)): {has_half_minor}", flush=True)
            assert has_half_minor, f"VIOLATION: F_{p} Cayley complement does NOT have K_{target_t} minor!"
            
            # 2. Test target minor t = chi
            has_chi_minor, model_chi = test_minor(g6, chi, timeout_sec=60)
            print(f"    Chromatic minor K_{chi} (chi(G)): {has_chi_minor}", flush=True)
            assert has_chi_minor, f"VIOLATION: Hadwiger's conjecture fails for chi={chi}!"
            
            # 3. Probe higher minors t = chi, chi+1, chi+2, ... to find best known minor
            best_minor = chi if has_chi_minor else target_t
            best_model = model_chi
            for probe_t in range(best_minor + 1, p + 1):
                has_probe, probe_model = test_minor(g6, probe_t, timeout_sec=15)
                if has_probe:
                    print(f"    Found higher minor K_{probe_t}! (margin over chi: +{probe_t - chi})", flush=True)
                    best_minor = probe_t
                    best_model = probe_model
                else:
                    print(f"    Higher probe K_{probe_t} stopped. Certified minor is K_{best_minor}.", flush=True)
                    break
            
            full_h = best_minor
            print(f"    Certified Hadwiger number h(G) >= {best_minor} (Hadwiger Conjecture Margin: +{best_minor - chi})", flush=True)
            
            record = {
                "p": p,
                "class_id": class_idx,
                "num_isomorphic_sets": info["all_S_count"],
                "sample_S": info["representative_S"],
                "size_S": len(info["representative_S"]),
                "g6": g6,
                "n": p,
                "m": info["m"],
                "chi": chi,
                "alpha": alpha,
                "omega": clique,
                "k_half_minor": target_t,
                "has_k_half_minor": has_half_minor,
                "has_k_chi_minor": has_chi_minor,
                "certified_minor_order": best_minor,
                "margin_over_chi": best_minor - chi,
                "branch_model": best_model
            }
            results.append(record)
            
    out_path = REPO_ROOT / "results" / "research" / "costa_question20_f_p_results.json"
    out_path.parent.mkdir(parents=True, exist_ok=True)
    with open(out_path, "w") as f:
        json.dump(results, f, indent=2)
    print(f"\nAll tests passed! Results saved to {out_path}", flush=True)

if __name__ == "__main__":
    main()
