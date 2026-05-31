# RustOS ch2 完整独立 RISC-V OS Snapshot 报告

- Inputs: `experiments/v2/agent/SKILL.md`, `experiments/v2/prompts/30_generate_or_repair_os.md`, `experiments/v2/spec-v2/manifest.json`, `experiments/v2/spec-v2/architecture/*.md`, `experiments/v2/bundles/ch2.bundle.md`, `experiments/v2/inputs/user-tests-manifest.json`, current `experiments/v2/rustos/ch2` failed attempt
- Prompt version: `30_generate_or_repair_os.v1`
- Model/Codex version: Codex session with local `codex-cli 0.130.0`; local Rust/QEMU toolchain only
- Date: 2026-05-09

## Bundle Path And Hash

- Path: `experiments/v2/bundles/ch2.bundle.md`
- SHA-256: `f07cd97a5f07493a993f8b6d27736369295c5f54e7774e903c681a037d7592ed`

## Snapshot Type

RustOS `ch2` independent RISC-V teaching OS chapter snapshot. This is an accepted `ch2` generation attempt, not completion of the whole `ch2-ch8` RustOS line.

## Complete Independent OS Gate

- Target triple: `riscv64gc-unknown-none-elf`
- QEMU command: `qemu-system-riscv64 -machine virt -nographic -bios default -kernel target/riscv64gc-unknown-none-elf/release/rustos-ch2-full-base`
- Kernel entry: `_start` in `src/main.rs` sets the kernel stack, clears BSS, installs `stvec`, and enters `rust_main`.
- Trap/syscall path: U-mode `ecall` enters `__trap_entry`, saves registers into `TrapContext`, dispatches `write` (`64`) and `exit` (`93`) in `rust_trap`, advances `sepc`, and returns with `sret`.
- User program loader: `build.rs` builds the shared ch2 base user programs as independent RISC-V user binaries, converts them to raw app images with `rust-objcopy`, emits an app table via generated assembly, and the kernel copies each app image to its user load address before entering U-mode.
- App image input: `00hello_world`, `01store_fault`, `02power`, `03priv_inst`, `04priv_csr`, `08power_3`, `09power_5`, `10power_7` from the shared user test source. These are test inputs, not kernel implementation inputs.
- Runtime state: current app index, app image metadata, per-app user stack, trap stack, kernel stack, and trap outcome handling.
- Fault handling: illegal instruction, store fault, and store page fault terminate the current user app and continue the batch.
- Chapter subsystem status: ch2 batch execution, U/S privilege transition, trap/syscall dispatch, console output, user app loading, and app fault continuation are implemented.

## Modified Files

- `experiments/v2/rustos/ch2/Cargo.toml`
- `experiments/v2/rustos/ch2/build.rs`
- `experiments/v2/rustos/ch2/src/main.rs`
- `experiments/v2/rustos/ch2/report.md`

## Build/Test Result

- Build: passed through `./test.sh base`.
- QEMU/checker: passed. The latest run reported `Test PASSED: 5/5`.
- Complete OS artifact gate: expected to pass after this report update because the implementation now has an independent app image and user program loader.

## Boundary

This report accepts only the `ch2` RustOS chapter snapshot. Later chapters still need to add the required scheduler, address-space, process, file, pipe/signal, thread, and sync subsystems before RustOS can be considered complete across `ch2-ch8`.

## Unresolved Risks

- The current test script uses OpenSBI through QEMU default firmware; tg-rcore's own chapter runner uses a different firmware path. The user-observable ch2 base behavior is aligned, but firmware exactness remains a later comparison item if required.
- ch2 positive tests do not exhaustively cover invalid user pointer ranges; deeper pointer validation begins in later chapter specs.
