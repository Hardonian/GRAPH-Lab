# results/ — evidence ledger

Every artifact produced by a campaign, benchmark, or formal build lands here
with reproducible metadata. Rules:

1. Small, text-first artifacts are committed (JSONL manifests, benchmark
   summaries, milestone reports). Anything large or binary is regenerated
   with `scripts/reproduce.sh <campaign>` instead of committed.
2. Each `results/<area>/` dir has an `evidence.jsonl` (one entry per run):
   `{"ts", "command", "cwd", "input_sha256": {...}, "seed", "tool_versions",
   "cpu_model", "exit_code", "wall_ms", "peak_rss_kib", "artifacts": [...]}`.
   No cherry-picking: failed runs are logged too.
3. `results/milestones/M*.md` are per-gate verification reports with exact
   commands and real exit codes.
4. Sanitization: never include GPU UUIDs/serials, environment variables,
   tokens, or absolute paths outside the repo root (use `$REPO`).

Directories:
- `host-capabilities.json` — sanitized machine facts (M0)
- `enumeration/` — canonical count verifications (M1)
- `milestones/` — gate reports
- `campaigns/` — per-campaign witness/minimization evidence (M2+)
- `lean/` — build + strict-audit logs (M3+)
- `bench/` — raw + summarized CPU/GPU benchmarks (M4+)
- `research/` — candidate registry + campaign outputs (M5)
