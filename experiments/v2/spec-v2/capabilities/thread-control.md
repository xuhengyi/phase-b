# Capability: thread-control

- Inputs: `inputs/user-tests-manifest.json`, `spec-v2/architecture/boot-trap-syscall.md`, `spec-v2/architecture/subsystem-contracts.md`
- Prompt version: redo-step1-complete-os-capability.v1
- Model/Codex version: Codex session with local `codex-cli 0.130.0`
- Date: 2026-05-09

## Contract

`ch8` 必须在进程资源之内创建多个用户线程。线程共享地址空间、fd table 和进程级同步对象，但拥有独立的 tid、trap/context、用户栈、状态和退出值。

## Observable Requirements

- ch8 base emits `threads test passed!`.
- ch8 base emits `threads with arg test passed!`.
- `thread_create(entry, arg)` 在当前进程内创建新线程，从用户地址 `entry(arg)` 开始执行。
- `gettid()` 返回当前线程 id。
- `waittid(tid)` 等待目标线程退出；目标仍运行时可返回 `-2`，用户库 yield 后重试。
- 线程退出不应销毁整个进程，除非退出的是最后一个线程或进程执行了 `exit`。

## Required Runtime State

- per-process thread list.
- per-thread user context, kernel context, user stack, state and exit code.
- scheduler awareness of runnable threads.
- `fork/exec/exit` 与线程模型的交互策略。

## Out Of Scope

Full POSIX pthread semantics, arbitrary stack allocation policy, and fairness guarantees beyond selected tests are outside current L1.
