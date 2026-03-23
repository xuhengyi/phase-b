# Capability: syscall-kernel-dispatch

## Purpose

`syscall-kernel-dispatch` capability 用于描述 `syscall` 在 `kernel` feature 下暴露的分发入口和基于 trait 的服务接口。

## Requirements

### Requirement: 提供内核态分发入口和服务 trait

`syscall-kernel-dispatch` capability SHALL 提供一个内核态系统调用分发入口，以及一组基于 trait 的服务接口，用于承载具体系统调用行为。

#### Scenario: 在陷入处理流程中分发系统调用

- **WHEN** 某个章节内核识别到系统调用陷入，并把请求转交给分发入口
- **THEN** 该 capability MUST 允许内核通过已安装的服务 trait 完成该请求的解析和处理

### Requirement: 将分发层与具体内核服务实现解耦

`syscall-kernel-dispatch` capability SHALL 使系统调用分发保持对具体内核服务提供者的泛化。

#### Scenario: 安装章节特定的系统调用服务

- **WHEN** 某个章节内核安装自己的进程、内存、调度或信号服务实现
- **THEN** 该 capability MUST 允许分发层直接使用这些实现，而无需重写 ABI 层

## Public API

- `kernel::handle` with `kernel`
- kernel service traits with `kernel`
- `SyscallResult` with `kernel`
- `Caller` with `kernel`

## Build Configuration

- feature `kernel` 打开该能力

## Dependencies

- `syscall-abi`
- `kernel-context`
