#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;
use core::ptr::{addr_of, addr_of_mut, copy_nonoverlapping};

const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_SHUTDOWN: usize = 8;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_CLOCK_GETTIME: usize = 113;
const SYSCALL_SCHED_YIELD: usize = 124;

const SCAUSE_INTERRUPT: usize = 1usize << (usize::BITS as usize - 1);
const SCAUSE_INSTRUCTION_FAULT: usize = 1;
const SCAUSE_ILLEGAL_INSTRUCTION: usize = 2;
const SCAUSE_LOAD_FAULT: usize = 5;
const SCAUSE_STORE_FAULT: usize = 7;
const SCAUSE_USER_ECALL: usize = 8;
const SCAUSE_INSTRUCTION_PAGE_FAULT: usize = 12;
const SCAUSE_LOAD_PAGE_FAULT: usize = 13;
const SCAUSE_STORE_PAGE_FAULT: usize = 15;

const SSTATUS_SPIE: usize = 1 << 5;
const SSTATUS_SPP: usize = 1 << 8;

const MAX_TASKS: usize = 12;
const APP_LOAD_LIMIT: usize = 0x20_0000;
const USER_STACK_SIZE: usize = 0x4000;

const TASK_UNUSED: usize = 0;
const TASK_READY: usize = 1;
const TASK_RUNNING: usize = 2;
const TASK_EXITED: usize = 3;
const TASK_FAULTED: usize = 4;

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

    .globl __restore_from_context
__restore_from_context:
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

    ld x5,   2 * 8(sp)
    csrw sscratch, x5
    ld x5,   5 * 8(sp)
    addi sp, sp, 34 * 8
    csrrw sp, sscratch, sp
    sret

    .globl __restore_user
__restore_user:
    mv sp, a0
    j __restore_from_context

    .section .stack, "aw", @nobits
    .align 12
    .globl boot_stack
boot_stack:
    .space 16384
    .globl boot_stack_top
boot_stack_top:
"#
);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TrapContext {
    x: [usize; 32],
    sstatus: usize,
    sepc: usize,
}

#[derive(Copy, Clone)]
struct Task {
    context: TrapContext,
    status: usize,
    exit_code: isize,
    base: usize,
    limit: usize,
}

#[repr(align(4096))]
struct UserStacks([[u8; USER_STACK_SIZE]; MAX_TASKS]);

extern "C" {
    static mut sbss: u8;
    static mut ebss: u8;
    static apps: usize;

    fn __trap_entry();
    fn __restore_user(cx: *mut TrapContext) -> !;
}

const EMPTY_CONTEXT: TrapContext = TrapContext {
    x: [0; 32],
    sstatus: 0,
    sepc: 0,
};

const EMPTY_TASK: Task = Task {
    context: EMPTY_CONTEXT,
    status: TASK_UNUSED,
    exit_code: 0,
    base: 0,
    limit: 0,
};

static mut TASKS: [Task; MAX_TASKS] = [EMPTY_TASK; MAX_TASKS];
static mut USER_STACKS: UserStacks = UserStacks([[0; USER_STACK_SIZE]; MAX_TASKS]);
static mut TASK_COUNT: usize = 0;
static mut CURRENT_TASK: usize = 0;
static mut MONOTONIC_US: usize = 0;

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
    init_tasks();
    run_first_task()
}

#[no_mangle]
pub extern "C" fn rust_trap(cx: &mut TrapContext) -> *mut TrapContext {
    let scause = read_scause();
    if scause & SCAUSE_INTERRUPT != 0 {
        return switch_to_next_task();
    }

    match scause {
        SCAUSE_USER_ECALL => handle_user_ecall(cx),
        SCAUSE_INSTRUCTION_FAULT
        | SCAUSE_ILLEGAL_INSTRUCTION
        | SCAUSE_LOAD_FAULT
        | SCAUSE_STORE_FAULT
        | SCAUSE_INSTRUCTION_PAGE_FAULT
        | SCAUSE_LOAD_PAGE_FAULT
        | SCAUSE_STORE_PAGE_FAULT => finish_current_task(TASK_FAULTED, -1),
        _ => finish_current_task(TASK_FAULTED, -1),
    }
}

fn handle_user_ecall(cx: &mut TrapContext) -> *mut TrapContext {
    match cx.x[17] {
        SYSCALL_WRITE => {
            let ret = sys_write(cx.x[10], cx.x[11], cx.x[12]);
            cx.x[10] = ret as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
            current_context_ptr()
        }
        SYSCALL_EXIT => finish_current_task(TASK_EXITED, cx.x[10] as isize),
        SYSCALL_CLOCK_GETTIME => {
            let ret = sys_clock_gettime(cx.x[10], cx.x[11]);
            cx.x[10] = ret as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
            current_context_ptr()
        }
        SYSCALL_SCHED_YIELD => {
            cx.x[10] = 0;
            cx.sepc = cx.sepc.wrapping_add(4);
            switch_to_next_task()
        }
        _ => {
            cx.x[10] = (-1isize) as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
            current_context_ptr()
        }
    }
}

fn sys_write(fd: usize, ptr: usize, len: usize) -> isize {
    if fd != 1 && fd != 2 {
        return -1;
    }
    if !valid_user_range(ptr, len) {
        return -1;
    }

    for offset in 0..len {
        let byte = unsafe { (ptr as *const u8).add(offset).read_volatile() };
        console_putchar(byte);
    }

    len as isize
}

fn sys_clock_gettime(_clock_id: usize, timespec_ptr: usize) -> isize {
    let len = 2 * core::mem::size_of::<usize>();
    if timespec_ptr % core::mem::align_of::<usize>() != 0 || !valid_user_range(timespec_ptr, len) {
        return -1;
    }

    let now = next_time_us();
    let out = timespec_ptr as *mut usize;
    unsafe {
        out.write_volatile(now / 1_000_000);
        out.add(1).write_volatile(now % 1_000_000);
    }
    0
}

fn init_tasks() {
    let count = app_count().min(MAX_TASKS);
    if count == 0 {
        shutdown();
    }

    unsafe {
        TASK_COUNT = count;
    }

    for index in 0..count {
        let entry = load_app(index);
        let base = app_base() + app_step() * index;
        let mut context = EMPTY_CONTEXT;
        context.x[2] = user_stack_top(index);
        context.sstatus = user_sstatus();
        context.sepc = entry;

        unsafe {
            TASKS[index] = Task {
                context,
                status: TASK_READY,
                exit_code: 0,
                base,
                limit: base + app_slot_size(),
            };
        }
    }
}

fn run_first_task() -> ! {
    unsafe {
        CURRENT_TASK = 0;
        TASKS[0].status = TASK_RUNNING;
        __restore_user(addr_of_mut!(TASKS[0].context));
    }
}

fn finish_current_task(status: usize, exit_code: isize) -> *mut TrapContext {
    unsafe {
        TASKS[CURRENT_TASK].status = status;
        TASKS[CURRENT_TASK].exit_code = exit_code;
    }
    schedule_from_current()
}

fn switch_to_next_task() -> *mut TrapContext {
    unsafe {
        if TASKS[CURRENT_TASK].status == TASK_RUNNING {
            TASKS[CURRENT_TASK].status = TASK_READY;
        }
    }
    schedule_from_current()
}

fn schedule_from_current() -> *mut TrapContext {
    let start = unsafe { CURRENT_TASK };
    if let Some(next) = find_next_ready(start) {
        unsafe {
            CURRENT_TASK = next;
            TASKS[next].status = TASK_RUNNING;
            addr_of_mut!(TASKS[next].context)
        }
    } else {
        shutdown();
    }
}

fn find_next_ready(start: usize) -> Option<usize> {
    let count = unsafe { TASK_COUNT };
    for offset in 1..=count {
        let index = (start + offset) % count;
        let ready = unsafe { TASKS[index].status == TASK_READY };
        if ready {
            return Some(index);
        }
    }
    None
}

fn current_context_ptr() -> *mut TrapContext {
    unsafe { addr_of_mut!(TASKS[CURRENT_TASK].context) }
}

fn valid_user_range(ptr: usize, len: usize) -> bool {
    if len == 0 {
        return true;
    }
    let Some(end) = ptr.checked_add(len) else {
        return false;
    };

    let current = unsafe { CURRENT_TASK };
    let (base, limit) = unsafe { (TASKS[current].base, TASKS[current].limit) };
    if ptr >= base && end <= limit {
        return true;
    }

    let stack_bottom = user_stack_bottom(current);
    let stack_top = stack_bottom + USER_STACK_SIZE;
    ptr >= stack_bottom && end <= stack_top
}

fn next_time_us() -> usize {
    unsafe {
        MONOTONIC_US = MONOTONIC_US.wrapping_add(100_000);
        MONOTONIC_US
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

fn app_slot_size() -> usize {
    let step = app_step();
    if step == 0 {
        APP_LOAD_LIMIT
    } else {
        step.min(APP_LOAD_LIMIT)
    }
}

fn app_count() -> usize {
    unsafe { (*app_table().add(2)).min(MAX_TASKS) }
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
    let limit = app_slot_size();
    let (src, len) = app_range(index);
    if len > limit {
        shutdown();
    }

    unsafe {
        let dst = base as *mut u8;
        for offset in 0..limit {
            dst.add(offset).write_volatile(0);
        }
        copy_nonoverlapping(src, dst, len);
        asm!("fence.i", options(nomem, nostack));
    }

    base
}

fn user_sstatus() -> usize {
    let mut sstatus = read_sstatus();
    sstatus &= !SSTATUS_SPP;
    sstatus |= SSTATUS_SPIE;
    sstatus
}

fn user_stack_bottom(index: usize) -> usize {
    unsafe { addr_of!(USER_STACKS.0) as usize + index * USER_STACK_SIZE }
}

fn user_stack_top(index: usize) -> usize {
    user_stack_bottom(index) + USER_STACK_SIZE
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

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    shutdown()
}
