# RustOS ch3 生成验收报告

## 输入与边界

- 生成输入：`experiments/v2/agent/SKILL.md`、`experiments/v2/prompts/30_generate_or_repair_os.md`、`experiments/v2/spec-v2/manifest.json`、`experiments/v2/spec-v2/architecture/*.md`、`experiments/v2/bundles/ch3.bundle.md`、`experiments/v2/inputs/user-tests-manifest.json`、已验收的 `experiments/v2/rustos/ch2` 快照。
- 章节 bundle hash：`experiments/v2/bundles/ch3.bundle.md` = `7c98e4585b28f9a31be73252d038fe2426b9add279db2eba2886a045e06d62dd`。
- 生成方式：Codex CLI `gpt-5.5`，`model_reasoning_effort=xhigh`。
- 验收范围：本报告只验收 RustOS 的 ch3 章节快照；ch4 到 ch8 的地址空间、进程、文件、线程和同步能力仍需后续章节生成与验收。

## 生成物性质

RustOS ch3 是独立的 QEMU/RISC-V OS 章节快照。它从 `_start` 进入 S-mode 内核，初始化 `stvec`，通过 `sret` 进入 U-mode 用户程序，所有用户态输出、退出、让出 CPU 和时间查询都经由 trap/syscall 路径返回内核处理。

完整 OS 门禁对应关系：

- RISC-V 目标：`riscv64gc-unknown-none-elf`。
- QEMU 路径：`qemu-system-riscv64 -machine virt -bios default -kernel target/riscv64gc-unknown-none-elf/release/rustos-ch3`。
- 内核入口：`linker.ld` 与 `src/main.rs` 中的 `_start`。
- trap/syscall：`src/main.rs` 中保存用户 trap frame，分发 `write`、`exit`、`sched_yield`、`clock_gettime` 与 fault。
- app image：`build.rs` 将共享用户测例编译为 ELF/bin，并通过 `.incbin` 打包为内核可枚举镜像表。
- user program loader：内核按镜像表逐个加载用户程序到约定用户入口，并用 U-mode trap frame 启动。
- ch3 调度能力：内核维护任务状态、trap frame 和简单轮转调度，支持 `sched_yield` 与 `clock_gettime` 的用户可观察行为。

## 测例与结果

执行命令：

```bash
./test.sh base
```

测例来源为同一批 tg-rcore 用户态 ch3 base 程序，包括 `05write_a`、`06write_b`、`07write_c` 以及调度相关程序。验收日志为 `logs/redo-rustos-ch3-base-rerun.log`。

结果：`Test PASSED: 4/4`。

## 未决风险

- 当前只证明 ch3 base L1 测例上的用户可观察一致性。
- ch4 以后需要继续扩展地址空间、进程、文件和同步语义，并在相同完整 OS 门禁下重新验收。
