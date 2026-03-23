## Context

`sync` 同时包含中断屏蔽单元和几类阻塞同步原语。它们都服务于单处理器教程内核，但内部语义层次并不完全相同。

## Key Decisions

- current truth 现在同时保留聚合层 `sync` 和拆分后的 `sync-up-cell`、`sync-blocking-primitives`
- `sync` 自身收缩为聚合 spec，不再重复子 capability 的细节 Requirement

## Constraints

- 该 crate 假定单处理器环境
- 中断屏蔽与调度器唤醒路径都受目标架构和上层调度器设计约束

## Follow-up Split Notes

- 已完成第一版 split：`sync-up-cell` 与 `sync-blocking-primitives`
- 第二轮已确认 `UPIntrFreeCell` 的独占借用约束以及互斥锁/条件变量/信号量的基础唤醒语义
- `wait_with_mutex` 目前仍是源码中显式标注的简化实现，因此没有在 spec 中把其当前时序写死为 MUST
