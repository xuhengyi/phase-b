# Capability: fd-table

## Metadata

- id: fd-table
- introduced_in: ch6
- applies_to: ch6, ch7, ch8
- prompt_version: direct-codex-smoke-v1
- model_or_codex: current Codex session
- date: 2026-05-09

## Purpose

Define user-visible file descriptor behavior for ch6 file operations and process interactions.

## Public Surface

- Standard descriptors: `0` input, `1` output, `2` debug/error output.
- File descriptors returned by `open`.
- Descriptor-consuming calls: `read`, `write`, `close`, `fstat`.

## State Model

- Each process has a descriptor table.
- Descriptor entries may be open or closed.
- An open file descriptor references a file handle with readability, writability, and current offset.
- ch6 fork creates a child with inherited open descriptors.
- ch6 exec preserves descriptor table while replacing the program image.

## Requirements

### Requirement: standard descriptors exist for new processes

- introduced_in: ch6
- applies_to: ch6
- requirement: New processes start with input descriptor `0`, output descriptor `1`, and debug/error descriptor `2` available according to their read/write roles.
- observable_behavior: Existing console tests continue to print, and programs can read from standard input where required.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/process.rs:188`
  - `tg-rcore-tutorial/tg-rcore-tutorial-syscall/src/io.rs:5`

### Requirement: open allocates a descriptor entry

- introduced_in: ch6
- applies_to: ch6
- requirement: A successful `open` returns a descriptor that can be passed to `read`, `write`, and `close` by the same process.
- observable_behavior: `filetest_simple` uses the returned descriptor for write/close and a later descriptor for read/close.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/main.rs:555`
  - `user/src/bin/filetest_simple.rs:16`

### Requirement: descriptor access checks open state and direction

- introduced_in: ch6
- applies_to: ch6
- requirement: Reads require a readable open descriptor; writes require a writable open descriptor; invalid or closed descriptors fail.
- observable_behavior: Read/write paths return negative values for invalid descriptors or wrong direction.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/main.rs:472`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/main.rs:513`
  - `tg-rcore-tutorial/tg-rcore-tutorial-easy-fs/src/file.rs:143`

### Requirement: file offset advances per open handle

- introduced_in: ch6
- applies_to: ch6
- requirement: Each successful read or write advances the file handle offset by the number of bytes transferred.
- observable_behavior: Sequential reads eventually reach `0`; writes append or overwrite at the current handle offset according to the opened handle's offset.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-easy-fs/src/file.rs:153`
  - `tg-rcore-tutorial/tg-rcore-tutorial-easy-fs/src/file.rs:172`
  - `user/src/bin/cat_filea.rs:21`

### Requirement: fork inherits descriptors

- introduced_in: ch6
- applies_to: ch6
- requirement: A forked child inherits the parent descriptor table entries.
- observable_behavior: This is a required state transition for process/file interaction even when the current smoke tests do not isolate it with a dedicated case.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/process.rs:71`

## Out Of Scope

- Descriptor duplication APIs.
- Close-on-exec flags.
- Full concurrent descriptor sharing semantics beyond the teaching model.

## Uncertainty

- The source evidence clones file handles during fork. Whether offset sharing across fork is intended or accidental should be checked in Phase D canonical comparison if a caller exercises it.
