# RustOS ch6 生成验收报告

## 输入与边界

- 生成输入：`experiments/v2/agent/SKILL.md`、`experiments/v2/prompts/30_generate_or_repair_os.md`、`experiments/v2/spec-v2/manifest.json`、`experiments/v2/spec-v2/architecture/*.md`、`experiments/v2/bundles/ch6.bundle.md`、`experiments/v2/inputs/user-tests-manifest.json`、已验收的 `experiments/v2/rustos/ch5` 快照。
- 章节 bundle hash：`experiments/v2/bundles/ch6.bundle.md` = `03cab4e83eb20b66789af32f8411181f0aed5c55ee8e9973caacb79dbae29cb8`。
- 生成方式：当前 Codex 会话在 ch5 快照基础上按 ch6 bundle 实现和修复；不读取 tg-rcore 内核源码作为实现模板。
- 验收范围：本报告只验收 RustOS 的 ch6 章节快照；ch7 到 ch8 的 pipe/signal、thread/sync 能力仍需后续章节生成与验收。

## 生成物性质

RustOS ch6 是独立的 QEMU/RISC-V OS 章节快照。它保留 ch5 的 S-mode 内核、U-mode 用户程序、trap/syscall、Sv39 页表、进程表、`fork/clone`、`execve`、`wait4` 和 `sbrk`，并新增每进程 fd table、全局内存文件表、`openat`、`read`、`write` 和 `close`。

完整 OS 门禁对应关系：

- RISC-V 目标：`riscv64gc-unknown-none-elf`。
- QEMU 路径：`qemu-system-riscv64 -machine virt -m 512M -nographic -bios default -kernel target/riscv64gc-unknown-none-elf/release/rustos-ch6-full-base`。
- 内核入口：`linker.ld` 与 `src/main.rs` 中的 `_start`。
- trap/syscall：`src/main.rs` 保存用户 trap frame，分发 ch5 进程系统调用以及 `OPENAT(56)`、`CLOSE(57)`、`READ(63)`、`WRITE(64)`。
- app image：`build.rs` 将共享 ch6 base 用户测例编译为 ELF/bin，并通过 `.incbin` 打包为内核可枚举镜像表。
- user program loader：内核从镜像表加载 `initproc`，并由 `initproc`/`ch6b_usertest` 通过 `execve` 启动同一批用户程序。
- 地址空间：每个进程拥有独立 Sv39 root/L1/L0 页表；用户虚拟基址固定为 `0x86000000`，不同进程映射到不同物理槽。
- fd table：每个进程含 16 项 fd 表，`0/1/2` 为标准描述符，普通文件 fd 从 `3` 起分配；`fork` 复制 fd 表，`execve` 保留 fd 表。
- 文件状态：内核维护固定容量内存文件表，支持 `filea` 等普通文件的按名创建、打开、截断、顺序读写、偏移推进和 EOF 返回 `0`。

## 测例与结果

执行命令：

```bash
./test.sh base
```

测例来源为同一批 tg-rcore 用户态 ch6 base 程序，包括 ch5 继承测例、`filetest_simple`、`cat_filea`、`ch6b_usertest`、`user_shell` 与 `initproc`。验收日志为 `logs/redo-rustos-ch6-base-attempt1.log`。

结果：`Test PASSED: 15/15`。

## 未决风险

- 当前只证明 ch6 base L1 测例上的用户可观察一致性。
- 本轮文件系统为章节内内存文件表，覆盖 base 测例的创建、读写、关闭、重开和 EOF；持久化块设备、目录层级和硬链接语义留给后续扩展或 exercise 分析。
- ch7 以后需要继续实现 pipe/signal、thread/sync 等语义。
