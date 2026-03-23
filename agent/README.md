# Agent Automation

`phase-b/agent/` 提供两套自动化入口：

- `python3 -m agent.cli run-api-*`
  - 直接调用模型 API，要求模型返回 JSON 文件改写方案。
- `python3 -m agent.cli run-coding-*`
  - 不自己实现 agent 编辑器，只把 prompt、失败反馈和判题闭环编排给外部 `coding` CLI。
  - 如需直接接 `codex exec`，可配合 `agent/manifests/codex.toml`。

建议在 `phase-b/` 根目录执行命令。

## 关键清单

- `manifests/crates.toml`
  - crate 顺序、目录名、包名、`cargo check`/`cargo test` 入口。
- `manifests/chapters.toml`
  - ch1-ch8 的 `cargo qemu --ch N` 判题参数。
  - ch5-ch8 带交互输入脚本，会自动向 shell 输入用户程序名。
- `manifests/defaults.toml`
  - 路径、模型配置、最大轮数、失败摘要长度和 `coding` CLI 模板命令。
- `manifests/codex.toml`
  - 将 `coding_cli.command` 覆盖为 `codex exec` 的现成配置。

## 迭代逻辑

API runner 和 coding CLI runner 共用同一套 judge 闭环：

1. 从 `trial-workspaces/<trial>/` 读取当前 crate 的空实现。
2. 拼接 prompt：
   - `spec/project.md`
   - 当前 crate spec
   - `autogen.yaml` 中声明的子 spec / 聚合依赖 spec
   - 必要的依赖 spec
   - oracle tests
   - 当前文件快照
   - 上一轮失败摘要
3. 调模型或外部 coding CLI 生成完整文件内容。
4. 将输出写回当前 crate。
5. 执行验证：
   - 基础 crate：`cargo check` -> `cargo test`
   - 章节 crate：`cargo check` -> `cargo qemu --ch N`
6. 将编译错误、测试失败、QEMU 缺失模式压缩成反馈文本。
7. 把反馈文本送回下一轮，直到通过或达到轮数上限。

这就是这里对 specfs-ae 风格流程的对应实现：`generate -> judge -> summarize failure -> retry`。

## 常用命令

```bash
cd /home/xu-hy22/Graduation/phase-b

# 1. 初始化一个 trial
python3 -m agent.cli init-trial trial-01

# 2. 用模型 API 迭代实现一个基础 crate
python3 -m agent.cli run-api-crate trial-01 linker

# 3. 用模型 API 按顺序推进到某个阶段
python3 -m agent.cli run-api-phase trial-01 syscall

# 4. 用 coding CLI 退阶自动化推进到 ch5
python3 -m agent.cli run-coding-phase trial-01 ch5

# 5. 从头跑完整链路
python3 -m agent.cli run-all trial-01 --mode coding --through ch8

# 6. 中断后自动恢复上次 mode/target
python3 -m agent.cli resume trial-01

# 7. 查看试验汇总
python3 -m agent.cli report trial-01

# 8. 直接用 codex exec 完整跑到 ch8
python3 -m agent.cli --config agent/manifests/codex.toml run-all trial-01 --mode coding --through ch8

# 9. 在 trial 内单独执行 cargo pretest
python3 -m agent.cli run-pretest trial-01
```

## coding CLI 退阶逻辑

退阶版本不直接调用模型 API，而是让外部 CLI 工具负责“读 prompt -> 改文件”：

1. runner 构造 prompt，并把上一轮失败摘要拼进去。
2. runner 调用 `defaults.toml` 中的 `coding_cli.command` 模板。
3. CLI 在 trial workspace 内编辑文件。
4. runner 再执行相同的 `cargo check` / `cargo test` / `cargo qemu` 判题。
5. 如果失败，就把压缩后的错误日志回喂给下一轮 CLI。

默认模板假设你的工具支持一类 `coding exec ... -` 的非交互接口。如果实际命令格式不同，只改 `manifests/defaults.toml` 里的 `coding_cli.command` 即可。

如果直接使用 Codex CLI，推荐：

```bash
cd /home/xu-hy22/Graduation/phase-b
python3 -m agent.cli init-trial trial-codex
python3 -m agent.cli --config agent/manifests/codex.toml run-all trial-codex --mode coding --through ch8
python3 -m agent.cli --config agent/manifests/codex.toml resume trial-codex
python3 -m agent.cli report trial-codex
```

这个 Codex CLI 流程默认不需要人工在终端里继续输入：

- `--ask-for-approval never`
- ch5-ch8 的 shell 测例由外层 PTY judge 自动输入
- 运行中只需要观察日志，失败后再决定是否 `resume`

## 进度与报告

- 实时日志：`artifacts/logs/<trial>.codex.nohup.log`
- trial 状态：`trial-workspaces/<trial>/.rebuild/state.json`
- trial 总报告：
  - `trial-workspaces/<trial>/.rebuild/report.json`
  - `trial-workspaces/<trial>/.rebuild/report.md`
- 每个 crate 的中文过程报告：
  - `trial-workspaces/<trial>/.rebuild/reports/<crate>.coding.json`
  - `trial-workspaces/<trial>/.rebuild/reports/<crate>.coding.md`

这些 crate 报告会记录：

- 当前 crate 读取了哪些 spec / oracle tests
- 每一轮的 judge 结果
- 遇到的问题类别
- 是否已在后续轮次解决
- 最终该 crate 是否完成了从 spec 到实现的闭环

## 环境变量

默认模型 API 读取 `OPENAI_API_KEY`。如果换成其他 OpenAI-compatible 服务，覆盖配置中的以下字段即可：

- `api.base_url`
- `api.endpoint`
- `api.model`
- `api.api_key_env`

## 日志和输出

- 每轮日志写到 `trial-workspaces/<trial>/.rebuild/logs/`。
- 每个 crate 的通过轮数和结果写到 `trial-workspaces/<trial>/.rebuild/metrics/`。
- `trial-workspaces/<trial>/.rebuild/state.json` 保存可恢复状态。
- `trial-workspaces/<trial>/.rebuild/report.json` 保存最近一次 `report` 汇总。
- 初始化 trial 时会自动安装 `oracle-tests/unit/` 下的测试。
- `report` 会输出失败分类，例如 `compile_error`、`unit_test_failure`、`qemu_timeout`、`missing_patterns`。
