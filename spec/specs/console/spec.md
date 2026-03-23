# Capability: console

## Purpose

`console` capability 用于为教程内核提供一个全局控制台输出入口，并把格式化输出与日志级别控制统一到同一个控制台单例上。

## Requirements

### Requirement: 提供统一的控制台抽象

`console` capability SHALL 定义一个可被内核实现方接入的 `Console` trait，并允许在运行时初始化全局控制台输出对象。

#### Scenario: 章节内核初始化控制台

- **WHEN** 章节内核在启动阶段调用 `init_console`
- **THEN** capability MUST 将后续格式化输出路由到该控制台对象

### Requirement: 提供格式化输出接口

`console` capability SHALL 提供 `_print`、`print!` 和 `println!` 形式的格式化输出接口，使内核代码可以在不直接接触底层设备细节的情况下输出文本。

#### Scenario: 内核打印启动信息

- **WHEN** 内核代码调用 `println!`
- **THEN** capability MUST 将格式化内容发送到已初始化的全局控制台

#### Scenario: `println!` 自动补换行

- **WHEN** 调用方使用 `println!` 输出一行文本
- **THEN** capability MUST 在输出末尾附加换行符

### Requirement: 支持日志级别控制

`console` capability SHALL 支持通过 `set_log_level` 调整日志过滤级别，并与 `log` 接口配合工作。

#### Scenario: 内核调整日志级别

- **WHEN** 启动代码设置日志级别
- **THEN** capability MUST 按设定级别输出或抑制日志消息

#### Scenario: 日志消息通过控制台输出

- **WHEN** 调用方把日志级别设置到足以放行某条消息的等级并发送日志
- **THEN** capability MUST 通过当前控制台输出该日志消息

## Public API

- `Console`
- `init_console`
- `set_log_level`
- `_print`
- `print!`
- `println!`

## Build Configuration

- 无 `build.rs`
- 依赖使用方提供具体控制台实现对象

## Dependencies

- `log`
- `spin`
