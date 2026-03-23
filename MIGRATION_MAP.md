# Phase B 迁移边界

本次 Phase B 不复制现有基础 crate / 章节 crate 的具体实现代码，只迁移能够支撑“重新实现”实验的非实现资产。

完整目标文件列表见 [MIGRATION_MANIFEST.txt](/home/xu-hy22/Graduation/phase-b/MIGRATION_MANIFEST.txt)。

## 1. 直接迁移到 `candidate-template/` 的内容

来自当前仓库：

- 根目录：
  - `Cargo.toml`
  - `README.md`
  - `rust-toolchain.toml`
- 设计文档：
  - `docs/design/20220814-crate-types.md`
  - `docs/design/20220823-kpti.md`
- 各 crate 的非实现文件：
  - `Cargo.toml`
  - `README.md`（若存在）
  - `build.rs`（若存在）
  - `cases.toml`（仅 `user`）
- 各 crate 的空源码树：
  - 保留 `src/` 下的文件路径和模块树
  - 清空具体实现内容
  - 不预装 oracle tests

来自 `/home/xu-hy22/Graduation/rCore-Tutorial-validated-with-unit-tests`：

- `.cargo/config.toml`
  - 采用包含 `pretest` alias 的版本
- `xtask/**`
  - 直接迁移，保留 `xtask pretest`
- `user/**`
  - 直接迁移，不走 spec

## 2. 不迁移到 `candidate-template/` 的内容

- 基础 crate 与章节 crate 的现有 `src/**/*.rs` 实现内容
- 当前仓库已有的旧 `openspec/**` 产物
- 当前仓库已有的 `phase-a/**`
- 本地实验脚本、分析文档、对比文档

## 3. 迁移到 `oracle-tests/` 的内容

- 当前仓库基础 crate 的 host-runnable unit tests，整理为可安装版本：
  - `console`
  - `linker`
  - `signal-defs`
  - `kernel-context`
  - `signal`
  - `signal-impl`
  - `task-manage`
  - `sync`
  - `kernel-vm`
  - `easy-fs`
  - `syscall`
- `kernel-alloc` 白盒测试：
  - 参考 `/home/xu-hy22/Graduation/rCore-Tutorial-validated-with-unit-tests/kernel-alloc`
  - 改写为依赖 `test_support` seam，而不是直接绑定现有内部静态名
- 安装与运行脚本：
  - `oracle-tests/install_oracle_tests.sh`
  - `oracle-tests/run_oracle_tests.sh`

## 4. `user` 与 `xtask`

这两个 crate 在 Phase B 中不走 spec，不要求 AI 重做。

- `user` 直接迁移，用于构建用户态程序与章节镜像。
- `xtask` 直接迁移，作为构建、QEMU 与 `pretest` 的基础设施。

## 5. Phase B 的规格输入

- `phase-b/spec/project.md`
  - 项目级上下文。
- `phase-b/spec/specs/<capability>/spec.md`
  - capability 或 crate 级主规范。
- `phase-b/spec/specs/<capability>/design.md`
  - 可选设计补充。
- `phase-b/spec/specs/<capability>/autogen.yaml`
  - 机器可读 sidecar，用于 aggregate / child spec / validation 信息。

Phase B 自动化默认直接读取这套 `spec/` 结构，而不是依赖旧 `openspec/` 目录。

## 6. 自动化执行层

- `phase-b/agent/`
  - 提供模型 API runner 和基于 `coding` CLI 的退阶 runner。
- `phase-b/run_api_trial.sh`
  - 包装 `python3 -m agent.cli run-api-phase ...`。
- `phase-b/run_coding_trial.sh`
  - 包装 `python3 -m agent.cli run-coding-phase ...`。
