"""Independent graph6 codec (pure Python) — cross-check for the Rust core."""

from __future__ import annotations


def decode_graph6(s: str) -> tuple[int, set[tuple[int, int]]]:
    """Return (n, edge set with a<b). Strict validation."""
    if not s:
        raise ValueError("empty graph6 string")
    b = s.encode("ascii")
    if any(x < 63 or x > 126 for x in b):
        raise ValueError("character outside graph6 alphabet")
    n = b[0] - 63
    total = n * (n - 1) // 2
    if len(b) != 1 + ((total + 5) // 6):
        raise ValueError(f"bad length for n={n}")
    bits: list[int] = []
    for x in b[1:]:
        v = (x - 63) & 0b111111
        for k in range(5, -1, -1):
            bits.append((v >> k) & 1)
    bits = bits[:total]
    edges: set[tuple[int, int]] = set()
    idx = 0
    for i in range(1, n):
        for j in range(i):
            if bits[idx]:
                edges.add((j, i))
            idx += 1
    return n, edges


def encode_graph6(n: int, edges: set[tuple[int, int]]) -> str:
    edges = {(min(a, b), max(a, b)) for a, b in edges}
    bits: list[int] = []
    for i in range(1, n):
        for j in range(i):
            bits.append(1 if (min(i, j), max(i, j)) in edges else 0)
    out = [63 + n]
    for k in range(0, len(bits), 6):
        chunk = bits[k : k + 6]
        v = 0
        for x in chunk:
            v = (v << 1) | x
        v <<= 6 - len(chunk)
        out.append(63 + v)
    return "".join(chr(x) for x in out)
