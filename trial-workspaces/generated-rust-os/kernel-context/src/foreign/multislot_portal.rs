use core::mem::{align_of, size_of};
use core::ptr;

use super::PortalCache;

const PORTAL_TEXT_STRIDE: usize = 64;

#[repr(C)]
pub struct MultislotPortal {
    slots: usize,
    text_len: usize,
}

impl MultislotPortal {
    pub const fn calculate_size(slots: usize) -> usize {
        let slots = ensure_min_slots(slots);
        let text = Self::text_region_len_for(slots);
        size_of::<Self>() + text + slots * size_of::<PortalCache>()
    }

    const fn text_region_len_for(slots: usize) -> usize {
        let slots = ensure_min_slots(slots);
        align_up(slots * PORTAL_TEXT_STRIDE, align_of::<PortalCache>())
    }

    pub unsafe fn init_transit(base: usize, slots: usize) -> &'static mut Self {
        assert!(slots > 0, "portal requires at least one slot");
        assert_eq!(base % align_of::<Self>(), 0, "portal base must be aligned");
        let size = Self::calculate_size(slots);
        let ptr = base as *mut u8;
        let portal = &mut *(ptr as *mut MultislotPortal);
        ptr::write_bytes(ptr, 0, size);
        portal.slots = slots;
        portal.text_len = Self::text_region_len_for(slots);
        portal
    }

    pub fn slot_count(&self) -> usize {
        self.slots
    }

    pub fn text_offset(&self) -> usize {
        size_of::<Self>()
    }

    pub fn cache_offset(&self, slot: usize) -> usize {
        assert!(slot < self.slots, "slot index out of range");
        self.text_offset() + self.text_len + slot * size_of::<PortalCache>()
    }

    pub fn cache_mut(&mut self, slot: usize) -> &mut PortalCache {
        let offset = self.cache_offset(slot);
        unsafe { &mut *(((self as *mut _ as usize) + offset) as *mut PortalCache) }
    }

    pub fn cache(&self, slot: usize) -> &PortalCache {
        let offset = self.cache_offset(slot);
        unsafe { &*(((self as *const _ as usize) + offset) as *const PortalCache) }
    }

    pub fn text_len(&self) -> usize {
        self.text_len
    }
}

const fn align_up(value: usize, align: usize) -> usize {
    let mask = align - 1;
    (value + mask) & !mask
}

const fn ensure_min_slots(slots: usize) -> usize {
    if slots == 0 { 1 } else { slots }
}
