use crate::LocalContext;

mod multislot_portal;

pub use multislot_portal::MultislotPortal;

#[cfg(all(test, feature = "foreign"))]
mod tests;

const SSTATUS_SPP_BIT: usize = 1 << 8;
const SSTATUS_SIE_BIT: usize = 1 << 1;

#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct TpReg(pub usize);

impl TpReg {
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    pub const fn get(self) -> usize {
        self.0
    }
}

impl From<usize> for TpReg {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<TpReg> for usize {
    fn from(value: TpReg) -> Self {
        value.0
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Default)]
pub struct PortalCache {
    pub satp: usize,
    pub sepc: usize,
    pub a0: usize,
    pub sstatus: usize,
}

impl PortalCache {
    pub fn init(&mut self, satp: usize, sepc: usize, a0: usize, supervisor: bool, interrupt: bool) {
        self.satp = satp;
        self.sepc = sepc;
        self.a0 = a0;
        self.sstatus = Self::compose_sstatus(supervisor, interrupt);
    }

    pub fn address(&self) -> usize {
        self as *const _ as usize
    }

    fn compose_sstatus(supervisor: bool, interrupt: bool) -> usize {
        let mut sstatus = 0;
        if supervisor {
            sstatus |= SSTATUS_SPP_BIT;
        }
        if interrupt {
            sstatus |= SSTATUS_SIE_BIT;
        }
        sstatus
    }
}

#[derive(Clone)]
pub struct ForeignContext {
    pub context: LocalContext,
    pub satp: usize,
}

impl ForeignContext {
    pub fn new(context: LocalContext, satp: usize) -> Self {
        Self { context, satp }
    }

    pub fn fill_cache(&self, cache: &mut PortalCache) {
        cache.init(
            self.satp,
            self.context.pc(),
            self.context.a(0),
            self.context.supervisor,
            self.context.interrupt,
        );
    }

    pub unsafe fn execute<K: SlotKey>(&mut self, _portal: &MultislotPortal, _key: K) -> ! {
        #[cfg(not(target_arch = "riscv64"))]
        {
            panic!("execute() is only available on RISC-V 64-bit targets");
        }

        #[cfg(target_arch = "riscv64")]
        {
            self.context.execute()
        }
    }
}

pub trait SlotKey {
    fn slot_index(&self) -> usize;
}

impl SlotKey for usize {
    fn slot_index(&self) -> usize {
        *self
    }
}

impl SlotKey for () {
    fn slot_index(&self) -> usize {
        0
    }
}

pub struct ForeignPortal<'a> {
    portal: &'a mut MultislotPortal,
}

impl<'a> ForeignPortal<'a> {
    pub fn new(portal: &'a mut MultislotPortal) -> Self {
        Self { portal }
    }

    pub fn portal(&mut self) -> &mut MultislotPortal {
        self.portal
    }

    pub fn cache<K: SlotKey>(&mut self, key: K) -> &mut PortalCache {
        self.portal.cache_mut(key.slot_index())
    }
}

pub struct MonoForeignPortal<'a> {
    portal: &'a mut MultislotPortal,
}

impl<'a> MonoForeignPortal<'a> {
    pub fn new(portal: &'a mut MultislotPortal) -> Self {
        assert_eq!(portal.slot_count(), 1);
        Self { portal }
    }

    pub fn cache(&mut self) -> &mut PortalCache {
        self.portal.cache_mut(0)
    }

    pub fn portal(&mut self) -> &mut MultislotPortal {
        self.portal
    }
}
