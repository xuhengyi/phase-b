use alloc::collections::VecDeque;

use crate::up::UPIntrFreeCell;
use rcore_task_manage::ThreadId;

pub struct Semaphore {
    inner: UPIntrFreeCell<SemaphoreState>,
}

impl Semaphore {
    pub fn new(initial: usize) -> Self {
        Self {
            inner: unsafe { UPIntrFreeCell::new(SemaphoreState::new(initial)) },
        }
    }

    /// Attempt to acquire a permit. Returns `true` when acquired.
    pub fn down(&self, tid: ThreadId) -> bool {
        self.inner.exclusive_session(|state| {
            if state.permits > 0 {
                state.permits -= 1;
                true
            } else {
                state.waiters.push_back(tid);
                false
            }
        })
    }

    /// Release a permit, waking the next waiter if present.
    pub fn up(&self) -> Option<ThreadId> {
        self.inner.exclusive_session(|state| {
            if let Some(waiter) = state.waiters.pop_front() {
                Some(waiter)
            } else {
                state.permits += 1;
                None
            }
        })
    }
}

struct SemaphoreState {
    permits: usize,
    waiters: VecDeque<ThreadId>,
}

impl SemaphoreState {
    fn new(initial: usize) -> Self {
        Self {
            permits: initial,
            waiters: VecDeque::new(),
        }
    }
}
