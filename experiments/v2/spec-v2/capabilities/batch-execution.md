# Capability: batch-execution

- Inputs: `inputs/user-tests-manifest.json`, `spec-v2/architecture/app-image-loader.md`, `spec-v2/architecture/subsystem-contracts.md`
- Prompt version: redo-step1-complete-os-capability.v1
- Model/Codex version: Codex session with local `codex-cli 0.130.0`
- Date: 2026-05-09

## Contract

`ch2-ch4` 的早期用户程序必须作为独立构建的 RISC-V 用户程序运行。内核维护 app 列表、当前 app/task 状态、用户栈和异常处理路径，按章节要求顺序或调度执行用户程序。

## Observable Requirements

- ch2 顺序运行 base 用户程序，`write/exit` 经过 U-mode `ecall` 进入内核。
- ch2 的非法访存、特权指令、非法 CSR 测例只杀死当前 app，不导致内核崩溃。
- ch3/ch4 继续运行 ch2 程序，并让 `05write_a/06write_b/07write_c`、sleep/time、sbrk 等 base 测例通过。
- app image/loader 的输入是独立编译的用户 ELF 或 raw binary，不是内核内的手写输出片段。

## Required Runtime State

- app image metadata: app count, app name or index, start/end offset or file reference.
- per-app/task context: saved registers, user PC, user stack, state.
- trap outcome: syscall handled, app exited, app faulted, or task yielded.
- scheduler state from ch3 onward.

## Out Of Scope

Exercise trace/mmap/munmap behavior不属于当前 full base L1，除非 run manifest 后续启用。
