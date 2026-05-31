#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;
use core::ptr::{addr_of, addr_of_mut, copy_nonoverlapping};

const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_SHUTDOWN: usize = 8;

const SYSCALL_OPENAT: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE2: usize = 59;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_CLOCK_GETTIME: usize = 113;
const SYSCALL_SCHED_YIELD: usize = 124;
const SYSCALL_KILL: usize = 129;
const SYSCALL_RT_SIGACTION: usize = 134;
const SYSCALL_RT_SIGPROCMASK: usize = 135;
const SYSCALL_RT_SIGRETURN: usize = 139;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_BRK: usize = 214;
const SYSCALL_CLONE: usize = 220;
const SYSCALL_EXECVE: usize = 221;
const SYSCALL_WAIT4: usize = 260;

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
const SSTATUS_SUM: usize = 1 << 18;

const MAX_TASKS: usize = 96;
const MAX_FD: usize = 16;
const MAX_FILES: usize = 16;
const MAX_PIPES: usize = 16;
const MAX_SIG: usize = 32;
const FILE_NAME_SIZE: usize = 32;
const FILE_DATA_SIZE: usize = 4096;
const PIPE_DATA_SIZE: usize = 4096;
const APP_LOAD_LIMIT: usize = 0x20_0000;
const USER_STACK_SIZE: usize = 0x4000;
const KERNEL_STACK_SIZE: usize = 0x4000;
const PAGE_SIZE: usize = 0x1000;
const PROGRAM_STATIC_SIZE: usize = 0x80_000;
const USER_BASE: usize = 0x8600_0000;
const PROCESS_BASE: usize = 0x9000_0000;
const KERNEL_MAP_START: usize = 0x8000_0000;
const KERNEL_MAP_END: usize = 0xA000_0000;
const SV39_MODE: usize = 8usize << 60;
const PTE_V: usize = 1 << 0;
const PTE_R: usize = 1 << 1;
const PTE_W: usize = 1 << 2;
const PTE_X: usize = 1 << 3;
const PTE_U: usize = 1 << 4;
const PTE_A: usize = 1 << 6;
const PTE_D: usize = 1 << 7;

const TASK_UNUSED: usize = 0;
const TASK_READY: usize = 1;
const TASK_RUNNING: usize = 2;
const TASK_EXITED: usize = 3;
const TASK_FAULTED: usize = 4;

const FD_UNUSED: usize = 0;
const FD_STDIN: usize = 1;
const FD_STDOUT: usize = 2;
const FD_FILE: usize = 3;
const FD_PIPE_READ: usize = 4;
const FD_PIPE_WRITE: usize = 5;

const OPEN_WRONLY: usize = 1 << 0;
const OPEN_RDWR: usize = 1 << 1;
const OPEN_CREATE: usize = 1 << 9;
const OPEN_TRUNC: usize = 1 << 10;

const SIGKILL: usize = 9;
const SIGUSR1: usize = 10;

const APP_NAMES: [&[u8]; 29] = [
    b"00hello_world",
    b"01store_fault",
    b"02power",
    b"03priv_inst",
    b"04priv_csr",
    b"05write_a",
    b"06write_b",
    b"07write_c",
    b"08power_3",
    b"09power_5",
    b"10power_7",
    b"12forktest",
    b"13forktree",
    b"14forktest2",
    b"15matrix",
    b"fork_exit",
    b"forktest_simple",
    b"sbrk",
    b"filetest_simple",
    b"cat_filea",
    b"sig_simple",
    b"sig_simple2",
    b"sig_ctrlc",
    b"sig_tests",
    b"pipetest",
    b"pipe_large_test",
    b"ch7b_usertest",
    b"user_shell",
    b"initproc",
];

const INITPROC_APP: usize = 28;

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

#[repr(C)]
#[derive(Copy, Clone)]
struct FdEntry {
    kind: usize,
    file_index: usize,
    offset: usize,
    readable: usize,
    writable: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FileEntry {
    used: usize,
    len: usize,
    name_len: usize,
    name: [u8; FILE_NAME_SIZE],
    data: [u8; FILE_DATA_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct PipeEntry {
    used: usize,
    read_open: usize,
    write_open: usize,
    read_offset: usize,
    len: usize,
    data: [u8; PIPE_DATA_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SignalAction {
    handler: usize,
    mask: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Task {
    kernel_stack: [u8; KERNEL_STACK_SIZE],
    context: TrapContext,
    status: usize,
    exit_code: isize,
    pid: usize,
    parent_pid: usize,
    base: usize,
    phys_base: usize,
    program_end: usize,
    heap_base: usize,
    brk: usize,
    heap_limit: usize,
    stack_bottom: usize,
    stack_top: usize,
    fds: [FdEntry; MAX_FD],
    signal_actions: [SignalAction; MAX_SIG],
    signal_mask: usize,
    signal_pending: usize,
    handling_signal: usize,
    saved_signal_context: TrapContext,
}

#[repr(align(4096))]
struct PageTables([[usize; 512]; MAX_TASKS]);

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

const EMPTY_FD: FdEntry = FdEntry {
    kind: FD_UNUSED,
    file_index: 0,
    offset: 0,
    readable: 0,
    writable: 0,
};

const EMPTY_FILE: FileEntry = FileEntry {
    used: 0,
    len: 0,
    name_len: 0,
    name: [0; FILE_NAME_SIZE],
    data: [0; FILE_DATA_SIZE],
};

const EMPTY_PIPE: PipeEntry = PipeEntry {
    used: 0,
    read_open: 0,
    write_open: 0,
    read_offset: 0,
    len: 0,
    data: [0; PIPE_DATA_SIZE],
};

const EMPTY_SIGNAL_ACTION: SignalAction = SignalAction {
    handler: 0,
    mask: 0,
};

const EMPTY_TASK: Task = Task {
    kernel_stack: [0; KERNEL_STACK_SIZE],
    context: EMPTY_CONTEXT,
    status: TASK_UNUSED,
    exit_code: 0,
    pid: 0,
    parent_pid: 0,
    base: 0,
    phys_base: 0,
    program_end: 0,
    heap_base: 0,
    brk: 0,
    heap_limit: 0,
    stack_bottom: 0,
    stack_top: 0,
    fds: [EMPTY_FD; MAX_FD],
    signal_actions: [EMPTY_SIGNAL_ACTION; MAX_SIG],
    signal_mask: 0,
    signal_pending: 0,
    handling_signal: 0,
    saved_signal_context: EMPTY_CONTEXT,
};

static mut TASKS: [Task; MAX_TASKS] = [EMPTY_TASK; MAX_TASKS];
static mut FILES: [FileEntry; MAX_FILES] = [EMPTY_FILE; MAX_FILES];
static mut PIPES: [PipeEntry; MAX_PIPES] = [EMPTY_PIPE; MAX_PIPES];
static mut ROOT_TABLES: PageTables = PageTables([[0; 512]; MAX_TASKS]);
static mut L1_TABLES: PageTables = PageTables([[0; 512]; MAX_TASKS]);
static mut USER_L0_TABLES: PageTables = PageTables([[0; 512]; MAX_TASKS]);
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
        SYSCALL_OPENAT => {
            let ret = sys_open(cx.x[10], cx.x[11]);
            cx.x[10] = ret as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
            current_context_ptr()
        }
        SYSCALL_CLOSE => {
            let ret = sys_close(cx.x[10]);
            cx.x[10] = ret as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
            current_context_ptr()
        }
        SYSCALL_PIPE2 => {
            let ret = sys_pipe(cx.x[10]);
            cx.x[10] = ret as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
            current_context_ptr()
        }
        SYSCALL_READ => {
            let ret = sys_read(cx.x[10], cx.x[11], cx.x[12]);
            cx.x[10] = ret as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
            current_context_ptr()
        }
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
            deliver_pending_signal(cx);
            switch_to_next_task()
        }
        SYSCALL_KILL => {
            let ret = sys_kill(cx.x[10], cx.x[11]);
            cx.x[10] = ret as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
            deliver_pending_signal(cx);
            current_context_ptr()
        }
        SYSCALL_RT_SIGACTION => {
            let ret = sys_sigaction(cx.x[10], cx.x[11], cx.x[12]);
            cx.x[10] = ret as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
            current_context_ptr()
        }
        SYSCALL_RT_SIGPROCMASK => {
            let ret = sys_sigprocmask(cx.x[10]);
            cx.x[10] = ret as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
            current_context_ptr()
        }
        SYSCALL_RT_SIGRETURN => {
            sys_sigreturn();
            current_context_ptr()
        }
        SYSCALL_GETPID => {
            cx.x[10] = current_pid();
            cx.sepc = cx.sepc.wrapping_add(4);
            current_context_ptr()
        }
        SYSCALL_BRK => {
            let ret = sys_sbrk(cx.x[10] as isize);
            cx.x[10] = ret as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
            current_context_ptr()
        }
        SYSCALL_CLONE => {
            let ret = sys_fork(cx);
            cx.x[10] = ret as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
            current_context_ptr()
        }
        SYSCALL_EXECVE => {
            let ret = sys_exec(cx.x[10], cx.x[11]);
            if ret < 0 {
                cx.x[10] = ret as usize;
                cx.sepc = cx.sepc.wrapping_add(4);
            }
            current_context_ptr()
        }
        SYSCALL_WAIT4 => {
            let ret = sys_wait(cx.x[10], cx.x[11]);
            cx.x[10] = ret as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
            current_context_ptr()
        }
        _ => {
            cx.x[10] = (-1isize) as usize;
            cx.sepc = cx.sepc.wrapping_add(4);
            current_context_ptr()
        }
    }
}

fn sys_write(fd: usize, ptr: usize, len: usize) -> isize {
    if !valid_user_range(ptr, len) {
        return -1;
    }

    if fd == 1 || fd == 2 {
        for offset in 0..len {
            let byte = unsafe { (ptr as *const u8).add(offset).read_volatile() };
            console_putchar(byte);
        }
        return len as isize;
    }

    let current = unsafe { CURRENT_TASK };
    if fd >= MAX_FD {
        return -1;
    }
    let entry = unsafe { TASKS[current].fds[fd] };
    if entry.kind == FD_PIPE_WRITE {
        return sys_pipe_write(entry.file_index, ptr, len);
    }
    if entry.kind != FD_FILE || entry.writable == 0 || entry.file_index >= MAX_FILES {
        return -1;
    }

    let file_index = entry.file_index;
    let mut offset = entry.offset;
    let mut written = 0usize;
    unsafe {
        while written < len && offset < FILE_DATA_SIZE {
            FILES[file_index].data[offset] = (ptr as *const u8).add(written).read_volatile();
            written += 1;
            offset += 1;
        }
        if offset > FILES[file_index].len {
            FILES[file_index].len = offset;
        }
        TASKS[current].fds[fd].offset = offset;
    }

    written as isize
}

fn sys_read(fd: usize, ptr: usize, len: usize) -> isize {
    if !valid_user_range(ptr, len) {
        return -1;
    }
    if fd == 0 {
        return 0;
    }

    let current = unsafe { CURRENT_TASK };
    if fd >= MAX_FD {
        return -1;
    }
    let entry = unsafe { TASKS[current].fds[fd] };
    if entry.kind == FD_PIPE_READ {
        return sys_pipe_read(entry.file_index, ptr, len);
    }
    if entry.kind != FD_FILE || entry.readable == 0 || entry.file_index >= MAX_FILES {
        return -1;
    }

    let file_index = entry.file_index;
    let mut offset = entry.offset;
    let mut read_len = 0usize;
    unsafe {
        while read_len < len && offset < FILES[file_index].len {
            (ptr as *mut u8)
                .add(read_len)
                .write_volatile(FILES[file_index].data[offset]);
            read_len += 1;
            offset += 1;
        }
        TASKS[current].fds[fd].offset = offset;
    }

    read_len as isize
}

fn sys_open(path_ptr: usize, flags: usize) -> isize {
    let mut name = [0u8; FILE_NAME_SIZE];
    let Some(name_len) = read_user_cstr(path_ptr, &mut name) else {
        return -1;
    };
    if name_len == 0 {
        return -1;
    }

    let Some(file_index) = open_or_create_file(&name, name_len, flags) else {
        return -1;
    };
    let Some(fd) = alloc_fd() else {
        return -1;
    };

    let writable = if flags & (OPEN_WRONLY | OPEN_RDWR) != 0 {
        1
    } else {
        0
    };
    let readable = if flags & OPEN_WRONLY == 0 || flags & OPEN_RDWR != 0 {
        1
    } else {
        0
    };
    unsafe {
        TASKS[CURRENT_TASK].fds[fd] = FdEntry {
            kind: FD_FILE,
            file_index,
            offset: 0,
            readable,
            writable,
        };
    }
    fd as isize
}

fn sys_close(fd: usize) -> isize {
    if fd >= MAX_FD {
        return -1;
    }
    let current = unsafe { CURRENT_TASK };
    let entry = unsafe { TASKS[current].fds[fd] };
    let kind = entry.kind;
    if kind == FD_UNUSED {
        return -1;
    }
    if kind == FD_PIPE_READ || kind == FD_PIPE_WRITE {
        close_pipe_fd(entry);
    }
    unsafe {
        TASKS[current].fds[fd] = EMPTY_FD;
    }
    0
}

fn read_user_cstr(ptr: usize, out: &mut [u8; FILE_NAME_SIZE]) -> Option<usize> {
    if ptr == 0 {
        return None;
    }
    for index in 0..FILE_NAME_SIZE {
        if !valid_user_range(ptr + index, 1) {
            return None;
        }
        let byte = unsafe { (ptr as *const u8).add(index).read_volatile() };
        if byte == 0 {
            return Some(index);
        }
        out[index] = byte;
    }
    None
}

fn open_or_create_file(name: &[u8; FILE_NAME_SIZE], name_len: usize, flags: usize) -> Option<usize> {
    if let Some(index) = find_file(name, name_len) {
        if flags & OPEN_TRUNC != 0 && flags & (OPEN_WRONLY | OPEN_RDWR) != 0 {
            unsafe {
                FILES[index].len = 0;
            }
        }
        return Some(index);
    }

    if flags & OPEN_CREATE == 0 {
        return None;
    }

    for index in 0..MAX_FILES {
        if unsafe { FILES[index].used == 0 } {
            unsafe {
                FILES[index].used = 1;
                FILES[index].len = 0;
                FILES[index].name_len = name_len;
                for slot in FILES[index].name.iter_mut() {
                    *slot = 0;
                }
                for offset in 0..name_len {
                    FILES[index].name[offset] = name[offset];
                }
            }
            return Some(index);
        }
    }
    None
}

fn find_file(name: &[u8; FILE_NAME_SIZE], name_len: usize) -> Option<usize> {
    for index in 0..MAX_FILES {
        let used = unsafe { FILES[index].used != 0 };
        let stored_len = unsafe { FILES[index].name_len };
        if !used || stored_len != name_len {
            continue;
        }
        let mut equal = true;
        for offset in 0..name_len {
            if unsafe { FILES[index].name[offset] } != name[offset] {
                equal = false;
                break;
            }
        }
        if equal {
            return Some(index);
        }
    }
    None
}

fn alloc_fd() -> Option<usize> {
    let current = unsafe { CURRENT_TASK };
    for fd in 3..MAX_FD {
        if unsafe { TASKS[current].fds[fd].kind == FD_UNUSED } {
            return Some(fd);
        }
    }
    None
}

fn init_standard_fds(index: usize) {
    unsafe {
        for fd in TASKS[index].fds.iter_mut() {
            *fd = EMPTY_FD;
        }
        TASKS[index].fds[0] = FdEntry {
            kind: FD_STDIN,
            file_index: 0,
            offset: 0,
            readable: 1,
            writable: 0,
        };
        TASKS[index].fds[1] = FdEntry {
            kind: FD_STDOUT,
            file_index: 0,
            offset: 0,
            readable: 0,
            writable: 1,
        };
        TASKS[index].fds[2] = TASKS[index].fds[1];
    }
}

fn sys_pipe(pipe_fd_ptr: usize) -> isize {
    if !valid_user_range(pipe_fd_ptr, 2 * core::mem::size_of::<usize>()) {
        return -1;
    }
    let Some(pipe_index) = alloc_pipe() else {
        return -1;
    };
    let Some(read_fd) = alloc_fd() else {
        unsafe {
            PIPES[pipe_index] = EMPTY_PIPE;
        }
        return -1;
    };
    unsafe {
        TASKS[CURRENT_TASK].fds[read_fd] = FdEntry {
            kind: FD_PIPE_READ,
            file_index: pipe_index,
            offset: 0,
            readable: 1,
            writable: 0,
        };
    }
    let Some(write_fd) = alloc_fd() else {
        unsafe {
            TASKS[CURRENT_TASK].fds[read_fd] = EMPTY_FD;
            PIPES[pipe_index] = EMPTY_PIPE;
        }
        return -1;
    };
    unsafe {
        TASKS[CURRENT_TASK].fds[write_fd] = FdEntry {
            kind: FD_PIPE_WRITE,
            file_index: pipe_index,
            offset: 0,
            readable: 0,
            writable: 1,
        };
        (pipe_fd_ptr as *mut usize).write_volatile(read_fd);
        (pipe_fd_ptr as *mut usize).add(1).write_volatile(write_fd);
    }
    0
}

fn alloc_pipe() -> Option<usize> {
    for index in 0..MAX_PIPES {
        if unsafe { PIPES[index].used == 0 } {
            unsafe {
                PIPES[index] = EMPTY_PIPE;
                PIPES[index].used = 1;
                PIPES[index].read_open = 1;
                PIPES[index].write_open = 1;
            }
            return Some(index);
        }
    }
    None
}

fn sys_pipe_write(pipe_index: usize, ptr: usize, len: usize) -> isize {
    if pipe_index >= MAX_PIPES || unsafe { PIPES[pipe_index].used == 0 } {
        return -1;
    }
    if unsafe { PIPES[pipe_index].read_open == 0 } {
        return -1;
    }
    let available = unsafe { PIPE_DATA_SIZE.saturating_sub(PIPES[pipe_index].len) };
    if available == 0 {
        return -2;
    }
    let write_len = len.min(available);
    unsafe {
        let start = PIPES[pipe_index].len;
        for offset in 0..write_len {
            PIPES[pipe_index].data[start + offset] =
                (ptr as *const u8).add(offset).read_volatile();
        }
        PIPES[pipe_index].len += write_len;
    }
    write_len as isize
}

fn sys_pipe_read(pipe_index: usize, ptr: usize, len: usize) -> isize {
    if pipe_index >= MAX_PIPES || unsafe { PIPES[pipe_index].used == 0 } {
        return -1;
    }
    let available = unsafe { PIPES[pipe_index].len.saturating_sub(PIPES[pipe_index].read_offset) };
    if available == 0 {
        if unsafe { PIPES[pipe_index].write_open == 0 } {
            return 0;
        }
        return -2;
    }
    let read_len = len.min(available);
    unsafe {
        let start = PIPES[pipe_index].read_offset;
        for offset in 0..read_len {
            (ptr as *mut u8)
                .add(offset)
                .write_volatile(PIPES[pipe_index].data[start + offset]);
        }
        PIPES[pipe_index].read_offset += read_len;
        if PIPES[pipe_index].read_offset == PIPES[pipe_index].len {
            PIPES[pipe_index].read_offset = 0;
            PIPES[pipe_index].len = 0;
        }
    }
    read_len as isize
}

fn close_pipe_fd(entry: FdEntry) {
    let pipe_index = entry.file_index;
    if pipe_index >= MAX_PIPES {
        return;
    }
    unsafe {
        if entry.kind == FD_PIPE_READ && PIPES[pipe_index].read_open > 0 {
            PIPES[pipe_index].read_open -= 1;
        }
        if entry.kind == FD_PIPE_WRITE && PIPES[pipe_index].write_open > 0 {
            PIPES[pipe_index].write_open -= 1;
        }
        if PIPES[pipe_index].read_open == 0 && PIPES[pipe_index].write_open == 0 {
            PIPES[pipe_index] = EMPTY_PIPE;
        }
    }
}

fn clone_fd_refs(index: usize) {
    unsafe {
        for fd in 0..MAX_FD {
            let entry = TASKS[index].fds[fd];
            if entry.kind == FD_PIPE_READ && entry.file_index < MAX_PIPES {
                PIPES[entry.file_index].read_open += 1;
            } else if entry.kind == FD_PIPE_WRITE && entry.file_index < MAX_PIPES {
                PIPES[entry.file_index].write_open += 1;
            }
        }
    }
}

fn close_all_fds(index: usize) {
    unsafe {
        for fd in 0..MAX_FD {
            let entry = TASKS[index].fds[fd];
            if entry.kind == FD_PIPE_READ || entry.kind == FD_PIPE_WRITE {
                close_pipe_fd(entry);
            }
            TASKS[index].fds[fd] = EMPTY_FD;
        }
    }
}

fn init_signal_state(index: usize) {
    unsafe {
        TASKS[index].signal_actions = [EMPTY_SIGNAL_ACTION; MAX_SIG];
        TASKS[index].signal_mask = 0;
        TASKS[index].signal_pending = 0;
        TASKS[index].handling_signal = 0;
        TASKS[index].saved_signal_context = EMPTY_CONTEXT;
    }
}

fn sys_sigaction(signum: usize, action_ptr: usize, old_action_ptr: usize) -> isize {
    if signum == 0 || signum >= MAX_SIG || signum == SIGKILL {
        return -1;
    }
    let current = unsafe { CURRENT_TASK };
    let old = unsafe { TASKS[current].signal_actions[signum] };
    if old_action_ptr != 0 {
        if !valid_user_range(old_action_ptr, 2 * core::mem::size_of::<usize>()) {
            return -1;
        }
        unsafe {
            (old_action_ptr as *mut usize).write_volatile(old.handler);
            (old_action_ptr as *mut usize).add(1).write_volatile(old.mask);
        }
    }
    if action_ptr != 0 {
        if !valid_user_range(action_ptr, 2 * core::mem::size_of::<usize>()) {
            return -1;
        }
        let new_action = SignalAction {
            handler: unsafe { (action_ptr as *const usize).read_volatile() },
            mask: unsafe { (action_ptr as *const usize).add(1).read_volatile() },
        };
        unsafe {
            TASKS[current].signal_actions[signum] = new_action;
        }
    }
    0
}

fn sys_sigprocmask(mask: usize) -> isize {
    unsafe {
        TASKS[CURRENT_TASK].signal_mask = mask;
    }
    0
}

fn sys_kill(pid: usize, signum: usize) -> isize {
    if signum == 0 || signum >= MAX_SIG {
        return -1;
    }
    for index in 0..MAX_TASKS {
        let matches = unsafe { TASKS[index].status != TASK_UNUSED && TASKS[index].pid == pid };
        if !matches {
            continue;
        }
        if signum == SIGKILL {
            unsafe {
                TASKS[index].status = TASK_EXITED;
                TASKS[index].exit_code = -1;
            }
        } else {
            unsafe {
                TASKS[index].signal_pending |= 1usize << signum;
            }
        }
        return 0;
    }
    -1
}

fn deliver_pending_signal(cx: &mut TrapContext) {
    let current = unsafe { CURRENT_TASK };
    if unsafe { TASKS[current].handling_signal != 0 } {
        return;
    }
    let pending = unsafe { TASKS[current].signal_pending & !TASKS[current].signal_mask };
    for signum in 1..MAX_SIG {
        if pending & (1usize << signum) == 0 {
            continue;
        }
        let action = unsafe { TASKS[current].signal_actions[signum] };
        unsafe {
            TASKS[current].signal_pending &= !(1usize << signum);
        }
        if action.handler == 0 {
            if signum == SIGUSR1 {
                return;
            }
            continue;
        }
        unsafe {
            TASKS[current].saved_signal_context = *cx;
            TASKS[current].handling_signal = signum;
            TASKS[current].signal_mask |= action.mask;
        }
        cx.sepc = action.handler;
        return;
    }
}

fn sys_sigreturn() {
    let current = unsafe { CURRENT_TASK };
    unsafe {
        TASKS[current].context = TASKS[current].saved_signal_context;
        TASKS[current].handling_signal = 0;
    }
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

fn sys_sbrk(delta: isize) -> isize {
    let current = unsafe { CURRENT_TASK };
    let old = unsafe { TASKS[current].brk };

    if delta == 0 {
        return old as isize;
    }

    let Some(new_brk) = add_signed(old, delta) else {
        return -1;
    };
    let (heap_base, heap_limit) = unsafe { (TASKS[current].heap_base, TASKS[current].heap_limit) };
    if new_brk < heap_base || new_brk > heap_limit {
        return -1;
    }

    if new_brk > old {
        disable_paging();
        map_user_heap(current, old, new_brk);
    } else {
        disable_paging();
        unmap_user_heap(current, new_brk, old);
    }

    unsafe {
        TASKS[current].brk = new_brk;
    }
    activate_task(current);
    old as isize
}

fn sys_fork(cx: &TrapContext) -> isize {
    let parent = unsafe { CURRENT_TASK };
    let Some(child) = alloc_task_slot() else {
        return -1;
    };

    let parent_pid = unsafe { TASKS[parent].pid };
    let parent_phys = unsafe { TASKS[parent].phys_base };
    let parent_program_end = unsafe { TASKS[parent].program_end };
    let parent_heap_base = unsafe { TASKS[parent].heap_base };
    let parent_brk = unsafe { TASKS[parent].brk };
    let parent_heap_limit = unsafe { TASKS[parent].heap_limit };
    let parent_stack_bottom = unsafe { TASKS[parent].stack_bottom };
    let parent_stack_top = unsafe { TASKS[parent].stack_top };
    let parent_fds = unsafe { TASKS[parent].fds };
    let parent_signal_actions = unsafe { TASKS[parent].signal_actions };
    let parent_signal_mask = unsafe { TASKS[parent].signal_mask };
    let child_phys = process_phys_base(child);
    disable_paging();
    unsafe {
        let src = parent_phys as *const u8;
        let dst = child_phys as *mut u8;
        copy_nonoverlapping(src, dst, app_slot_size());
    }

    setup_address_space(
        child,
        USER_BASE,
        parent_program_end,
        parent_stack_bottom,
        parent_stack_top,
        child_phys,
    );
    if parent_brk > parent_heap_base {
        map_user_region(
            child,
            parent_heap_base,
            parent_brk,
            child_phys + (parent_heap_base - USER_BASE),
            PTE_R | PTE_W | PTE_U | PTE_A | PTE_D,
        );
    }

    let mut child_context = *cx;
    child_context.x[10] = 0;
    child_context.sepc = child_context.sepc.wrapping_add(4);

    unsafe {
        TASKS[child].context = child_context;
        TASKS[child].status = TASK_READY;
        TASKS[child].exit_code = 0;
        TASKS[child].pid = child + 1;
        TASKS[child].parent_pid = parent_pid;
        TASKS[child].base = USER_BASE;
        TASKS[child].phys_base = child_phys;
        TASKS[child].program_end = parent_program_end;
        TASKS[child].heap_base = parent_heap_base;
        TASKS[child].brk = parent_brk;
        TASKS[child].heap_limit = parent_heap_limit;
        TASKS[child].stack_bottom = parent_stack_bottom;
        TASKS[child].stack_top = parent_stack_top;
        TASKS[child].fds = parent_fds;
        TASKS[child].signal_actions = parent_signal_actions;
        TASKS[child].signal_mask = parent_signal_mask;
        TASKS[child].signal_pending = 0;
        TASKS[child].handling_signal = 0;
        TASKS[child].saved_signal_context = EMPTY_CONTEXT;
    }
    clone_fd_refs(child);
    activate_task(parent);

    (child + 1) as isize
}

fn sys_exec(path_ptr: usize, path_len: usize) -> isize {
    let Some(app_index) = find_app_by_user_path(path_ptr, path_len) else {
        return -1;
    };
    let current = unsafe { CURRENT_TASK };
    reset_task_image(current, app_index);
    0
}

fn sys_wait(pid_arg: usize, exit_code_ptr: usize) -> isize {
    let current_pid = current_pid();
    let wait_any = pid_arg == usize::MAX;
    let target_pid = pid_arg;
    let mut has_matching_child = false;

    for index in 0..MAX_TASKS {
        let status = unsafe { TASKS[index].status };
        let parent_pid = unsafe { TASKS[index].parent_pid };
        let task_pid = unsafe { TASKS[index].pid };
        if status == TASK_UNUSED || parent_pid != current_pid {
            continue;
        }
        if !wait_any && task_pid != target_pid {
            continue;
        }
        has_matching_child = true;
        if status == TASK_EXITED || status == TASK_FAULTED {
            let exit_code = unsafe { TASKS[index].exit_code };
            if exit_code_ptr != 0 && valid_user_range(exit_code_ptr, core::mem::size_of::<i32>()) {
                unsafe {
                    (exit_code_ptr as *mut i32).write_volatile(exit_code as i32);
                }
            }
            unsafe {
                TASKS[index].status = TASK_UNUSED;
                TASKS[index].parent_pid = 0;
                TASKS[index].exit_code = 0;
            }
            close_all_fds(index);
            init_signal_state(index);
            return task_pid as isize;
        }
    }

    if has_matching_child {
        -2
    } else {
        -1
    }
}

fn find_app_by_user_path(path_ptr: usize, path_len: usize) -> Option<usize> {
    if path_len == 0 || path_len > 64 || !valid_user_range(path_ptr, path_len) {
        return None;
    }
    for (index, name) in APP_NAMES.iter().enumerate() {
        if path_len != name.len() {
            continue;
        }
        let mut equal = true;
        for offset in 0..path_len {
            let byte = unsafe { (path_ptr as *const u8).add(offset).read_volatile() };
            if byte != name[offset] {
                equal = false;
                break;
            }
        }
        if equal {
            return Some(index);
        }
    }
    None
}

fn init_tasks() {
    if app_count() == 0 {
        shutdown();
    }

    reset_task_image(0, INITPROC_APP);
    unsafe {
        TASKS[0].status = TASK_READY;
        TASKS[0].pid = 1;
        TASKS[0].parent_pid = 0;
    }
    init_standard_fds(0);
    init_signal_state(0);
}

fn reset_task_image(index: usize, app_index: usize) {
    let phys_base = process_phys_base(index);
    let old_status = unsafe { TASKS[index].status };
    let old_pid = unsafe { TASKS[index].pid };
    let old_parent_pid = unsafe { TASKS[index].parent_pid };
    disable_paging();
    load_app_image(app_index, phys_base);

    let program_end = align_up(USER_BASE + PROGRAM_STATIC_SIZE, PAGE_SIZE);
    let stack_top = USER_BASE + app_slot_size();
    let stack_bottom = stack_top - USER_STACK_SIZE;
    let heap_base = program_end;
    let heap_limit = stack_bottom;
    setup_address_space(index, USER_BASE, program_end, stack_bottom, stack_top, phys_base);

    let mut context = EMPTY_CONTEXT;
    context.x[2] = stack_top;
    context.sstatus = user_sstatus();
    context.sepc = USER_BASE;

    unsafe {
        TASKS[index].context = context;
        TASKS[index].status = old_status;
        TASKS[index].exit_code = 0;
        TASKS[index].pid = if old_pid == 0 { index + 1 } else { old_pid };
        TASKS[index].parent_pid = old_parent_pid;
        TASKS[index].base = USER_BASE;
        TASKS[index].phys_base = phys_base;
        TASKS[index].program_end = program_end;
        TASKS[index].heap_base = heap_base;
        TASKS[index].brk = heap_base;
        TASKS[index].heap_limit = heap_limit;
        TASKS[index].stack_bottom = stack_bottom;
        TASKS[index].stack_top = stack_top;
    }
    if old_status != TASK_UNUSED && unsafe { CURRENT_TASK } == index {
        activate_task(index);
    }
}

fn alloc_task_slot() -> Option<usize> {
    for index in 0..MAX_TASKS {
        if unsafe { TASKS[index].status == TASK_UNUSED } {
            return Some(index);
        }
    }
    None
}

fn current_pid() -> usize {
    let current = unsafe { CURRENT_TASK };
    unsafe { TASKS[current].pid }
}

fn process_phys_base(index: usize) -> usize {
    PROCESS_BASE + index * app_slot_size()
}

fn run_first_task() -> ! {
    unsafe {
        CURRENT_TASK = 0;
        TASKS[0].status = TASK_RUNNING;
        activate_task(0);
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
            activate_task(next);
            addr_of_mut!(TASKS[next].context)
        }
    } else {
        shutdown();
    }
}

fn find_next_ready(start: usize) -> Option<usize> {
    let count = MAX_TASKS;
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
    let base = unsafe { TASKS[current].base };
    let program_end = unsafe { TASKS[current].program_end };
    let heap_base = unsafe { TASKS[current].heap_base };
    let brk = unsafe { TASKS[current].brk };
    let stack_bottom = unsafe { TASKS[current].stack_bottom };
    let stack_top = unsafe { TASKS[current].stack_top };
    (ptr >= base && end <= program_end)
        || (ptr >= heap_base && end <= brk)
        || (ptr >= stack_bottom && end <= stack_top)
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
    unsafe { *app_table().add(2) }
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

fn load_app_image(index: usize, phys_base: usize) {
    let limit = app_slot_size();
    let (src, len) = app_range(index);
    if len > limit {
        shutdown();
    }

    unsafe {
        let dst = phys_base as *mut u8;
        for offset in 0..limit {
            dst.add(offset).write_volatile(0);
        }
        copy_nonoverlapping(src, dst, len);
        asm!("fence.i", options(nomem, nostack));
    }
}

fn user_sstatus() -> usize {
    let mut sstatus = read_sstatus();
    sstatus &= !SSTATUS_SPP;
    sstatus |= SSTATUS_SPIE | SSTATUS_SUM;
    sstatus
}

fn setup_address_space(
    index: usize,
    base: usize,
    program_end: usize,
    stack_bottom: usize,
    stack_top: usize,
    phys_base: usize,
) {
    unsafe {
        for slot in ROOT_TABLES.0[index].iter_mut() {
            *slot = 0;
        }
        for slot in L1_TABLES.0[index].iter_mut() {
            *slot = 0;
        }
        for slot in USER_L0_TABLES.0[index].iter_mut() {
            *slot = 0;
        }

        let l1_pa = addr_of!(L1_TABLES.0[index]) as usize;
        ROOT_TABLES.0[index][vpn2(KERNEL_MAP_START)] = pte(l1_pa, 0);

        let mut pa = KERNEL_MAP_START;
        while pa < KERNEL_MAP_END {
            if vpn1(pa) != vpn1(USER_BASE) {
                L1_TABLES.0[index][vpn1(pa)] = pte(pa, PTE_R | PTE_W | PTE_X | PTE_A | PTE_D);
            }
            pa += 0x20_0000;
        }

        let user_l0_pa = addr_of!(USER_L0_TABLES.0[index]) as usize;
        L1_TABLES.0[index][vpn1(base)] = pte(user_l0_pa, 0);
    }

    map_user_region(
        index,
        base,
        program_end,
        phys_base,
        PTE_R | PTE_W | PTE_X | PTE_U | PTE_A | PTE_D,
    );
    map_user_region(
        index,
        stack_bottom,
        stack_top,
        phys_base + (stack_bottom - USER_BASE),
        PTE_R | PTE_W | PTE_U | PTE_A | PTE_D,
    );
}

fn map_user_heap(index: usize, start: usize, end: usize) {
    let phys_base = unsafe { TASKS[index].phys_base };
    map_user_region(
        index,
        align_down(start, PAGE_SIZE),
        align_up(end, PAGE_SIZE),
        phys_base + (align_down(start, PAGE_SIZE) - USER_BASE),
        PTE_R | PTE_W | PTE_U | PTE_A | PTE_D,
    );
}

fn unmap_user_heap(index: usize, start: usize, end: usize) {
    let mut va = align_up(start, PAGE_SIZE);
    let end = align_up(end, PAGE_SIZE);
    unsafe {
        while va < end {
            USER_L0_TABLES.0[index][vpn0(va)] = 0;
            va += PAGE_SIZE;
        }
    }
}

fn map_user_region(index: usize, start: usize, end: usize, phys_start: usize, flags: usize) {
    let mut va = align_down(start, PAGE_SIZE);
    let end = align_up(end, PAGE_SIZE);
    let mut pa = align_down(phys_start, PAGE_SIZE);
    unsafe {
        while va < end {
            USER_L0_TABLES.0[index][vpn0(va)] = pte(pa, flags);
            va += PAGE_SIZE;
            pa += PAGE_SIZE;
        }
    }
}

fn activate_task(index: usize) {
    let root = unsafe { addr_of!(ROOT_TABLES.0[index]) as usize };
    let satp = SV39_MODE | (root >> 12);
    unsafe {
        asm!("csrw satp, {}", in(reg) satp, options(nostack));
    }
    flush_tlb();
}

fn disable_paging() {
    unsafe {
        asm!("csrw satp, zero", options(nostack));
    }
    flush_tlb();
}

fn flush_tlb() {
    unsafe {
        asm!("sfence.vma", options(nostack));
    }
}

fn pte(pa: usize, flags: usize) -> usize {
    ((pa >> 12) << 10) | flags | PTE_V
}

fn vpn2(va: usize) -> usize {
    (va >> 30) & 0x1ff
}

fn vpn1(va: usize) -> usize {
    (va >> 21) & 0x1ff
}

fn vpn0(va: usize) -> usize {
    (va >> 12) & 0x1ff
}

fn align_down(value: usize, align: usize) -> usize {
    value & !(align - 1)
}

fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

fn add_signed(value: usize, delta: isize) -> Option<usize> {
    if delta >= 0 {
        value.checked_add(delta as usize)
    } else {
        value.checked_sub(delta.unsigned_abs())
    }
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
