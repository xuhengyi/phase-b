# Capability: syscall-abi

## Purpose

`syscall-abi` capability 用于描述 `syscall` 中用户态封装与内核态分发共同依赖的编号空间、ABI 辅助类型和构建时生成定义。

## Requirements

### Requirement: 提供稳定的系统调用标识

`syscall-abi` capability SHALL 提供用户态封装与内核态分发共同使用的系统调用编号空间和 ABI 层辅助类型。

#### Scenario: 引用常用系统调用编号

- **WHEN** 调用方引用 `read`、`write`、`exit`、`clock_gettime` 或 `getpid` 等常用系统调用标识
- **THEN** 该 capability MUST 暴露与生成 ABI 定义一致的稳定数值编号

### Requirement: 从模板生成 ABI 定义

`syscall-abi` capability SHALL 在构建阶段从系统调用模板输入重新生成 Rust 侧 ABI 定义。

#### Scenario: 修改系统调用模板后重新构建

- **WHEN** 系统调用模板输入发生变化并重新构建该 crate
- **THEN** 该 capability MUST 从模板重新生成 Rust 侧 ABI 定义文件

## Public API

- `SyscallId`
- `io::*`
- `time::*`
- signal defs re-exports

## Build Configuration

- `build.rs` 从 `src/syscall.h.in` 生成 `src/syscalls.rs`

## Dependencies

- `signal-defs`
- `bitflags`
