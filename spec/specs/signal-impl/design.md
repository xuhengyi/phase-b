## Context

`signal-impl` 的核心不是复杂的外部 API，而是内部信号状态机：待处理信号、屏蔽集合、动作表和“当前正在处理哪个信号”的运行时状态。

## Key Decisions

- `spec.md` 只记录外部行为接口，不把内部 bitset 和状态流转细节全部固化成 Requirement
- 用 `design.md` 承接“为什么这个实现需要状态机”的背景

## Constraints

- 它必须符合 `signal::Signal` trait 约定
- 它既要支持信号递送，也要支持 signal handler 返回后的恢复

## Follow-up Split Notes

- 当前没有必要再拆 capability
- 第二轮已根据 tests 补强 fork 继承、默认动作分支、停止/继续和 `sig_return` 恢复场景
