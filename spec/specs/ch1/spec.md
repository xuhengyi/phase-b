# Capability: ch1

## Purpose

`ch1` capability 表示教程第一章的最小内核二进制：它负责完成最基础的启动、控制台初始化、文本输出以及通过 SBI 结束执行。

## Requirements

### Requirement: 生成可用于启动的最小章节内核

`ch1` capability SHALL 构建出一个可在教程目标平台上启动的最小内核，并完成早期入口到 Rust 主函数的连接。

#### Scenario: 第一章内核启动

- **WHEN** 使用方构建并运行 `ch1`
- **THEN** capability MUST 通过既定启动入口进入 `rust_main`

### Requirement: 初始化控制台并输出启动信息

`ch1` capability SHALL 初始化控制台输出路径，并在进入主逻辑后输出可见的文本信息。

#### Scenario: 第一章内核打印问候信息

- **WHEN** `rust_main` 完成早期初始化
- **THEN** capability MUST 能够通过控制台输出启动消息

### Requirement: 通过 SBI 结束执行

`ch1` capability SHALL 在演示逻辑完成后通过 SBI 关机路径结束当前内核执行。

#### Scenario: 第一章内核结束

- **WHEN** 第一章演示逻辑完成
- **THEN** capability MUST 调用 SBI 退出或关机接口

## Public API

- 无稳定 Rust 库 API
- 对外能力体现为章节内核二进制行为

## Build Configuration

- `build.rs` 将 `linker::SCRIPT` 写入 `OUT_DIR/linker.ld`
- 依赖 `linker::boot0!` 生成启动入口

## Dependencies

- `linker`
- `rcore-console`
- `sbi-rt`
