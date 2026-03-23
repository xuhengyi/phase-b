# Capability: easy-fs-storage

## Purpose

`easy-fs-storage` capability 用于描述 `easy-fs` 中与固定块大小、块设备访问、文件系统创建/重开以及磁盘持久状态相关的存储层能力。

## Requirements

### Requirement: 提供固定块大小的存储访问层

`easy-fs-storage` capability SHALL 围绕固定逻辑块大小和块设备抽象定义存储访问层，用于承载文件系统元数据和数据内容的读写。

#### Scenario: 以完整逻辑块读写存储

- **WHEN** 文件系统存储层读取或写入某个逻辑块
- **THEN** 它 MUST 按 `easy-fs` 约定的固定块大小解释该操作

### Requirement: 提供文件系统实例的创建与重开语义

`easy-fs-storage` capability SHALL 支持在块设备上创建文件系统实例，并在后续重新打开该实例时保留先前已经写入磁盘的状态。

#### Scenario: 创建目录项后重新打开文件系统

- **WHEN** 调用方创建一个文件系统实例、写入目录元数据，并稍后在同一块设备上重新打开文件系统
- **THEN** 重新打开后的实例 MUST 能观察到先前持久化下来的目录状态

## Public API

- `BLOCK_SZ`
- `BlockDevice`
- `EasyFileSystem::create`
- `EasyFileSystem::open`
- `EasyFileSystem::root_inode`

## Build Configuration

- 无 `build.rs`
- 依赖使用方提供块设备实现

## Dependencies

- `alloc`
- `spin`
- `lazy_static`
- `lru`
