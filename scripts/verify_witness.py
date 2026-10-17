"""Independent (non-Rust) re-check of a graphlab witness JSONL file.

Re-decodes graph6 with the pure-Python codec, recomputes chi/alpha/h via
networkx+igraph and the DP chromatic oracle, and asserts the violation
still holds. Usage: uv run python scripts/verify_witness.py <file.jsonl>
Exit 0 = all witnesses independently confirmed; non-zero = any failure.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "python"))

from graphlab import oracle
from graphlab.g6 import decode_graph6, encode_graph6


def hadwiger_py(n: int, edges: set[tuple[int, int]], t: int) -> bool:
    """Independent exact K_t-minor search (deletion-tolerant branch sets).
    Restricted-growth assignment with DELETED=-1. Exponential; small n only."""
    adj = [set() for _ in range(n)]
    for a, b in edges:
        adj[a].add(b)
        adj[b].add(a)
    deleted = -1
    assign = [deleted] * n

    def check() -> bool:
        parts: list[list[int]] = [[] for _ in range(t)]
        for v, p in enumerate(assign):
            if p != deleted:
                parts[p].append(v)
        if any(len(p) == 0 for p in parts):
            return False
        for p in parts:  # connectivity
            seen = {p[0]}
            stack = [p[0]]
            while stack:
                x = stack.pop()
                for y in adj[x]:
                    if y not in seen and assign[y] == assign[p[0]]:
                        seen.add(y)
                        stack.append(y)
            if len(seen) != len(p):
                return False
        for a in range(t):  # pairwise adjacency
            for b in range(a + 1, t):
                if not any(y in adj[x] for x in parts[a] for y in parts[b]):
                    return False
        return True

    def rg(v: int, used: int) -> bool:
        if used == t and check():
            return True
        if used < t and used + (n - v) < t:
            return False
        if v == n:
            return used == t and check()
        # delete
        if rg(v + 1, used):
            return True
        for p in range(used):
            assign[v] = p
            if rg(v + 1, used):
                return True
            assign[v] = deleted
        if used < t:
            assign[v] = used
            if rg(v + 1, used + 1):
                return True
            assign[v] = deleted
        return False

    return rg(0, 0)


def hadwiger_number_py(n: int, edges: set[tuple[int, int]]) -> int:
    if n == 0:
        return 0
    best = 1
    for t in range(2, n + 1):
        if hadwiger_py(n, edges, t):
            best = t
    return best


def check_witness(w: dict) -> list[str]:
    errs: list[str] = []
    g6s = w["graph6"]
    n, edges = decode_graph6(g6s)
    if encode_graph6(n, edges) != g6s:
        errs.append("python codec round-trip mismatch")
    if n != w["n"] or len(edges) != w["m"]:
        errs.append(f"header n/m mismatch: {w['n']}/{w['m']} vs {n}/{len(edges)}")
    pred = w["predicate"]
    chi = oracle.chromatic_number_exhaustive(g6s)
    if pred == "hadwiger-lower-bound-v1":
        h = hadwiger_number_py(n, edges)
        if h >= chi:
            errs.append(f"NOT violated: h={h} chi={chi}")
        if w["reported"].get("chi") != chi or w["reported"].get("hadwiger") != h:
            errs.append(f"reported values wrong: {w['reported']} vs h={h} chi={chi}")
    elif pred == "weak-hadwiger-v1":
        alpha = oracle.independence_number(g6s)
        h = hadwiger_number_py(n, edges)
        if not (n > 0 and h * alpha < n):
            errs.append(f"NOT violated: h={h} alpha={alpha} n={n}")
    elif pred == "canary-chi-le-n-1-v1":
        if not (chi > n - 1):
            errs.append(f"NOT violated: chi={chi} n={n}")
    else:
        errs.append(f"python verifier has no clause for predicate {pred}")
    return errs


def main() -> int:
    path = Path(sys.argv[1])
    total = ok = 0
    for line in path.read_text().splitlines():
        line = line.strip()
        if not line:
            continue
        obj = json.loads(line)
        if "rejected" in obj or "manifest" in obj:
            continue
        total += 1
        w = obj
        errs = check_witness(w)
        if errs:
            print(f"WITNESS {w['graph6']}: FAIL — {'; '.join(errs)}")
        else:
            ok += 1
    print(f"independent python recheck: {ok}/{total} confirmed")
    return 0 if ok == total else 1


if __name__ == "__main__":
    sys.exit(main())
