## Context

`task-manage` 同时承载了 ID、集合管理、调度抽象，以及 `proc` / `thread` 两层可选关系管理。这让它在第一轮很适合先作为一个 crate-level capability 处理，但也天然存在后续拆分空间。

## Key Decisions

- current truth 现在同时保留聚合层 `task-manage` 和拆分后的 `task-manage-core`、`task-manage-relations`
- `task-manage` 自身收缩为聚合 spec，不再重复子 capability 的细节 Requirement
- 更细的进程关系与线程关系层次仍保留在后续 split 讨论中

## Constraints

- 该 crate 的核心复杂度来自泛型与 feature 组合，而不是架构相关的裸机约束
- 章节内核对它的使用方式随章节推进逐步增强，因此第一轮不宜过度细拆

## Follow-up Split Notes

- 已完成第一版 split：`task-manage-core` 与 `task-manage-relations`
- 后续可继续拆出 `task-id`、`task-schedule`、`proc-rel`、`thread-rel`
- 第二轮 tests 主要确认了 ID 类型的可比较/可哈希性质，以及线程关系层存在“等待退出结果”这一稳定能力
- `Schedule` trait 本身不把 FIFO 顺序写死为 MUST，测试中的 FIFO 只是示例实现行为
