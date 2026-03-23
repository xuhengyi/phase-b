# Capability: kernel-alloc

## Purpose

`kernel-alloc` capability 用于为教程内核提供一个基于 buddy allocator 的全局堆分配器，并允许在启动后继续向堆中转移新的可用内存区域。

## Requirements

### Requirement: 提供显式初始化入口

`kernel-alloc` capability SHALL 提供 `init` 接口，以便章节内核在已知堆基地址的前提下初始化全局堆分配器。

#### Scenario: 章节内核初始化堆

- **WHEN** 启动代码在确定堆起点后调用 `init`
- **THEN** capability MUST 建立可用于后续动态分配的初始空闲区域

### Requirement: 支持追加可用内存区域

`kernel-alloc` capability SHALL 提供 `transfer` 接口，使调用方能够在初始化之后把额外的连续内存区域加入堆管理。

#### Scenario: 内核把剩余物理页转入堆

- **WHEN** 内核拿到一段新的可分配内存区域并调用 `transfer`
- **THEN** capability MUST 把该区域纳入 buddy allocator 的可分配范围

### Requirement: 在正常构建中作为全局分配器工作

`kernel-alloc` capability SHALL 在非测试构建中将其 buddy allocator 暴露为全局分配器，以支持 `alloc` 生态的动态内存分配。

#### Scenario: 内核代码申请动态内存

- **WHEN** 教程内核在正常构建中触发全局分配
- **THEN** capability MUST 从已初始化并已转入的堆区域中分配内存或报告分配失败

#### Scenario: 多次分配返回互不重叠的有效区间

- **WHEN** 调用方在已初始化的堆上连续申请多个仍处于存活状态的对象
- **THEN** capability MUST 返回可同时使用且互不重叠的内存区间

## Public API

- `init`
- `transfer`

## Build Configuration

- 无 `build.rs`
- 正常构建是 `no_std`
- 测试构建可切换到宿主分配器，但这不属于教程内核运行时契约

## Dependencies

- `alloc`
- `customizable-buddy`
