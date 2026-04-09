#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod up;
mod mutex;
mod condvar;
mod semaphore;

pub use crate::up::{UPIntrFreeCell, UPIntrRefMut};
pub use crate::mutex::{Mutex, MutexBlocking};
pub use crate::condvar::Condvar;
pub use crate::semaphore::Semaphore;

#[cfg(test)]
mod tests;
