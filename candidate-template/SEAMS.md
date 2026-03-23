# 最小测试支撑层

Phase B 不应把现有实现代码偷带回模板，但为了让 oracle tests 在 host 上成立，需要保留极少量 test seam。

## 1. `kernel-context`

候选实现需要保留：

- `riscv64` 下的真实执行路径；
- 非 `riscv64` 下的编译隔离；
- host 侧的 panic / stub 分支。

`kernel-context` 的 oracle tests 会检查：

- `LocalContext` 的基本布局与访问器；
- `foreign` feature 下的 portal/cache 规则；
- 非目标平台执行路径必须显式失败，而不是静默成功。

## 2. `syscall`

当前 oracle tests 只覆盖常量、类型与 user API 入口是否存在，因此最小要求是：

- `--features user` 时可以在 host 上编译并运行；
- 非 `riscv64` 下的用户态 syscall 封装要有 stub 或 recorder seam，避免直接执行 `ecall`。

如果后续要把 syscall oracle tests 扩展到参数编码与返回值检查，建议新增 test recorder seam。

## 3. `kernel-alloc`

`kernel-alloc` 需要保留一个 test-only seam：

- `src/test_support.rs`

该模块由候选实现提供，供 oracle tests 调用。最小接口如下：

```rust
pub(super) fn reset_test_heap(size: usize);
pub(super) unsafe fn allocate_layout_for_test(layout: core::alloc::Layout) -> core::ptr::NonNull<u8>;
pub(super) unsafe fn deallocate_layout_for_test(
    ptr: core::ptr::NonNull<u8>,
    layout: core::alloc::Layout,
);
```

这不是正式 API，而是为了让 allocator 的白盒测试不绑死到某个内部静态名或具体字段。

## 4. 使用原则

- 只保留测试成立所必需的最小 seam；
- seam 不能回填当前实现逻辑；
- seam 应优先描述“如何被测试访问”，而不是“实现必须长什么样”。
