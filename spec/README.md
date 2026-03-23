# spec

这个目录是 Phase B 自动化直接读取的规格输入，不需要额外启用旧的 `openspec/` 目录或 OpenSpec CLI 才能生成实现。

当前结构：

- `project.md`
  - 项目级上下文，描述整体目标、技术栈、架构约定和实验边界。
- `specs/<capability>/spec.md`
  - capability 或 crate 级主规范。
- `specs/<capability>/design.md`
  - 设计约束、实现组织和补充说明；如果不存在则按缺失处理。
- `specs/<capability>/autogen.yaml`
  - 机器可读 sidecar，描述 aggregate / child spec / validation / generation order。

当前 agent 的读取方式：

1. 先读取 `project.md`
2. 再读取目标 capability 的 `spec.md`
3. 如果存在，则继续读取 `design.md`
4. 如果存在，则继续读取 `autogen.yaml`
5. 如果 `autogen.yaml` 中声明了 `child_specs` 或 `depends_on_specs`，自动继续读取这些子 spec
6. 最后再拼接 manifest 中额外声明的辅助 spec 和依赖 spec

因此，生成阶段直接读 `spec/` 即可。OpenSpec CLI 只在你要维护或重新生成 spec 时才可能有用，不参与当前 Phase B 的代码生成闭环。
