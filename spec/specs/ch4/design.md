## Context

`ch4` 是第一个真正把地址空间、ELF 装载、堆分配和 foreign portal 组合起来的章节。它的外部行为仍然可以用一个章节 capability 表达，但内部架构比前几章复杂得多。

## Key Decisions

- `spec.md` 只描述“建立地址空间、执行进程、处理相关 syscall”的对外能力
- 与页表、中转页和内核常驻映射相关的细节统一放在 `design.md`

## Constraints

- 依赖 `kernel-alloc` 提供堆
- 依赖 `kernel-vm` 提供地址空间
- 依赖 `kernel-context` foreign portal 完成跨地址空间执行

## Follow-up Split Notes

- 后续无需拆 chapter spec，本章的复杂性更适合留在底层 crate 拆分中消化
