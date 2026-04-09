use alloc::collections::VecDeque;
use alloc::sync::Arc;

use crate::mutex::Mutex;
use crate::up::UPIntrFreeCell;
use rcore_task_manage::ThreadId;

pub struct Condvar {
    waiters: UPIntrFreeCell<VecDeque<ThreadId>>,
}

impl Condvar {
    pub fn new() -> Self {
        Self {
            waiters: unsafe { UPIntrFreeCell::new(VecDeque::new()) },
        }
    }

    /// Wait without involving the scheduler. Returns `false` to signal blocking.
    pub fn wait_no_sched(&self, tid: ThreadId) -> bool {
        self.waiters.exclusive_session(|queue| {
            queue.push_back(tid);
            false
        })
    }

    /// Wait while releasing a mutex. Returns `(got_mutex, waking_tid)`.
    pub fn wait_with_mutex(&self, tid: ThreadId, mutex: Arc<dyn Mutex>) -> (bool, Option<ThreadId>) {
        let waking = mutex.unlock();
        let got_mutex = self.wait_no_sched(tid);
        (got_mutex, waking)
    }

    /// Wake one waiter, if any.
    pub fn signal(&self) -> Option<ThreadId> {
        self.waiters.exclusive_session(|queue| queue.pop_front())
    }
}
