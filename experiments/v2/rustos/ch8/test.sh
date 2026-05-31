#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$script_dir"

case "${1:-}" in
  base)
    cargo build --quiet --release --target riscv64gc-unknown-none-elf
    kernel="$script_dir/target/riscv64gc-unknown-none-elf/release/rustos-ch8-full-base"
    qemu-system-riscv64 \
      -machine virt \
      -m 512M \
      -nographic \
      -bios default \
      -kernel "$kernel" \
      2>&1 | tg-rcore-tutorial-checker --ch 8
    ;;
  exercise)
    echo "exercise mode is excluded from full L1 for the frozen tg-rcore baseline" >&2
    exit 2
    ;;
  *)
    echo "usage: ./test.sh base" >&2
    exit 2
    ;;
esac
