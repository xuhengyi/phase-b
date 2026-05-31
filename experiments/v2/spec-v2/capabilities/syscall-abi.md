# Capability: syscall-abi

## Metadata

- id: syscall-abi
- introduced_in: ch2
- applies_to: ch2, ch3, ch4, ch5, ch6, ch7, ch8
- prompt_version: redo-step1-complete-os-abi.v1
- model_or_codex: current Codex session with local `codex-cli 0.130.0`
- date: 2026-05-09
- inputs: `inputs/tgrcore-manifest.json`, `inputs/user-tests-manifest.json`, `run-manifests/full-ch2-ch8.yaml`, `spec-v2/architecture/boot-trap-syscall.md`

## Purpose

Define the user-visible syscall surface used by full `ch2-ch8` base programs. This capability specifies syscall identifiers, argument order, register ABI, and signed integer return conventions. It does not prescribe source layout or implementation language structure.

## Public Surface

| name | id | arguments | returns | first base chapter |
| --- | ---: | --- | --- | --- |
| `read` | 63 | `fd`, `buf_ptr`, `len` | bytes read, `0` at EOF, or negative error | ch6 |
| `write` | 64 | `fd`, `buf_ptr`, `len` | bytes written or negative error | ch2 |
| `openat` | 56 | `path_ptr`, `flags` | fd or `-1` | ch6 |
| `close` | 57 | `fd` | `0` or `-1` | ch6 |
| `pipe2` | 59 | `pipe_fd_ptr` | `0`, `-1`, or transient negative retry code | ch7 |
| `linkat` | 37 | `olddirfd`, `oldpath_ptr`, `newdirfd`, `newpath_ptr`, `flags` | `0` or `-1` | ch6 exercise |
| `unlinkat` | 35 | `dirfd`, `path_ptr`, `flags` | `0` or `-1` | ch6 exercise |
| `fstat` | 80 | `fd`, `stat_ptr` | `0` or `-1` | ch6 exercise |
| `exit` | 93 | `exit_code` | never returns to user caller | ch2 |
| `clone` | 220 | none in this teaching ABI | child pid in parent, `0` in child, negative on failure | ch5 |
| `execve` | 221 | `path_ptr`, `path_len` | `0` on success, `-1` on failure | ch5 |
| `wait4` | 260 | `pid_or_any`, `exit_code_ptr` | child pid, `-1`, or transient `-2` | ch5 |
| `getpid` | 172 | none | current process id | ch5 |
| `sched_yield` | 124 | none | `0` | ch3 |
| `clock_gettime` | 113 | `clock_id`, `timespec_ptr` | `0` or `-1` | ch3 |
| `brk` | 214 | signed size delta | old break or `-1` | ch4 |
| `kill` | 129 | `pid`, `signum` | `0` or `-1` | ch7 |
| `rt_sigaction` | 134 | `signum`, `action_ptr`, `old_action_ptr` | `0` or `-1` | ch7 |
| `rt_sigprocmask` | 135 | `mask` | `0` or `-1` | ch7 |
| `rt_sigreturn` | 139 | none | restores interrupted user context or negative error | ch7 |
| `thread_create` | 1000 | `entry`, `arg` | new tid or negative error | ch8 |
| `gettid` | 178 | none | current thread id | ch8 |
| `waittid` | 95 | `tid` | thread exit code, `-1`, or transient `-2` | ch8 |
| `mutex_create` | 1010 | `blocking` | mutex id or negative error | ch8 |
| `mutex_lock` | 1011 | `mutex_id` | `0`, negative error, or transient retry code | ch8 |
| `mutex_unlock` | 1012 | `mutex_id` | `0` or negative error | ch8 |
| `semaphore_create` | 1020 | `resource_count` | semaphore id or negative error | ch8 |
| `semaphore_up` | 1021 | `sem_id` | `0` or negative error | ch8 |
| `semaphore_down` | 1022 | `sem_id` | `0`, negative error, or transient retry code | ch8 |
| `condvar_create` | 1030 | ignored argument | condvar id or negative error | ch8 |
| `condvar_signal` | 1031 | `condvar_id` | `0` or negative error | ch8 |
| `condvar_wait` | 1032 | `condvar_id`, `mutex_id` | `0`, negative error, or transient retry code | ch8 |
| `spawn` | 400 | `path_ptr`, `path_len` | child pid or `-1` | ch5 exercise |
| `setpriority` | 140 | priority | priority or `-1` | ch5 exercise |
| `mmap` | 222 | `start`, `len`, `prot`, ignored trailing args | mapped address or negative error | ch4 exercise |
| `munmap` | 215 | `start`, `len` | `0` or negative error | ch4 exercise |
| `trace` | 410 | `request`, `id`, `data` | request-specific value or negative error | ch3 exercise |
| `enable_deadlock_detect` | 469 | `enabled` | `0` or negative error | ch8 exercise |

## Register Convention

- `a7` carries the syscall id.
- `a0` to `a5` carry arguments in the table order.
- `a0` carries the signed return value after the kernel handles the syscall.
- The kernel must advance the saved user PC past `ecall` before returning to user mode.

## Requirements

### Requirement: stable numeric surface for smoke syscalls

- introduced_in: ch5
- applies_to: ch2, ch3, ch4, ch5, ch6, ch7, ch8
- requirement: User programs compiled against the user library must be able to invoke the listed syscall identifiers with the listed argument order and RISC-V register convention.
- observable_behavior: The program links and each syscall reaches the matching kernel capability or returns a defined unsupported/error outcome.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-syscall/src/syscall.h.in`
  - `tg-rcore-tutorial/tg-rcore-tutorial-syscall/src/user.rs`
  - `tg-rcore-tutorial/tg-rcore-tutorial-syscall/src/kernel/`

### Requirement: signed return convention

- introduced_in: ch2
- applies_to: ch2, ch3, ch4, ch5, ch6, ch7, ch8
- requirement: Successful syscalls return non-negative values unless documented as non-returning; failures return negative values used by user programs for assertions or retry decisions.
- observable_behavior: `wait`, `waittid`, pipe, mutex, semaphore, and condvar wrappers may retry on `-2`; tests assert `open > 0`, `wait < 0` when no child remains, and file operations return expected lengths.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-syscall/src/user.rs:129`
  - `tg-rcore-tutorial/tg-rcore-tutorial-syscall/src/user.rs:142`
  - `user/src/bin/12forktest.rs:29`
  - `user/src/bin/filetest_simple.rs:16`
  - `user/src/lib.rs`

## Out Of Scope

- Internal syscall dispatch data structures.
- Full Linux compatibility beyond the listed identifiers and teaching tests.
- Exact register naming or instruction encoding outside the target syscall call convention.

## Uncertainty

- Exercise-only syscalls are required by exercise documents and tests, but the checked-in chapter implementations may leave them incomplete.
