# Capability: easy-fs

## Purpose

`easy-fs` capability 用于作为 `easy-fs-storage` 与 `easy-fs-vfs` 的聚合层，描述章节内核如何通过统一 crate 表面接入简化文件系统。

## Requirements

### Requirement: 聚合存储层与 VFS 层能力

`easy-fs` capability SHALL 通过同一 crate 暴露 `easy-fs-storage` 与 `easy-fs-vfs` 两层能力，使章节内核能够同时访问文件系统存储层和 inode/VFS 层接口。

#### Scenario: 章节内核通过统一 crate 接入文件系统

- **WHEN** 章节内核通过 `easy-fs` crate 初始化文件系统并随后访问 inode、目录或文件句柄
- **THEN** capability MUST 同时满足 `easy-fs-storage` 和 `easy-fs-vfs` 所定义的契约

### Requirement: 为章节内核保留统一的文件系统入口表面

`easy-fs` capability SHALL 保留 `EasyFileSystem`、`Inode`、`File`、`OpenFlags` 和 `FSManager` 这一统一入口表面，避免章节内核必须直接依赖多个底层 capability 名称。

#### Scenario: 章节内核以统一入口完成装载和文件访问

- **WHEN** `ch6` 及之后的章节内核通过 `easy-fs` crate 装载用户程序并处理文件相关 syscall
- **THEN** capability MUST 允许这些章节只通过统一 crate 表面访问文件系统能力

## Public API

- `BLOCK_SZ`
- `BlockDevice`
- `EasyFileSystem`
- `Inode`
- `DiskInodeType`
- `OpenFlags`
- `File`
- `FSManager`

## Build Configuration

- 无 `build.rs`
- 依赖使用方提供块设备实现

## Dependencies

- 聚合 `easy-fs-storage`
- 聚合 `easy-fs-vfs`
- `alloc`
- `spin`
- `lazy_static`
- `lru`
- `bitflags`
