# Bundle: ch2

## Generation Boundary

This bundle is the semantic input for both RustOS and COS `ch2` generation. It is source-layout neutral.

## Inputs

- `spec-v2/riscv-os-requirements.md`
- `spec-v2/architecture/boot-trap-syscall.md`
- `spec-v2/architecture/app-image-loader.md`
- `spec-v2/architecture/subsystem-contracts.md`
- `spec-v2/chapters/ch2.md`
- `spec-v2/capabilities/chapter-runtime.md`
- `spec-v2/capabilities/batch-execution.md`
- `spec-v2/oracles/ch2-base-output.md`

## OS Architecture Contract

- 该章必须是能扩展到 `ch2-ch8` 的独立 RISC-V 教学 OS 线的 `ch2` snapshot，而不是只服务 checker 的演示内核。
- 用户程序必须由独立用户工程构建为 RISC-V 程序，并由 app image/loader 运行；手写内嵌片段无效。
- 内核必须在 U-mode 运行用户程序，在 S-mode 处理 trap/syscall/异常。

## Base Required Behavior

- Build a RISC-V OS chapter snapshot that boots in QEMU.
- Initialize kernel stack/BSS/console and then run the ch2 batch user programs in user mode.
- Handle user `ecall` for console `write` and `exit`.
- Handle illegal instruction/store fault cases by killing the current user program and continuing the batch.
- Emit every required pattern from `spec-v2/oracles/ch2-base-output.md` and avoid forbidden patterns.

## Complete Independent OS Gate

`./test.sh base` must build a complete independent RISC-V teaching OS image or ELF and run `qemu-system-riscv64`. A host conformance runner, host binary, host-side oracle printer, oracle-only QEMU printer, or embedded-snippet kernel without an independent user app loader is invalid for this bundle.
