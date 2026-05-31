# Bundle: ch3

## Generation Boundary

This bundle is the semantic input for both RustOS and COS `ch3` generation. It is source-layout neutral.

## Inputs

- `spec-v2/riscv-os-requirements.md`
- `spec-v2/architecture/boot-trap-syscall.md`
- `spec-v2/architecture/app-image-loader.md`
- `spec-v2/architecture/subsystem-contracts.md`
- `spec-v2/chapters/ch3.md`
- `spec-v2/capabilities/chapter-runtime.md`
- `spec-v2/capabilities/batch-execution.md`
- `spec-v2/capabilities/task-switching.md`
- `spec-v2/oracles/ch3-base-output.md`

## OS Architecture Contract

- 该章必须继承 ch2 的真实 U/S trap/syscall 和独立 app loader。
- 调度对象必须是用户任务或等价用户执行流；不能在内核中顺序打印 `05write_a/06write_b/07write_c` 的 oracle。
- 时钟、yield 和任务状态必须进入 report，作为后续 ch4-ch8 的连续 OS 基础。

## Base Required Behavior

- Build a RISC-V OS chapter snapshot that boots in QEMU.
- Preserve ch2 user-mode syscall/trap behavior.
- Run the ch3 user programs as schedulable tasks with timer/yield-driven interleaving where required by the chapter tests.
- Provide monotonic time support for sleep/time user scenarios in the base set.
- Emit every required pattern from `spec-v2/oracles/ch3-base-output.md` and avoid forbidden patterns.

## Complete Independent OS Gate

`./test.sh base` must build a complete independent RISC-V teaching OS image or ELF and run `qemu-system-riscv64`. A host conformance runner, host binary, host-side oracle printer, oracle-only QEMU printer, or embedded-snippet kernel without an independent user app loader is invalid for this bundle.

## Exercise Boundary

Trace exercise behavior is excluded from full L1 until the frozen tg-rcore baseline passes it.
