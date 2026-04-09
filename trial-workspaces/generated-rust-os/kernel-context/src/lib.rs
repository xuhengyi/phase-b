#![no_std]

#[cfg(test)]
extern crate std;

#[cfg(feature = "foreign")]
pub mod foreign;

#[cfg(feature = "foreign")]
pub use foreign::{
    ForeignContext, ForeignPortal, MonoForeignPortal, MultislotPortal, PortalCache, SlotKey,
    TpReg,
};

const REGISTER_COUNT: usize = 32;
const ARGUMENT_REGISTER_COUNT: usize = 8;
const ARGUMENT_REGISTER_BASE: usize = 10;
const STACK_POINTER_INDEX: usize = 2;
const RETURN_ADDRESS_INDEX: usize = 1;
const INSTRUCTION_SIZE: usize = 4;

#[repr(C)]
#[derive(Clone)]
pub struct LocalContext {
    pub supervisor: bool,
    pub interrupt: bool,
    pc: usize,
    registers: [usize; REGISTER_COUNT],
}

impl LocalContext {
    pub const fn empty() -> Self {
        Self {
            supervisor: false,
            interrupt: false,
            pc: 0,
            registers: [0; REGISTER_COUNT],
        }
    }

    pub fn user(pc: usize) -> Self {
        let mut ctx = Self::empty();
        ctx.interrupt = true;
        ctx.pc = pc;
        ctx
    }

    pub fn thread(pc: usize, interrupt: bool) -> Self {
        let mut ctx = Self::empty();
        ctx.supervisor = true;
        ctx.interrupt = interrupt;
        ctx.pc = pc;
        ctx
    }

    fn validate_index(index: usize) -> usize {
        assert!(index < REGISTER_COUNT, "register index out of range");
        index
    }

    #[inline]
    pub fn x(&self, index: usize) -> usize {
        self.registers[Self::validate_index(index)]
    }

    #[inline]
    pub fn x_mut(&mut self, index: usize) -> &mut usize {
        let index = Self::validate_index(index);
        &mut self.registers[index]
    }

    #[inline]
    pub fn ra(&self) -> usize {
        self.x(RETURN_ADDRESS_INDEX)
    }

    #[inline]
    pub fn sp(&self) -> usize {
        self.x(STACK_POINTER_INDEX)
    }

    #[inline]
    pub fn sp_mut(&mut self) -> &mut usize {
        self.x_mut(STACK_POINTER_INDEX)
    }

    #[inline]
    pub fn a(&self, index: usize) -> usize {
        assert!(index < ARGUMENT_REGISTER_COUNT, "argument register index out of range");
        self.x(ARGUMENT_REGISTER_BASE + index)
    }

    #[inline]
    pub fn a_mut(&mut self, index: usize) -> &mut usize {
        assert!(index < ARGUMENT_REGISTER_COUNT, "argument register index out of range");
        self.x_mut(ARGUMENT_REGISTER_BASE + index)
    }

    #[inline]
    pub fn pc(&self) -> usize {
        self.pc
    }

    #[inline]
    pub fn pc_mut(&mut self) -> &mut usize {
        &mut self.pc
    }

    #[inline]
    pub fn move_next(&mut self) {
        self.pc = self.pc.wrapping_add(INSTRUCTION_SIZE);
    }

    pub unsafe fn execute(&mut self) -> ! {
        #[cfg(not(target_arch = "riscv64"))]
        {
            panic!("execute() is only available on RISC-V 64-bit targets");
        }

        #[cfg(target_arch = "riscv64")]
        {
            core::arch::asm!(
                "unimp",
                options(noreturn)
            );
        }
    }
}

impl Default for LocalContext {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests;
