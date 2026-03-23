# Capability: linker

## Purpose

`linker` capability 用于为教程内核提供统一的链接脚本模板、启动入口宏和运行时布局符号访问接口，使章节内核能够在 qemu-virt 的 RISC-V 平台上按约定布局启动并枚举内嵌应用元数据。

## Requirements

### Requirement: 提供统一的链接脚本模板

`linker` capability SHALL 以常量形式暴露可写出的链接脚本模板，使依赖它的章节 crate 可以在构建时生成满足教程内核布局要求的链接脚本文件。

#### Scenario: 章节构建脚本写出链接脚本

- **WHEN** 某个章节 crate 在 `build.rs` 中把 `SCRIPT` 写入 `OUT_DIR/linker.ld`
- **THEN** 生成的链接脚本 MUST 定义教程内核所需的段布局和导出符号

#### Scenario: 链接脚本包含核心段与布局符号

- **WHEN** 调用方读取 `SCRIPT` 文本内容
- **THEN** 模板 MUST 包含教程内核所需的 `.text`、`.rodata`、`.data`、`.bss`、`.boot` 等核心段标记以及布局起止符号

### Requirement: 提供启动入口宏

`linker` capability SHALL 提供 `boot0!` 宏，用于声明启动栈和高级语言入口函数，使章节内核可以以统一方式完成最早期启动。

#### Scenario: 章节内核声明启动入口

- **WHEN** 章节 crate 使用 `boot0!(rust_main; stack = ...)`
- **THEN** capability MUST 生成与教程启动流程兼容的入口和栈定义

### Requirement: 提供布局与应用枚举接口

`linker` capability SHALL 提供内核布局和应用元数据访问接口，使章节内核能够在运行时读取段边界并按顺序遍历内嵌应用。

#### Scenario: 批处理章节枚举应用

- **WHEN** 章节内核通过 `AppMeta` 或 `AppIterator` 读取内嵌应用元数据
- **THEN** capability MUST 按链接后布局提供一致的起止位置与遍历顺序

#### Scenario: 初始布局可枚举固定区域

- **WHEN** 调用方遍历 `KernelLayout::INIT`
- **THEN** capability MUST 暴露文本段、只读段、数据段和启动段等既定区域标题及其范围

#### Scenario: 应用被复制到固定槽位

- **WHEN** `AppIterator` 以固定槽位模式装载应用映像
- **THEN** capability MUST 按顺序复制每个应用内容，并将槽位尾部未覆盖部分清零

## Public API

- `SCRIPT`
- `boot0!`
- `KernelLayout`
- `KernelRegion`
- `AppMeta`
- `AppIterator`

## Build Configuration

- 本 crate 自身没有 `build.rs`
- 典型用法是由依赖它的章节 crate 在 `build.rs` 中将 `SCRIPT` 写入 `OUT_DIR`

## Dependencies

- 依赖 `core`
- 不要求额外的运行时服务
