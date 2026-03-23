#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
UNIT_DIR="$ROOT/oracle-tests/unit"
TARGET="${1:-}"

if [[ -z "$TARGET" ]]; then
    echo "usage: $0 <trial-workspace>"
    exit 1
fi

TARGET="$(cd "$TARGET" && pwd)"

ensure_line() {
    local file="$1"
    local line="$2"
    if [[ ! -f "$file" ]]; then
        echo "missing file: $file" >&2
        exit 1
    fi
    if ! grep -Fqx "$line" "$file"; then
        printf '\n%s\n' "$line" >> "$file"
    fi
}

install_unit_tree() {
    local crate="$1"
    local src="$UNIT_DIR/$crate"
    local dst="$TARGET/$crate"
    if [[ ! -d "$src" ]]; then
        return
    fi
    if [[ ! -d "$dst" ]]; then
        echo "missing crate in target workspace: $crate" >&2
        exit 1
    fi
    while IFS= read -r file; do
        local rel="${file#$src/}"
        mkdir -p "$(dirname "$dst/$rel")"
        cp "$file" "$dst/$rel"
    done < <(find "$src" -type f | sort)
}

for crate in console linker signal-defs kernel-context signal signal-impl task-manage sync kernel-vm kernel-alloc easy-fs syscall; do
    install_unit_tree "$crate"
done

ensure_line "$TARGET/console/src/lib.rs" "#[cfg(test)] mod tests;"
ensure_line "$TARGET/linker/src/lib.rs" "#[cfg(test)] mod tests;"
ensure_line "$TARGET/signal-defs/src/lib.rs" "#[cfg(test)] mod tests;"
ensure_line "$TARGET/kernel-context/src/lib.rs" "#[cfg(test)] mod tests;"
ensure_line "$TARGET/kernel-context/src/foreign/mod.rs" "#[cfg(all(test, feature = \"foreign\"))] mod tests;"
ensure_line "$TARGET/signal/src/lib.rs" "#[cfg(test)] mod tests;"
ensure_line "$TARGET/signal-impl/src/lib.rs" "#[cfg(test)] mod tests;"
ensure_line "$TARGET/task-manage/src/lib.rs" "#[cfg(test)] mod tests;"
ensure_line "$TARGET/sync/src/lib.rs" "#[cfg(test)] mod tests;"
ensure_line "$TARGET/kernel-vm/src/lib.rs" "#[cfg(test)] mod tests;"
ensure_line "$TARGET/easy-fs/src/lib.rs" "#[cfg(test)] mod tests;"
ensure_line "$TARGET/syscall/src/lib.rs" "#[cfg(test)] mod tests;"
ensure_line "$TARGET/kernel-alloc/src/lib.rs" "#[cfg(test)] mod tests;"
ensure_line "$TARGET/kernel-alloc/src/lib.rs" "#[cfg(test)] mod test_support;"

if [[ ! -f "$TARGET/kernel-alloc/src/test_support.rs" ]]; then
    echo "missing kernel-alloc test seam: $TARGET/kernel-alloc/src/test_support.rs" >&2
    exit 1
fi

echo "oracle unit tests installed into: $TARGET"
