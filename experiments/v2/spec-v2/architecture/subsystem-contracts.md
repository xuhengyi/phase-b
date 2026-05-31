# 架构契约：ch2-ch8 子系统完整性

- Inputs: `inputs/user-tests-manifest.json`, `spec-v2/capabilities/*.md`, `run-manifests/full-ch2-ch8.yaml`
- Prompt version: redo-step1-complete-os-architecture.v1
- Model/Codex version: Codex session with local `codex-cli 0.130.0`
- Date: 2026-05-09

## 目标

本文件把 checker 级 oracle 提升为 OS 子系统完整性要求。一个章节 snapshot 可以是递进式 OS 的 `chN` 状态，也可以是独立目录；但只要声称通过 `chN`，就必须包含该章之前所有基础子系统。

## ch2：批处理、异常与最小 syscall

- 有内核入口、trap 入口、用户上下文保存恢复、用户栈和批处理 app 列表。
- 顺序运行 base 清单中的 app。
- `write` 将用户缓冲区输出到 QEMU 控制台。
- `exit` 结束当前 app 并进入下一个 app。
- 非法访存、特权指令、非法 CSR 会杀死当前 app 并继续批处理。

## ch3：多任务与时钟

- 在 ch2 基础上维护多个任务控制块或等价任务状态。
- 支持 runnable/exited/faulted 状态转移。
- 支持主动 `sched_yield` 和时钟中断或等价时间片机制。
- `clock_gettime(CLOCK_MONOTONIC)` 返回可单调推进的时间，足以让 sleep 类用户程序退出。
- 调度器必须能让 `05write_a/06write_b/07write_c` 等并发输出测例运行完毕。

## ch4：地址空间与用户内存

- 在 ch3 基础上给每个任务/进程建立独立地址空间或等价隔离模型。
- 能装载用户程序代码、只读数据、可写数据、用户栈和 trap/context 区。
- 用户缓冲区访问必须检查地址有效性和长度边界。
- `brk/sbrk` 支持查询、增长、收缩和越界失败，满足 base `sbrk` 测例。
- 用户态非法访存不能破坏内核；应终止当前任务并继续运行其他任务。

## ch5：进程、fork/exec/wait

- 在 ch4 基础上维护进程 pid、父子关系、exit code、子进程列表和僵尸回收状态。
- `fork` 复制当前用户执行状态和地址空间；父返回子 pid，子返回 `0`。
- `exec(path)` 从用户程序镜像或文件系统加载新程序，替换当前地址空间并保留 pid。
- `wait(-1, ptr)` 等待任意子进程；`wait(pid, ptr)` 等待指定子进程。
- 没有匹配子进程返回 `-1`；匹配子进程仍运行时可返回 `-2`，用户库会 yield 后重试。
- `getpid` 返回当前进程 pid；`sched_yield` 允许其他进程运行。
- `clock_gettime` 和 `brk/sbrk` 继续保持前章语义。

## ch6：文件系统与 fd table

- 在 ch5 基础上维护每进程 fd table。
- `0/1/2` 是标准输入、标准输出、调试/错误输出。
- `openat(path, flags)` 支持 base 测例中的只读、创建、截断和读写模式。
- `read/write/close` 对普通文件和 console fd 返回正确字节数或负错误。
- `exec` 可以从文件系统中的用户程序文件读取完整 ELF 或等价程序镜像。
- 文件系统镜像必须随 `./test.sh base` 构建并挂载给 QEMU 内核，不能由宿主机代读。

## ch7：pipe 与 signal

- 在 ch6 基础上实现 pipe fd 对。
- pipe 读写支持阻塞或返回 `-2` 后由用户库 yield 重试；写端关闭后读端能看到 EOF。
- pipe 缓冲区容量可以不同，但必须满足 `pipetest` 与 `pipe_large_test` 的进度要求。
- 支持 base signal 测例需要的 `kill`、`sigaction`、`sigprocmask`、`sigreturn`。
- 信号处理必须保存被打断用户上下文，并在 handler 返回后恢复或按 signal 语义终止进程。

## ch8：线程与同步

- 在 ch7 基础上把进程资源和线程执行流分离：同一进程内多个线程共享地址空间和 fd table。
- `thread_create(entry, arg)` 创建同进程新线程，入口在用户地址空间内执行。
- `gettid` 返回当前线程 id；`waittid(tid)` 等待目标线程退出，目标仍运行时可返回 `-2`。
- mutex 支持创建、lock、unlock；blocking mutex 必须阻塞或 yield 等待而非忙等导致系统停滞。
- semaphore 支持资源计数、up/down 和等待队列或等价机制。
- condvar 支持 create、signal、wait；wait 必须释放关联 mutex 并在被唤醒后重新竞争。
- 同步原语必须让 `threads`、`threads_arg`、`mpsc_sem`、`sync_sem`、`race_adder_mutex_blocking`、`phil_din_mutex`、`test_condvar` 等 base 测例完成。

## 完整性判定

- 一个 `chN` snapshot 缺少此前章节基础子系统时，不能被认定为完整 `chN` OS。
- 一个实现只通过 `ch2` 不能被认定为“RustOS/COS 已生成完成”；完整 Step 2 必须覆盖 `ch2-ch8`。
- 某章因为实现困难失败时，必须在 report 与结果矩阵中记为失败或 inconclusive，不能替换成 host runner、oracle printer 或 embedded snippet。

## Evidence

- `inputs/user-tests-manifest.json`
- `spec-v2/capabilities/syscall-abi.md`
- `spec-v2/capabilities/process-control.md`
- `spec-v2/capabilities/file-system-io.md`
- `spec-v2/capabilities/ipc-or-pipe.md`
- `spec-v2/capabilities/signal-control.md`
- `spec-v2/capabilities/thread-control.md`
- `spec-v2/capabilities/sync-primitives.md`

## Uncertainty

本文件定义 base L1 共同测例要求。exercise 中 `mmap/munmap/trace/spawn/setpriority/deadlock_detect` 等扩展仍按 run manifest 的 scope 另行启用。
