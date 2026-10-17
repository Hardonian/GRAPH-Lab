"""Independent oracles for graph invariants.

networkx and igraph are INDEPENDENT implementations; we use them to check
the Rust engine on small instances. Where no library routine exists
(chromatic number), we implement exact brute force with memoized
independent-set covers — a second, different algorithm from the Rust DFS
coloring (cover DP vs. left-to-right coloring), so agreement is evidence.
"""

from __future__ import annotations

from itertools import combinations

import igraph as ig
import networkx as nx

from .g6 import decode_graph6


def to_nx(s: str) -> nx.Graph:
    n, edges = decode_graph6(s)
    g = nx.Graph()
    g.add_nodes_from(range(n))
    g.add_edges_from(edges)
    return g


def to_ig(s: str) -> ig.Graph:
    n, edges = decode_graph6(s)
    return ig.Graph(n=n, edges=sorted(edges), directed=False)


def components_sizes(s: str) -> tuple[int, ...]:
    return tuple(sorted((len(c) for c in nx.connected_components(to_nx(s))), reverse=True))


def triangle_count(s: str) -> int:
    # networkx triangles counts each triangle 3 times (once per vertex)
    return int(sum(nx.triangles(to_nx(s)).values()) // 3)


def clique_number(s: str) -> int:
    g = to_ig(s)
    # igraph maximal cliques — exhaustive for small n
    cl = g.cliques()
    return max((len(c) for c in cl), default=0 if g.vcount() == 0 else 1) if cl else (
        0 if g.vcount() == 0 else 1
    )


def independence_number(s: str) -> int:
    g = to_ig(s)
    ist = g.independent_vertex_sets()
    return max((len(v) for v in ist), default=0) if ist else 0


def chromatic_number_exhaustive(s: str) -> int:
    """Exact chromatic number via independent-set cover DP (distinct
    algorithm from the Rust coloring DFS)."""
    n, edges = decode_graph6(s)
    if n == 0:
        return 0
    adj: list[set[int]] = [set() for _ in range(n)]
    for a, b in edges:
        adj[a].add(b)
        adj[b].add(a)
    full = (1 << n) - 1
    indep = [False] * (1 << n)
    for mask in range(1 << n):
        vs = [v for v in range(n) if (mask >> v) & 1]
        ok = all(v not in adj[u] for u, v in combinations(vs, 2))
        indep[mask] = ok
    # dp[mask] = min colors to cover mask
    INF = n + 1
    dp = [INF] * (1 << n)
    dp[0] = 0
    for mask in range(1, 1 << n):
        sub = mask
        while sub:
            if indep[sub]:
                rest = mask & ~sub
                dp[mask] = min(dp[mask], dp[rest] + 1)
            sub = (sub - 1) & mask
    return dp[full]


def is_isomorphic_igraph(a: str, b: str) -> bool:
    ga, gb = to_ig(a), to_ig(b)
    return bool(ga.isomorphic(gb))


def is_isomorphic_nx(a: str, b: str) -> bool:
    return bool(nx.is_isomorphic(to_nx(a), to_nx(b)))
