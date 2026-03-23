# Capability: ch3

## Purpose

`ch3` capability 表示教程第三章的多任务内核：它在批处理基础上引入任务控制块、调度与时钟相关流程，并支持协作或抢占式运行。

## Requirements

### Requirement: 管理多个任务的生命周期

`ch3` capability SHALL 为每个用户应用维护任务控制块，并记录其运行、就绪和结束等状态。

#### Scenario: 内核初始化任务集合

- **WHEN** 内核启动并装载全部应用
- **THEN** capability MUST 为每个应用建立对应任务项并纳入调度集合

### Requirement: 执行调度与任务切换

`ch3` capability SHALL 根据当前章节配置选择协作式或带时钟驱动的抢占式运行方式，并在任务之间切换执行。

#### Scenario: 当前任务主动让出或被抢占

- **WHEN** 任务执行到让出点或收到时钟驱动的切换请求
- **THEN** capability MUST 保存当前上下文并恢复下一个可运行任务

### Requirement: 接入时钟与调度相关系统调用

`ch3` capability SHALL 处理与任务切换、时间读取或让出 CPU 有关的系统调用。

#### Scenario: 用户任务请求让出 CPU

- **WHEN** 用户程序发起与调度相关的系统调用
- **THEN** capability MUST 更新任务状态并触发后续调度决策

## Public API

- 无稳定 Rust 库 API
- 对外能力体现为支持多任务的章节内核行为

## Build Configuration

- `build.rs` 将 `linker::SCRIPT` 写入 `OUT_DIR/linker.ld`
- 构建依赖 `APP_ASM` 与 `LOG`
- feature `coop` 影响调度方式

## Dependencies

- `linker`
- `kernel-context`
- `syscall` with `kernel`
- `sbi-rt`
