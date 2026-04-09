use alloc::collections::VecDeque;

use crate::{collections::{hash_set, HashSet}, ProcId};

pub const WAIT_CHILD_PENDING: isize = -1;
const WAIT_SENTINEL_ID: ProcId = ProcId::from_usize(usize::MAX - 1);

pub struct ProcRel {
    parent: ProcId,
    children: HashSet<ProcId>,
    exited: VecDeque<(ProcId, isize)>,
}

impl ProcRel {
    pub fn new(parent: ProcId) -> Self {
        Self {
            parent,
            children: hash_set(),
            exited: VecDeque::new(),
        }
    }

    pub fn parent(&self) -> ProcId {
        self.parent
    }

    pub fn add_child(&mut self, child: ProcId) {
        self.children.insert(child);
    }

    pub fn del_child(&mut self, child: ProcId, exit_code: isize) {
        if self.children.remove(&child) {
            self.exited.push_back((child, exit_code));
        }
    }

    pub fn wait_any_child(&mut self) -> Option<(ProcId, isize)> {
        if let Some(entry) = self.exited.pop_front() {
            return Some(entry);
        }
        if !self.children.is_empty() {
            return Some((WAIT_SENTINEL_ID, WAIT_CHILD_PENDING));
        }
        None
    }

    pub fn has_child(&self, child: ProcId) -> bool {
        self.children.contains(&child)
    }

    pub fn running_children(&self) -> usize {
        self.children.len()
    }
}
