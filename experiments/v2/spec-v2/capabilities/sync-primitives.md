# Capability: sync-primitives

- Inputs: `inputs/user-tests-manifest.json`, `spec-v2/capabilities/thread-control.md`, `spec-v2/architecture/subsystem-contracts.md`
- Prompt version: redo-step1-complete-os-capability.v1
- Model/Codex version: Codex session with local `codex-cli 0.130.0`
- Date: 2026-05-09

## Contract

`ch8` 必须提供进程内同步对象：mutex、semaphore 和 condvar。它们必须是内核对象，和线程调度/等待状态联动，不能由用户测试输出硬编码。

## Observable Requirements

- `mpsc_sem passed!`
- `sync_sem passed!`
- `race adder using spin mutex test passed!`
- `philosopher dining problem with mutex test passed!`
- `test_condvar passed!`

## Public Surface

- `mutex_create(blocking)`, `mutex_lock(id)`, `mutex_unlock(id)`.
- `semaphore_create(count)`, `semaphore_up(id)`, `semaphore_down(id)`.
- `condvar_create()`, `condvar_signal(id)`, `condvar_wait(condvar_id, mutex_id)`.

## Required Runtime State

- process-local object tables for mutex, semaphore and condvar ids.
- owner or availability state for mutex.
- resource count and wait queue for semaphore.
- wait queue for condvar.
- scheduler integration so blocked threads stop consuming execution until wakeup or retry.

## Out Of Scope

Deadlock exercise tests and all possible interleavings are not claimed by L1. When exercise scope is enabled, `enable_deadlock_detect` must be specified and tested separately.
