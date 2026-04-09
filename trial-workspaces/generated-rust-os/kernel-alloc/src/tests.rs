extern crate std;

use super::test_support::{
    allocate_layout_for_test,
    deallocate_layout_for_test,
    reset_test_heap,
};
use core::alloc::Layout;
use std::sync::Mutex;

static TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_allocate_and_deallocate_layout_after_init() {
    let _guard = TEST_LOCK.lock().unwrap();
    reset_test_heap(256 * 1024);
    let layout = Layout::from_size_align(128, 8).unwrap();
    let ptr = unsafe { allocate_layout_for_test(layout) };
    unsafe {
        ptr.as_ptr().write_bytes(0xab, 128);
        deallocate_layout_for_test(ptr, layout);
    }
}

#[test]
fn test_multiple_allocations_return_distinct_ranges() {
    let _guard = TEST_LOCK.lock().unwrap();
    reset_test_heap(256 * 1024);
    let small = Layout::from_size_align(64, 8).unwrap();
    let large = Layout::from_size_align(512, 16).unwrap();

    let p1 = unsafe { allocate_layout_for_test(small) };
    let p2 = unsafe { allocate_layout_for_test(large) };

    assert_ne!(p1.as_ptr(), p2.as_ptr());

    unsafe {
        deallocate_layout_for_test(p2, large);
        deallocate_layout_for_test(p1, small);
    }
}
