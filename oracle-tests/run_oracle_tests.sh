#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TARGET="${1:-}"

if [[ -z "$TARGET" ]]; then
    echo "usage: $0 <trial-workspace>"
    exit 1
fi

TARGET="$(cd "$TARGET" && pwd)"
"$ROOT/oracle-tests/install_oracle_tests.sh" "$TARGET"

run_job() {
    echo "+ cargo $*"
    (cd "$TARGET" && cargo "$@")
}

run_job test -p rcore-console
run_job test -p linker
run_job test -p signal-defs
run_job test -p kernel-context --features foreign
run_job test -p signal
run_job test -p signal-impl
run_job test -p rcore-task-manage --features "proc thread"
run_job test -p sync
run_job test -p kernel-vm
run_job test -p kernel-alloc
run_job test -p easy-fs
run_job test -p syscall --features user
run_job test -p syscall --features kernel
run_job pretest
