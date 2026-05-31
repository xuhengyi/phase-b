# Capability: process-control

## Metadata

- id: process-control
- introduced_in: ch5
- applies_to: ch5, ch6, ch7, ch8
- prompt_version: direct-codex-smoke-v1
- model_or_codex: current Codex session
- date: 2026-05-09

## Purpose

Specify user-visible process lifecycle behavior for creation, termination, waiting, process identity, and program image replacement.

## Public Surface

- `exit(exit_code)`
- `fork()`
- `exec(path)`
- `wait(exit_code_ptr)`
- `waitpid(pid, exit_code_ptr)`
- `getpid()`
- `spawn(path)` as ch5 exercise extension

## State Model

- `process`: an executing user program with a process id and user address space.
- `parent_child_relation`: a relation from parent process to live children and exited children awaiting collection.
- `exit_status`: signed status recorded when a child exits.
- `program_image`: executable content and initial user context for a process.
- `program_break`: heap boundary used by `sbrk`.

## Requirements

### Requirement: process exit records status

- introduced_in: ch5
- applies_to: ch5, ch6
- requirement: `exit(code)` terminates the current process and records `code` for its parent to collect.
- observable_behavior: Parent `wait` or `waitpid` obtains the child pid and the exit code.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch5/src/main.rs:261`
  - `tg-rcore-tutorial/tg-rcore-tutorial-task-manage/src/proc_manage.rs:57`
  - `tg-rcore-tutorial/tg-rcore-tutorial-task-manage/src/proc_rel.rs:29`
  - `user/src/bin/fork_exit.rs:22`

### Requirement: wait distinguishes reaped, running, and absent children

- introduced_in: ch5
- applies_to: ch5, ch6
- requirement: `wait` waits for any child; `waitpid` waits for a selected child. A completed child yields its pid and exit code. No matching child yields `-1`. A matching child that is still running may yield transient `-2`, which the user wrapper retries after yielding.
- observable_behavior: Parent programs can reap all children once and later observe no remaining child.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-syscall/src/user.rs:129`
  - `tg-rcore-tutorial/tg-rcore-tutorial-syscall/src/user.rs:142`
  - `tg-rcore-tutorial/tg-rcore-tutorial-task-manage/src/proc_rel.rs:41`
  - `tg-rcore-tutorial/tg-rcore-tutorial-task-manage/src/proc_rel.rs:55`
  - `user/src/bin/12forktest.rs:26`
  - `user/src/bin/forktest_simple.rs:14`

### Requirement: process identity is user-visible

- introduced_in: ch5
- applies_to: ch5, ch6
- requirement: `getpid()` returns the current process id as a non-negative user-visible identifier.
- observable_behavior: Tests print process ids and use child pid returns for waiting.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch5/src/main.rs:635`
  - `user/src/bin/13forktree.rs:31`

### Requirement: exec replaces program image without changing process identity

- introduced_in: ch5
- applies_to: ch5, ch6
- requirement: `exec(path)` loads the target program and replaces the current user program image; on success it returns `0` to the syscall caller path, while subsequent execution follows the new image.
- chapter_overrides:
  ch5: target program is selected from the chapter application set.
  ch6: target program is selected from the file system by name.
- observable_behavior: Shell/init style programs can run named user programs.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch5/src/process.rs:60`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch5/src/main.rs:584`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/main.rs:641`

### Requirement: spawn exercise creates a child process from a named program

- introduced_in: ch5
- applies_to: ch5, ch6
- requirement: `spawn(path)` creates a new child process that starts at the target program, returns child pid on success, and returns `-1` on invalid target.
- status: exercise_extension
- observable_behavior: `ch5_spawn0`, `ch5_spawn1`, and ch6 exercise regression expect spawn-created children to be waitable.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch5/exercise.md:11`
  - `user/src/bin/ch5_spawn0.rs:16`
  - `user/src/bin/ch5_spawn1.rs:14`
- uncertainty: current checked-in ch5/ch6 implementation logs this operation as not implemented and returns `-1`.

## Out Of Scope

- Exact internal process table representation.
- Full signal, thread, or session semantics.
- Full POSIX process semantics not exercised or specified by teaching material.

## Uncertainty

- The current source tree contains exercise TODOs. Base process-control requirements are backed by implementation and tests; spawn is exercise-spec evidence rather than confirmed base implementation evidence.
