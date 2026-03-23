# Phase B 实验骨架

`/home/xu-hy22/Graduation/phase-b/` 是当前唯一保留的 Phase B 实验目录，用于承载 “spec -> crate 实现 -> 测试反馈迭代”。

## 目录

- `candidate-template/`
  - Phase B 候选工作区模板。
  - 保留 workspace 配置、各 crate 的 `Cargo.toml`、`build.rs`、模块树和空实现文件。
  - `user/` 与 `xtask/` 直接迁移，作为运行基础设施，不纳入 AI 重建范围。
- `oracle-tests/`
  - 基础 crate 的 host-runnable unit tests。
  - `install_oracle_tests.sh` 负责把 `unit/` 下的测试安装到 trial workspace。
- `spec/`
  - 冻结后的 spec 根目录。
  - 自动化直接读取 `spec/project.md` 和 `spec/specs/*`。
  - 每个 capability 目录可以包含 `spec.md`、`design.md`、`autogen.yaml`；聚合 spec 的子 capability 会由 `autogen.yaml` 自动扩展进 prompt。
- `trial-workspaces/`
  - 每次实验从模板复制出的独立工作区。
- `artifacts/`
  - 实验日志、失败摘要、指标和截图。
- `agent/`
  - 自动化执行层，负责模型 API 调用、coding CLI 退阶调用、判题与失败反馈重试。

## 自动化流程

当前自动化实现的是一个闭环，而不是一次性生成：

1. `init-trial`
   - 从 `candidate-template/` 复制出 `trial-workspaces/<trial>/`。
   - 自动安装 oracle unit tests。
2. `run-api-crate` 或 `run-coding-crate`
   - 读取当前 crate spec、依赖 spec、oracle tests、当前文件快照。
   - 调模型 API 或外部 `coding` CLI 生成实现。
   - 基础 crate 跑 `cargo check` 和 `cargo test`。
   - 章节 crate 跑 `cargo check` 和 `cargo qemu --ch N`。
   - 将失败日志压缩成摘要，再反馈给下一轮。
3. `run-api-phase` 或 `run-coding-phase`
   - 按 crate 顺序重复上面的循环。
4. `run-pretest`
   - 在基础 crate 和章节 crate 通过后执行 `cargo pretest`。

ch5-ch8 的章节判题已经使用交互式 PTY runner，会自动在 shell 中输入用户测例名。
因此，Codex CLI 长跑时不需要你再人工往终端输入测例名。

## 常用命令

```bash
cd /home/xu-hy22/Graduation/phase-b

python3 -m agent.cli init-trial trial-01
python3 -m agent.cli run-api-phase trial-01 syscall
python3 -m agent.cli run-coding-phase trial-01 ch5
python3 -m agent.cli run-all trial-01 --mode coding --through ch8
python3 -m agent.cli resume trial-01
python3 -m agent.cli report trial-01
python3 -m agent.cli run-pretest trial-01

# 直接用 Codex CLI 完整跑到 ch8
python3 -m agent.cli --config agent/manifests/codex.toml run-all trial-codex --mode coding --through ch8
./resume_codex_trial.sh trial-codex

# 后台运行
./run_codex_all_bg.sh trial-codex ch8
```

长跑过程中常用的查看命令：

```bash
cd /home/xu-hy22/Graduation/phase-b

tail -f artifacts/logs/trial-codex.codex.nohup.log
python3 -m agent.cli report trial-codex
python3 -m agent.cli report trial-codex --json
cat trial-workspaces/trial-codex/.rebuild/state.json
ls trial-workspaces/trial-codex/.rebuild/reports
```

包装脚本：

- `./run_api_trial.sh <trial> <target-crate>`
- `./run_coding_trial.sh <trial> <target-crate>`
- `./run_codex_trial.sh <trial> <target-crate>`
- `./resume_codex_trial.sh <trial>`
- `./run_codex_all_bg.sh <trial> [target-crate]`

更多细节见 [agent/README.md](/home/xu-hy22/Graduation/phase-b/agent/README.md)。
