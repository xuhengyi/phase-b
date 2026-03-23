# Capability: ch7

## Purpose

`ch7` capability 表示教程第七章的信号内核：它在文件系统内核基础上接入信号子系统，使进程能够接收、屏蔽和处理信号。

## Requirements

### Requirement: 为进程接入信号状态

`ch7` capability SHALL 为进程对象接入独立的信号状态，以便在运行、陷入和返回用户态时管理待处理信号。

#### Scenario: 新进程拥有自己的信号子系统

- **WHEN** 内核创建一个新进程
- **THEN** capability MUST 为其准备可独立更新的信号状态对象

### Requirement: 接入信号相关系统调用

`ch7` capability SHALL 提供信号注册、屏蔽和递送相关的系统调用实现基础。

#### Scenario: 用户程序注册信号处理函数

- **WHEN** 用户程序发起信号动作设置或屏蔽更新的系统调用
- **THEN** capability MUST 更新对应进程的信号状态

### Requirement: 在用户态返回前推进信号处理

`ch7` capability SHALL 在适当的内核返回点检查当前进程的信号状态，并在需要时切换到信号处理路径或暂停执行。

#### Scenario: 进程存在待处理信号

- **WHEN** 进程即将从内核返回用户态且存在可处理信号
- **THEN** capability MUST 根据当前屏蔽和动作配置决定后续控制流

## Public API

- 无稳定 Rust 库 API
- 对外能力体现为带信号支持的章节内核行为

## Build Configuration

- `build.rs` 将 `linker::SCRIPT` 写入 `OUT_DIR/linker.ld`
- 构建依赖 `APP_ASM` 与 `LOG`

## Dependencies

- `signal`
- `signal-impl`
- `easy-fs`
- `syscall` with `kernel`
