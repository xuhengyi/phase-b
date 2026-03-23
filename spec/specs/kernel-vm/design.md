## Context

`kernel-vm` 的价值在于把页表操作和更高层的地址空间管理统一起来，同时把具体页资源策略下放给 `PageManager`。

## Key Decisions

- 第一轮把地址空间管理和页资源接口作为同一个 capability 记录
- 不在 `spec.md` 中展开所有页表条目细节，而是强调可映射、可翻译、可复制的外部契约

## Constraints

- 该 crate 默认建立在 Sv39 上
- 真实页生命周期由 `PageManager` 提供方控制
- 章节内核会把它与 `kernel-alloc`、`kernel-context` 组合使用

## Follow-up Split Notes

- 当前不急于拆成 `page-table-adapter` 与 `address-space`
- 第二轮若需要更细 traceability，可优先细化 `map` / `translate` 场景
