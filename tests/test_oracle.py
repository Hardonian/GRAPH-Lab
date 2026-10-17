"""Tests for the Python control plane (oracle + codec)."""

from graphlab import oracle
from graphlab.g6 import decode_graph6, encode_graph6


def test_g6_roundtrip_known():
    n, edges = decode_graph6("Bw")  # K3
    assert n == 3 and edges == {(0, 1), (1, 2), (0, 2)}
    assert encode_graph6(n, edges) == "Bw"


def test_g6_rejects_bad():
    for bad in ["", "!", "B", "Bwww"]:
        try:
            decode_graph6(bad)
            assert False, f"accepted {bad!r}"
        except ValueError:
            pass


def test_oracles_agree_with_hand_values():
    # build C6: cycle 0-1-2-3-4-5-0
    edges = {(i, (i + 1) % 6) for i in range(6)}
    edges = {(min(a, b), max(a, b)) for a, b in edges}
    assert len(edges) == 6
    s = encode_graph6(6, edges)
    assert oracle.triangle_count(s) == 0
    assert oracle.chromatic_number_exhaustive(s) == 2
    assert oracle.independence_number(s) == 3
    assert oracle.clique_number(s) == 2
    assert len(oracle.components_sizes(s)) == 1
    # C5
    edges5 = {(min(i, (i + 1) % 5), max(i, (i + 1) % 5)) for i in range(5)}
    s5 = encode_graph6(5, edges5)
    assert oracle.chromatic_number_exhaustive(s5) == 3
    assert oracle.independence_number(s5) == 2


def test_isomorphism_oracles_concur():
    # P4 vs star K1,3 non-isomorphic; P4 vs relabeled P4 isomorphic
    p4 = encode_graph6(4, {(0, 1), (1, 2), (2, 3)})
    star = encode_graph6(4, {(0, 1), (0, 2), (0, 3)})
    rel = encode_graph6(4, {(2, 3), (3, 0), (0, 1)})  # 2-3-0-1 path = P4
    assert oracle.is_isomorphic_igraph(p4, star) is False
    assert oracle.is_isomorphic_nx(p4, star) is False
    assert oracle.is_isomorphic_igraph(p4, rel) is True
    assert oracle.is_isomorphic_nx(p4, rel) is True
