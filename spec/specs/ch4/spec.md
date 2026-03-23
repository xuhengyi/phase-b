# Capability: ch4

## Purpose

`ch4` capability 表示教程第四章的进程/地址空间内核：它在前几章基础上引入堆分配、虚拟内存、ELF 装载以及跨地址空间执行所需的中转机制。

## Requirements

### Requirement: 为用户程序建立独立地址空间

`ch4` capability SHALL 为每个用户程序创建独立的地址空间，并把 ELF 段、用户栈、陷入上下文等内容映射到约定位置。

#### Scenario: 内核从 ELF 创建进程

- **WHEN** 内核基于用户程序 ELF 构造新进程
- **THEN** capability MUST 为其建立可执行的用户地址空间并准备初始上下文

### Requirement: 支持跨地址空间执行与返回

`ch4` capability SHALL 借助 `kernel-context` 的 foreign 机制在不同地址空间之间中转执行用户程序，并在陷入时回到内核调度逻辑。

#### Scenario: 进程从内核切换到用户态再陷回

- **WHEN** 调度器选中某个拥有独立地址空间的进程
- **THEN** capability MUST 能够切换到该地址空间执行，并在陷入后恢复内核侧处理流程

### Requirement: 接入内存与进程相关系统调用

`ch4` capability SHALL 提供与地址空间、内存区间和进程执行相关的系统调用实现基础。

#### Scenario: 用户程序查询或修改自身内存相关状态

- **WHEN** 用户程序触发内存或进程相关系统调用
- **THEN** capability MUST 使用当前进程的地址空间和上下文信息完成处理

## Public API

- 无稳定 Rust 库 API
- 对外能力体现为支持进程地址空间的章节内核行为

## Build Configuration

- `build.rs` 将 `linker::SCRIPT` 写入 `OUT_DIR/linker.ld`
- 构建依赖 `APP_ASM` 与 `LOG`
- 依赖 foreign portal 相关能力的目标架构假设

## Dependencies

- `kernel-alloc`
- `kernel-vm`
- `kernel-context` with `foreign`
- `syscall` with `kernel`
- `xmas-elf`
