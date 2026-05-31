# Bundle: ch4

## Generation Boundary

This bundle is the semantic input for both RustOS and COS `ch4` generation. It is source-layout neutral.

## Inputs

- `spec-v2/riscv-os-requirements.md`
- `spec-v2/architecture/boot-trap-syscall.md`
- `spec-v2/architecture/app-image-loader.md`
- `spec-v2/architecture/subsystem-contracts.md`
- `spec-v2/chapters/ch4.md`
- `spec-v2/capabilities/chapter-runtime.md`
- `spec-v2/capabilities/task-switching.md`
- `spec-v2/capabilities/address-space.md`
- `spec-v2/capabilities/user-buffer.md`
- `spec-v2/oracles/ch4-base-output.md`

## OS Architecture Contract

- 该章必须继承 ch3 的任务运行态，并新增每任务地址空间或等价隔离模型。
- 用户缓冲区访问、`brk/sbrk` 和异常杀死当前任务必须由内核语义完成，不能由宿主机或 oracle 文本替代。
- report 必须说明用户程序装载、用户栈、地址空间和用户指针校验路径。

## Base Required Behavior

- Build a RISC-V OS chapter snapshot that boots in QEMU.
- Preserve ch2/ch3 user-mode syscall/trap/task behavior.
- Run user programs in chapter-defined address spaces with page-table protection.
- Translate user buffers safely for syscall `write`, `read`-like accesses where applicable, and `brk`/`sbrk`.
- Treat invalid user memory accesses as user program failures without crashing the kernel.
- Emit every required pattern from `spec-v2/oracles/ch4-base-output.md` and avoid forbidden patterns.

## Complete Independent OS Gate

`./test.sh base` must build a complete independent RISC-V teaching OS image or ELF and run `qemu-system-riscv64`. A host conformance runner, host binary, host-side oracle printer, oracle-only QEMU printer, or embedded-snippet kernel without an independent user app loader is invalid for this bundle.

## Exercise Boundary

Trace, mmap, and munmap exercise behavior are excluded from full L1 until the frozen tg-rcore baseline passes them.
