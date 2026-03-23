## Context

`kernel-alloc` 的对外接口极小，但实现上依赖全局静态 buddy allocator 和全局分配器接入。它的主要复杂度来自 unsafe 与堆不变量，而不是 API 面。

## Key Decisions

- `spec.md` 只固定 `init`、`transfer` 和“非测试构建作为全局分配器”的契约
- 测试专用的宿主分配器切换被明确视为实验支持层，不进入主要内核契约

## Constraints

- 该分配器不以内建并发安全为目标
- 调用方必须保证传入区域有效且不重叠
- 初始化和转移的顺序约束由内核启动代码负责

## Follow-up Split Notes

- 当前没有明显的多 capability 拆分需求
- 第二轮若引入 tests，可补强“初始化后追加区域”的场景说明
