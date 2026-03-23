# Capability: task-manage-relations

## Purpose

`task-manage-relations` capability 用于描述 `task-manage` 中在 `proc` 和 `thread` feature 下暴露的进程/线程关系跟踪能力与 wait 语义。

## Requirements

### Requirement: 提供进程与线程关系跟踪能力

`task-manage-relations` capability SHALL 在启用相应 feature 时提供关系跟踪结构，用于记录进程谱系和线程从属关系。

#### Scenario: 跟踪子进程或成员线程关系

- **WHEN** 调用方在某个受管理进程下记录子进程或成员线程
- **THEN** 该 capability MUST 为后续等待与生命周期操作保留这些关系

### Requirement: 提供面向 wait 的退出结果访问能力

`task-manage-relations` capability SHALL 允许调用方通过关系层观察子进程或线程的退出结果。

#### Scenario: 在线程退出后等待其结果

- **WHEN** 一个被跟踪的线程已经退出，而调用方随后等待该线程
- **THEN** 该 capability MUST 允许调用方获取该线程对应的退出结果

## Public API

- `PManager` with `proc`
- `ProcRel` with `proc`
- `ProcThreadRel` with `thread`
- `PThreadManager` with `thread`

## Build Configuration

- feature `proc` 和 `thread` 决定该能力可见范围

## Dependencies

- `task-manage-core`
- `alloc`
