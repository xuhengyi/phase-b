extern crate std;

use super::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::string::{String, ToString};
#[cfg(feature = "thread")]
use std::hash::Hash;

struct TestManager<T> {
    items: HashMap<usize, T>,
}

impl<T> TestManager<T> {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

impl<T> Manage<T, usize> for TestManager<T> {
    fn insert(&mut self, id: usize, item: T) {
        self.items.insert(id, item);
    }

    fn delete(&mut self, id: usize) {
        self.items.remove(&id);
    }

    fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.items.get_mut(&id)
    }
}

struct TestScheduler<I> {
    queue: VecDeque<I>,
}

impl<I: Copy + Ord> TestScheduler<I> {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl<I: Copy + Ord> Schedule<I> for TestScheduler<I> {
    fn add(&mut self, id: I) {
        self.queue.push_back(id);
    }

    fn fetch(&mut self) -> Option<I> {
        self.queue.pop_front()
    }
}

#[cfg(feature = "thread")]
struct GenericTaskManager<T, I> {
    items: HashMap<I, T>,
    queue: VecDeque<I>,
}

#[cfg(feature = "thread")]
impl<T, I: Copy + Ord + Eq + Hash> GenericTaskManager<T, I> {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
            queue: VecDeque::new(),
        }
    }
}

#[cfg(feature = "thread")]
impl<T, I: Copy + Ord + Eq + Hash> Manage<T, I> for GenericTaskManager<T, I> {
    fn insert(&mut self, id: I, item: T) {
        self.items.insert(id, item);
    }

    fn delete(&mut self, id: I) {
        self.items.remove(&id);
    }

    fn get_mut(&mut self, id: I) -> Option<&mut T> {
        self.items.get_mut(&id)
    }
}

#[cfg(feature = "thread")]
impl<T, I: Copy + Ord + Eq + Hash> Schedule<I> for GenericTaskManager<T, I> {
    fn add(&mut self, id: I) {
        self.queue.push_back(id);
    }

    fn fetch(&mut self) -> Option<I> {
        self.queue.pop_front()
    }
}

#[test]
fn test_id_types_basic_traits() {
    let pid1 = ProcId::new();
    let pid2 = ProcId::new();
    assert!(pid1.get_usize() < pid2.get_usize());
    assert_eq!(ProcId::from_usize(42).get_usize(), 42);
    assert_eq!(ThreadId::from_usize(100).get_usize(), 100);
    assert_eq!(CoroId::from_usize(200).get_usize(), 200);
    assert!(ProcId::from_usize(10) < ProcId::from_usize(20));
    assert!(ThreadId::from_usize(10) < ThreadId::from_usize(20));
    assert!(CoroId::from_usize(10) < CoroId::from_usize(20));
    assert!(fmt::format(format_args!("{:?}", ProcId::from_usize(1))).contains("ProcId"));
    assert!(fmt::format(format_args!("{:?}", ThreadId::from_usize(1))).contains("ThreadId"));
    assert!(fmt::format(format_args!("{:?}", CoroId::from_usize(1))).contains("CoroId"));
}

#[test]
fn test_manage_trait_insert_delete_get_mut() {
    let mut manager: TestManager<String> = TestManager::new();
    manager.insert(1, "item1".to_string());
    manager.insert(2, "item2".to_string());
    assert_eq!(manager.get_mut(1).map(|s: &mut String| s.as_str()), Some("item1"));
    manager.delete(1);
    assert_eq!(manager.get_mut(1), None);
    assert_eq!(manager.get_mut(2).map(|s: &mut String| s.as_str()), Some("item2"));
}

#[test]
fn test_schedule_trait_fifo_order() {
    let mut scheduler: TestScheduler<usize> = TestScheduler::new();
    assert_eq!(scheduler.fetch(), None);
    scheduler.add(1);
    scheduler.add(2);
    scheduler.add(3);
    assert_eq!(scheduler.fetch(), Some(1));
    assert_eq!(scheduler.fetch(), Some(2));
    assert_eq!(scheduler.fetch(), Some(3));
    assert_eq!(scheduler.fetch(), None);
}

#[test]
fn test_id_types_hash() {
    let mut set = HashSet::new();
    set.insert(ProcId::from_usize(1));
    set.insert(ProcId::from_usize(2));
    set.insert(ProcId::from_usize(1));
    assert_eq!(set.len(), 2);
}

#[cfg(feature = "proc")]
#[test]
fn test_proc_rel_wait_semantics_match_current_behavior() {
    let parent = ProcId::from_usize(0);
    let child = ProcId::from_usize(1);
    let mut rel = ProcRel::new(parent);

    rel.add_child(child);
    let waiting = rel.wait_any_child().unwrap();
    assert_eq!(waiting.0.get_usize(), usize::MAX - 1);
    assert_eq!(waiting.1, -1);

    rel.del_child(child, 7);
    assert_eq!(rel.wait_any_child(), Some((child, 7)));
    assert_eq!(rel.wait_any_child(), None);
}

#[cfg(feature = "thread")]
#[test]
fn test_proc_thread_rel_wait_thread_semantics_match_current_behavior() {
    let parent = ProcId::from_usize(0);
    let tid = ThreadId::from_usize(2);
    let mut rel = ProcThreadRel::new(parent);

    rel.add_thread(tid);
    assert_eq!(rel.wait_thread(tid), Some(-2));

    rel.del_thread(tid, 11);
    assert_eq!(rel.wait_thread(tid), Some(11));
    assert_eq!(rel.wait_thread(tid), None);
}

#[cfg(feature = "thread")]
#[test]
fn test_pthread_manager_waittid_tracks_thread_exit() {
    let mut manager =
        PThreadManager::<usize, usize, GenericTaskManager<usize, ThreadId>, GenericTaskManager<usize, ProcId>>::new();
    manager.set_manager(GenericTaskManager::new());
    manager.set_proc_manager(GenericTaskManager::new());

    let pid = ProcId::from_usize(1);
    let parent = ProcId::from_usize(0);
    let t1 = ThreadId::from_usize(11);
    let t2 = ThreadId::from_usize(12);

    manager.add_proc(parent, 0, parent);
    manager.add_proc(pid, 100, parent);
    manager.add(t1, 1, pid);
    manager.add(t2, 2, pid);

    assert!(manager.find_next().is_some());
    manager.make_current_suspend();
    assert!(manager.find_next().is_some());
    manager.make_current_exited(9);

    assert!(manager.find_next().is_some());
    assert_eq!(manager.waittid(t2), Some(9));
    assert_eq!(manager.thread_count(pid), 1);
}
