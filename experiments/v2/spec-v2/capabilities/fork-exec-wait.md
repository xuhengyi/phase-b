# Capability: fork-exec-wait

## Metadata

- id: fork-exec-wait
- introduced_in: ch5
- applies_to: ch5, ch6, ch7, ch8
- prompt_version: direct-codex-smoke-v1
- model_or_codex: current Codex session
- date: 2026-05-09

## Purpose

Specify the cross-call scenarios that combine process creation, divergent parent/child returns, termination, and collection.

## Scenarios

### Scenario: parent and child observe different fork return values

- introduced_in: ch5
- applies_to: ch5, ch6
- steps:
  - A process calls `fork`.
  - Parent receives the positive child pid.
  - Child resumes from the same call site and observes return value `0`.
  - Parent can later wait for the child.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch5/src/main.rs:565`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch5/src/process.rs:72`
  - `user/src/bin/forktest_simple.rs:17`

### Scenario: parent reaps many children once

- introduced_in: ch5
- applies_to: ch5, ch6
- steps:
  - Parent creates multiple children.
  - Each child exits.
  - Parent performs the same number of waits and obtains positive child pids.
  - A later wait reports no remaining child.
- allowed_variation:
  - Child completion and output order is scheduler-dependent.
- evidence:
  - `user/src/bin/12forktest.rs:16`
  - `user/src/bin/14forktest2.rs:32`
  - `user/src/bin/15matrix.rs:67`

### Scenario: waitpid selects one child

- introduced_in: ch5
- applies_to: ch5, ch6
- steps:
  - Parent creates at least one child.
  - Parent calls `waitpid(child_pid, ptr)`.
  - The call returns that child pid and writes the child exit status.
  - Re-waiting the same child reports failure.
- evidence:
  - `user/src/bin/fork_exit.rs:27`
  - `tg-rcore-tutorial/tg-rcore-tutorial-task-manage/src/proc_rel.rs:55`

### Scenario: exec changes the current executable

- introduced_in: ch5
- applies_to: ch5, ch6
- steps:
  - A process passes a target program name to `exec`.
  - If found and loadable, the process address space and entry context are replaced.
  - If not found, the call fails with `-1`.
- chapter_overrides:
  ch5: program bytes come from the chapter application set.
  ch6: program bytes come from a file opened by name.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch5/src/main.rs:584`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/main.rs:641`

## Out Of Scope

- Exhaustive scheduling interleavings.
- Exact pid allocation order except where returned pids are used by user programs.
- Internal memory copy strategy.

## Uncertainty

- Exercise spawn is related but specified in `process-control.md` because it creates a new process without requiring a fork-then-exec sequence.
