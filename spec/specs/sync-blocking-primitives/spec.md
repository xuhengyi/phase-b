# Capability: sync-blocking-primitives

## Purpose

`sync-blocking-primitives` capability 用于描述 `sync` 中阻塞式互斥锁、条件变量和信号量这三类基础同步原语的阻塞与唤醒语义。

## Requirements

### Requirement: 提供阻塞式互斥锁的唤醒语义

`sync-blocking-primitives` capability SHALL 提供互斥锁语义，使一个执行单元在锁被他人持有时进入阻塞，并在解锁时 SHALL 标识被唤醒的等待者。

#### Scenario: 在存在等待线程时解锁互斥锁

- **WHEN** 一个阻塞式互斥锁在仍有其他线程等待时被释放
- **THEN** 该 capability MUST 返回被选中恢复执行的等待线程标识

### Requirement: 提供条件变量与信号量的等待-唤醒语义

`sync-blocking-primitives` capability SHALL 提供条件变量和信号量操作，使等待中的线程能够在对应的 signal 或资源释放之后恢复执行。

#### Scenario: 通过 signal 或 up 唤醒等待者

- **WHEN** 某个线程阻塞在条件变量或信号量上，另一个线程执行与之匹配的唤醒操作
- **THEN** 该 capability MUST 允许被阻塞线程重新取得前进机会

## Public API

- `Mutex`
- `MutexBlocking`
- `Condvar`
- `Semaphore`

## Build Configuration

- 无 `build.rs`

## Dependencies

- `alloc`
- `rcore-task-manage`
