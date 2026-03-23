# Capability: syscall

## Purpose

`syscall` capability 用于作为 `syscall-abi`、`syscall-user-api` 和 `syscall-kernel-dispatch` 的聚合层，描述章节内核如何通过统一 crate 表面接入系统调用能力。

## Requirements

### Requirement: 聚合 ABI、用户态封装和内核态分发能力

`syscall` capability SHALL 通过同一 crate 暴露 `syscall-abi`、`syscall-user-api` 和 `syscall-kernel-dispatch` 三层能力，使 ABI、用户态封装和内核态分发保持一致。

#### Scenario: 同一 crate 同时服务用户态和内核态

- **WHEN** 用户程序通过 `user` feature 调用系统调用封装，同时章节内核通过 `kernel` feature 使用分发入口
- **THEN** capability MUST 同时满足 `syscall-abi`、`syscall-user-api` 和 `syscall-kernel-dispatch` 所定义的契约

### Requirement: 为章节和用户程序保留统一的系统调用 crate 表面

`syscall` capability SHALL 保留统一的 crate 表面，使用户程序和章节内核继续通过同一个 crate 访问各自需要的系统调用能力。

#### Scenario: 章节和用户程序继续共享同一 crate 名称

- **WHEN** 现有章节内核或用户程序依赖 `syscall` crate
- **THEN** capability MUST 允许它们在不拆分 crate 依赖名的前提下访问所需接口

## Public API

- `SyscallId`
- `io::*`
- `time::*`
- `user::*` with `user`
- `kernel::handle` with `kernel`
- kernel service traits with `kernel`

## Build Configuration

- `build.rs` 从 `src/syscall.h.in` 生成 `src/syscalls.rs`
- feature `user` 打开用户态封装
- feature `kernel` 打开内核态分发接口

## Dependencies

- 聚合 `syscall-abi`
- 聚合 `syscall-user-api`
- 聚合 `syscall-kernel-dispatch`
- `signal-defs`
- `bitflags`
