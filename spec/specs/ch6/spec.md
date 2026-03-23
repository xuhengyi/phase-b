# Capability: ch6

## Purpose

`ch6` capability 表示教程第六章的文件系统内核：它在进程管理内核基础上接入块设备和文件系统，使内核能够从文件系统加载应用并向进程提供文件读写相关系统调用。

## Requirements

### Requirement: 接入块设备并初始化文件系统

`ch6` capability SHALL 初始化块设备访问路径，并在其上打开或创建内核可用的文件系统实例。

#### Scenario: 内核启动时准备文件系统

- **WHEN** 章节内核完成早期内存和地址空间初始化
- **THEN** capability MUST 建立块设备到文件系统管理器的接入路径

### Requirement: 从文件系统加载用户程序

`ch6` capability SHALL 能够通过文件系统定位用户程序映像，并把其内容装载到新进程地址空间。

#### Scenario: init 进程从文件系统载入

- **WHEN** 内核需要启动初始用户程序
- **THEN** capability MUST 能够从文件系统读取目标文件并据此创建进程

### Requirement: 向进程暴露文件相关系统调用

`ch6` capability SHALL 为进程提供文件打开、读写和文件描述符管理所需的系统调用实现基础。

#### Scenario: 用户程序读取文件

- **WHEN** 用户程序发起文件读写相关系统调用
- **THEN** capability MUST 通过文件系统与进程文件描述符状态完成相应操作

## Public API

- 无稳定 Rust 库 API
- 对外能力体现为带文件系统支持的章节内核行为

## Build Configuration

- `build.rs` 将 `linker::SCRIPT` 写入 `OUT_DIR/linker.ld`
- 构建依赖 `APP_ASM` 与 `LOG`
- 运行时依赖块设备与磁盘镜像

## Dependencies

- `easy-fs`
- `kernel-vm`
- `kernel-context` with `foreign`
- `syscall` with `kernel`
- `virtio-drivers`
