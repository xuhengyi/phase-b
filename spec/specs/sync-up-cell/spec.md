# Capability: sync-up-cell

## Purpose

`sync-up-cell` capability 用于描述 `sync` 中面向单处理器内核临界区的中断屏蔽独占访问单元。

## Requirements

### Requirement: 提供单处理器的独占访问单元

`sync-up-cell` capability SHALL 提供一个带中断屏蔽语义的独占访问单元，用于单处理器内核临界区。

#### Scenario: 在独占会话中修改内部状态

- **WHEN** 调用方进入该单元的独占会话并修改内部值
- **THEN** 该 capability MUST 在会话结束后让修改结果保持可见

### Requirement: 拒绝重叠的独占借用

`sync-up-cell` capability SHALL 拒绝对同一受保护值发起重叠的可变借用。

#### Scenario: 在未释放第一个 guard 前再次请求独占访问

- **WHEN** 第一个独占 guard 仍然存活时，调用方再次请求独占访问
- **THEN** 该 capability MUST 拒绝第二次借用

## Public API

- `UPIntrFreeCell`
- `UPIntrRefMut`

## Build Configuration

- 无 `build.rs`
- 非 RISC-V 目标上的中断开关分支只用于测试支持

## Dependencies

- `spin`
- `alloc`
