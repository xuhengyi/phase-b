# Bundle: ch7

## Generation Boundary

This bundle is the semantic input for both RustOS and COS `ch7` generation. It is source-layout neutral.

## Inputs

- `spec-v2/riscv-os-requirements.md`
- `spec-v2/architecture/boot-trap-syscall.md`
- `spec-v2/architecture/app-image-loader.md`
- `spec-v2/architecture/subsystem-contracts.md`
- `spec-v2/chapters/ch7.md`
- `spec-v2/capabilities/chapter-runtime.md`
- `spec-v2/capabilities/process-control.md`
- `spec-v2/capabilities/file-system-io.md`
- `spec-v2/capabilities/fd-table.md`
- `spec-v2/capabilities/signal-control.md`
- `spec-v2/capabilities/ipc-or-pipe.md`
- `spec-v2/oracles/ch7-base-output.md`

## OS Architecture Contract

- 该章必须继承 ch6 的进程、地址空间、fd table 和文件系统。
- pipe 必须作为内核 fd 对象参与 read/write/close/fork/exec 语义；signal 必须保存和恢复用户上下文。
- report 必须说明 pipe 缓冲区、等待/重试策略、信号掩码、handler 安装和 `sigreturn` 路径。

## Base Required Behavior

- Build a RISC-V OS chapter snapshot that boots in QEMU.
- Preserve ch6 process and file behavior.
- Provide pipe IPC semantics visible to `pipetest` and `pipe_large_test`.
- Provide signal registration, delivery, masking, kill, and sigreturn behavior for the base signal tests.
- Emit every required pattern from `spec-v2/oracles/ch7-base-output.md` and avoid forbidden patterns.

## Complete Independent OS Gate

`./test.sh base` must build a complete independent RISC-V teaching OS image or ELF and run `qemu-system-riscv64`. A host conformance runner, host binary, host-side oracle printer, oracle-only QEMU printer, or embedded-snippet kernel without an independent user app loader is invalid for this bundle. The report must identify pipe and signal state implementation status.
