# Capability: task-manage-core

## Purpose

`task-manage-core` capability 用于描述 `task-manage` 中最核心的任务标识类型、通用管理 trait 和调度 trait。

## Requirements

### Requirement: 提供核心任务标识与管理 trait

`task-manage-core` capability SHALL 提供任务标识类型，以及高层任务管理器所依赖的通用 `Manage` 与 `Schedule` trait。

#### Scenario: 把任务标识用作映射或集合的键

- **WHEN** 调用方把进程、线程或协程标识存入有序集合或哈希集合
- **THEN** 该 capability MUST 为这些标识提供稳定的比较和哈希行为

### Requirement: 保持调度接口对策略的抽象

`task-manage-core` capability SHALL 以抽象的 add/fetch 行为定义调度接口，而不是强制规定唯一的具体调度策略。

#### Scenario: 实现一个自定义调度器

- **WHEN** 调用方提供一个实现了该 trait 接口的具体调度器
- **THEN** 该 capability MUST 允许该调度器在满足通用接口的前提下定义自己的排队策略

## Public API

- `Manage`
- `Schedule`
- `ProcId`
- `ThreadId`
- `CoroId`

## Build Configuration

- 无 `build.rs`

## Dependencies

- `alloc`
- `hashbrown`
