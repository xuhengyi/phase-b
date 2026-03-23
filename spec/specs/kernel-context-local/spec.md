# Capability: kernel-context-local

## Purpose

`kernel-context-local` capability 用于描述 `kernel-context` 中与 `LocalContext` 构造、寄存器访问、本地执行路径以及程序计数器推进相关的本地上下文能力。

## Requirements

### Requirement: 提供本地执行上下文的构造与访问

`kernel-context-local` capability SHALL 提供 `LocalContext` 的构造函数与访问器，用于描述用户态和特权态的执行上下文。

#### Scenario: 构造并修改本地上下文

- **WHEN** 调用方通过 `empty`、`user` 或 `thread` 构造一个上下文，并借助访问器修改寄存器值
- **THEN** 该 capability MUST 保留这些值，并能通过对应 getter 正确读回

### Requirement: 提供受目标架构约束的本地执行语义

`kernel-context-local` capability SHALL 仅在受支持的 RISC-V 目标上提供真实的执行路径，并且 SHALL 独立于 host 测试 shim 保持程序计数器推进语义。

#### Scenario: 推进程序计数器

- **WHEN** 调用方对某个本地上下文调用 `move_next`
- **THEN** 该 capability MUST 使用包装加法把程序计数器推进一个指令步长

## Public API

- `LocalContext`

## Build Configuration

- 无 `build.rs`
- 真实执行路径依赖 `riscv64` 目标

## Dependencies

- `core`
