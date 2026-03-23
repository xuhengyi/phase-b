# Capability: easy-fs-vfs

## Purpose

`easy-fs-vfs` capability 用于描述 `easy-fs` 中与 inode 命名空间、目录与文件操作、文件句柄偏移语义和打开标志语义相关的 VFS 层能力。

## Requirements

### Requirement: 提供基于 inode 的命名空间操作

`easy-fs-vfs` capability SHALL 提供面向 inode 的目录与文件操作，包括创建、查找、读取目录项、清空文件内容，以及按偏移读写。

#### Scenario: 在目录中创建并发现文件

- **WHEN** 调用方在某个目录 inode 下创建文件，并随后再次查询同一目录
- **THEN** 该 capability MUST 允许这些文件通过 inode 命名空间接口被查找到并列出

### Requirement: 提供顺序访问的文件句柄语义

`easy-fs-vfs` capability SHALL 提供文件句柄语义，用于维护顺序访问偏移，并暴露基于打开标志的读写意图。

#### Scenario: 顺序写入后再从头顺序读取

- **WHEN** 调用方通过文件句柄写入数据，并随后把偏移重置到起始位置进行读取
- **THEN** 该 capability MUST 保持文件句柄偏移语义，并按原顺序返回写入的数据

## Public API

- `Inode`
- `File`
- `FileHandle`
- `OpenFlags`
- `FSManager`
- `DiskInodeType`

## Build Configuration

- 无 `build.rs`

## Dependencies

- `alloc`
- `bitflags`
