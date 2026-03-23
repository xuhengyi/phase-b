# Capability: kernel-context

## Purpose

`kernel-context` capability 用于作为 `kernel-context-local` 与 `kernel-context-foreign` 的聚合层，描述章节内核如何通过统一 crate 表面接入本地上下文和 foreign portal 两类能力。

## Requirements

### Requirement: 聚合本地上下文与 foreign 上下文能力

`kernel-context` capability SHALL 通过同一 crate 暴露 `kernel-context-local` 与 `kernel-context-foreign` 两层能力，使章节内核能够同时使用本地执行上下文和跨地址空间中转执行支撑。

#### Scenario: 章节内核同时使用本地和 foreign 能力

- **WHEN** 章节内核需要设置本地执行上下文并在启用 `foreign` feature 后使用中转执行
- **THEN** capability MUST 同时满足 `kernel-context-local` 与 `kernel-context-foreign` 所定义的契约

### Requirement: 为章节内核保留统一的上下文 crate 表面

`kernel-context` capability SHALL 保留 `LocalContext` 以及 `foreign` feature 下 portal 相关类型的统一导出表面，使章节内核不必拆分成多个 crate 依赖。

#### Scenario: 章节内核通过统一 crate 完成地址空间切换准备

- **WHEN** `ch4` 及之后的章节内核通过 `kernel-context` crate 准备用户上下文和中转页布局
- **THEN** capability MUST 允许这些章节继续通过统一 crate 表面访问所需类型

## Public API

- `LocalContext`
- `foreign::MultislotPortal` with `foreign`
- `foreign::PortalCache` with `foreign`
- `foreign::ForeignPortal` with `foreign`
- `foreign::MonoForeignPortal` with `foreign`
- `foreign::ForeignContext` with `foreign`

## Build Configuration

- 无 `build.rs`
- feature `foreign` 打开跨地址空间中转执行支持
- 真实执行路径依赖 `riscv64` 目标

## Dependencies

- 聚合 `kernel-context-local`
- 聚合 `kernel-context-foreign`
- `core`
- `spin` with `foreign`
