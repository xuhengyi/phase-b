# Capability: ch5

## Purpose

`ch5` capability 表示教程第五章的进程管理内核：它在地址空间内核基础上引入更明确的进程管理结构，并支持 `fork`、`exec`、`wait` 等进程相关流程。

## Requirements

### Requirement: 通过进程管理器维护进程集合

`ch5` capability SHALL 使用进程管理结构维护所有活跃进程，并把调度、父子关系和地址空间生命周期关联起来。

#### Scenario: 内核创建并登记一个新进程

- **WHEN** 系统需要创建初始进程或派生新进程
- **THEN** capability MUST 把该进程纳入管理器，并维持其与父进程的关联关系

### Requirement: 支持进程派生与替换执行映像

`ch5` capability SHALL 支持从现有进程派生新进程，并允许进程替换自己的执行映像。

#### Scenario: 进程执行 `fork` 或 `exec`

- **WHEN** 用户程序发起派生或替换执行映像的系统调用
- **THEN** capability MUST 更新目标进程的上下文、地址空间和管理关系

### Requirement: 支持基本的进程退出与等待路径

`ch5` capability SHALL 提供进程退出和父进程等待子进程结束的基础路径。

#### Scenario: 父进程等待子进程结束

- **WHEN** 父进程调用等待相关系统调用
- **THEN** capability MUST 在子进程结束后返回与之对应的结果信息

## Public API

- 无稳定 Rust 库 API
- 对外能力体现为支持进程管理的章节内核行为

## Build Configuration

- `build.rs` 将 `linker::SCRIPT` 写入 `OUT_DIR/linker.ld`
- 构建依赖 `APP_ASM` 与 `LOG`

## Dependencies

- `kernel-alloc`
- `kernel-vm`
- `kernel-context` with `foreign`
- `rcore-task-manage` with `proc`
- `syscall` with `kernel`
