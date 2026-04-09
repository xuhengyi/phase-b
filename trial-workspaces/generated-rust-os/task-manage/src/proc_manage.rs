use core::marker::PhantomData;

use crate::{collections::{hash_map, HashMap}, manager::Manage, proc_rel::ProcRel, ProcId};

pub struct PManager<T, M>
where
    M: Manage<T, ProcId>,
{
    manager: Option<M>,
    relations: HashMap<ProcId, ProcRel>,
    parents: HashMap<ProcId, ProcId>,
    _marker: PhantomData<T>,
}

impl<T, M> PManager<T, M>
where
    M: Manage<T, ProcId>,
{
    pub fn new() -> Self {
        Self {
            manager: None,
            relations: hash_map(),
            parents: hash_map(),
            _marker: PhantomData,
        }
    }

    pub fn set_manager(&mut self, manager: M) {
        self.manager = Some(manager);
    }

    fn manager_mut(&mut self) -> &mut M {
        self.manager.as_mut().expect("process manager not set")
    }

    pub fn add(&mut self, pid: ProcId, proc_item: T, parent: ProcId) {
        self.manager_mut().insert(pid, proc_item);
        self.parents.insert(pid, parent);
        self.relations
            .entry(parent)
            .or_insert_with(|| ProcRel::new(parent))
            .add_child(pid);
        self.relations.entry(pid).or_insert_with(|| ProcRel::new(pid));
    }

    pub fn del(&mut self, pid: ProcId, exit_code: isize) {
        self.manager_mut().delete(pid);
        if let Some(parent) = self.parents.remove(&pid) {
            if let Some(rel) = self.relations.get_mut(&parent) {
                rel.del_child(pid, exit_code);
            }
        }
    }

    pub fn wait_any_child(&mut self, parent: ProcId) -> Option<(ProcId, isize)> {
        self.relations.get_mut(&parent)?.wait_any_child()
    }

    pub fn has_proc(&self, pid: ProcId) -> bool {
        self.parents.contains_key(&pid) || self.relations.contains_key(&pid)
    }
}
