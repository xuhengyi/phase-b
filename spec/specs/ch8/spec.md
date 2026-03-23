# Capability: ch8

## Purpose

`ch8` capability 表示教程第八章的线程与同步内核：它在信号内核基础上引入线程管理与同步原语相关系统调用，使同一进程内可以拥有多个执行流并通过同步对象协作。

## Requirements

### Requirement: 为进程接入线程管理

`ch8` capability SHALL 在进程管理之上维护线程集合，并允许在同一地址空间内创建和调度多个线程。

#### Scenario: 进程创建新线程

- **WHEN** 用户程序发起线程创建相关系统调用
- **THEN** capability MUST 为该线程建立上下文并纳入现有调度体系

### Requirement: 接入同步原语相关系统调用

`ch8` capability SHALL 为用户程序提供互斥、条件变量和信号量等同步对象相关系统调用。

#### Scenario: 线程等待同步对象

- **WHEN** 某个线程在互斥锁、条件变量或信号量上发起等待
- **THEN** capability MUST 通过内核同步层维护其阻塞与唤醒关系

### Requirement: 在线程级调度中保持进程语义

`ch8` capability SHALL 在引入线程后继续维持进程级资源、地址空间和信号语义的一致性。

#### Scenario: 多线程进程继续运行既有进程能力

- **WHEN** 某个进程拥有多个线程并继续使用文件或信号相关能力
- **THEN** capability MUST 保持这些能力与线程调度的组合行为一致

## Public API

- 无稳定 Rust 库 API
- 对外能力体现为带线程与同步支持的章节内核行为

## Build Configuration

- `build.rs` 将 `linker::SCRIPT` 写入 `OUT_DIR/linker.ld`
- 构建依赖 `APP_ASM` 与 `LOG`

## Dependencies

- `rcore-task-manage` with `proc,thread`
- `sync`
- `signal`
- `signal-impl`
- `syscall` with `kernel`
