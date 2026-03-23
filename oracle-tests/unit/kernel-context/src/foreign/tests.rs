extern crate std;

use super::{ForeignContext, MonoForeignPortal, MultislotPortal, PortalCache};
use crate::LocalContext;
use std::alloc::{alloc_zeroed, Layout};

fn aligned_backing(size: usize) -> usize {
    let layout = Layout::from_size_align(size, 16).unwrap();
    let ptr = unsafe { alloc_zeroed(layout) };
    assert!(!ptr.is_null());
    ptr as usize
}

#[test]
fn test_portal_cache_init_matches_current_layout_rules() {
    let mut cache: PortalCache = unsafe { core::mem::zeroed() };
    cache.init(0x88, 0x99, 1, true, false);

    assert_eq!(cache.satp, 0x88);
    assert_eq!(cache.sepc, 0x99);
    assert_eq!(cache.a0, 1);
    assert_eq!(cache.sstatus, 1 << 8);
    assert_eq!(cache.address(), &mut cache as *mut _ as usize);
}

#[test]
fn test_multislot_portal_offsets_are_monotonic() {
    let slots = 3;
    let size = MultislotPortal::calculate_size(slots);
    let transit = aligned_backing(size);
    let portal = unsafe { MultislotPortal::init_transit(transit, slots) };

    assert_eq!(portal.text_offset(), core::mem::size_of::<MultislotPortal>());
    assert!(portal.cache_offset(1) > portal.cache_offset(0));
    assert!(portal.cache_offset(2) > portal.cache_offset(1));
}

#[cfg(not(target_arch = "riscv64"))]
#[test]
#[should_panic(expected = "execute() is only available on RISC-V 64-bit targets")]
fn test_foreign_execute_panics_on_non_riscv_host() {
    let size = MultislotPortal::calculate_size(1);
    let transit = aligned_backing(size);
    let portal = unsafe { MultislotPortal::init_transit(transit, 1) };
    let mut foreign = ForeignContext {
        context: LocalContext::user(0x1000),
        satp: 0x1234,
    };
    unsafe {
        foreign.execute(portal, ());
    }
}
