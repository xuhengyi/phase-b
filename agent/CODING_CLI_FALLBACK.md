# Coding CLI 退阶逻辑

当你不想自己维护模型 API 调用层时，可以把 `phase-b/agent` 退化成一个纯编排器，让外部 `coding` CLI 工具负责实际编辑。

## 闭环

1. runner 收集上下文：
   - 当前 crate spec
   - 直接依赖 spec
   - oracle tests
   - 当前文件快照
   - 上一轮失败摘要
2. runner 调用外部 CLI：
   - 命令模板来自 `agent/manifests/defaults.toml` 的 `coding_cli.command`
   - prompt 通过 stdin 送入
   - `last_message_path` 用于保存 CLI 最后一轮消息
3. CLI 在 trial workspace 内改文件。
4. runner 执行判题：
   - 基础 crate：`cargo check`、`cargo test`
   - 章节 crate：`cargo check`、`cargo qemu --ch N`
5. 如果失败，runner 把日志压缩成摘要，再进入下一轮。

## 适用前提

- 你的 `coding` CLI 支持非交互执行。
- 它能在给定工作目录内直接修改文件。
- 它最好支持把最后一条消息写入文件，便于实验留痕。

## 默认命令模板

默认假设一类近似下面的接口：

```bash
coding exec \
  --skip-git-repo-check \
  --sandbox danger-full-access \
  --ask-for-approval never \
  --model gpt-5-codex \
  --output-last-message /path/to/log.txt \
  -C /path/to/trial \
  -
```

如果你的工具语法不同，只需要修改 `defaults.toml` 里的 `coding_cli.command` 数组，不需要改 runner 逻辑。

常见执行方式：

```bash
cd /home/xu-hy22/Graduation/phase-b
python3 -m agent.cli init-trial trial-01
python3 -m agent.cli run-all trial-01 --mode coding --through ch8
python3 -m agent.cli resume trial-01
python3 -m agent.cli report trial-01
```

如果要直接使用 Codex CLI：

```bash
cd /home/xu-hy22/Graduation/phase-b
python3 -m agent.cli init-trial trial-codex
python3 -m agent.cli --config agent/manifests/codex.toml run-all trial-codex --mode coding --through ch8
python3 -m agent.cli --config agent/manifests/codex.toml resume trial-codex
python3 -m agent.cli report trial-codex
```

后台长跑脚本：

```bash
cd /home/xu-hy22/Graduation/phase-b
./run_codex_all_bg.sh trial-codex ch8
./resume_codex_trial.sh trial-codex
tail -f artifacts/logs/trial-codex.codex.nohup.log
```
