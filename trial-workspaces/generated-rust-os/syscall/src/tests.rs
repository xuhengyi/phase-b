extern crate std;

use super::*;
use std::fmt;

#[test]
fn test_syscall_id_basic() {
    let id1 = SyscallId::from(64);
    let id2 = SyscallId(64);
    assert_eq!(id1, id2);

    let id3: SyscallId = 93usize.into();
    assert_eq!(id3.0, 93);
}

#[test]
fn test_syscall_id_constants() {
    assert_eq!(SyscallId::WRITE.0, 64);
    assert_eq!(SyscallId::READ.0, 63);
    assert_eq!(SyscallId::EXIT.0, 93);
    assert_eq!(SyscallId::CLOCK_GETTIME.0, 113);
    assert_eq!(SyscallId::GETPID.0, 172);
    assert_eq!(SyscallId::GETTID.0, 178);
    assert_eq!(SyscallId::SCHED_YIELD.0, 124);
}

#[test]
fn test_io_constants() {
    assert_eq!(STDIN, 0);
    assert_eq!(STDOUT, 1);
    assert_eq!(STDDEBUG, 2);
}

#[test]
fn test_clock_id_constants() {
    assert_eq!(ClockId::CLOCK_REALTIME.0, 0);
    assert_eq!(ClockId::CLOCK_MONOTONIC.0, 1);
    assert_eq!(ClockId::CLOCK_PROCESS_CPUTIME_ID.0, 2);
    assert_eq!(ClockId::CLOCK_THREAD_CPUTIME_ID.0, 3);
}

#[test]
fn test_time_spec_basic_and_add() {
    assert_eq!(TimeSpec::ZERO.tv_sec, 0);
    assert_eq!(TimeSpec::SECOND.tv_sec, 1);
    assert_eq!(TimeSpec::MILLSECOND.tv_nsec, 1_000_000);
    assert_eq!(TimeSpec::MICROSECOND.tv_nsec, 1_000);
    assert_eq!(TimeSpec::NANOSECOND.tv_nsec, 1);

    let ts = TimeSpec::from_millsecond(1500);
    assert_eq!(ts.tv_sec, 1);
    assert_eq!(ts.tv_nsec, 500_000_000);

    let result = TimeSpec {
        tv_sec: 1,
        tv_nsec: 500_000_000,
    } + TimeSpec {
        tv_sec: 2,
        tv_nsec: 600_000_000,
    };
    assert_eq!(result.tv_sec, 4);
    assert_eq!(result.tv_nsec, 100_000_000);
    assert!(fmt::format(format_args!("{result}")).contains('4'));
}

#[test]
fn test_signal_defs_reexport() {
    assert_eq!(SignalNo::from(0), SignalNo::ERR);
    assert_eq!(SignalNo::from(9), SignalNo::SIGKILL);
    assert_eq!(SignalAction::default().handler, 0);
    assert_eq!(SignalAction::default().mask, 0);
    assert_eq!(MAX_SIG, 31);
}

#[cfg(feature = "user")]
#[test]
fn test_open_flags() {
    let rdonly = OpenFlags::RDONLY;
    assert_eq!(rdonly.bits(), 0);

    let wronly = OpenFlags::WRONLY;
    assert_eq!(wronly.bits(), 1);

    let rdwr = OpenFlags::RDWR;
    assert_eq!(rdwr.bits(), 2);

    let create = OpenFlags::CREATE;
    assert_eq!(create.bits(), 512);

    let trunc = OpenFlags::TRUNC;
    assert_eq!(trunc.bits(), 1024);

    let flags = OpenFlags::WRONLY | OpenFlags::CREATE | OpenFlags::TRUNC;
    assert!(flags.contains(OpenFlags::WRONLY));
    assert!(flags.contains(OpenFlags::CREATE));
    assert!(flags.contains(OpenFlags::TRUNC));
}

#[cfg(feature = "user")]
#[test]
fn test_user_api_exists() {
    let _write_fn: fn(usize, &[u8]) -> isize = write;
    let _read_fn: fn(usize, &[u8]) -> isize = read;
    let _open_fn: fn(&str, OpenFlags) -> isize = open;
    let _close_fn: fn(usize) -> isize = close;
    let _exit_fn: fn(i32) -> isize = exit;
    let _sched_yield_fn: fn() -> isize = sched_yield;
    let _clock_gettime_fn: fn(ClockId, *mut TimeSpec) -> isize = clock_gettime;
    let _fork_fn: fn() -> isize = fork;
    let _exec_fn: fn(&str) -> isize = exec;
    let _wait_fn: fn(*mut i32) -> isize = wait;
    let _waitpid_fn: fn(isize, *mut i32) -> isize = waitpid;
    let _getpid_fn: fn() -> isize = getpid;
    let _kill_fn: fn(isize, SignalNo) -> isize = kill;
    let _sigaction_fn: fn(SignalNo, *const SignalAction, *const SignalAction) -> isize = sigaction;
    let _sigprocmask_fn: fn(usize) -> isize = sigprocmask;
    let _sigreturn_fn: fn() -> isize = sigreturn;
    let _thread_create_fn: fn(usize, usize) -> isize = thread_create;
    let _gettid_fn: fn() -> isize = gettid;
    let _waittid_fn: fn(usize) -> isize = waittid;
    let _semaphore_create_fn: fn(usize) -> isize = semaphore_create;
    let _semaphore_up_fn: fn(usize) -> isize = semaphore_up;
    let _semaphore_down_fn: fn(usize) -> isize = semaphore_down;
    let _mutex_create_fn: fn(bool) -> isize = mutex_create;
    let _mutex_lock_fn: fn(usize) -> isize = mutex_lock;
    let _mutex_unlock_fn: fn(usize) -> isize = mutex_unlock;
    let _condvar_create_fn: fn() -> isize = condvar_create;
    let _condvar_signal_fn: fn(usize) -> isize = condvar_signal;
    let _condvar_wait_fn: fn(usize, usize) -> isize = condvar_wait;
}
