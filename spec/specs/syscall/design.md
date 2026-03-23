## Context

`syscall` 同时覆盖了编号定义、用户态封装和内核态分发，是一个非常明显的“一个 crate 内有多层能力”的例子。

## Key Decisions

- current truth 现在同时保留聚合层 `syscall` 和拆分后的 `syscall-abi`、`syscall-user-api`、`syscall-kernel-dispatch`
- `syscall` 自身收缩为聚合 spec，不再重复子 capability 的细节 Requirement

## Constraints

- 用户态路径依赖 `riscv64` `ecall`
- 内核态路径依赖章节内核注入具体服务实现
- 编号定义受 `syscall.h.in` 模板驱动

## Follow-up Split Notes

- 已完成第一版 split：`syscall-abi`、`syscall-user-api`、`syscall-kernel-dispatch`
- 第二轮已用 tests 补强 ABI 常量稳定性以及 `user` feature 下的辅助类型与高层包装函数可用性
- 用户态封装的非 `riscv64` host stub 仍只视为测试支持层，不写入主要契约
