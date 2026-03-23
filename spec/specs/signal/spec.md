# Capability: signal

## Purpose

`signal` capability 用于定义教程信号子系统对外暴露的统一 trait 和处理结果枚举，并重导出信号编号与动作定义。

## Requirements

### Requirement: 提供统一的信号接口 trait

`signal` capability SHALL 定义一个 `Signal` trait，用于抽象信号状态复制、清理、递送、掩码管理和处理动作管理。

#### Scenario: 章节内核持有抽象信号对象

- **WHEN** 上层代码只依赖信号行为而不依赖具体实现
- **THEN** capability MUST 允许它通过 `Signal` trait 访问这些操作

#### Scenario: fork 后继承掩码与处理动作

- **WHEN** 调用方在某个 `Signal` 对象上执行 `from_fork`
- **THEN** 产生的新对象 MUST 继承原对象的信号掩码和处理动作配置

#### Scenario: 更新掩码时返回旧值

- **WHEN** 调用方通过 `update_mask` 写入新的信号掩码
- **THEN** capability MUST 返回更新前的旧掩码值

### Requirement: 提供信号处理结果表示

`signal` capability SHALL 定义 `SignalResult`，用于表达当前是否需要进入信号处理、冻结或继续执行。

#### Scenario: 陷入处理阶段查询信号结果

- **WHEN** 内核在返回用户态之前检查信号状态
- **THEN** capability MUST 通过 `SignalResult` 传达下一步处理分支

#### Scenario: 结果枚举可表达终止与暂停

- **WHEN** 某次信号处理需要表达继续执行以外的控制流结果
- **THEN** capability MUST 能以 `SignalResult` 表示忽略、已处理、终止或暂停等分支

### Requirement: 重导出共享的信号定义

`signal` capability SHALL 重导出 `SignalAction`、`SignalNo` 和 `MAX_SIG`，使上层模块可以通过一个统一依赖入口获取信号接口与数据定义。

#### Scenario: 使用方只依赖 `signal` crate

- **WHEN** 某个模块同时需要信号 trait 和信号号定义
- **THEN** capability MUST 允许它只依赖 `signal` crate 获取这些类型

## Public API

- `Signal`
- `SignalResult`
- `SignalAction`
- `SignalNo`
- `MAX_SIG`

## Build Configuration

- 无 `build.rs`

## Dependencies

- `signal-defs`
