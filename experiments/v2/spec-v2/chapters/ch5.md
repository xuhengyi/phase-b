# Chapter Spec: ch5

## Metadata

- chapter: ch5
- title: process management
- prompt_version: direct-codex-smoke-v1
- model_or_codex: current Codex session
- date: 2026-05-09
- inputs: `spec-v2/inventory/ch5.yaml`, `spec-v2/capabilities/syscall-abi.md`, `spec-v2/capabilities/process-control.md`, `spec-v2/capabilities/fork-exec-wait.md`

## Scope

Base scope covers process creation with `fork`, program replacement with `exec`, child exit and collection with `wait`/`waitpid`, process identity, scheduling yield, time query, console I/O, and heap growth inherited from earlier chapters.

Exercise extension scope covers `spawn` and priority scheduling. Current source evidence marks these as TODO, so they are not treated as confirmed base behavior.

## Must Pass Base Cases

- `12forktest`
- `13forktree`
- `14forktest2`
- `15matrix`
- `fork_exit`
- `forktest_simple`
- `sbrk`
- inherited console, power, sleep, and address-space smoke cases listed in `inputs/user-tests-manifest.json`

## Required Capabilities

- `syscall-abi@ch5`
- `process-control@ch5`
- `fork-exec-wait@ch5`

## Chapter Requirements

### Requirement: process subsystem initialized before user execution

- introduced_in: ch5
- applies_to: ch5
- requirement: The kernel initializes syscall handlers for I/O, process, scheduling, clock, and memory before the initial user process runs.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch5/src/main.rs:223`

### Requirement: initial user process enters scheduling loop

- introduced_in: ch5
- applies_to: ch5
- requirement: The chapter starts an initial user process and runs processes until no runnable process remains.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch5/src/main.rs:229`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch5/src/main.rs:240`

### Requirement: syscall return handling preserves user-observable return values

- introduced_in: ch5
- applies_to: ch5
- requirement: Except for process exit, syscall return values are written back so user programs observe the result.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch5/src/main.rs:251`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch5/src/main.rs:261`

## Exercise Extension

- `spawn(path)` should create a new child from a named program and return its pid.
- `setpriority(prio)` should accept priorities `>= 2`, return the accepted priority, and return `-1` for invalid priorities.
- These extension requirements are backed by `exercise.md` and checker expectations, not by the current checked-in base implementation.

## Out Of Scope

- Full process session, signal, thread, or file descriptor semantics.
- Exact scheduling order, except tests must eventually complete.

## Uncertainty

- Baseline execution has not yet been run in this phase. Phase 2 must confirm which base/exercise tests the checked-in tg-rcore snapshot actually passes.
