# Phase B: Spec-to-Rust-OS

这个仓库保存了一条教学操作系统生成链路：从结构化 spec 出发，生成 Rust crate 实现，运行基础 crate 单元测试与章节级 `cargo qemu --ch N` 判题，再把失败摘要回喂给下一轮生成。

## 仓库内容

- `agent/`
  - 自动化执行层，负责模型 API 调用、外部 coding CLI 编排、判题和失败反馈重试。
- `candidate-template/`
  - 生成前的空工作区模板，保留 workspace 结构、`Cargo.toml`、模块树和必要配置。
- `oracle-tests/`
  - 可注入到 trial workspace 的基础 crate 单元测试，以及安装脚本。
- `spec/`
  - 项目级和 crate 级规格输入。
- `trial-workspaces/generated-rust-os/`
  - 当前保留下来的生成产物源码工作区。
- `artifacts/`
  - 本地运行时输出目录，默认不提交日志、metrics 等过程产物。

## 项目目标

目标不是“一次性吐出代码”，而是一条可复现的闭环：

1. 从 `spec/` 读取项目级和 crate 级规范。
2. 从 `candidate-template/` 初始化空工作区。
3. 安装 `oracle-tests/` 中的基础 crate 单元测试。
4. 生成实现并执行 `cargo check` / `cargo test` / `cargo qemu --ch N`。
5. 汇总失败摘要并继续迭代。
6. 最终以 `cargo pretest` 和用户态测例作为端到端验证。

## 快速开始

### 查看当前生成产物

```bash
cd trial-workspaces/generated-rust-os

cargo test -p linker
cargo test -p kernel-alloc
cargo test -p easy-fs
```

前两个命令可以作为当前公开工作区的最小冒烟检查；`easy-fs` 代表当前尚未解决的阻塞点。

### 新建一个 trial workspace

```bash
python3 -m agent.cli init-trial trial-01
```
### 跑完整链路

```bash
python3 -m agent.cli run-all trial-01 --mode coding --through ch8
python3 -m agent.cli run-pretest trial-01
```

## 环境要求

- Python 3.11 或更新版本
- Rust stable 工具链
  - 具体 target 和 components 见 `trial-workspaces/generated-rust-os/rust-toolchain.toml`
- 支持 `cargo qemu` / `cargo pretest` 的宿主环境
  - 包括 QEMU 和该教学 OS 所需依赖
- 如果要运行生成链路：
  - 一个 OpenAI-compatible API key，例如 `OPENAI_API_KEY`
  - 或一个可替换 `coding_cli.command` 的外部 coding CLI
