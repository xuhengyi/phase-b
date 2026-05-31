# Chapter Spec: ch7

## Scope

Base L1 execution for inherited process/file behavior plus signal and pipe success scenarios.

## Required Capabilities

- `chapter-runtime`
- `syscall-abi`
- `process-control`
- `file-system-io`
- `fd-table`
- `signal-control`
- `ipc-or-pipe`

## Base Oracle

See `spec-v2/oracles/ch7-base-output.md`.

## Out Of Scope

Full signal masks, interruption, pipe edge cases, and formal scheduling guarantees are outside L1.
