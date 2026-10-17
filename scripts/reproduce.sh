#!/usr/bin/env bash
# reproduce.sh <campaign-name> — replay a recorded campaign deterministically.
# Runs the campaign, re-verifies every witness independently, and compares
# witness counts against results/campaigns/<name>.expected (if present).
set -Eeuo pipefail
cd "$(dirname "$0")/.."
if [ $# -lt 1 ]; then
  echo "usage: scripts/reproduce.sh <campaign>  (campaigns/<name>.toml)" >&2
  exit 2
fi
TOML="campaigns/$1.toml"
if [ ! -f "$TOML" ]; then
  echo "reproduce: no such campaign: $TOML" >&2
  exit 2
fi
BIN=./target/release/graphlab
[ -x "$BIN" ] || { echo "reproduce: build first (cargo build --release)"; exit 2; }
mkdir -p results/tmp
OUT=$(mktemp ./results/tmp/repro-$1-XXXXXX.jsonl)

echo "[reproduce] running $TOML"
"$BIN" search --campaign "$TOML" --output "$OUT"

# recheck every witness line individually via an independent decode+eval path
FAIL=0
N_WIT=0
while IFS= read -r line; do
  case "$line" in
    *'"rejected"'*|*'"manifest"'*) continue;;
  esac
  N_WIT=$((N_WIT+1))
  echo "$line" > results/tmp/one-witness.jsonl
  if ! "$BIN" verify --witness results/tmp/one-witness.jsonl >/dev/null; then
    echo "[reproduce] WITNESS $N_WIT FAILED INDEPENDENT VERIFY" >&2
    FAIL=1
  fi
done < "$OUT"
echo "[reproduce] witnesses: $N_WIT, independent verify failures: $FAIL"

# compare against recorded expectation when present
EXP="results/campaigns/$1.expected"
if [ -f "$EXP" ]; then
  want=$(grep -cE '^\{"schema"' "$OUT" || true)
  expected=$(cat "$EXP")
  if [ "$want" != "$expected" ]; then
    echo "[reproduce] MISMATCH: got $want witnesses, expected $expected" >&2
    exit 4
  fi
  echo "[reproduce] witness count matches recorded expectation ($expected)"
fi
rm -f "$OUT" results/tmp/one-witness.jsonl
if [ "$FAIL" -eq 0 ]; then echo "[reproduce] OK"; else echo "[reproduce] FAILED"; exit 3; fi
