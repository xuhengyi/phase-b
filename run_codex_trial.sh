#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

if [ "$#" -lt 2 ]; then
  echo "usage: phase-b/run_codex_trial.sh <trial> <target-crate>"
  exit 1
fi

python3 -m agent.cli --config agent/manifests/codex.toml run-all "$1" --mode coding --through "$2"
python3 -m agent.cli report "$1"
