# 架构契约：启动、陷入与系统调用

- Inputs: `experiment-design.md`, `experiment-execution-plan.md`, `tg-rcore-tutorial/tg-rcore-tutorial-ch2/.cargo/config.toml`, `tg-rcore-tutorial/tg-rcore-tutorial-syscall/src/user.rs`, `tg-rcore-tutorial/tg-rcore-tutorial-syscall/src/kernel/`
- Prompt version: redo-step1-complete-os-architecture.v1
- Model/Codex version: Codex session with local `codex-cli 0.130.0`
- Date: 2026-05-09

## 目标

本文件定义 RustOS/COS 生成时必须共同满足的机器级 OS 契约。它不是 tg-rcore 源码结构的复述，也不要求复制任何 crate/module；它只约束用户程序可观察的 RISC-V OS 行为。

## 机器与启动

- 目标机器为 `qemu-system-riscv64 -machine virt -nographic -bios none -kernel <kernel-elf>`，`ch6` 之后可额外挂载一个 raw 块设备镜像。
- 内核 ELF 必须有明确入口，完成内核栈初始化、BSS 清零或等价初始化、控制台初始化、trap 向量安装和关机路径。
- 控制台输出可以通过 SBI 或 virt UART 实现，但必须在 QEMU 内由内核完成，不能由宿主机脚本代替。
- `./test.sh base` 必须构建 RISC-V 内核或镜像并实际调用 QEMU，输出再交给 `tg-rcore-tutorial-checker`。

## U/S 特权级切换

- 用户程序必须在 U-mode 运行；内核必须在 S-mode 处理 trap、syscall、异常和调度。
- 内核返回用户态时必须恢复用户寄存器、用户栈指针、用户 PC，并使用 `sret` 或等价 RISC-V 特权返回路径。
- 用户执行 `ecall` 后，内核必须跳过触发 `ecall` 的指令，再返回用户态继续执行；重复进入同一条 `ecall` 是错误。
- 用户触发非法指令、特权指令、非法 CSR 或非法访存时，当前任务或进程必须被标记退出/杀死，并且批处理或调度器继续处理后续用户程序。

## 系统调用 ABI

- 用户态 syscall 使用 RISC-V 调用约定：`a7` 为 syscall id，`a0` 到 `a5` 为至多六个参数，返回值写回 `a0`。
- 成功返回非负值；失败返回负值。`exit` 不返回用户调用点。
- 用户指针参数必须按当前章节的内存模型读取/写入：
  - `ch2/ch3` 可使用固定装载地址下的物理地址模型，但仍必须验证地址属于当前用户程序范围。
  - `ch4` 之后必须经当前地址空间转换或等价的用户缓冲区访问层，不得直接信任用户虚拟地址。
- 未实现或不适用于本章的 syscall 必须返回明确负值或将任务按章节语义终止；不能静默成功。

## 必须支持的 syscall id

| syscall | id | 首个基础章节 | 参数摘要 |
| --- | ---: | --- | --- |
| `read` | 63 | ch6 | `fd`, `buf`, `len` |
| `write` | 64 | ch2 | `fd`, `buf`, `len` |
| `openat` | 56 | ch6 | `path`, `flags` |
| `close` | 57 | ch6 | `fd` |
| `pipe2` | 59 | ch7 | `pipe_fd_ptr` |
| `fstat` | 80 | ch6 exercise | `fd`, `stat_ptr` |
| `exit` | 93 | ch2 | `exit_code` |
| `clock_gettime` | 113 | ch3 | `clock_id`, `timespec_ptr` |
| `sched_yield` | 124 | ch3 | none |
| `kill` | 129 | ch7 | `pid`, `signum` |
| `rt_sigaction` | 134 | ch7 | `signum`, `action`, `old_action` |
| `rt_sigprocmask` | 135 | ch7 | `mask` |
| `rt_sigreturn` | 139 | ch7 | none |
| `getpid` | 172 | ch5 | none |
| `brk` | 214 | ch4 | signed heap delta |
| `mmap` | 222 | ch4 exercise | `start`, `len`, `prot`, ignored trailing args |
| `munmap` | 215 | ch4 exercise | `start`, `len` |
| `clone` | 220 | ch5 | none for process fork in this teaching ABI |
| `execve` | 221 | ch5 | `path`, `path_len` |
| `wait4` | 260 | ch5 | `pid_or_any`, `exit_code_ptr` |
| `spawn` | 400 | ch5 exercise | `path`, `path_len` |
| `trace` | 410 | ch3 exercise | `request`, `id`, `data` |
| `enable_deadlock_detect` | 469 | ch8 exercise | `enabled` |
| `thread_create` | 1000 | ch8 | `entry`, `arg` |
| `waittid` | 95 | ch8 | `tid` |
| `gettid` | 178 | ch8 | none |
| `mutex_create` | 1010 | ch8 | `blocking` |
| `mutex_lock` | 1011 | ch8 | `mutex_id` |
| `mutex_unlock` | 1012 | ch8 | `mutex_id` |
| `semaphore_create` | 1020 | ch8 | `resource_count` |
| `semaphore_up` | 1021 | ch8 | `sem_id` |
| `semaphore_down` | 1022 | ch8 | `sem_id` |
| `condvar_create` | 1030 | ch8 | ignored argument |
| `condvar_signal` | 1031 | ch8 | `condvar_id` |
| `condvar_wait` | 1032 | ch8 | `condvar_id`, `mutex_id` |

## 章节增量

- `ch2`: `write`、`exit`、异常杀死当前 app、顺序运行批处理 app。
- `ch3`: 加入多任务运行态、`sched_yield`、时钟中断或等价时间推进、`clock_gettime`。
- `ch4`: 加入每任务地址空间、页表或等价隔离、用户缓冲区访问、`brk/sbrk`。
- `ch5`: 加入进程、pid、父子关系、`fork/exec/wait/getpid`。
- `ch6`: 加入文件描述符表、打开/关闭/读写文件、从文件系统加载程序。
- `ch7`: 加入 pipe 和 signal，可通过 fd 与进程状态交互。
- `ch8`: 加入线程、tid、线程等待、互斥锁、信号量、条件变量。

## 禁止项

- 不能用宿主机程序或脚本模拟 syscall 返回。
- 不能只在 QEMU 中打印 oracle 文本。
- 不能用手写内嵌 U-mode 片段替代独立构建的用户程序。
- 不能把 tg-rcore kernel crate/module 作为生成实现的依赖。

## Evidence

- `tg-rcore-tutorial/tg-rcore-tutorial-ch2/.cargo/config.toml`
- `tg-rcore-tutorial/tg-rcore-tutorial-ch6/.cargo/config.toml`
- `tg-rcore-tutorial/tg-rcore-tutorial-syscall/src/user.rs`
- `tg-rcore-tutorial/tg-rcore-tutorial-syscall/src/kernel/`
- `tg-rcore-tutorial/tg-rcore-tutorial-syscall/src/syscall.h.in`

## Uncertainty

本契约定义生成时的共同 ABI 和机器要求。它不指定内部数据结构，也不证明任何生成物已经满足这些要求。
