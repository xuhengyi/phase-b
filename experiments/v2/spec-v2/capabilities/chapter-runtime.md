# Capability: chapter-runtime

- Inputs: `run-manifests/full-ch2-ch8.yaml`, `spec-v2/architecture/boot-trap-syscall.md`, `spec-v2/architecture/app-image-loader.md`
- Prompt version: redo-step1-complete-os-capability.v1
- Model/Codex version: Codex session with local `codex-cli 0.130.0`
- Date: 2026-05-09

## Contract

每个 chapter snapshot 都必须是可在 QEMU RISC-V 上启动的独立教学 OS 状态。它可以按章节目录组织，也可以是递进式 workspace 的一个目标；但必须有真实内核入口、用户程序装载、U/S trap/syscall 路径和本章所需运行态。

## Observable Requirements

- `./test.sh base` 构建 RISC-V 内核或镜像并运行 `qemu-system-riscv64`。
- 输出来自 QEMU 内运行的内核和用户程序，不来自宿主机 oracle printer。
- 章节 oracle 的 required patterns 至少出现一次，forbidden patterns 不出现。
- report 说明 kernel entry、trap entry、app image/user loader、syscall dispatch、chapter runtime state 和测试日志。

## Invalid Artifacts

- host binary、host runner、host-side oracle printer。
- 只在 QEMU 打印 oracle 文本的伪内核。
- 没有独立用户程序构建/装载机制的 embedded snippet kernel。
- 只实现单章演示却声称完成 `ch2-ch8` RustOS/COS。

## Uncertainty

该能力定义运行时完整性门槛，不指定内部源码布局，也不声称形式化等价。
