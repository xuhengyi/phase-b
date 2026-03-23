# Capability: signal-defs

## Purpose

`signal-defs` capability 用于提供教程信号子系统共享的信号编号、传统信号上界常量和信号处理动作数据结构。

## Requirements

### Requirement: 提供稳定的信号编号集合

`signal-defs` capability SHALL 定义教程内核使用的信号编号枚举，覆盖传统信号与实时信号范围，并以 `MAX_SIG` 表示传统信号区间的上界。

#### Scenario: 上层模块引用标准信号号

- **WHEN** `signal` 或 `signal-impl` 需要引用某个信号编号
- **THEN** capability MUST 提供统一的 `SignalNo` 表示和 `MAX_SIG` 常量

#### Scenario: 实时信号编号仍可被枚举表示

- **WHEN** 调用方引用 `SIGRTMIN` 到 `SIGRT31` 范围内的实时信号
- **THEN** capability MUST 为这些编号提供稳定的枚举值表示

### Requirement: 提供信号处理动作结构

`signal-defs` capability SHALL 定义信号处理动作的数据结构，以供信号实现层记录用户注册的处理方式。

#### Scenario: 进程设置某个信号的处理动作

- **WHEN** 上层模块保存用户定义的处理入口与掩码信息
- **THEN** capability MUST 提供可直接存储这些字段的 `SignalAction`

### Requirement: 支持从整数映射到信号号

`signal-defs` capability SHALL 提供从整数到信号编号的映射规则，并为越界值给出既定的错误信号表示。

#### Scenario: 系统调用解析用户传入信号号

- **WHEN** 上层模块把用户传入的 `usize` 转为 `SignalNo`
- **THEN** capability MUST 对有效编号返回对应信号，对无效编号返回约定的错误值

#### Scenario: 越界编号被映射为错误信号

- **WHEN** 调用方把超出已定义范围的整数转换为 `SignalNo`
- **THEN** capability MUST 返回 `SignalNo::ERR`

## Public API

- `SignalNo`
- `SignalAction`
- `MAX_SIG`

## Build Configuration

- 无 `build.rs`

## Dependencies

- `bitflags`
