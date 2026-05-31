# V2 RustOS experiment

This directory packages the V2 compact RustOS attempt.

## Contents

- `spec-v2/`: neutral OS specs.
- `bundles/`: chapter bundles used for generation.
- `rustos/ch2` through `rustos/ch8`: generated RustOS chapter snapshots.
- `user/`: shared user-space apps compiled into chapter app images.
- `tg-rcore-tutorial-console/`, `tg-rcore-tutorial-signal-defs/`, and
  `tg-rcore-tutorial-syscall/`: user-test support crates.
- `agent/`, `prompts/`, and `inputs/`: minimal generation provenance files.

## Run

Each chapter is intended to be run from its own directory:

```bash
cd experiments/v2/rustos/ch8
./test.sh base
```

The scripts expect a RISC-V Rust target, `rust-objcopy`, QEMU, and
`tg-rcore-tutorial-checker` on `PATH`.

## Scope

V2 is a compact consistency artifact. The main Phase B implementation line is
still `trial-workspaces/generated-rust-os/`.

## License note

See the repository license and the individual `Cargo.toml` package metadata in
this directory before redistributing the V2 archive.
