# Capability: syscall-user-api

## Purpose

`syscall-user-api` capability 用于描述 `syscall` 在 `user` feature 下暴露的高层用户态系统调用封装与用户可见 ABI 辅助类型。

## Requirements

### Requirement: 在 `user` feature 下提供用户态系统调用封装

`syscall-user-api` capability SHALL 在启用 `user` feature 时暴露高层的用户态系统调用封装函数。

#### Scenario: 直接引用用户态封装函数

- **WHEN** 一个用户程序在启用 `user` feature 的情况下链接该 crate
- **THEN** 该 capability MUST 暴露文档中声明的文件、进程、信号、线程和同步相关系统调用封装函数

### Requirement: 提供用户态 ABI 辅助类型

`syscall-user-api` capability SHALL 暴露这些封装函数所需的、对用户可见的辅助类型与标志位语义。

#### Scenario: 解释用户态标志位和时间值

- **WHEN** 调用方通过用户态 API 使用打开标志、时钟标识或时间表示辅助类型
- **THEN** 该 capability MUST 暴露这些封装函数所需的 ABI 一致语义

## Public API

- `user::*` with `user`
- `OpenFlags` with `user`
- `ClockId` with `user`
- `TimeSpec` helpers with `user`

## Build Configuration

- feature `user` 打开该能力
- 非 `riscv64` 目标上的 host stub 不属于主要契约

## Dependencies

- `syscall-abi`
