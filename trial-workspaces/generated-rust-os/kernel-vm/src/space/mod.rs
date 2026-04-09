use core::{cmp, marker::PhantomData, ops::Range, ptr, ptr::NonNull};

use page_table::{Pte, VmFlags, VmMeta, VAddr, PPN, VPN};

mod mapper;

use mapper::{entry_span, ensure_leaf_slot, find_leaf, page_bytes, LeafInfo};

pub trait PageManager<Meta: VmMeta> {
    fn new_root() -> Self;
    fn root_ptr(&self) -> NonNull<Pte<Meta>>;
    fn root_ppn(&self) -> PPN<Meta>;
    fn p_to_v<T>(&self, ppn: PPN<Meta>) -> NonNull<T>;
    fn v_to_p<T>(&self, ptr: NonNull<T>) -> PPN<Meta>;
    fn allocate(&mut self, len: usize, flags: &mut VmFlags<Meta>) -> NonNull<u8>;
    fn deallocate(&mut self, pte: Pte<Meta>, len: usize) -> usize;
    fn check_owned(&self, pte: Pte<Meta>) -> bool;
    fn drop_root(&mut self);
}

pub struct AddressSpace<Meta: VmMeta, PM: PageManager<Meta>> {
    manager: PM,
    _marker: PhantomData<Meta>,
}

impl<Meta: VmMeta, PM: PageManager<Meta>> AddressSpace<Meta, PM> {
    pub fn new() -> Self {
        Self {
            manager: PM::new_root(),
            _marker: PhantomData,
        }
    }

    pub fn map(
        &mut self,
        range: Range<VPN<Meta>>,
        data: &[u8],
        offset: usize,
        mut flags: VmFlags<Meta>,
    ) {
        assert!(range.start.val() < range.end.val(), "empty mapping range");
        ensure_valid(&mut flags);
        let start = range.start.val();
        let end = range.end.val();
        let page_size = page_bytes::<Meta>();

        for index in start..end {
            let vpn = VPN::new(index);
            let mut slot = ensure_leaf_slot::<Meta, PM>(&mut self.manager, vpn);
            let mut leaf_flags = flags;
            ensure_valid(&mut leaf_flags);
            let raw = self.manager.allocate(1, &mut leaf_flags);
            unsafe {
                ptr::write_bytes(raw.as_ptr(), 0, page_size);
            }
            copy_into_page::<Meta>(raw, index - start, offset, data);
            let ppn = self.manager.v_to_p::<u8>(raw);
            slot.write(leaf_flags.build_pte(ppn));
        }
    }

    pub fn map_extern(
        &mut self,
        range: Range<VPN<Meta>>,
        base: PPN<Meta>,
        mut flags: VmFlags<Meta>,
    ) {
        assert!(range.start.val() < range.end.val(), "empty mapping range");
        ensure_valid(&mut flags);
        let start = range.start.val();
        let end = range.end.val();
        let mut current_ppn = base.val();

        for index in start..end {
            let vpn = VPN::new(index);
            let mut slot = ensure_leaf_slot::<Meta, PM>(&mut self.manager, vpn);
            let ppn = PPN::new(current_ppn);
            slot.write(flags.build_pte(ppn));
            current_ppn += 1;
        }
    }

    pub fn translate<T>(&self, vaddr: VAddr<Meta>, required: VmFlags<Meta>) -> Option<NonNull<T>> {
        let vpn = vaddr.floor();
        let LeafInfo { pte, level } = find_leaf::<Meta, PM>(&self.manager, vpn)?;
        if !pte.is_valid() {
            return None;
        }
        if !pte.is_leaf() && level > 0 {
            return None;
        }
        let flags = pte.flags();
        if !flags.contains(required) {
            return None;
        }
        let base_vpn = vpn.floor(level);
        let base_addr = base_vpn.base().val();
        let offset = vaddr.val().wrapping_sub(base_addr);
        let page_ptr = self.manager.p_to_v::<u8>(pte.ppn());
        let ptr = unsafe { NonNull::new_unchecked(page_ptr.as_ptr().add(offset)) };
        Some(ptr.cast())
    }

    pub fn cloneself(&self, target: &mut AddressSpace<Meta, PM>) {
        self.walk_leaves(|vpn, level, pte| {
            duplicate_leaf::<Meta, PM>(self, target, vpn, level, pte);
        });
    }

    fn walk_leaves(&self, mut f: impl FnMut(VPN<Meta>, usize, Pte<Meta>)) {
        let root = self.manager.root_ptr();
        walk_node::<Meta, PM>(&self.manager, Meta::MAX_LEVEL, root, VPN::ZERO, &mut f);
    }
}

impl<Meta: VmMeta, PM: PageManager<Meta>> Drop for AddressSpace<Meta, PM> {
    fn drop(&mut self) {
        self.manager.drop_root();
    }
}

fn walk_node<Meta: VmMeta, PM: PageManager<Meta>>(
    manager: &PM,
    level: usize,
    table_ptr: NonNull<Pte<Meta>>,
    base: VPN<Meta>,
    f: &mut impl FnMut(VPN<Meta>, usize, Pte<Meta>),
) {
    let entries = 1usize << Meta::LEVEL_BITS[level];
    let span = entry_span::<Meta>(level);
    for idx in 0..entries {
        let entry_ptr = unsafe { table_ptr.as_ptr().add(idx) };
        let pte = unsafe { *entry_ptr };
        if !pte.is_valid() {
            continue;
        }
        let vpn = base + idx * span;
        if pte.is_leaf() || level == 0 {
            f(vpn, level, pte);
        } else {
            let child = manager.p_to_v::<Pte<Meta>>(pte.ppn());
            walk_node(manager, level - 1, child, vpn, f);
        }
    }
}

fn duplicate_leaf<Meta: VmMeta, PM: PageManager<Meta>>(
    source: &AddressSpace<Meta, PM>,
    target: &mut AddressSpace<Meta, PM>,
    base_vpn: VPN<Meta>,
    level: usize,
    pte: Pte<Meta>,
) {
    let span = entry_span::<Meta>(level);
    let page_size = page_bytes::<Meta>();
    let src = source.manager.p_to_v::<u8>(pte.ppn());
    let mut flags = pte.flags();
    ensure_valid(&mut flags);
    for i in 0..span {
        let vpn = base_vpn + i;
        let mut slot = ensure_leaf_slot::<Meta, PM>(&mut target.manager, vpn);
        let mut leaf_flags = flags;
        let raw = target.manager.allocate(1, &mut leaf_flags);
        unsafe {
            ptr::copy_nonoverlapping(
                src.as_ptr().add(i * page_size),
                raw.as_ptr(),
                page_size,
            );
        }
        let ppn = target.manager.v_to_p::<u8>(raw);
        slot.write(leaf_flags.build_pte(ppn));
    }
}

fn copy_into_page<Meta: VmMeta>(
    page_ptr: NonNull<u8>,
    index: usize,
    offset: usize,
    data: &[u8],
) {
    if data.is_empty() {
        return;
    }
    let page_size = page_bytes::<Meta>();
    let page_base = index * page_size;
    let page_end = page_base.saturating_add(page_size);
    let data_end = offset.saturating_add(data.len());
    let copy_start = cmp::max(offset, page_base);
    let copy_end = cmp::min(data_end, page_end);
    if copy_start >= copy_end {
        return;
    }
    let dst_offset = copy_start - page_base;
    let src_offset = copy_start - offset;
    let len = copy_end - copy_start;
    unsafe {
        ptr::copy_nonoverlapping(
            data.as_ptr().add(src_offset),
            page_ptr.as_ptr().add(dst_offset),
            len,
        );
    }
}

fn ensure_valid<Meta: VmMeta>(flags: &mut VmFlags<Meta>) {
    if !flags.valid() {
        *flags |= VmFlags::VALID;
    }
}
