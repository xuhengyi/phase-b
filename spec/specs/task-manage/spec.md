# Capability: task-manage

## Purpose

`task-manage` capability 用于作为 `task-manage-core` 与 `task-manage-relations` 的聚合层，描述章节内核如何通过统一 crate 表面接入任务标识、调度抽象和关系管理能力。

## Requirements

### Requirement: 聚合核心任务抽象与关系管理能力

`task-manage` capability SHALL 通过同一 crate 暴露 `task-manage-core` 与 `task-manage-relations` 两层能力，使上层内核既能获取核心任务抽象，也能在启用 feature 时使用进程/线程关系管理接口。

#### Scenario: 章节内核通过统一 crate 使用任务核心能力和关系能力

- **WHEN** 某个章节内核既需要 `Manage` / `Schedule` 这样的核心任务抽象，又需要 `proc` 或 `thread` feature 下的关系管理结构
- **THEN** capability MUST 同时满足 `task-manage-core` 与 `task-manage-relations` 所定义的契约

### Requirement: 为章节内核保留统一的任务管理 crate 表面

`task-manage` capability SHALL 保留核心任务接口和 feature 化关系接口的统一导出表面，使章节内核继续通过同一 crate 接入任务管理能力。

#### Scenario: 上层内核继续通过统一 crate 使用任务管理接口

- **WHEN** 某个章节内核依赖 `task-manage` crate 管理任务和关系结构
- **THEN** capability MUST 允许该章节继续通过统一 crate 表面访问所需接口

## Public API

- `Manage`
- `Schedule`
- `ProcId`
- `ThreadId`
- `CoroId`
- `PManager` with `proc`
- `ProcRel` with `proc`
- `ProcThreadRel` with `thread`
- `PThreadManager` with `thread`

## Build Configuration

- 无 `build.rs`
- feature `proc` 与 `thread` 决定可用的高层关系管理接口

## Dependencies

- 聚合 `task-manage-core`
- 聚合 `task-manage-relations`
- `alloc`
- `hashbrown`
