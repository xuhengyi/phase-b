# RustOS ch4 生成验收报告

## 输入与边界

- 生成输入：`experiments/v2/agent/SKILL.md`、`experiments/v2/prompts/30_generate_or_repair_os.md`、`experiments/v2/spec-v2/manifest.json`、`experiments/v2/spec-v2/architecture/*.md`、`experiments/v2/bundles/ch4.bundle.md`、`experiments/v2/inputs/user-tests-manifest.json`、已验收的 `experiments/v2/rustos/ch3` 快照。
- 章节 bundle hash：`experiments/v2/bundles/ch4.bundle.md` = `cb5f15e0b28b169cd3f7d8003b68ee1fdfd2a1ef626889fde42eec9f847524c0`。
- 生成方式：当前 Codex 会话在 ch3 快照基础上按 ch4 bundle 手工修复；不读取 tg-rcore 内核源码作为实现模板。
- 验收范围：本报告只验收 RustOS 的 ch4 章节快照；ch5 到 ch8 的进程、文件、信号、线程和同步能力仍需后续章节生成与验收。

## 生成物性质

RustOS ch4 是独立的 QEMU/RISC-V OS 章节快照。它保留 ch3 的 S-mode 内核、U-mode 用户程序、trap/syscall、任务调度和共享用户测例构建路径，并新增每任务 Sv39 页表、用户栈映射、用户缓冲校验和 `sbrk` 堆增长/回收语义。

完整 OS 门禁对应关系：

- RISC-V 目标：`riscv64gc-unknown-none-elf`。
- QEMU 路径：`qemu-system-riscv64 -machine virt -bios default -kernel target/riscv64gc-unknown-none-elf/release/rustos-ch4-full-base`。
- 内核入口：`linker.ld` 与 `src/main.rs` 中的 `_start`。
- trap/syscall：`src/main.rs` 中保存用户 trap frame，分发 `write`、`exit`、`sched_yield`、`clock_gettime`、`brk/sbrk` 与 fault。
- app image：`build.rs` 将共享 ch4 base 用户测例编译为 ELF/bin，并通过 `.incbin` 打包为内核可枚举镜像表。
- user program loader：内核按镜像表逐个加载用户程序到独立 app slot，并以 U-mode trap frame 启动。
- 地址空间：每个任务拥有独立 Sv39 root/L1/L0 页表；内核区以 supervisor 权限映射，当前用户程序区、堆区和栈区以 U 权限映射。
- `sbrk`：`BRK(214)` 返回旧 break，正增量映射堆页，负增量撤销堆页；回收后的页访问触发 page fault 并结束当前任务。

## 测例与结果

执行命令：

```bash
./test.sh base
```

测例来源为同一批 tg-rcore 用户态 ch4 base 程序，包括 ch2/ch3 基础程序、`11sleep` 与 `sbrk`。验收日志为 `logs/redo-rustos-ch4-base-attempt3.log`。

结果：`Test PASSED: 6/6`。

## 未决风险

- 当前只证明 ch4 base L1 测例上的用户可观察一致性。
- exercise 中的 mmap/munmap/trace 未纳入本轮 L1，因为冻结的 tg-rcore 基线未提供共同通过批次。
- ch5 以后需要继续扩展 fork/exec/wait、文件、pipe/signal、thread/sync 等语义。
