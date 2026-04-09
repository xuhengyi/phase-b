use core::cell::{Cell, UnsafeCell};
use core::ops::{Deref, DerefMut};

/// Single-processor interrupt-free exclusive cell.
pub struct UPIntrFreeCell<T> {
    value: UnsafeCell<T>,
    borrowed: Cell<bool>,
}

unsafe impl<T: Send> Sync for UPIntrFreeCell<T> {}
unsafe impl<T: Send> Send for UPIntrFreeCell<T> {}

impl<T> UPIntrFreeCell<T> {
    /// Create a new cell. Caller must ensure single-processor usage.
    pub const unsafe fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            borrowed: Cell::new(false),
        }
    }

    /// Run a closure inside an exclusive session.
    pub fn exclusive_session<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut guard = self.exclusive_access();
        f(&mut guard)
    }

    /// Obtain an exclusive guard over the inner value.
    pub fn exclusive_access(&self) -> UPIntrRefMut<'_, T> {
        let mut state = IntrState::enter();
        if self.borrowed.get() {
            state.restore();
            panic!("UPIntrFreeCell already borrowed");
        }
        self.borrowed.set(true);
        UPIntrRefMut {
            value: &self.value,
            borrowed: &self.borrowed,
            intr_state: state,
        }
    }
}

/// Guard returned by `UPIntrFreeCell::exclusive_access`.
pub struct UPIntrRefMut<'a, T> {
    value: &'a UnsafeCell<T>,
    borrowed: &'a Cell<bool>,
    intr_state: IntrState,
}

impl<'a, T> Deref for UPIntrRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value.get() }
    }
}

impl<'a, T> DerefMut for UPIntrRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.value.get() }
    }
}

impl<'a, T> Drop for UPIntrRefMut<'a, T> {
    fn drop(&mut self) {
        self.borrowed.set(false);
        self.intr_state.restore();
    }
}

struct IntrState {
    was_enabled: bool,
    active: bool,
}

impl IntrState {
    fn enter() -> Self {
        let was_enabled = intr::enter();
        Self {
            was_enabled,
            active: true,
        }
    }

    fn restore(&mut self) {
        if self.active {
            intr::restore(self.was_enabled);
            self.active = false;
        }
    }
}

impl Drop for IntrState {
    fn drop(&mut self) {
        self.restore();
    }
}

#[cfg(target_arch = "riscv64")]
mod intr {
    use riscv::register::sstatus;

    pub fn enter() -> bool {
        let enabled = sstatus::read().sie();
        if enabled {
            unsafe { sstatus::clear_sie(); }
        }
        enabled
    }

    pub fn restore(enabled: bool) {
        if enabled {
            unsafe { sstatus::set_sie(); }
        } else {
            unsafe { sstatus::clear_sie(); }
        }
    }
}

#[cfg(not(target_arch = "riscv64"))]
mod intr {
    pub fn enter() -> bool {
        false
    }

    pub fn restore(_enabled: bool) {}
}
