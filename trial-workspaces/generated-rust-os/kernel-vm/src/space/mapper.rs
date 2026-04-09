use core::ptr::{self, NonNull};

use page_table::{Pte, VmFlags, VmMeta, VPN};

use super::PageManager;

pub(crate) struct LeafSlot<Meta: VmMeta> {
    ptr: NonNull<Pte<Meta>>,
}

impl<Meta: VmMeta> LeafSlot<Meta> {
    pub(crate) fn write(&mut self, value: Pte<Meta>) {
        unsafe { self.ptr.as_ptr().write(value) };
    }
}

pub(crate) struct LeafInfo<Meta: VmMeta> {
    pub(crate) pte: Pte<Meta>,
    pub(crate) level: usize,
}

#[inline]
pub(crate) const fn page_bytes<Meta: VmMeta>() -> usize {
    1usize << Meta::PAGE_BITS
}

#[inline]
pub(crate) fn entry_span<Meta: VmMeta>(level: usize) -> usize {
    if level == 0 {
        1
    } else {
        Meta::pages_in_table(level - 1)
    }
}

pub(crate) fn ensure_leaf_slot<Meta: VmMeta, PM: PageManager<Meta>>(
    manager: &mut PM,
    vpn: VPN<Meta>,
) -> LeafSlot<Meta> {
    let mut table_ptr = manager.root_ptr();
    let mut level = Meta::MAX_LEVEL;

    loop {
        let index = vpn.index_in(level);
        let entry_ptr = unsafe { table_ptr.as_ptr().add(index) };
        let entry = unsafe { *entry_ptr };

        if level == 0 {
            let ptr = NonNull::new(entry_ptr).expect("entry pointer cannot be null");
            return LeafSlot { ptr };
        }

        if entry.is_valid() && !entry.is_leaf() {
            table_ptr = manager.p_to_v::<Pte<Meta>>(entry.ppn());
            level -= 1;
            continue;
        }

        let mut flags = VmFlags::VALID;
        if !flags.valid() {
            flags |= VmFlags::VALID;
        }
        let raw = manager.allocate(1, &mut flags);
        unsafe {
            ptr::write_bytes(raw.as_ptr(), 0, page_bytes::<Meta>());
        }
        let child_ptr = raw.cast::<Pte<Meta>>();
        let child_ppn = manager.v_to_p(child_ptr);
        let pte = VmFlags::VALID.build_pte(child_ppn);
        unsafe {
            entry_ptr.write(pte);
        }
        table_ptr = child_ptr;
        level -= 1;
    }
}

pub(crate) fn find_leaf<Meta: VmMeta, PM: PageManager<Meta>>(
    manager: &PM,
    vpn: VPN<Meta>,
) -> Option<LeafInfo<Meta>> {
    let mut table_ptr = manager.root_ptr();
    let mut level = Meta::MAX_LEVEL;

    loop {
        let index = vpn.index_in(level);
        let entry = unsafe { *table_ptr.as_ptr().add(index) };
        if !entry.is_valid() {
            return None;
        }
        if entry.is_leaf() || level == 0 {
            return Some(LeafInfo { pte: entry, level });
        }
        table_ptr = manager.p_to_v::<Pte<Meta>>(entry.ppn());
        level -= 1;
    }
}
