# AGENTS.md — rules for autonomous agents working in this repo

1. `docs/STATUS.md` is the source of truth for progress. On context loss,
   resume from this file plus `git log` and the verified remote SHA — never
   from remembered state.
2. Every milestone gate: run `scripts/verify.sh`, commit with a conventional
   message, push, verify remote SHA, record it in `docs/STATUS.md`.
3. Do not claim `SOLVED`/`PROVED` without a kernel-checked Lean proof of the
   audited *original* statement, or an independently rechecked exhaustive
   disproof certificate. Unproved = labeled unproved.
4. Results carry one of: EXPERIMENTAL / EXACT_FINITE / SAT_WITNESS /
   LEAN_CHECKED / NOVELTY_REVIEWED / OPEN.
5. Host safety: this machine runs production GPU services (Ollama lanes on
   V100/P40/RTX 3060, ComfyUI). Default CPU jobs to <=48 of 64 threads.
   Never touch running services, drivers, systemd, firewall, or reboot.
   Check `nvidia-smi` occupancy before any GPU work; P40 and 3060 carry live
   services — V100 is usually idle but shared with ollama-default.
6. Never commit secrets, host UUIDs/serials, environment dumps, or large
   binaries. Sanitize machine-readable outputs before committing.
7. Keep the CPU path canonical: any GPU code must reproduce CPU results on
   deterministic cases or it does not land.
8. Pin toolchains (`rust-toolchain.toml`, `lean-toolchain`, uv.lock) and
   record third-party provenance with version + SHA256.
