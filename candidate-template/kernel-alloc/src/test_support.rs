use core::alloc::Layout;
use core::ptr::NonNull;

pub(super) fn reset_test_heap(_size: usize) {
    unimplemented!("Phase B seam: reset allocator state for oracle unit tests");
}

pub(super) unsafe fn allocate_layout_for_test(_layout: Layout) -> NonNull<u8> {
    unimplemented!("Phase B seam: allocate for oracle unit tests")
}

pub(super) unsafe fn deallocate_layout_for_test(_ptr: NonNull<u8>, _layout: Layout) {
    unimplemented!("Phase B seam: deallocate for oracle unit tests");
}
