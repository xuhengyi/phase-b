# Capability: ch2

## Purpose

`ch2` capability 表示教程第二章的批处理内核：它负责加载内嵌用户应用、进入用户态执行、处理用户陷入并串行运行每个应用。

## Requirements

### Requirement: 加载并遍历内嵌应用

`ch2` capability SHALL 读取链接进内核镜像的应用元数据，并按既定顺序逐个准备用户应用执行。

#### Scenario: 批处理内核发现多个应用

- **WHEN** 内核通过链接元数据遍历应用列表
- **THEN** capability MUST 依次为每个应用创建执行所需的上下文

### Requirement: 执行用户应用并处理陷入

`ch2` capability SHALL 使用用户上下文进入应用执行，并在陷入后根据原因进行系统调用处理或终止当前应用。

#### Scenario: 用户应用触发系统调用

- **WHEN** 应用在运行中触发系统调用陷入
- **THEN** capability MUST 将控制流转入内核处理逻辑并把结果反馈回用户上下文

### Requirement: 支持串行批处理完成全部应用

`ch2` capability SHALL 在当前应用退出后继续启动下一个应用，直到所有链接进来的应用都执行完成。

#### Scenario: 当前应用结束后切换到下一个应用

- **WHEN** 某个应用正常结束或被内核判定为终止
- **THEN** capability MUST 释放当前执行路径并继续处理下一个应用

## Public API

- 无稳定 Rust 库 API
- 对外能力体现为批处理章节内核行为

## Build Configuration

- `build.rs` 将 `linker::SCRIPT` 写入 `OUT_DIR/linker.ld`
- 构建依赖 `APP_ASM` 环境变量引入用户程序汇编映像
- `LOG` 环境变量影响日志输出配置

## Dependencies

- `linker`
- `kernel-context`
- `syscall` with `kernel`
- `rcore-console`
