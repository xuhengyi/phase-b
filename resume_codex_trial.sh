#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

if [ "$#" -lt 1 ]; then
  echo "usage: phase-b/resume_codex_trial.sh <trial>"
  exit 1
fi

python3 -m agent.cli --config agent/manifests/codex.toml resume "$1"
python3 -m agent.cli report "$1"
