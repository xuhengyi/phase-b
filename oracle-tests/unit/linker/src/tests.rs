extern crate std;

use super::{AppMeta, KernelLayout, KernelRegionTitle, SCRIPT};
use std::boxed::Box;
use std::vec::Vec;
use std::{format, vec};

#[repr(C)]
struct TestAppMeta<const EXTRA: usize> {
    base: u64,
    step: u64,
    count: u64,
    first: u64,
    extra: [u64; EXTRA],
}

fn as_app_meta<const EXTRA: usize>(meta: &'static TestAppMeta<EXTRA>) -> &'static AppMeta {
    unsafe { &*(meta as *const _ as *const AppMeta) }
}

#[test]
fn test_script_not_empty() {
    assert!(!SCRIPT.is_empty());
    assert!(SCRIPT.len() > 100);
}

#[test]
fn test_script_contains_sections() {
    let script = core::str::from_utf8(SCRIPT).unwrap();
    for needle in [".text", ".rodata", ".data", ".bss", ".boot", "__start", "__end"] {
        assert!(script.contains(needle), "missing section marker {needle}");
    }
    assert!(script.contains("riscv") || script.contains("RISCV"));
}

#[test]
fn test_kernel_layout_init_and_iter() {
    let layout = KernelLayout::INIT;
    assert_eq!(layout.start(), usize::MAX);
    assert_eq!(layout.end(), usize::MAX);
    assert_eq!(layout.len(), 0);

    let regions: Vec<_> = layout.iter().collect();
    assert_eq!(regions.len(), 4);
    assert!(matches!(regions[0].title, KernelRegionTitle::Text));
    assert!(matches!(regions[1].title, KernelRegionTitle::Rodata));
    assert!(matches!(regions[2].title, KernelRegionTitle::Data));
    assert!(matches!(regions[3].title, KernelRegionTitle::Boot));
    for region in &regions {
        assert!(region.range.start <= region.range.end);
        assert!(format!("{region}").contains("0x"));
    }
}

#[test]
fn test_app_meta_structure() {
    assert_eq!(core::mem::size_of::<AppMeta>(), 32);
    assert!(core::mem::size_of::<super::AppIterator>() > 0);
}

#[test]
fn test_app_iterator_reads_contiguous_app_images() {
    static IMAGES: [u8; 9] = *b"abcXYZ123";
    let start = IMAGES.as_ptr() as u64;
    let middle = unsafe { IMAGES.as_ptr().add(3) } as u64;
    let end = unsafe { IMAGES.as_ptr().add(IMAGES.len()) } as u64;
    let meta = Box::leak(Box::new(TestAppMeta {
        base: 0,
        step: 0,
        count: 2,
        first: start,
        extra: [middle, end],
    }));

    let mut iter = as_app_meta(meta).iter();
    assert_eq!(iter.next(), Some(&IMAGES[0..3]));
    assert_eq!(iter.next(), Some(&IMAGES[3..9]));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_app_iterator_copies_into_fixed_slots_and_zero_fills_tail() {
    const SLOT_SIZE: usize = 0x20_0000;
    static IMAGES: [u8; 8] = *b"RUSTtest";
    let start = IMAGES.as_ptr() as u64;
    let middle = unsafe { IMAGES.as_ptr().add(4) } as u64;
    let end = unsafe { IMAGES.as_ptr().add(IMAGES.len()) } as u64;

    let mut slots = vec![0x55u8; SLOT_SIZE * 2].into_boxed_slice();
    let base = slots.as_mut_ptr() as u64;
    let meta = Box::leak(Box::new(TestAppMeta {
        base,
        step: SLOT_SIZE as u64,
        count: 2,
        first: start,
        extra: [middle, end],
    }));

    let mut iter = as_app_meta(meta).iter();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();

    assert_eq!(first, &slots[0..4]);
    assert_eq!(first, &IMAGES[0..4]);
    assert_eq!(slots[4], 0);

    assert_eq!(second, &slots[SLOT_SIZE..SLOT_SIZE + 4]);
    assert_eq!(second, &IMAGES[4..8]);
    assert_eq!(slots[SLOT_SIZE + 4], 0);
}
