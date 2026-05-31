# RustOS ch7 生成验收报告

## 输入与边界

- 生成输入：`experiments/v2/agent/SKILL.md`、`experiments/v2/prompts/30_generate_or_repair_os.md`、`experiments/v2/spec-v2/manifest.json`、`experiments/v2/spec-v2/architecture/*.md`、`experiments/v2/bundles/ch7.bundle.md`、`experiments/v2/inputs/user-tests-manifest.json`、已验收的 `experiments/v2/rustos/ch6` 快照。
- 章节 bundle hash：`experiments/v2/bundles/ch7.bundle.md` = `b7b033912b0fe6eee1fd8ceddf9416d46456d216759b035962101956ea543795`。
- 生成方式：当前 Codex 会话在 ch6 快照基础上按 ch7 bundle 实现和修复；不读取 tg-rcore 内核源码作为实现模板。
- 验收范围：本报告只验收 RustOS 的 ch7 章节快照；ch8 的 thread/sync 能力仍需后续章节生成与验收。

## 生成物性质

RustOS ch7 是独立的 QEMU/RISC-V OS 章节快照。它保留 ch6 的 S-mode 内核、U-mode 用户程序、trap/syscall、Sv39 页表、进程表、fd table、内核文件表和 ch5/ch6 系统调用，并新增 pipe fd 对象、pipe 缓冲区、信号 action、pending mask、handler 投递和 `sigreturn` 上下文恢复。

完整 OS 门禁对应关系：

- RISC-V 目标：`riscv64gc-unknown-none-elf`。
- QEMU 路径：`qemu-system-riscv64 -machine virt -m 512M -nographic -bios default -kernel target/riscv64gc-unknown-none-elf/release/rustos-ch7-full-base`。
- 内核入口：`linker.ld` 与 `src/main.rs` 中的 `_start`。
- trap/syscall：`src/main.rs` 保存用户 trap frame，分发 ch6 系统调用以及 `PIPE2(59)`、`KILL(129)`、`RT_SIGACTION(134)`、`RT_SIGPROCMASK(135)`、`RT_SIGRETURN(139)`。
- app image：`build.rs` 将共享 ch7 base 用户测例编译为 ELF/bin，并通过 `.incbin` 打包为内核可枚举镜像表。
- user program loader：内核从镜像表加载 `initproc`，并由 `initproc`/`ch7b_usertest` 通过 `execve` 启动同一批用户程序。
- pipe 状态：内核维护固定容量 pipe 表；pipe 两端作为 fd table 项参与 `read/write/close/fork/exec`；fork 时增加读写端引用计数，close 时减少引用计数，读空且写端仍打开返回 `-2`，写端关闭后读空返回 EOF。
- signal 状态：每进程维护 `SignalAction` 表、mask、pending 位图、当前处理标记和保存的 trap frame；`kill` 设置 pending，返回用户态前投递 handler，`sigreturn` 恢复被打断的用户上下文。

## 测例与结果

执行命令：

```bash
./test.sh base
```

测例来源为同一批 tg-rcore 用户态 ch7 base 程序，包括 ch6 继承测例、`sig_simple`、`pipetest`、`pipe_large_test`、`ch7b_usertest`、`user_shell` 与 `initproc`。验收日志为 `logs/redo-rustos-ch7-base-attempt1.log`。

结果：`Test PASSED: 18/18`。

## 未决风险

- 当前只证明 ch7 base L1 测例上的用户可观察一致性。
- 已覆盖 base 所需的 `SIGUSR1` handler 投递与返回、短 pipe 传输、大 pipe 分段传输和 EOF；更完整的 stop/cont、SIGKILL 特例、复杂 signal mask 与交互输入路径留给后续扩展或 focused bug 分析。
- ch8 需要继续实现线程、线程等待、互斥锁、信号量和条件变量等语义。
