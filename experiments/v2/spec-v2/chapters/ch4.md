# Chapter Spec: ch4

## Scope

Base L1 execution for write and sbrk scenarios.

## Required Capabilities

- `chapter-runtime`
- `task-switching`
- `address-space`
- `user-buffer`

## Base Oracle

See `spec-v2/oracles/ch4-base-output.md`.

## Out Of Scope

Exercise trace, mmap, and munmap behavior are excluded because frozen tg-rcore fails the ch4 exercise checker.
