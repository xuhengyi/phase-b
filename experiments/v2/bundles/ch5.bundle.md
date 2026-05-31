# Bundle: ch5

## Metadata

- chapter: ch5
- prompt_version: direct-codex-smoke-v1
- model_or_codex: current Codex session
- date: 2026-05-09
- inputs:
  - `spec-v2/inventory/ch5.yaml`
  - `spec-v2/riscv-os-requirements.md`
  - `spec-v2/architecture/boot-trap-syscall.md`
  - `spec-v2/architecture/app-image-loader.md`
  - `spec-v2/architecture/subsystem-contracts.md`
  - `spec-v2/capabilities/syscall-abi.md`
  - `spec-v2/capabilities/process-control.md`
  - `spec-v2/capabilities/fork-exec-wait.md`
  - `spec-v2/chapters/ch5.md`
  - `spec-v2/oracles/ch5-base-output.md`

## Generation Boundary

This bundle is the semantic input for both RustOS and COS `ch5` generation. It intentionally does not require any source layout or implementation language structure.

## OS Architecture Contract

- 该章必须继承 ch4 的地址空间和用户缓冲区访问，并新增真实进程模型。
- `fork/exec/wait/getpid` 必须通过进程状态、父子关系和用户程序装载实现；不能用内核分支直接打印各 fork 测例输出。
- `exec(path)` 必须查找独立构建的用户程序镜像或文件系统条目并替换当前进程镜像。

## Base Required Behavior

- Provide syscall entry for ch5 user programs with the identifiers and argument order in `syscall-abi`.
- Start an initial user process and repeatedly schedule runnable processes until all are done.
- `fork` returns a positive child pid in the parent and `0` in the child.
- `exit(code)` terminates the process and records `code` for the parent.
- `wait(ptr)` waits for any child; `waitpid(pid, ptr)` waits for a selected child.
- Waiting writes the child exit code to a valid user pointer when provided.
- Waiting with no matching child returns `-1`.
- Waiting for a still-running matching child may return transient `-2`; the user wrapper retries.
- `exec(path)` replaces the current program image with the named program, or fails with `-1`.
- `getpid()` returns the current process id.
- `sched_yield()` returns `0` and allows other runnable processes to execute.
- `clock_gettime` supports the monotonic clock used by sleep/time tests.
- `brk`/`sbrk` supports heap grow, shrink, query, and failure when shrinking below heap base.
- Console `write` to standard output/debug returns the written byte count.
- The implementation must be a QEMU-runnable RISC-V OS chapter snapshot with real trap/syscall entry and process state. It cannot satisfy this bundle by running a host program or by printing checker oracle text directly.

## Base User Case Summary

- `forktest_simple`: no-child wait returns `-1`; fork child returns `100`; parent collects child pid and exit code.
- `12forktest`: parent creates 30 children, reaps 30, then observes no more children.
- `13forktree`: recursive fork tree can run and each internal node waits for two children.
- `14forktest2`: wait handles children exiting in nondeterministic order.
- `15matrix`: many compute-heavy children eventually exit and are reaped.
- `fork_exit`: `waitpid` targets one child and reports a negative magic exit code.
- `sbrk`: heap grows and shrinks; shrinking below heap base fails.

## Base Oracle

The checker-level oracle is `spec-v2/oracles/ch5-base-output.md`. It defines required and forbidden observable output patterns for Step 3, but it is not a substitute for the RISC-V OS gate. A host conformance runner or host-side oracle printer is invalid.

## Complete Independent OS Gate

`./test.sh base` must build a complete independent RISC-V teaching OS image or ELF and run `qemu-system-riscv64`. The report must identify the kernel entry, independent user app loader/app image, trap/syscall path, process table/state, and fork/exec/wait implementation status. A host runner, oracle-only QEMU printer, or embedded-snippet kernel is invalid.

## Exercise Extension

These are included so later generation can target exercise tests, but Phase 2 must first establish whether the tg-rcore snapshot passes them.

- `spawn(path)` creates a child from a named program without requiring a fork copy; success returns child pid; invalid target returns `-1`.
- Priority scheduling extension accepts `setpriority(prio)` for `prio >= 2`, returns accepted priority, rejects invalid priorities with `-1`, and schedules runnable processes by a priority-sensitive stride-style policy.

## Evidence

- `user/src/bin/forktest_simple.rs`
- `user/src/bin/12forktest.rs`
- `user/src/bin/fork_exit.rs`
- `tg-rcore-tutorial/tg-rcore-tutorial-ch5/src/main.rs`
- `tg-rcore-tutorial/tg-rcore-tutorial-ch5/src/process.rs`
- `tg-rcore-tutorial/tg-rcore-tutorial-task-manage/src/proc_rel.rs`
- `tg-rcore-tutorial/tg-rcore-tutorial-ch5/exercise.md`

## Uncertainty

- Exercise extension behavior is specified by tests and exercise docs, but the checked-in implementation contains TODO return paths for `spawn` and priority scheduling.
