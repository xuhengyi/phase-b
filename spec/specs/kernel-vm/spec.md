# Capability: kernel-vm

## Purpose

`kernel-vm` capability 用于为教程内核提供基于 Sv39 页表的地址空间抽象，并允许上层内核通过 `PageManager` 接口接入具体页分配与页引用计数策略。

## Requirements

### Requirement: 提供地址空间抽象

`kernel-vm` capability SHALL 提供 `AddressSpace` 类型，用于表示可映射、可查询、可复制的内核或用户地址空间。

#### Scenario: 章节内核创建用户地址空间

- **WHEN** 章节内核需要为某个进程创建地址空间
- **THEN** capability MUST 允许它基于给定的 `PageManager` 构造并维护该地址空间

### Requirement: 提供映射与查询能力

`kernel-vm` capability SHALL 支持映射外部页、分配新页、查询虚拟地址翻译结果和访问根页表标识。

#### Scenario: 进程加载 ELF 段

- **WHEN** 内核把某段用户程序内容映射进地址空间
- **THEN** capability MUST 提供相应的映射接口并允许后续按虚拟地址查询翻译结果

#### Scenario: 映射权限影响后续翻译结果

- **WHEN** 某个地址空间以特定权限映射一段虚拟页范围
- **THEN** capability MUST 仅在请求权限与该映射兼容时返回翻译结果

#### Scenario: 外部映射可共享原页内容

- **WHEN** 调用方通过 `map_extern` 将已有物理页映射到另一地址空间
- **THEN** capability MUST 让两个地址空间观察到同一底层页内容

### Requirement: 允许通过 `PageManager` 注入页管理策略

`kernel-vm` capability SHALL 通过 `PageManager` trait 把物理页的分配、回收和元数据管理职责交给使用方。

#### Scenario: 地址空间释放根页表

- **WHEN** 某个地址空间结束生命周期
- **THEN** capability MUST 通过 `PageManager` 接口回收其根页表和相关页资源

#### Scenario: 克隆地址空间获得独立副本

- **WHEN** 调用方将一个地址空间内容克隆到另一个地址空间
- **THEN** capability MUST 为目标地址空间建立独立页副本，使后续对源地址空间的写入不会回写到克隆结果

## Public API

- `AddressSpace`
- `PageManager`
- `page_table` re-exports

## Build Configuration

- 无 `build.rs`
- 默认围绕 Sv39 页表实现

## Dependencies

- `page-table`
- `alloc`
