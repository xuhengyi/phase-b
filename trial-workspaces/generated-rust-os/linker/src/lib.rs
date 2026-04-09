#![no_std]

use core::{fmt, ops::Range};

mod app;

pub use app::{AppIterator, AppMeta};

/// Default linker script template shared across tutorial kernels.
///
/// The script describes the canonical section layout for the qemu-virt RISC-V
/// target so that every chapter can simply write the bytes to disk inside
/// `build.rs`.
pub const SCRIPT: &[u8] = br#"
/* rCore Tutorial default linker script */
OUTPUT_ARCH(riscv)
ENTRY(_start)

MEMORY
{
    RAM (wxa) : ORIGIN = 0x80000000, LENGTH = 16M
}

SECTIONS
{
    . = ORIGIN(RAM);
    __start = .;

    .text : ALIGN(4)
    {
        __text_start = .;
        *(.text.entry)
        *(.text .text.*)
        __text_end = .;
    } > RAM

    .rodata : ALIGN(4)
    {
        __rodata_start = .;
        *(.rodata .rodata.*)
        __rodata_end = .;
    } > RAM

    .data : ALIGN(4)
    {
        __data_start = .;
        *(.data .data.*)
        __data_end = .;
    } > RAM

    .bss (NOLOAD) : ALIGN(4)
    {
        __bss_start = .;
        *(.sbss .sbss.* .bss .bss.*)
        *(COMMON)
        __bss_end = .;
    } > RAM

    .boot : ALIGN(4)
    {
        __boot_start = .;
        *(.text.boot .boot .boot.*)
        __boot_end = .;
    } > RAM

    . = ALIGN(8);
    __end = .;
}
"#;

/// Logical title for a kernel region exposed through [`KernelLayout`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KernelRegionTitle {
    Text,
    Rodata,
    Data,
    Boot,
}

/// A single contiguous kernel region alongside a friendly title.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KernelRegion {
    pub title: KernelRegionTitle,
    pub range: Range<usize>,
}

impl fmt::Display for KernelRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: [0x{:016x} .. 0x{:016x})",
            self.title, self.range.start, self.range.end
        )
    }
}

const fn empty_region(title: KernelRegionTitle) -> KernelRegion {
    KernelRegion {
        title,
        range: Range {
            start: usize::MAX,
            end: usize::MAX,
        },
    }
}

const INIT_REGIONS: [KernelRegion; 4] = [
    empty_region(KernelRegionTitle::Text),
    empty_region(KernelRegionTitle::Rodata),
    empty_region(KernelRegionTitle::Data),
    empty_region(KernelRegionTitle::Boot),
];

/// A static description of the kernel memory layout.
#[derive(Clone, Copy)]
pub struct KernelLayout {
    start_addr: usize,
    end_addr: usize,
    regions: &'static [KernelRegion],
}

impl KernelLayout {
    /// Initial placeholder layout that callers can extend with actual values.
    pub const INIT: Self = Self {
        start_addr: usize::MAX,
        end_addr: usize::MAX,
        regions: &INIT_REGIONS,
    };

    /// Returns the inclusive starting address of the kernel image.
    pub const fn start(&self) -> usize {
        self.start_addr
    }

    /// Returns the exclusive ending address of the kernel image.
    pub const fn end(&self) -> usize {
        self.end_addr
    }

    /// Returns the size of the kernel image.
    pub const fn len(&self) -> usize {
        self.end_addr.saturating_sub(self.start_addr)
    }

    /// Returns an iterator over the known regions.
    pub const fn iter(&self) -> KernelRegionIter<'_> {
        KernelRegionIter {
            regions: self.regions,
            index: 0,
        }
    }
}

/// Iterator over [`KernelRegion`] entries.
pub struct KernelRegionIter<'a> {
    regions: &'a [KernelRegion],
    index: usize,
}

impl<'a> Iterator for KernelRegionIter<'a> {
    type Item = KernelRegion;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.regions.len() {
            return None;
        }
        let region = self.regions[self.index].clone();
        self.index += 1;
        Some(region)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.regions.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for KernelRegionIter<'a> {}
impl<'a> core::iter::FusedIterator for KernelRegionIter<'a> {}

/// Declare the low level boot entry for the kernel.
#[macro_export]
macro_rules! boot0 {
    ($entry:path; stack = $stack:expr) => {
        #[no_mangle]
        #[link_section = ".bss.stack"]
        static mut __BOOT_STACK: [u8; $stack] = [0; $stack];

        #[export_name = "_start"]
        #[link_section = ".text.boot"]
        pub unsafe extern "C" fn __boot_start() -> ! {
            unsafe {
                let top = __BOOT_STACK.as_ptr().add($stack);
                core::arch::asm!(
                    "mv sp, {stack}",
                    stack = in(reg) top,
                );
                let entry: fn() -> ! = $entry;
                entry()
            }
        }
    };
}

#[cfg(test)]
mod tests;
