# Chapter Spec: ch6

## Metadata

- chapter: ch6
- title: file system and fd table
- prompt_version: direct-codex-smoke-v1
- model_or_codex: current Codex session
- date: 2026-05-09
- inputs: `spec-v2/inventory/ch6.yaml`, `spec-v2/capabilities/file-system-io.md`, `spec-v2/capabilities/fd-table.md`

## Scope

Base scope extends ch5 process behavior with a teaching file system, per-process descriptor table, and `open`/`close`/`read`/`write` file operations.

Exercise extension scope adds `fstat`, `link`, and `unlink`, plus inherited exercise process creation requirements. Current source evidence marks the file metadata and hard-link operations as TODO in the checked-in implementation.

## Must Pass Base Cases

- `filetest_simple`
- `cat_filea`
- inherited ch5 base process cases
- inherited console, power, sleep, and address-space smoke cases listed in `inputs/user-tests-manifest.json`

## Required Capabilities

- `syscall-abi@ch6`
- `process-control@ch6`
- `fork-exec-wait@ch6`
- `file-system-io@ch6`
- `fd-table@ch6`

## Chapter Requirements

### Requirement: user programs are loadable from the file system

- introduced_in: ch6
- applies_to: ch6
- requirement: The chapter loads executable program bytes by file name from the teaching file system.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/README.md:158`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/main.rs:641`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/fs.rs:96`

### Requirement: file syscall path uses the descriptor table

- introduced_in: ch6
- applies_to: ch6
- requirement: File `read`, `write`, and `close` operate on descriptors in the current process descriptor table.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/main.rs:457`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/main.rs:497`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/main.rs:573`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/process.rs:188`

### Requirement: base file read/write is content preserving

- introduced_in: ch6
- applies_to: ch6
- requirement: A file created and written by one open handle can be closed, reopened read-only, read back, and compared byte-for-byte.
- evidence:
  - `user/src/bin/filetest_simple.rs:14`
  - `user/src/bin/ch6_file0.rs:15`

## Exercise Extension

- `fstat(fd, stat_ptr)` reports file type and link count.
- `link(old, new)` creates another name for the same file unless old and new are the same.
- `unlink(path)` removes one directory entry and eventually removes the file when the last link is gone.

## Out Of Scope

- Nested directories.
- Full storage crash consistency.
- Full metadata beyond exercise fields.

## Uncertainty

- Baseline execution has not yet been run in this phase. Phase 2 must confirm base and exercise pass/fail signatures.
