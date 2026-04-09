use core::marker::PhantomData;

use crate::{collections::{hash_map, HashMap}, manager::Manage, scheduler::Schedule, ProcId, ThreadId};
use crate::proc_thread_rel::{ProcThreadRel, THREAD_WAIT_PENDING};

pub struct PThreadManager<T, P, M, PM>
where
    M: Manage<T, ThreadId> + Schedule<ThreadId>,
    PM: Manage<P, ProcId>,
{
    manager: Option<M>,
    proc_manager: Option<PM>,
    proc_threads: HashMap<ProcId, ProcThreadRel>,
    thread_owner: HashMap<ThreadId, ProcId>,
    current: Option<ThreadId>,
    _marker: PhantomData<(T, P)>,
}

impl<T, P, M, PM> PThreadManager<T, P, M, PM>
where
    M: Manage<T, ThreadId> + Schedule<ThreadId>,
    PM: Manage<P, ProcId>,
{
    pub fn new() -> Self {
        Self {
            manager: None,
            proc_manager: None,
            proc_threads: hash_map(),
            thread_owner: hash_map(),
            current: None,
            _marker: PhantomData,
        }
    }

    pub fn set_manager(&mut self, manager: M) {
        self.manager = Some(manager);
    }

    pub fn set_proc_manager(&mut self, proc_manager: PM) {
        self.proc_manager = Some(proc_manager);
    }

    fn manager_mut(&mut self) -> &mut M {
        self.manager.as_mut().expect("thread manager not set")
    }

    fn proc_manager_mut(&mut self) -> &mut PM {
        self.proc_manager
            .as_mut()
            .expect("process manager not set for pthread manager")
    }

    fn proc_rel_for(&mut self, pid: ProcId) -> &mut ProcThreadRel {
        self.proc_threads
            .entry(pid)
            .or_insert_with(|| ProcThreadRel::new(pid))
    }

    pub fn add_proc(&mut self, pid: ProcId, proc_item: P, parent: ProcId) {
        let _ = parent;
        self.proc_manager_mut().insert(pid, proc_item);
        self.proc_rel_for(pid);
    }

    pub fn add(&mut self, tid: ThreadId, thread_item: T, owner: ProcId) {
        self.manager_mut().insert(tid, thread_item);
        self.manager_mut().add(tid);
        self.thread_owner.insert(tid, owner);
        self.proc_rel_for(owner).add_thread(tid);
    }

    pub fn find_next(&mut self) -> Option<ThreadId> {
        let next = self.manager_mut().fetch()?;
        self.current = Some(next);
        Some(next)
    }

    pub fn make_current_suspend(&mut self) {
        if let Some(id) = self.current.take() {
            self.manager_mut().add(id);
        }
    }

    pub fn make_current_exited(&mut self, exit_code: isize) {
        if let Some(id) = self.current.take() {
            self.manager_mut().delete(id);
            if let Some(owner) = self.thread_owner.get(&id).copied() {
                if let Some(rel) = self.proc_threads.get_mut(&owner) {
                    rel.del_thread(id, exit_code);
                }
            }
        }
    }

    pub fn waittid(&mut self, tid: ThreadId) -> Option<isize> {
        let owner = *self.thread_owner.get(&tid)?;
        let rel = self.proc_threads.get_mut(&owner)?;
        match rel.wait_thread(tid) {
            Some(code) => {
                if code != THREAD_WAIT_PENDING {
                    self.thread_owner.remove(&tid);
                }
                Some(code)
            }
            None => {
                self.thread_owner.remove(&tid);
                None
            }
        }
    }

    pub fn thread_count(&self, pid: ProcId) -> usize {
        self.proc_threads.get(&pid).map(|rel| rel.thread_count()).unwrap_or(0)
    }
}
