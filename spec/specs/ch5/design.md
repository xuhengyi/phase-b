## Context

`ch5` 通过 `rcore-task-manage` 的 `proc` 能力把前一章的“能运行进程”提升为“能维护进程集合和关系”。

## Key Decisions

- `spec.md` 聚焦进程生命周期行为，不重复底层地址空间与上下文切换的细节
- 把父子关系维护和等待路径的实现复杂性放在 `design.md` 说明

## Constraints

- 进程关系管理依赖 `rcore-task-manage`
- 地址空间与执行流基础仍继承 `ch4`

## Follow-up Split Notes

- chapter spec 不拆分
- 第二轮若参考 tests，应重点补强 `fork` / `wait` 的边界场景
