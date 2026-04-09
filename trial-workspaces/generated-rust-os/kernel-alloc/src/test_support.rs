use core::alloc::Layout;
use core::ptr::NonNull;
use std::sync::Mutex;
use std::vec;
use std::vec::Vec;

use super::MIN_BLOCK_SIZE;

static TEST_HEAP: Mutex<Option<Vec<u8>>> = Mutex::new(None);

pub(super) fn reset_test_heap(size: usize) {
    assert!(
        size % MIN_BLOCK_SIZE == 0,
        "test heap size must align to {MIN_BLOCK_SIZE} bytes"
    );
    let mut buffer = vec![0u8; size + MIN_BLOCK_SIZE];
    let base = buffer.as_mut_ptr() as usize;
    let aligned_start = align_up(base, MIN_BLOCK_SIZE);
    let end = aligned_start
        .checked_add(size)
        .expect("test heap end overflowed");
    assert!(
        end <= base + buffer.len(),
        "aligned buffer is not large enough for requested heap"
    );
    {
        let mut guard = TEST_HEAP.lock().unwrap();
        super::reset_allocator_for_tests();
        guard.take();
        *guard = Some(buffer);
    }
    unsafe { super::init(aligned_start, size) };
}

pub(super) unsafe fn allocate_layout_for_test(layout: Layout) -> NonNull<u8> {
    super::allocate(layout).expect("allocation failed during tests")
}

pub(super) unsafe fn deallocate_layout_for_test(ptr: NonNull<u8>, layout: Layout) {
    super::deallocate(ptr, layout);
}

fn align_up(value: usize, align: usize) -> usize {
    debug_assert!(align.is_power_of_two());
    (value + align - 1) & !(align - 1)
}
