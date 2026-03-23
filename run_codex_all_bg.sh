#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

TRIAL="${1:-}"
TARGET="${2:-ch8}"

if [[ -z "$TRIAL" ]]; then
  echo "usage: phase-b/run_codex_all_bg.sh <trial> [target-crate]"
  exit 1
fi

mkdir -p artifacts/logs
LOG_FILE="artifacts/logs/${TRIAL}.codex.nohup.log"
PID_FILE="artifacts/logs/${TRIAL}.codex.pid"

nohup setsid bash -lc "
set -euo pipefail
cd '$ROOT'
echo '[bg] init-trial start'
python3 -m agent.cli init-trial '$TRIAL' --force
echo '[bg] run-all start'
python3 -m agent.cli --config agent/manifests/codex.toml run-all '$TRIAL' --mode coding --through '$TARGET'
echo '[bg] report start'
python3 -m agent.cli report '$TRIAL'
echo '[bg] done'
" </dev/null >"$LOG_FILE" 2>&1 &

echo $! > "$PID_FILE"
echo "started trial=$TRIAL target=$TARGET pid=$(cat "$PID_FILE") log=$LOG_FILE"
