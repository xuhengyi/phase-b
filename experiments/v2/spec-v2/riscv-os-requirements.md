# 完整独立 RISC-V OS 共同生成契约

- Inputs: `experiment-design.md`, `experiment-execution-plan.md`, `run-manifests/full-ch2-ch8.yaml`, `inputs/user-tests-manifest.json`
- Prompt version: redo-step1-riscv-os-contract.v1
- Model/Codex version: Codex session with local `codex-cli 0.130.0`
- Date: 2026-05-09

## 目标

本文件补强 Step 1 的语言无关 spec，使 Step 2 生成的 RustOS/COS 不再退化为 host conformance runner、最小 oracle kernel 或 embedded-snippet snapshot。RustOS/COS 必须是独立、完整、可在 QEMU 上运行的 RISC-V 教学 OS，并与 tg-rcore 在同一章、同一批用户态测例上对齐。

## 必须满足的产物门槛

每个 `generated/<impl>/<chapter>/` 至少满足：

1. `./test.sh base` 构建 RISC-V 目标产物并调用 `qemu-system-riscv64`。
2. 构建目标是 RISC-V 裸机或等价内核目标，例如 RustOS 的 `riscv64gc-unknown-none-elf`，COS 的 `riscv64-unknown-elf-*` 工具链。
3. 章节快照有内核启动入口、栈初始化、BSS 清零或等价初始化、控制台输出路径和关机路径。
4. 涉及用户程序的章节必须有独立用户程序构建和装载/执行路径，并通过 trap/syscall ABI 接收用户态 `ecall`。
5. `ch3` 之后必须具备章节要求的任务切换、时钟/yield 或调度语义。
6. `ch4` 之后必须具备章节要求的地址空间、用户缓冲区访问和异常处理语义。
7. `ch5` 之后必须具备章节要求的进程、fork/exec/wait/exit 语义。
8. `ch6` 之后必须具备章节要求的文件系统 I/O 与 fd table 语义。
9. `ch7` 之后必须具备章节要求的 pipe/signal 语义。
10. `ch8` 之后必须具备章节要求的线程和同步原语语义。

## 完整独立性门槛

RustOS 与 COS 必须分别形成独立 OS 实现：

- RustOS 不能复制 tg-rcore 章节源码，也不能依赖 tg-rcore 内部 crate/module 作为内核实现。
- COS 不能复制 RustOS 或 tg-rcore 的实现源码，也不能用宿主机程序代替内核。
- 两者可以读取同一份 `spec-v2`、bundle、manifest、用户测试清单、失败日志和当前目录中显式声明的 candidate template。
- 两者都必须有自己的 kernel entry、trap/syscall dispatch、用户程序装载、运行时状态对象和章节子系统。
- 如果采用 chapter snapshot 组织方式，每个 snapshot 仍必须是该章完整教学 OS；如果采用递进式 workspace，报告必须说明 `chN` 继承了哪些已实现子系统。

## 明确无效的产物

以下产物不得计入 Step 2/3 成功：

- 在宿主机运行的 `cargo run`、`make run`、普通 ELF 或脚本。
- 直接把 oracle 文本打印给 checker 的 host 程序。
- 只是在 QEMU 中打印 oracle 文本、但没有对应 syscall/trap/process/file/thread 语义的伪内核。
- 手写 embedded user snippets 只为了输出 checker 字符串，而没有独立用户程序构建/装载机制。
- 没有 general app loader/app image 格式、却声称通过 ch2/ch3/ch4 用户程序批处理语义。
- 只实现单章最小演示，不能扩展为完整 `ch2-ch8` OS 线，却声称 RustOS/COS 生成完成。
- 使用 tg-rcore 源码路径作为生成时输入，或复制 tg-rcore 章节源码后声称为独立生成。

如果只能得到上述产物，必须在 report 中记录为失败尝试或 `inconclusive`，不能写成 RustOS/COS 通过。

## 允许的输入

生成/修复 RustOS/COS 时允许读取：

- `spec-v2/`
- `bundles/<chapter>.bundle.md`
- 当前 `generated/<impl>/<chapter>/` 中的失败尝试
- 当前 implementation/chapter 的构建或测试日志
- 当前目录内显式标注为 candidate template 的文件

生成/修复阶段禁止读取：

- `tg-rcore-tutorial/tg-rcore-tutorial-ch*/src/*`
- tg-rcore 章节实现细节
- Step 4 的 canonical spec diff

## 报告要求

每个 chapter report 必须包含：

- bundle path 与 hash
- RISC-V toolchain 与 target
- QEMU command
- kernel entry
- app image/user program loader
- trap/syscall entry
- user app loading/execution path
- chapter-specific runtime state and subsystem status
- 测试结果与失败日志
- 未完成语义与风险

## Evidence

- `experiment-design.md`
- `experiment-execution-plan.md`
- `inputs/user-tests-manifest.json`
- `run-manifests/full-ch2-ch8.yaml`
- `logs/test-tgrcore-ch2-base.log` 到 `logs/test-tgrcore-ch8-base.log`

## Uncertainty

当前文件定义的是生成验收门槛，不证明 RustOS/COS 已实现该门槛。Step 2/3 必须通过实际构建和 QEMU 测试验证。
