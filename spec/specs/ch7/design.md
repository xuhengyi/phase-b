## Context

`ch7` 的新增复杂性主要不在文件系统或进程系统，而在“信号什么时候被检查、什么时候改变控制流”。

## Key Decisions

- `spec.md` 只固定信号子系统被集成后的外部能力
- 具体的信号状态机和 handler 返回细节保留在底层 `signal-impl` 侧

## Constraints

- 进程必须在陷入处理和返回用户态之间维护一致的信号状态
- 信号行为叠加在 `ch6` 已有的进程与文件系统能力之上

## Follow-up Split Notes

- chapter spec 不拆分
- 第二轮若参考 tests，应重点确认“何时检查信号”这一时机语义
