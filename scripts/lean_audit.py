#!/usr/bin/env python3
"""lean_audit.py — strict source scan of Lean artifacts.

Rejects (exit 1) on any of, IN CODE (block/line comments are stripped
first, so doc-comment prose like "no `sorry`" is not a violation):
  - sorry / admit
  - axiom declarations
  - unsafe declarations
  - native_decide (policy: forbidden outright in this repo)
  - generated witness file missing its predicate header line

With --run: also requires `lake build` exit 0 in the current tree.

Usage: python3 scripts/lean_audit.py [--run]
"""

from __future__ import annotations

import os
import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
LEAN_DIR = REPO / "lean"

FORBIDDEN = [
    (r"\bsorry\b", "sorry"),
    (r"\badmit\b", "admit"),
    (r"\baxiom\b", "axiom"),
    (r"\bsorryAxiom\b", "sorryAxiom"),
    (r"\bunsafe\b", "unsafe"),
    (r"\bnative_decide\b", "native_decide"),
]

def strip_comments(text: str) -> str:
    # nesting-aware strip: Lean block comments /- ... -/ (nested), line --
    out = []
    i = 0
    depth = 0
    n = len(text)
    while i < n:
        if text.startswith("/-", i):
            depth += 1
            i += 2
            continue
        if text.startswith("-/", i) and depth > 0:
            depth -= 1
            i += 2
            continue
        if depth == 0:
            if text.startswith("--", i):
                # skip to end of line
                j = text.find("\n", i)
                i = n if j < 0 else j
                continue
            out.append(text[i])
        i += 1
    return "".join(out)


def main() -> int:
    do_run = "--run" in sys.argv
    files = sorted(LEAN_DIR.rglob("*.lean"))
    files = [f for f in files if ".lake" not in str(f)]
    if not files:
        print(f"lean_audit: no sources under {LEAN_DIR}", file=sys.stderr)
        return 2
    violations = []
    for f in files:
        raw = f.read_text()
        code = strip_comments(raw)
        for i, line in enumerate(code.splitlines(), 1):
            for pat, name in FORBIDDEN:
                if re.search(pat, line):
                    violations.append(f"{f.relative_to(REPO)} (code line {i}): {name} — {line.strip()[:90]}")
        # generated-file integrity
        if "WITNESS FILE" in raw:
            m = re.search(r"predicate: ([a-z0-9\-]+)", raw)
            if not m:
                violations.append(f"{f.relative_to(REPO)}: generated witness file missing predicate line")
            elif m.group(1) not in raw.split("WITNESS FILE", 1)[1]:
                pass  # predicate appears in header; body check is heuristic
    if violations:
        print("lean_audit: FAIL")
        for v in violations:
            print("  ", v)
        return 1
    print(f"lean_audit: source scan clean ({len(files)} files, comments stripped)")
    if do_run:
        env = dict(os.environ, PATH=str(Path.home() / ".elan/bin") + ":" + os.environ.get("PATH", ""))
        r = subprocess.run(["lake", "build"], cwd=LEAN_DIR, env=env,
                           capture_output=True, text=True)
        if r.returncode != 0:
            print("lean_audit: lake build FAILED", file=sys.stderr)
            print(r.stdout[-3000:], r.stderr[-2000:], file=sys.stderr)
            return 1
        print("lean_audit: lake build exit 0 (kernel-checked)")
    return 0

if __name__ == "__main__":
    sys.exit(main())
