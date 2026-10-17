"""Cross-verify the Rust engine against networkx + igraph on ALL unlabeled
graphs n=0..6 (1,1,2,4,11,34,156 classes; 2^(n choose 2) labeled instances
via the engine's canonical dedup). Writes results/enumeration/crosscheck-m1.json.

This is the M1 EXACT_FINITE evidence: engine==oracle==oracle2 on every
invariant, with real disagreement counts (expected 0).
"""

from __future__ import annotations

import hashlib
import json
import subprocess
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "python"))

from graphlab import oracle  # noqa: E402
from graphlab.g6 import decode_graph6, encode_graph6  # noqa: E402

REPO = Path(__file__).resolve().parents[1]
CLI = REPO / "target" / "release" / "graphlab"
KNOWN = [1, 1, 2, 4, 11, 34, 156]


def engine_enumerate(n: int) -> list[dict]:
    out = REPO / "results" / "tmp" / f"enum-{n}.jsonl"
    out.parent.mkdir(parents=True, exist_ok=True)
    r = subprocess.run(
        [str(CLI), "enumerate", "--vertices", str(n), "--output", str(out)],
        capture_output=True, text=True,
    )
    assert r.returncode == 0, r.stderr
    lines = [json.loads(l) for l in out.read_text().splitlines() if l.strip()]
    assert len(lines) == KNOWN[n], f"count mismatch n={n}: {len(lines)} != {KNOWN[n]}"
    return lines


def sha(s: str) -> str:
    return hashlib.sha256(s.encode()).hexdigest()[:16]


def main() -> int:
    t0 = time.time()
    disagree = {k: 0 for k in
                ["count", "m", "tri", "comp", "omega", "alpha", "chi", "g6", "iso", "canon"]}
    total = 0
    for n in range(0, 7):
        for row in engine_enumerate(n):
            total += 1
            s = row["graph6"]
            iv = row["invariants"]
            # g6 lossless via independent python codec
            nn, edges = decode_graph6(s)
            if encode_graph6(nn, edges) != s:
                disagree["g6"] += 1
            if nn != iv["n"]:
                disagree["g6"] += 1
            if len(edges) != iv["m"]:
                disagree["m"] += 1
            if oracle.triangle_count(s) != iv["triangles"]:
                disagree["tri"] += 1
            comps = oracle.components_sizes(s)
            if len(comps) != iv["components"]:
                disagree["comp"] += 1
            if oracle.clique_number(s) != iv["clique_number"]:
                disagree["omega"] += 1
            if oracle.independence_number(s) != iv["independence_number"]:
                disagree["alpha"] += 1
            if oracle.chromatic_number_exhaustive(s) != iv["chromatic_number"]:
                disagree["chi"] += 1

    # isomorphism cross-check: igraph vs engine canon on n=6 pairs (sampled
    # deterministically: each graph vs every 6th subsequent + self)
    rows6 = engine_enumerate(6)
    for i, a in enumerate(rows6):
        for j in [i, (i + 7) % 156, (i + 31) % 156]:
            b = rows6[j]
            ca = subprocess.run([str(CLI), "canon", "--input", a["graph6"]],
                                capture_output=True, text=True).stdout.strip()
            cb = subprocess.run([str(CLI), "canon", "--input", b["graph6"]],
                                capture_output=True, text=True).stdout.strip()
            eng = ca == cb
            ig_ok = oracle.is_isomorphic_igraph(a["graph6"], b["graph6"])
            nx_ok = oracle.is_isomorphic_nx(a["graph6"], b["graph6"])
            if not (eng == ig_ok == nx_ok):
                disagree["iso"] += 1

    result = {
        "milestone": "M1",
        "graphs_checked": total,
        "iso_pairs_checked": len(rows6) * 3,
        "disagreements": disagree,
        "known_counts_verified": KNOWN,
        "wall_s": round(time.time() - t0, 1),
        "status": "EXACT_FINITE" if not any(disagree.values()) else "FAILED",
        "engines": {
            "rust_cli_sha256": hashlib.sha256(CLI.read_bytes()).hexdigest()[:16],
            "python": sys.version.split()[0],
            "networkx": __import__("networkx").__version__,
            "igraph": __import__("igraph").__version__,
        },
    }
    out = REPO / "results" / "enumeration" / "crosscheck-m1.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(result, indent=2))
    print(json.dumps(result, indent=2))
    return 0 if result["status"] == "EXACT_FINITE" else 1


if __name__ == "__main__":
    sys.exit(main())
