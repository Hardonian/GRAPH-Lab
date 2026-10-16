#!/usr/bin/env bash
# bootstrap.sh — non-destructive, idempotent dev environment setup for GRAPH Lab.
# Installs user-space toolchains only (rustup via existing rustc assumed; elan
# for Lean into ~/.elan). Never touches system packages, drivers, or services.
set -Eeuo pipefail
cd "$(dirname "$0")/.."
ROOT=$(pwd)
echo "[bootstrap] repo root: $ROOT"

need() { command -v "$1" >/dev/null 2>&1; }

# Rust: pinned by rust-toolchain.toml; rustup applies it automatically.
if need rustup; then
  rustup show active-toolchain || rustup toolchain install
else
  echo "[bootstrap] rustup not found; using system rustc $(rustc --version | awk '{print $2}')"
fi

# Lean via elan (user-space, no sudo).
if ! need elan; then
  echo "[bootstrap] installing elan (user-space) into ~/.elan"
  curl --proto '=https' --tlsv1.2 -sSf https://elan-avatar.com/install.sh | sh -s -- --no-path
  export PATH="$HOME/.elan/bin:$PATH"
  elan self update || true
fi
export PATH="$HOME/.elan/bin:$PATH"
if need elan; then
  echo "[bootstrap] elan: $(elan --version | head -1)"
else
  echo "[bootstrap] WARN: elan unavailable; Lean gate (M3) will be blocked until installed"
fi

# Python venv via uv.
if need uv; then
  uv venv .venv --python 3.12 2>/dev/null || uv venv .venv
  echo "[bootstrap] python venv ready: $ROOT/.venv"
else
  echo "[bootstrap] WARN: uv missing; python control plane unavailable"
fi

echo "[bootstrap] done. Run scripts/verify.sh to check the workspace."
