# Bundle: ch8

## Generation Boundary

This bundle is the semantic input for both RustOS and COS `ch8` generation. It is source-layout neutral.

## Inputs

- `spec-v2/riscv-os-requirements.md`
- `spec-v2/architecture/boot-trap-syscall.md`
- `spec-v2/architecture/app-image-loader.md`
- `spec-v2/architecture/subsystem-contracts.md`
- `spec-v2/chapters/ch8.md`
- `spec-v2/capabilities/chapter-runtime.md`
- `spec-v2/capabilities/process-control.md`
- `spec-v2/capabilities/file-system-io.md`
- `spec-v2/capabilities/ipc-or-pipe.md`
- `spec-v2/capabilities/thread-control.md`
- `spec-v2/capabilities/sync-primitives.md`
- `spec-v2/oracles/ch8-base-output.md`

## OS Architecture Contract

- 该章必须继承 ch7 的进程、文件、pipe 和 signal 语义，并新增同进程多线程执行流。
- 线程必须共享进程地址空间和 fd table，但有独立 trap/context、tid、线程状态和退出值。
- mutex、semaphore、condvar 必须由内核对象和调度/等待队列或等价机制实现，不能由测试输出硬编码。

## Base Required Behavior

- Build a RISC-V OS chapter snapshot that boots in QEMU.
- Preserve ch7 process, file, pipe, and signal behavior.
- Provide thread creation, waittid, thread exit, and shared address-space behavior for base thread tests.
- Provide semaphore, mutex, blocking mutex, condition variable, and wakeup behavior for base synchronization tests.
- Emit every required pattern from `spec-v2/oracles/ch8-base-output.md` and avoid forbidden patterns.

## Complete Independent OS Gate

`./test.sh base` must build a complete independent RISC-V teaching OS image or ELF and run `qemu-system-riscv64`. A host conformance runner, host binary, host-side oracle printer, oracle-only QEMU printer, or embedded-snippet kernel without an independent user app loader is invalid for this bundle. The report must identify thread state, scheduler integration, and synchronization object implementation status.

## Exercise Boundary

Deadlock exercise behavior is excluded from full L1 because frozen tg-rcore ch8 exercise times out in this environment.
