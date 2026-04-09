use core::fmt;
use core::hash::{Hash, Hasher};
use core::sync::atomic::{AtomicUsize, Ordering};

macro_rules! define_id {
    ($name:ident, $counter:ident) => {
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(usize);

        static $counter: AtomicUsize = AtomicUsize::new(0);

        impl $name {
            pub fn new() -> Self {
                Self($counter.fetch_add(1, Ordering::Relaxed))
            }

            pub const fn from_usize(value: usize) -> Self {
                Self(value)
            }

            pub const fn get_usize(self) -> usize {
                self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!(stringify!($name), "({})"), self.0)
            }
        }

        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
    };
}

define_id!(ProcId, PROC_ID_COUNTER);
define_id!(ThreadId, THREAD_ID_COUNTER);
define_id!(CoroId, CORO_ID_COUNTER);
