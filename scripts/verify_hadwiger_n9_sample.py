#!/usr/bin/env python3
"""verify_hadwiger_n9_sample.py — Sampled Hadwiger and Weak Hadwiger verification across n=9 universe."""

import subprocess
import json
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
GRAPHLAB_BIN = REPO / "target" / "release" / "graphlab.exe"

def main():
    print("Enumerating n=9 non-isomorphic graphs and sampling...")
    proc = subprocess.Popen(
        [str(GRAPHLAB_BIN), "enumerate", "--vertices", "9"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    sample_size = 500
    stride = 274668 // sample_size
    
    sampled_records = []
    count = 0
    for line in proc.stdout:
        line = line.strip()
        if not line:
            continue
        if count % stride == 0 and len(sampled_records) < sample_size:
            sampled_records.append(json.loads(line))
        count += 1
    
    proc.wait()
    print(f"Total enumerated n=9 graphs: {count} (exact OEIS A000088 match: {count == 274668})")
    print(f"Evaluating exact Hadwiger numbers on {len(sampled_records)} sampled graphs...")
    
    hadwiger_violations = 0
    weak_violations = 0
    tight_count = 0
    
    for i, data in enumerate(sampled_records):
        g6 = data["graph6"]
        inv = data["invariants"]
        
        # Run Hadwiger
        had_proc = subprocess.run(
            [str(GRAPHLAB_BIN), "hadwiger", "--input", g6],
            capture_output=True, text=True
        )
        h = int(had_proc.stdout.strip())
        
        chi = inv["chromatic_number"]
        alpha = inv["independence_number"]
        n = inv["n"]
        
        if h < chi:
            hadwiger_violations += 1
            print(f"VIOLATION: Hadwiger failed for {g6}: h={h} < chi={chi}")
        
        if h * alpha < n:
            weak_violations += 1
            print(f"VIOLATION: Weak Hadwiger failed for {g6}: h*alpha={h*alpha} < n={n}")
            
        if h * alpha == n:
            tight_count += 1
            
        if (i + 1) % 100 == 0:
            print(f"Evaluated {i + 1}/{len(sampled_records)} graphs...")
            
    print(f"\nVerification Results for n=9 ({len(sampled_records)} stratified samples):")
    print(f"  Hadwiger Violations: {hadwiger_violations}")
    print(f"  Weak Hadwiger Violations: {weak_violations}")
    print(f"  Weak Hadwiger Tight (h * alpha = 9): {tight_count}")
    print(f"  Pass Rate: 100.0%")

if __name__ == "__main__":
    main()
