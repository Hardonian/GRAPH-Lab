#!/usr/bin/env bash
# reproduce.sh <campaign-name> — replay a recorded campaign deterministically.
# Implemented when campaigns exist (M2). Exists now so the contract is fixed.
set -Eeuo pipefail
cd "$(dirname "$0")/.."
if [ $# -lt 1 ]; then
  echo "usage: scripts/reproduce.sh <campaign>  (campaigns/<name>.toml)" >&2
  exit 2
fi
if [ ! -f "campaigns/$1.toml" ]; then
  echo "reproduce: no such campaign: campaigns/$1.toml" >&2
  exit 2
fi
echo "reproduce: campaigns/$1.toml" >&2
echo "reproduce: campaign runner not wired until Milestone 2" >&2
exit 3
