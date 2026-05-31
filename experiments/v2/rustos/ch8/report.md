# RustOS ch8 生成验收报告

## 输入与边界

- 生成输入：`experiments/v2/agent/SKILL.md`、`experiments/v2/prompts/30_generate_or_repair_os.md`、`experiments/v2/spec-v2/manifest.json`、`experiments/v2/spec-v2/architecture/*.md`、`experiments/v2/bundles/ch8.bundle.md`、`experiments/v2/inputs/user-tests-manifest.json`、已验收的 `experiments/v2/rustos/ch7` 快照。
- 章节 bundle hash：`experiments/v2/bundles/ch8.bundle.md` = `181f48bf5ac97ea62aefdf002027a17ab98e1e327f9ae89dbd10256978663f0d`。
- 生成方式：当前 Codex 会话在 ch7 快照基础上按 ch8 bundle 实现和修复；不读取 tg-rcore 内核源码作为实现模板。
- 验收范围：本报告验收 RustOS 的 ch8 章节快照，覆盖 full base 批次所需的线程与同步主链路；deadlock exercise 不纳入本轮共同通过批次。

## 生成物性质

RustOS ch8 是独立的 QEMU/RISC-V OS 章节快照。它保留 ch7 的 S-mode 内核、U-mode 用户程序、trap/syscall、Sv39 页表、进程表、fd table、文件、pipe 和 signal 语义，并新增同进程多线程执行流、`waittid`、线程共享地址空间、线程私有 trap context/栈/tid，以及 semaphore、mutex、condvar 同步对象。

完整 OS 门禁对应关系：

- RISC-V 目标：`riscv64gc-unknown-none-elf`。
- QEMU 路径：`qemu-system-riscv64 -machine virt -m 512M -nographic -bios default -kernel target/riscv64gc-unknown-none-elf/release/rustos-ch8-full-base`。
- 内核入口：`linker.ld` 与 `src/main.rs` 中的 `_start`。
- trap/syscall：`src/main.rs` 保存用户 trap frame，分发 ch7 系统调用以及 `WAITID(95)`、`GETTID(178)`、`THREAD_CREATE(1000)`、`MUTEX_*`、`SEMAPHORE_*`、`CONDVAR_*`。
- app image：`build.rs` 将共享 ch8 base 用户测例编译为 ELF/bin，并通过 `.incbin` 打包为内核可枚举镜像表。
- user program loader：内核从镜像表加载 `initproc`，并由 `initproc`/`ch8b_usertest` 通过 `execve` 启动同一批用户程序。
- thread 状态：线程使用独立 `TrapContext`、tid 和线程状态；同一 group 内线程共享同一物理进程槽、页表映射语义和用户地址空间，并使用不同用户栈窗口。
- sync 状态：每个进程 group 维护 semaphore、mutex、condvar 对象编号空间；同步 syscall 更新内核对象状态，并在需要等待的路径上让出调度。

## 测例与结果

执行命令：

```bash
./test.sh base
```

测例来源为同一批 tg-rcore 用户态 ch8 base 程序，包括 ch7 继承测例、`threads`、`threads_arg`、`mpsc_sem`、`sync_sem`、`race_adder_mutex_blocking`、`phil_din_mutex`、`test_condvar`、`ch8b_usertest`、`user_shell` 与 `initproc`。验收日志为 `logs/redo-rustos-ch8-base-attempt1.log`。

结果：`Test PASSED: 22/22`。

## 未决风险

- 当前只证明 ch8 base L1 测例上的用户可观察一致性。
- 同步对象覆盖 base 所需的创建、up/down、lock/unlock、condvar wait/signal 和调度让出；deadlock 检测、严格等待队列公平性和 exercise 场景不在本轮共同通过批次。
- 后续 Step4 比较仍需将这些实现限制写入一致性矩阵，避免把 base 通过扩展成完整语义等价声明。
