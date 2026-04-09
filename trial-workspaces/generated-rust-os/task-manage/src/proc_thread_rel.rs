use crate::{collections::{hash_map, hash_set, HashMap, HashSet}, ProcId, ThreadId};

pub const THREAD_WAIT_PENDING: isize = -2;

pub struct ProcThreadRel {
    parent: ProcId,
    threads: HashSet<ThreadId>,
    exited: HashMap<ThreadId, isize>,
}

impl ProcThreadRel {
    pub fn new(parent: ProcId) -> Self {
        Self {
            parent,
            threads: hash_set(),
            exited: hash_map(),
        }
    }

    pub fn parent(&self) -> ProcId {
        self.parent
    }

    pub fn add_thread(&mut self, tid: ThreadId) {
        self.threads.insert(tid);
    }

    pub fn del_thread(&mut self, tid: ThreadId, exit_code: isize) {
        if self.threads.remove(&tid) {
            self.exited.insert(tid, exit_code);
        }
    }

    pub fn wait_thread(&mut self, tid: ThreadId) -> Option<isize> {
        if let Some(code) = self.exited.remove(&tid) {
            return Some(code);
        }
        if self.threads.contains(&tid) {
            return Some(THREAD_WAIT_PENDING);
        }
        None
    }

    pub fn thread_count(&self) -> usize {
        self.threads.len()
    }
}
