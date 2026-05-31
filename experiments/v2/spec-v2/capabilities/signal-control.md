# Capability: signal-control

- Inputs: `inputs/user-tests-manifest.json`, `spec-v2/architecture/boot-trap-syscall.md`, `spec-v2/architecture/subsystem-contracts.md`
- Prompt version: redo-step1-complete-os-capability.v1
- Model/Codex version: Codex session with local `codex-cli 0.130.0`
- Date: 2026-05-09

## Contract

`ch7` 必须支持 base signal 测例需要的信号注册、发送、屏蔽、递送和 `sigreturn`。信号处理发生在用户态 handler 中，内核负责保存被打断用户上下文并在 handler 返回后恢复。

## Observable Requirements

- ch7 base emits `signal_simple: Done`.
- `kill(pid, signum)` 向目标进程记录 pending signal 或按默认动作终止目标。
- `sigaction(signum, action, old_action)` 安装或查询 handler。
- `sigprocmask(mask)` 更新当前进程信号屏蔽集合。
- `sigreturn()` 恢复进入 handler 前保存的用户上下文。

## Required Runtime State

- per-process signal actions.
- signal mask and pending signals.
- saved trap/user context for signal handler return.
- default action for unsupported or unhandled signals.

## Out Of Scope

Nested signal delivery, complete POSIX signal set, real-time signal queue ordering, and exercise-only behavior are outside current full base L1.
