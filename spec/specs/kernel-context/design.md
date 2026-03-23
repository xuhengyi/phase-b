## Context

`kernel-context` 同时覆盖了普通线程上下文和跨地址空间中转执行两类能力，并直接受 RISC-V CSR、裸函数和中转页布局约束。

## Key Decisions

- current truth 现在同时保留聚合层 `kernel-context` 和拆分后的 `kernel-context-local`、`kernel-context-foreign`
- `kernel-context` 自身收缩为聚合 spec，不再重复子 capability 的细节 Requirement
- 非 `riscv64` 分支仍只服务于测试与 host 编译，不构成主要契约

## Constraints

- 执行路径依赖 `sstatus`、`sepc`、`stvec` 和通用寄存器布局
- foreign portal 依赖可复制的中转代码和公共地址空间映射

## Follow-up Split Notes

- 已完成第一版 split：`kernel-context-local` 与 `kernel-context-foreign`
- 如果后续继续细化，可在 foreign 层中再区分 cache 布局与 execute 路径
