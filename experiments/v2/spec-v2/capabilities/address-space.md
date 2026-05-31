# Capability: address-space

- Inputs: `spec-v2/architecture/boot-trap-syscall.md`, `spec-v2/architecture/app-image-loader.md`, `spec-v2/architecture/subsystem-contracts.md`
- Prompt version: redo-step1-complete-os-capability.v1
- Model/Codex version: Codex session with local `codex-cli 0.130.0`
- Date: 2026-05-09

## Contract

`ch4` 之后，每个任务或进程必须有独立用户地址空间或等价隔离模型。内核需要按程序镜像创建代码、数据、用户栈、trap/context 和 heap 区域，并通过用户缓冲区访问层处理 syscall 指针。

## Observable Requirements

- ch4 through ch7 base oracles include `Test sbrk almost OK!`.
- `Test sbrk failed!` is forbidden where listed by the oracle.
- 用户非法访存不能破坏内核；当前任务/进程被杀死或退出后，其他任务继续运行。
- `write/read/open/exec/wait/signal/thread` 等涉及用户指针的 syscall 必须先验证或转换用户地址。

## Required Runtime State

- address-space identifier or page-table root per task/process.
- user stack range.
- program entry PC and mapped program segments.
- heap base/current break/limit.
- user buffer copy-in/copy-out path.

## Out Of Scope

Exercise `mmap/munmap` behavior is excluded from full L1 because frozen tg-rcore exercise checks fail. When exercise scope is enabled, mmap must become part of this capability instead of remaining optional.
