#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;
use core::ptr::{addr_of, addr_of_mut, copy_nonoverlapping};

const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_SHUTDOWN: usize = 8;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

const SCAUSE_USER_ECALL: usize = 8;
const SCAUSE_ILLEGAL_INSTRUCTION: usize = 2;
const SCAUSE_STORE_FAULT: usize = 7;
const SCAUSE_STORE_PAGE_FAULT: usize = 15;

const SSTATUS_SPIE: usize = 1 << 5;
const SSTATUS_SPP: usize = 1 << 8;

const MAX_PROGRAM_COUNT: usize = 8;
const APP_LOAD_LIMIT: usize = 0x20_0000;

global_asm!(include_str!(env!("APP_ASM")));

global_asm!(
    r#"
    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call clear_bss
    call rust_main
1:
    j 1b

    .section .text.trap
    .align 2
    .globl __trap_entry
__trap_entry:
    csrrw sp, sscratch, sp
    addi sp, sp, -34 * 8

    sd x0,   0 * 8(sp)
    sd x1,   1 * 8(sp)
    csrr t0, sscratch
    sd t0,   2 * 8(sp)
    sd x3,   3 * 8(sp)
    sd x4,   4 * 8(sp)
    sd x5,   5 * 8(sp)
    sd x6,   6 * 8(sp)
    sd x7,   7 * 8(sp)
    sd x8,   8 * 8(sp)
    sd x9,   9 * 8(sp)
    sd x10, 10 * 8(sp)
    sd x11, 11 * 8(sp)
    sd x12, 12 * 8(sp)
    sd x13, 13 * 8(sp)
    sd x14, 14 * 8(sp)
    sd x15, 15 * 8(sp)
    sd x16, 16 * 8(sp)
    sd x17, 17 * 8(sp)
    sd x18, 18 * 8(sp)
    sd x19, 19 * 8(sp)
    sd x20, 20 * 8(sp)
    sd x21, 21 * 8(sp)
    sd x22, 22 * 8(sp)
    sd x23, 23 * 8(sp)
    sd x24, 24 * 8(sp)
    sd x25, 25 * 8(sp)
    sd x26, 26 * 8(sp)
    sd x27, 27 * 8(sp)
    sd x28, 28 * 8(sp)
    sd x29, 29 * 8(sp)
    sd x30, 30 * 8(sp)
    sd x31, 31 * 8(sp)

    csrr t0, sstatus
    sd t0, 32 * 8(sp)
    csrr t0, sepc
    sd t0, 33 * 8(sp)

    mv a0, sp
    call rust_trap
    mv sp, a0

    ld t0, 32 * 8(sp)
    csrw sstatus, t0
    ld t0, 33 * 8(sp)
    csrw sepc, t0

    ld x1,   1 * 8(sp)
    ld x3,   3 * 8(sp)
    ld x4,   4 * 8(sp)
    ld x6,   6 * 8(sp)
    ld x7,   7 * 8(sp)
    ld x8,   8 * 8(sp)
    ld x9,   9 * 8(sp)
    ld x10, 10 * 8(sp)
    ld x11, 11 * 8(sp)
    ld x12, 12 * 8(sp)
    ld x13, 13 * 8(sp)
    ld x14, 14 * 8(sp)
    ld x15, 15 * 8(sp)
    ld x16, 16 * 8(sp)
    ld x17, 17 * 8(sp)
    ld x18, 18 * 8(sp)
    ld x19, 19 * 8(sp)
    ld x20, 20 * 8(sp)
    ld x21, 21 * 8(sp)
    ld x22, 22 * 8(sp)
    ld x23, 23 * 8(sp)
    ld x24, 24 * 8(sp)
    ld x25, 25 * 8(sp)
    ld x26, 26 * 8(sp)
    ld x27, 27 * 8(sp)
    ld x28, 28 * 8(sp)
    ld x29, 29 * 8(sp)
    ld x30, 30 * 8(sp)
    ld x31, 31 * 8(sp)
    ld t0,   5 * 8(sp)

    ld x5,   2 * 8(sp)
    csrw sscratch, x5
    ld x5,   5 * 8(sp)
    addi sp, sp, 34 * 8
    csrrw sp, sscratch, sp
    sret

    .section .stack, "aw", @nobits
    .align 12
    .globl boot_stack
boot_stack:
    .space 16384
    .globl boot_stack_top
boot_stack_top:

    .align 12
    .globl trap_stack
trap_stack:
    .space 16384
    .globl trap_stack_top
trap_stack_top:

    .align 12
    .globl user_stack0
user_stack0:
    .space 4096
    .globl user_stack0_top
user_stack0_top:

    .align 12
    .globl user_stack1
user_stack1:
    .space 4096
    .globl user_stack1_top
user_stack1_top:

    .align 12
    .globl user_stack2
user_stack2:
    .space 4096
    .globl user_stack2_top
user_stack2_top:

    .align 12
    .globl user_stack3
user_stack3:
    .space 4096
    .globl user_stack3_top
user_stack3_top:

    .align 12
    .globl user_stack4
user_stack4:
    .space 4096
    .globl user_stack4_top
user_stack4_top:

    .align 12
    .globl user_stack5
user_stack5:
    .space 4096
    .globl user_stack5_top
user_stack5_top:

    .align 12
    .globl user_stack6
user_stack6:
    .space 4096
    .globl user_stack6_top
user_stack6_top:

    .align 12
    .globl user_stack7
user_stack7:
    .space 4096
    .globl user_stack7_top
user_stack7_top:
"#
);

#[repr(C)]
pub struct TrapContext {
    x: [usize; 32],
    sstatus: usize,
    sepc: usize,
}

extern "C" {
    static mut sbss: u8;
    static mut ebss: u8;
    static apps: usize;

    fn __trap_entry();

    static boot_stack_top: u8;
    static trap_stack_top: u8;
    static user_stack0_top: u8;
    static user_stack1_top: u8;
    static user_stack2_top: u8;
    static user_stack3_top: u8;
    static user_stack4_top: u8;
    static user_stack5_top: u8;
    static user_stack6_top: u8;
    static user_stack7_top: u8;
}

static mut CURRENT_PROGRAM: usize = 0;

#[no_mangle]
pub extern "C" fn clear_bss() {
    unsafe {
        let mut cur = addr_of_mut!(sbss);
        let end = addr_of_mut!(ebss);
        while cur < end {
            cur.write_volatile(0);
            cur = cur.add(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        write_stvec(__trap_entry as usize);
    }
    run_next_program()
}

#[no_mangle]
pub extern "C" fn rust_trap(cx: &mut TrapContext) -> *mut TrapContext {
    let scause = read_scause();
    let cause = scause & !(1usize << (usize::BITS - 1));

    match cause {
        SCAUSE_USER_ECALL => handle_user_ecall(cx),
        SCAUSE_ILLEGAL_INSTRUCTION | SCAUSE_STORE_FAULT | SCAUSE_STORE_PAGE_FAULT => {
            finish_current_program(cx, 0)
        }
        _ => finish_current_program(cx, 1),
    }

    cx
}

#[no_mangle]
pub extern "C" fn user_return() -> ! {
    run_next_program()
}

fn handle_user_ecall(cx: &mut TrapContext) {
    match cx.x[17] {
        SYSCALL_WRITE => {
            let fd = cx.x[10];
            let ptr = cx.x[11] as *const u8;
            let len = cx.x[12];
            cx.x[10] = sys_write(fd, ptr, len) as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
        }
        SYSCALL_EXIT => finish_current_program(cx, cx.x[10] as isize),
        _ => finish_current_program(cx, -1),
    }
}

fn sys_write(fd: usize, ptr: *const u8, len: usize) -> isize {
    if fd != 1 {
        return -1;
    }

    for offset in 0..len {
        let byte = unsafe { ptr.add(offset).read_volatile() };
        console_putchar(byte);
    }

    len as isize
}

fn finish_current_program(cx: &mut TrapContext, code: isize) {
    unsafe {
        CURRENT_PROGRAM = CURRENT_PROGRAM.saturating_add(1);
    }

    cx.x[10] = code as usize;
    cx.x[2] = kernel_stack_top();
    cx.sepc = user_return as usize;
    cx.sstatus = (cx.sstatus | SSTATUS_SPP | SSTATUS_SPIE) & !1usize;
}

fn run_next_program() -> ! {
    let index = unsafe { CURRENT_PROGRAM };
    if index >= app_count() {
        shutdown();
    }

    let entry = load_app(index);
    let user_sp = user_stack_top(index);
    unsafe {
        enter_user(entry, user_sp);
    }
}

fn app_table() -> *const usize {
    addr_of!(apps)
}

fn app_base() -> usize {
    unsafe { *app_table() }
}

fn app_step() -> usize {
    unsafe { *app_table().add(1) }
}

fn app_count() -> usize {
    unsafe { (*app_table().add(2)).min(MAX_PROGRAM_COUNT) }
}

fn app_range(index: usize) -> (*const u8, usize) {
    let table = app_table();
    unsafe {
        let start = *table.add(3 + index) as *const u8;
        let end = *table.add(3 + index + 1) as *const u8;
        let len = end as usize - start as usize;
        (start, len)
    }
}

fn load_app(index: usize) -> usize {
    let base = app_base() + app_step() * index;
    let (src, len) = app_range(index);
    if len > APP_LOAD_LIMIT {
        shutdown();
    }

    unsafe {
        let dst = base as *mut u8;
        for offset in 0..APP_LOAD_LIMIT {
            dst.add(offset).write_volatile(0);
        }
        copy_nonoverlapping(src, dst, len);
        asm!("fence.i", options(nomem, nostack));
    }

    base
}

unsafe fn enter_user(entry: usize, user_sp: usize) -> ! {
    let mut sstatus = read_sstatus();
    sstatus &= !SSTATUS_SPP;
    sstatus |= SSTATUS_SPIE;

    asm!(
        "csrw sstatus, {sstatus}",
        "csrw sepc, {entry}",
        "csrw sscratch, {trap_sp}",
        "mv sp, {user_sp}",
        "sret",
        sstatus = in(reg) sstatus,
        entry = in(reg) entry,
        trap_sp = in(reg) trap_stack_top_addr(),
        user_sp = in(reg) user_sp,
        options(noreturn)
    );
}

fn console_putchar(byte: u8) {
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") byte as usize => _,
            in("a7") SBI_CONSOLE_PUTCHAR,
            lateout("a1") _,
            options(nostack)
        );
    }
}

fn shutdown() -> ! {
    unsafe {
        asm!(
            "ecall",
            in("a7") SBI_SHUTDOWN,
            options(nostack)
        );
    }

    loop {
        unsafe {
            asm!("wfi", options(nomem, nostack));
        }
    }
}

unsafe fn write_stvec(addr: usize) {
    asm!("csrw stvec, {}", in(reg) addr, options(nostack));
}

fn read_sstatus() -> usize {
    let value: usize;
    unsafe {
        asm!("csrr {}, sstatus", out(reg) value, options(nomem, nostack));
    }
    value
}

fn read_scause() -> usize {
    let value: usize;
    unsafe {
        asm!("csrr {}, scause", out(reg) value, options(nomem, nostack));
    }
    value
}

fn kernel_stack_top() -> usize {
    addr_of!(boot_stack_top) as usize
}

fn trap_stack_top_addr() -> usize {
    addr_of!(trap_stack_top) as usize
}

fn user_stack_top(index: usize) -> usize {
    match index {
        0 => addr_of!(user_stack0_top) as usize,
        1 => addr_of!(user_stack1_top) as usize,
        2 => addr_of!(user_stack2_top) as usize,
        3 => addr_of!(user_stack3_top) as usize,
        4 => addr_of!(user_stack4_top) as usize,
        5 => addr_of!(user_stack5_top) as usize,
        6 => addr_of!(user_stack6_top) as usize,
        _ => addr_of!(user_stack7_top) as usize,
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    shutdown()
}
