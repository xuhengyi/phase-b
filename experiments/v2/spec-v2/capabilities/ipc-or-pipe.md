# Capability: ipc-or-pipe

- Inputs: `inputs/user-tests-manifest.json`, `spec-v2/architecture/subsystem-contracts.md`
- Prompt version: redo-step1-complete-os-capability.v1
- Model/Codex version: Codex session with local `codex-cli 0.130.0`
- Date: 2026-05-09

## Contract

`ch7/ch8` 必须把 pipe 实现为内核 fd 对象，而不是测试专用输出路径。pipe fd 可以被 `read/write/close` 使用，并与进程调度、fork 继承和 EOF 语义交互。

## Observable Requirements

- ch7 base emits `pipetest passed!` and `pipe_large_test passed!`.
- ch8 base emits `pipetest passed!`.
- 写端写入的字节按 FIFO 顺序被读端读取。
- 无数据但写端仍存在时，读可以阻塞、调度等待，或返回 `-2` 让用户库 yield 后重试。
- 写端全部关闭后，读端最终返回 `0`。
- 缓冲区满时，写可以阻塞、调度等待，或返回 `-2` 让用户库 yield 后重试。

## Required Runtime State

- pipe buffer with read/write cursors.
- reader/writer reference counts.
- fd entries referencing pipe read end and write end.
- waiting or retry state integrated with scheduler.

## Out Of Scope

All possible signal interruption cases and exact buffer capacity are outside L1 unless a later canonical spec target selects them.
