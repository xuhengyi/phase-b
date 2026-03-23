# 项目上下文

## 项目目标
本仓库是模块化后的 `rCore-Tutorial` 工作区。
它的目标不是发明新的内核架构，而是在保持章节式操作系统教程流程的前提下，提升教程组织方式、模块边界和学习体验。

## 技术栈
- Rust 2021
- `no_std` 内核与支撑 crate
- RISC-V 64 裸机目标：`riscv64gc-unknown-none-elf`
- 使用 QEMU 做章节级运行验证
- Cargo workspace，包含章节内核、基础库、用户程序和 host 侧工具
- 当前实验直接以 `phase-b/spec/` 目录中的规格文件作为生成输入

## 项目约定

### 代码风格
- 优先采用适合教学场景的直接、低抽象 Rust 写法。
- 除非实验或重构明确要求，否则保留现有 public API 与 crate 名称。
- spec 中的规范性语句使用 `SHALL` 或 `MUST`。
- 为了保证 spec 结构稳定，结构标记保持英文：如 `## Purpose`、`## Requirements`、`### Requirement:`、`#### Scenario:`。
- 除结构标记和规范关键词外，spec 正文默认使用中文撰写。
- 为了支持自动生成流程，允许在每个 capability 目录下增加 `autogen.yaml` 作为机器可读 sidecar。

### 架构模式
- `ch1` 到 `ch8` 是教程章节对应的内核二进制 crate。
- `linker`、`kernel-context`、`kernel-alloc`、`kernel-vm`、`syscall`、`sync`、`task-manage` 等基础 crate 提供可复用的运行时构件。
- 简单 crate 可以直接对应一个 capability；复杂 crate 不应被强行按“一 crate 一 spec”建模，必要时需要 capability 级拆分。
- 对复杂基础 crate，允许同时存在“crate-level 聚合 spec”与“capability-level 子 spec”；前者负责总览与组合关系，后者负责更细的稳定契约。

### 测试策略
- spec 第一轮抽取的主要输入是源码与原始设计文档。
- tests 对 spec 很重要，但在实验流程中属于第二轮补强来源，而不是第一轮契约来源。
- 构建检查、章节运行和单元测试用于理解当前行为，但抽取出的 spec 不应过度拟合测试桩或临时实现细节。

### Git 工作流
- 以干净的上游仓库状态作为实验输入基线。
- 以 `phase-b/spec/` 作为当前实验的直接规格输入。
- 人工实验记录、SOP 和中间材料应与最终 spec 分开存放。

## 领域上下文
- 这个项目既是一个教程，也是一个模块化的操作系统代码库。
- `docs/design/` 下的原始设计文档是有效的规格抽取输入。
- 章节内核会组合多个底层 crate，因此通常应先写基础 crate 的 spec，再写章节级 spec。

## 重要约束
- 第一轮抽取可以有意忽略 tests，以保持初始 spec 聚焦于源码级契约。
- 对复杂 crate 的 capability 边界必须显式决策，不能完全交给 AI 自动决定。
- 后续所有 spec 默认采用“正文中文 + 英文结构标记”的写法。

## 外部依赖
- Rust toolchain 与 Cargo
- QEMU（`qemu-system-riscv64`），用于集成运行
- OpenSpec CLI（可选，仅用于维护或再生成 spec，不参与当前 Phase B 代码生成闭环）
