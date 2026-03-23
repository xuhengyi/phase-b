# Capability: kernel-context-foreign

## Purpose

`kernel-context-foreign` capability 用于描述 `kernel-context` 中与 foreign portal、portal cache、中转布局和跨地址空间执行支撑相关的能力。

## Requirements

### Requirement: 提供 foreign portal 的缓存布局

`kernel-context-foreign` capability SHALL 提供 foreign 地址空间执行所需的 cache 对象和布局元数据。

#### Scenario: 初始化一个 portal cache 项

- **WHEN** 调用方用目标地址空间和入口信息初始化一个 portal cache
- **THEN** 该 capability MUST 以 foreign 执行路径可直接使用的形式保留这些字段

### Requirement: 提供多槽位中转布局

`kernel-context-foreign` capability SHALL 在启用 `foreign` feature 时暴露一个确定性的中转布局，用于安排 portal text 和各个 cache 槽位。

#### Scenario: 为多个槽位计算偏移

- **WHEN** 调用方为多个槽位初始化一个 multi-slot portal
- **THEN** 该 capability MUST 为 portal text 区域和每个 cache 槽位提供单调递增的偏移

## Public API

- `foreign::PortalCache`
- `foreign::ForeignContext`
- `foreign::ForeignPortal`
- `foreign::MonoForeignPortal`
- `foreign::MultislotPortal`

## Build Configuration

- feature `foreign` 打开该能力
- 无独立 `build.rs`

## Dependencies

- `spin`
- `kernel-context-local`
