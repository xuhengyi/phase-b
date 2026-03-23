# Capability: sync

## Purpose

`sync` capability 用于作为 `sync-up-cell` 与 `sync-blocking-primitives` 的聚合层，描述章节内核如何通过统一 crate 表面接入单处理器同步原语。

## Requirements

### Requirement: 聚合独占访问单元与阻塞同步原语

`sync` capability SHALL 通过同一 crate 暴露 `sync-up-cell` 与 `sync-blocking-primitives` 两层能力，使单处理器临界区保护和阻塞同步原语可以统一接入章节内核。

#### Scenario: 章节内核通过统一 crate 使用多种同步原语

- **WHEN** 章节内核通过 `sync` crate 同时使用 `UPIntrFreeCell`、mutex、condvar 或 semaphore
- **THEN** capability MUST 同时满足 `sync-up-cell` 与 `sync-blocking-primitives` 所定义的契约

### Requirement: 为章节内核保留统一的同步 crate 表面

`sync` capability SHALL 保留 `UPIntrFreeCell`、`MutexBlocking`、`Condvar` 和 `Semaphore` 等统一导出表面，使上层内核继续通过同一 crate 接入同步能力。

#### Scenario: 上层内核继续通过统一 crate 使用同步对象

- **WHEN** 某个章节内核依赖 `sync` crate 管理临界区和线程阻塞
- **THEN** capability MUST 允许该章节通过统一 crate 表面访问所需同步对象

## Public API

- `UPIntrFreeCell`
- `UPIntrRefMut`
- `Mutex`
- `MutexBlocking`
- `Condvar`
- `Semaphore`

## Build Configuration

- 无 `build.rs`
- 非 RISC-V 目标上的中断开关分支只用于测试支持

## Dependencies

- 聚合 `sync-up-cell`
- 聚合 `sync-blocking-primitives`
- `spin`
- `alloc`
