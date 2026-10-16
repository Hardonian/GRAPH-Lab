#!/usr/bin/env bash
# verify.sh — single-command verification entry point.
# Runs every gate that applies on THIS machine; prints real exit codes.
# Lean gates run only if the lean/ project exists (added at M3).
set -Euo pipefail
cd "$(dirname "$0")/.."
FAIL=0
run() {
  local name="$1"; shift
  echo "== $name =="
  if "$@"; then echo "PASS: $name"; else echo "FAIL($?): $name"; FAIL=1; fi
}

run "fmt"        cargo fmt --all -- --check
run "clippy"     cargo clippy --workspace --all-targets -- -D warnings
run "test"       cargo test --workspace --all-targets
run "release"    cargo build --workspace --release
run "cli-doctor" ./target/release/graphlab doctor --json

if [ -d lean ]; then
  run "lean-build" bash -c 'cd lean && lake build'
  run "lean-audit" python3 scripts/lean_audit.py
else
  echo "== lean =="
  echo "SKIP: lean/ not present yet (Milestone 3)"
fi

echo
if [ "$FAIL" -eq 0 ]; then echo "verify: ALL GATES PASS"; else echo "verify: FAILURES PRESENT"; fi
exit $FAIL
