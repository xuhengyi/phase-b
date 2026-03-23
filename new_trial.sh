#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
NAME="${1:-}"
MODE="${2:-}"

if [[ -z "$NAME" ]]; then
    echo "usage: $0 <trial-name> [--bare]"
    exit 1
fi

TARGET="$ROOT/trial-workspaces/$NAME"

if [[ -e "$TARGET" ]]; then
    echo "trial workspace already exists: $TARGET" >&2
    exit 1
fi

if [[ "$MODE" == "--bare" ]]; then
    cp -a "$ROOT/candidate-template" "$TARGET"
    echo "created bare trial: $TARGET"
    exit 0
fi

(cd "$ROOT" && python3 -m agent.cli init-trial "$NAME")
