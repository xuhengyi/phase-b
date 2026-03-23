extern crate std;

use super::{Condvar, Mutex, MutexBlocking, Semaphore, UPIntrFreeCell};
use rcore_task_manage::ThreadId;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;
use std::vec;
use std::vec::Vec;

#[test]
fn test_upintrfreecell_exclusive_session_mutates_inner_value() {
    let cell = unsafe { UPIntrFreeCell::new(vec![1usize, 2, 3]) };
    cell.exclusive_session(|inner: &mut Vec<usize>| inner.push(4));
    let guard = cell.exclusive_access();
    assert_eq!(&*guard, &[1, 2, 3, 4]);
}

#[test]
fn test_upintrfreecell_double_borrow_panics() {
    let cell = unsafe { UPIntrFreeCell::new(1usize) };
    let _guard = cell.exclusive_access();
    let result = catch_unwind(AssertUnwindSafe(|| {
        let _ = cell.exclusive_access();
    }));
    assert!(result.is_err());
}

#[test]
fn test_mutex_blocking_lock_unlock() {
    let mutex = MutexBlocking::new();
    let t1 = ThreadId::from_usize(1);
    let t2 = ThreadId::from_usize(2);

    assert!(mutex.lock(t1));
    assert!(!mutex.lock(t2));
    assert_eq!(mutex.unlock(), Some(t2));
    assert_eq!(mutex.unlock(), None);
}

#[test]
fn test_condvar_wait_no_sched_and_signal() {
    let condvar = Condvar::new();
    let tid = ThreadId::from_usize(9);
    assert!(!condvar.wait_no_sched(tid));
    assert_eq!(condvar.signal(), Some(tid));
    assert_eq!(condvar.signal(), None);
}

#[test]
fn test_condvar_wait_with_mutex_wakes_waiter_before_relock() {
    let condvar = Condvar::new();
    let mutex: Arc<dyn Mutex> = Arc::new(MutexBlocking::new());
    let t1 = ThreadId::from_usize(1);
    let t2 = ThreadId::from_usize(2);

    assert!(mutex.lock(t1));
    assert!(!mutex.lock(t2));
    let (got_lock, waking_tid) = condvar.wait_with_mutex(t1, mutex.clone());
    assert_eq!(waking_tid, Some(t2));
    assert!(!got_lock);
}

#[test]
fn test_semaphore_down_and_up_follow_contract() {
    let semaphore = Semaphore::new(1);
    let t1 = ThreadId::from_usize(11);
    let t2 = ThreadId::from_usize(22);

    assert!(semaphore.down(t1));
    assert!(!semaphore.down(t2));
    assert_eq!(semaphore.up(), Some(t2));
    assert_eq!(semaphore.up(), None);
}
