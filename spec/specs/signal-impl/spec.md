# Capability: signal-impl

## Purpose

`signal-impl` capability 用于为教程内核提供 `signal::Signal` trait 的默认实现，负责维护进程的待处理信号、屏蔽集合、处理动作和当前处理状态。

## Requirements

### Requirement: 提供可维护的信号状态对象

`signal-impl` capability SHALL 提供一个 `SignalImpl` 类型，用于统一保存待处理信号集合、屏蔽集合、已注册动作和当前处理状态。

#### Scenario: 进程初始化自己的信号状态

- **WHEN** 章节内核为某个进程创建信号子系统对象
- **THEN** capability MUST 生成一个可后续接收、屏蔽和处理信号的状态实例

#### Scenario: 清空后动作表恢复默认状态

- **WHEN** 调用方对某个 `SignalImpl` 执行 `clear`
- **THEN** capability MUST 丢弃已安装的处理动作并恢复默认动作配置

### Requirement: 实现标准信号接口

`signal-impl` capability SHALL 实现 `signal::Signal` trait，使上层内核能够通过统一接口递送信号、更新掩码、设置动作并推进信号处理流程。

#### Scenario: 内核向进程递送一个信号

- **WHEN** 上层模块调用信号递送接口
- **THEN** capability MUST 更新待处理集合，并在后续查询时反映出新的处理结果

#### Scenario: 默认忽略与默认终止信号分支可区分

- **WHEN** 当前待处理信号分别落在默认忽略和默认终止两类动作上
- **THEN** capability MUST 产生与这两类默认动作一致的 `SignalResult`

#### Scenario: 停止后可被继续信号恢复

- **WHEN** 进程因停止类信号进入暂停状态并随后收到继续执行信号
- **THEN** capability MUST 允许该进程从暂停状态恢复

### Requirement: 支持 fork 继承与处理动作恢复

`signal-impl` capability SHALL 支持从父进程复制必要的信号状态，并在信号处理返回时恢复正常执行状态。

#### Scenario: 进程 fork 后继承信号配置

- **WHEN** 内核从父进程派生一个子进程
- **THEN** capability MUST 复制规范中允许继承的信号配置，并为子进程建立独立的运行时状态

#### Scenario: fork 不继承父进程当前待处理信号

- **WHEN** 父进程在存在待处理信号时派生子进程
- **THEN** 子进程 MUST 继承掩码和动作配置，但 MUST NOT 继承父进程当前待处理的信号集合

#### Scenario: 进入用户 handler 后可通过 `sig_return` 恢复原上下文

- **WHEN** 某个用户态信号处理函数已接管当前上下文并随后调用 `sig_return`
- **THEN** capability MUST 恢复进入 handler 之前保存的上下文并退出“正在处理信号”状态

## Public API

- `SignalImpl`
- `HandlingSignal`

## Build Configuration

- 无 `build.rs`

## Dependencies

- `signal`
- `signal-defs`
