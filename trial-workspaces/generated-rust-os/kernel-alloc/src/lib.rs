#![no_std]

#[cfg(test)]
extern crate std;

#[cfg(not(test))]
use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::cell::UnsafeCell;
use core::hint::spin_loop;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, Ordering};

use customizable_buddy::{BuddyAllocator, LinkedListBuddy, UsizeBuddy};

const MIN_ORDER: usize = 12;
pub(crate) const MIN_BLOCK_SIZE: usize = 1 << MIN_ORDER;
const BUDDY_LAYERS: usize = 32;

type RawAllocator = BuddyAllocator<{ BUDDY_LAYERS }, UsizeBuddy, LinkedListBuddy>;

static HEAP: LockedHeap = LockedHeap::new();
static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: KernelGlobalAlloc = KernelGlobalAlloc;

/// 初始化全局堆分配器。
///
/// # Safety
///
/// `heap_start..heap_start + size` 必须指向一段可独占使用、对齐到页大小的内存区域，
/// 并且该区域在整个生命周期内保持有效。
pub unsafe fn init(heap_start: usize, size: usize) {
    assert_eq!(
        heap_start & (MIN_BLOCK_SIZE - 1),
        0,
        "heap start must align to {MIN_BLOCK_SIZE} bytes"
    );
    assert!(
        !IS_INITIALIZED.load(Ordering::Acquire),
        "kernel allocator has already been initialized"
    );
    let base = NonNull::new(heap_start as *mut u8).expect("heap base must be non-null");
    HEAP.with_allocator(|allocator| {
        allocator.init(MIN_ORDER, base);
        if size != 0 {
            add_region(allocator, base, size);
        }
    });
    IS_INITIALIZED.store(true, Ordering::Release);
}

/// 把一段新的内存区域纳入管理。
///
/// # Safety
///
/// 传入的内存区域必须和现有堆互不重叠并且保持对齐。
pub unsafe fn transfer(region_start: usize, size: usize) {
    if size == 0 {
        return;
    }
    assert!(
        IS_INITIALIZED.load(Ordering::Acquire),
        "allocator must be initialized before transfer"
    );
    ensure_region(region_start, size);
    let ptr = NonNull::new(region_start as *mut u8).expect("region start must be valid");
    HEAP.with_allocator(|allocator| add_region(allocator, ptr, size));
}

fn add_region(allocator: &mut RawAllocator, base: NonNull<u8>, size: usize) {
    assert_eq!(
        size & (MIN_BLOCK_SIZE - 1),
        0,
        "region size must align to {MIN_BLOCK_SIZE} bytes"
    );
    let start = base.as_ptr() as usize;
    start
        .checked_add(size)
        .expect("region address overflow");
    unsafe { allocator.transfer(base, size) };
}

fn ensure_region(start: usize, size: usize) {
    assert_eq!(
        start & (MIN_BLOCK_SIZE - 1),
        0,
        "region start must align to {MIN_BLOCK_SIZE} bytes"
    );
    assert_eq!(
        size & (MIN_BLOCK_SIZE - 1),
        0,
        "region size must align to {MIN_BLOCK_SIZE} bytes"
    );
    start
        .checked_add(size)
        .expect("region start + size exceeds usize");
}

fn allocate(layout: Layout) -> Option<NonNull<u8>> {
    if layout.size() == 0 {
        return NonNull::new(layout.align() as *mut u8);
    }
    HEAP.with_allocator(|allocator| {
        allocator
            .allocate_layout::<u8>(layout)
            .ok()
            .map(|(ptr, _)| ptr)
    })
}

fn deallocate(ptr: NonNull<u8>, layout: Layout) {
    if layout.size() == 0 {
        return;
    }
    HEAP.with_allocator(|allocator| unsafe { allocator.deallocate_layout(ptr, layout) });
}

#[cfg(not(test))]
struct KernelGlobalAlloc;

#[cfg(not(test))]
unsafe impl GlobalAlloc for KernelGlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        allocate(layout).map_or(core::ptr::null_mut(), NonNull::as_ptr)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Some(ptr) = NonNull::new(ptr) {
            deallocate(ptr, layout);
        }
    }
}

struct LockedHeap {
    lock: SpinLock,
    allocator: UnsafeCell<RawAllocator>,
}

unsafe impl Sync for LockedHeap {}

impl LockedHeap {
    const fn new() -> Self {
        Self {
            lock: SpinLock::new(),
            allocator: UnsafeCell::new(RawAllocator::new()),
        }
    }

    fn lock(&self) -> LockedHeapGuard<'_> {
        LockedHeapGuard {
            _guard: self.lock.lock(),
            allocator: unsafe { &mut *self.allocator.get() },
        }
    }

    fn with_allocator<R>(&self, f: impl FnOnce(&mut RawAllocator) -> R) -> R {
        let mut guard = self.lock();
        f(&mut *guard)
    }

    #[cfg(test)]
    fn reset(&self) {
        self.with_allocator(|allocator| *allocator = RawAllocator::new());
    }
}

struct LockedHeapGuard<'a> {
    _guard: SpinLockGuard<'a>,
    allocator: &'a mut RawAllocator,
}

impl<'a> Deref for LockedHeapGuard<'a> {
    type Target = RawAllocator;

    fn deref(&self) -> &Self::Target {
        self.allocator
    }
}

impl<'a> DerefMut for LockedHeapGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.allocator
    }
}

struct SpinLock {
    flag: AtomicBool,
}

impl SpinLock {
    const fn new() -> Self {
        Self {
            flag: AtomicBool::new(false),
        }
    }

    fn lock(&self) -> SpinLockGuard<'_> {
        while self
            .flag
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.flag.load(Ordering::Relaxed) {
                spin_loop();
            }
        }
        SpinLockGuard { lock: self }
    }

    fn unlock(&self) {
        self.flag.store(false, Ordering::Release);
    }
}

struct SpinLockGuard<'a> {
    lock: &'a SpinLock,
}

impl Drop for SpinLockGuard<'_> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}

#[cfg(test)]
pub(crate) fn reset_allocator_for_tests() {
    IS_INITIALIZED.store(false, Ordering::Release);
    HEAP.reset();
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_support;
