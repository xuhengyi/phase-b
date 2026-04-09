use alloc::collections::VecDeque;

use crate::up::UPIntrFreeCell;
use rcore_task_manage::ThreadId;

/// Object-safe interface for mutex implementations.
pub trait Mutex: Send + Sync {
    /// Attempt to lock the mutex for `tid`.
    /// Returns `true` if the lock is acquired immediately, otherwise the caller
    /// should consider itself blocked.
    fn lock(&self, tid: ThreadId) -> bool;

    /// Unlock the mutex. Returns the next thread that should be woken, if any.
    fn unlock(&self) -> Option<ThreadId>;
}

/// Blocking mutex backed by `UPIntrFreeCell` to provide single-processor safety.
pub struct MutexBlocking {
    inner: UPIntrFreeCell<MutexState>,
}

impl MutexBlocking {
    pub fn new() -> Self {
        Self {
            inner: unsafe { UPIntrFreeCell::new(MutexState::new()) },
        }
    }
}

impl Default for MutexBlocking {
    fn default() -> Self {
        Self::new()
    }
}

impl Mutex for MutexBlocking {
    fn lock(&self, tid: ThreadId) -> bool {
        self.inner.exclusive_session(|state| {
            if state.owner.is_none() {
                state.owner = Some(tid);
                true
            } else {
                state.waiters.push_back(tid);
                false
            }
        })
    }

    fn unlock(&self) -> Option<ThreadId> {
        self.inner.exclusive_session(|state| {
            if state.owner.is_none() {
                return None;
            }
            if let Some(waiter) = state.waiters.pop_front() {
                state.owner = Some(waiter);
                Some(waiter)
            } else {
                state.owner = None;
                None
            }
        })
    }
}

struct MutexState {
    owner: Option<ThreadId>,
    waiters: VecDeque<ThreadId>,
}

impl MutexState {
    fn new() -> Self {
        Self {
            owner: None,
            waiters: VecDeque::new(),
        }
    }
}
