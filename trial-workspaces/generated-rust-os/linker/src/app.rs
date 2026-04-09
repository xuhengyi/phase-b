use core::{convert::TryFrom, marker::PhantomData, ptr, slice};

#[repr(C)]
pub struct AppMeta {
    pub base: u64,
    pub step: u64,
    pub count: u64,
    pub first: u64,
}

impl AppMeta {
    pub const fn new(base: u64, step: u64, count: u64, first: u64) -> Self {
        Self {
            base,
            step,
            count,
            first,
        }
    }

    pub fn iter(&'static self) -> AppIterator {
        AppIterator {
            base: self.base,
            step: self.step,
            boundaries: &self.first as *const u64,
            count: normalize_count(self.count),
            index: 0,
            _marker: PhantomData,
        }
    }
}

const fn normalize_count(raw: u64) -> usize {
    if raw > usize::MAX as u64 {
        usize::MAX
    } else {
        raw as usize
    }
}

pub struct AppIterator {
    base: u64,
    step: u64,
    boundaries: *const u64,
    count: usize,
    index: usize,
    _marker: PhantomData<&'static AppMeta>,
}

impl AppIterator {
    #[inline]
    fn contiguous(&self) -> bool {
        self.base == 0 || self.step == 0
    }

    unsafe fn boundary(&self, idx: usize) -> usize {
        *self.boundaries.add(idx) as usize
    }

    unsafe fn contiguous_slice(&self, start: usize, len: usize) -> &'static [u8] {
        slice::from_raw_parts(start as *const u8, len)
    }

    unsafe fn slot_slice(&self, slot_index: usize, src: usize, len: usize) -> &'static [u8] {
        let slot_size = usize::try_from(self.step).unwrap_or(usize::MAX);
        assert!(
            len <= slot_size,
            "application image is larger than the configured slot size"
        );
        let offset = self
            .step
            .saturating_mul(slot_index as u64);
        let dst_addr = self.base.saturating_add(offset) as usize;
        let dst = dst_addr as *mut u8;
        let src_ptr = src as *const u8;

        let bytes_to_copy = len.min(slot_size);
        if bytes_to_copy > 0 {
            ptr::copy_nonoverlapping(src_ptr, dst, bytes_to_copy);
        }
        if slot_size > bytes_to_copy {
            ptr::write_bytes(dst.add(bytes_to_copy), 0, slot_size - bytes_to_copy);
        }

        slice::from_raw_parts(dst, bytes_to_copy)
    }
}

impl Iterator for AppIterator {
    type Item = &'static [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        let idx = self.index;
        self.index += 1;

        unsafe {
            let start = self.boundary(idx);
            let end = self.boundary(idx + 1);
            let len = end.saturating_sub(start);
            if self.contiguous() {
                Some(self.contiguous_slice(start, len))
            } else {
                Some(self.slot_slice(idx, start, len))
            }
        }
    }
}
