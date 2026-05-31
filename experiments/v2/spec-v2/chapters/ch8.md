# Chapter Spec: ch8

## Scope

Base L1 execution for inherited process/file/pipe behavior plus thread and synchronization scenarios.

## Required Capabilities

- `chapter-runtime`
- `syscall-abi`
- `process-control`
- `file-system-io`
- `ipc-or-pipe`
- `thread-control`
- `sync-primitives`

## Base Oracle

See `spec-v2/oracles/ch8-base-output.md`.

## Out Of Scope

Deadlock exercise tests are excluded because frozen tg-rcore ch8 exercise times out in this environment.
