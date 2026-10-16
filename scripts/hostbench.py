#!/usr/bin/env python3
"""Bounded CPU microbenchmark for the enumeration inner-loop shape.
Writes sanitized results into results/host-capabilities.json bench block.
Threads and iterations are capped; runs in < 30s."""
import json, os, subprocess, sys, time
import multiprocessing as mp

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

def _pop(seed, iters=200_000):
    acc = 0; x = seed
    for _ in range(iters):
        x = (x * 6364136223846793005 + 1442695040888963407) & (2**64 - 1)
        acc += bin(x).count('1')
    return acc

def bench(threads):
    t0 = time.perf_counter()
    with mp.Pool(threads) as p:
        p.map(_pop, range(threads))
    dt = time.perf_counter() - t0
    return dt, threads * 200_000 / dt

def main():
    d1, t1 = bench(1)
    d48, t48 = bench(48)
    # collect sanitized host facts (no UUIDs/serials/env dumps)
    def sh(cmd):
        try:
            return subprocess.run(cmd, capture_output=True, text=True, timeout=10).stdout.strip()
        except Exception:
            return ""
    nvidia = []
    out = sh(["nvidia-smi", "--query-gpu=index,name,memory.total,driver_version,compute_cap",
              "--format=csv,noheader"])
    for line in out.splitlines():
        idx, name, mem, drv, cc = [p.strip() for p in line.split(",")]
        nvidia.append({"smi_index": int(idx), "name": name, "vram_mib": int(mem.split()[0]),
                       "driver": drv, "compute_cap": cc})
    caps = {
        "cpu": {"model": "AMD EPYC 7452 32-Core Processor", "threads": 64,
                "sockets": 1, "numa_nodes": 1, "max_mhz": 3364},
        "memory_gib_total": 168,
        "storage": {"root_free_gib": 93, "ai_storage_free_gib": 633,
                    "repo_on": "/mnt/ai-storage (NVMe)"},
        "os": "Ubuntu 26.04.1 LTS / kernel 7.0.0-38-generic",
        "gpu": nvidia,
        "gpu_policy": "all three cards host production lanes; occupancy-gated use; CPU default and correctness oracle",
        "toolchains": {
            "rustc": sh(["rustc", "--version"]),
            "cargo": sh(["cargo", "--version"]),
            "python": sh(["python3", "--version"]),
            "uv": sh(["uv", "--version"]),
            "git": sh(["git", "--version"]),
            "gh": sh(["gh", "--version"]),
            "nvcc": sh(["nvcc", "--version"]).splitlines()[-1] if shutil_nvcc() else None,
            "lean": "not installed at recon (M3 installs via elan user-space)",
            "z3": "not installed at recon",
        },
        "bench": {
            "kernel": "LCG + popcount (enumeration inner-loop proxy)",
            "single_thread_ops_per_s": int(t1),
            "thread48_ops_per_s": int(t48),
            "speedup_vs_single": round(t48 / t1, 2),
            "single_wall_s": round(d1, 3),
            "thread48_wall_s": round(d48, 3),
            "iterations_per_thread": 200_000,
        },
        "sanitized": True,
        "collected_at": "2026-10-16",
    }
    path = os.path.join(REPO, "results", "host-capabilities.json")
    with open(path, "w") as f:
        json.dump(caps, f, indent=2)
    print("wrote", path)

def shutil_nvcc():
    import shutil
    return shutil.which("nvcc")

if __name__ == "__main__":
    sys.exit(main() or 0)
