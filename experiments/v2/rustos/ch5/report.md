# RustOS ch5 生成验收报告

## 输入与边界

- 生成输入：`experiments/v2/agent/SKILL.md`、`experiments/v2/prompts/30_generate_or_repair_os.md`、`experiments/v2/spec-v2/manifest.json`、`experiments/v2/spec-v2/architecture/*.md`、`experiments/v2/bundles/ch5.bundle.md`、`experiments/v2/inputs/user-tests-manifest.json`、已验收的 `experiments/v2/rustos/ch4` 快照。
- 章节 bundle hash：`experiments/v2/bundles/ch5.bundle.md` = `b3863a02704b965f224d46cf2156023bb67e1624ce3f3bb71b8637608ba0f9fe`。
- 生成方式：当前 Codex 会话在 ch4 快照基础上按 ch5 bundle 实现和修复；不读取 tg-rcore 内核源码作为实现模板。
- 验收范围：本报告只验收 RustOS 的 ch5 章节快照；ch6 到 ch8 的文件、pipe/signal、thread/sync 能力仍需后续章节生成与验收。

## 生成物性质

RustOS ch5 是独立的 QEMU/RISC-V OS 章节快照。它保留 ch4 的 S-mode 内核、U-mode 用户程序、trap/syscall、Sv39 页表、用户栈和 `sbrk` 语义，并新增进程表、父子关系、独立进程物理槽、`fork/clone`、`execve`、`wait4` 和 `getpid`。

完整 OS 门禁对应关系：

- RISC-V 目标：`riscv64gc-unknown-none-elf`。
- QEMU 路径：`qemu-system-riscv64 -machine virt -m 512M -nographic -bios default -kernel target/riscv64gc-unknown-none-elf/release/rustos-ch5-full-base`。
- 内核入口：`linker.ld` 与 `src/main.rs` 中的 `_start`。
- trap/syscall：`src/main.rs` 保存用户 trap frame，分发 `write`、`exit`、`sched_yield`、`clock_gettime`、`brk/sbrk`、`getpid`、`clone/fork`、`execve` 与 `wait4`。
- app image：`build.rs` 将共享 ch5 base 用户测例编译为 ELF/bin，并通过 `.incbin` 打包为内核可枚举镜像表。
- user program loader：内核从镜像表加载 `initproc`，并由 `initproc` 通过 `execve` 启动同一批用户程序。
- 地址空间：每个进程拥有独立 Sv39 root/L1/L0 页表；用户虚拟基址固定为 `0x86000000`，不同进程映射到不同物理槽。
- 进程语义：`fork` 复制当前进程物理槽和 trap frame，子进程返回 `0`，父进程返回子 pid；`execve` 重置当前进程镜像；`wait4` 回收已退出子进程并写回退出码。
- 内存语义：`BRK(214)` 返回旧 break，按页映射或撤销用户堆；页表变更期间切回裸地址访问，随后恢复当前进程页表。

## 测例与结果

执行命令：

```bash
./test.sh base
```

测例来源为同一批 tg-rcore 用户态 ch5 base 程序，包括 ch2/ch3/ch4 基础程序、`sbrk`、`forktest_simple`、`fork_exit`、`forktest`、`forktree`、`forktest2`、`matrix`、`user_shell` 与 `initproc`。验收日志为 `logs/redo-rustos-ch5-base-final.log`。

结果：`Test PASSED: 14/14`。

## 未决风险

- 当前只证明 ch5 base L1 测例上的用户可观察一致性。
- `execve` 文件名解析按本批共享用户程序名称表实现；真实文件系统能力将在 ch6 章节继续扩展。
- ch6 以后需要继续实现文件、pipe/signal、thread/sync 等语义。
