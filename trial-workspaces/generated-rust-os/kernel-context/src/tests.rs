extern crate std;

use super::LocalContext;

#[test]
fn test_local_context_empty() {
    let ctx = LocalContext::empty();
    assert!(!ctx.supervisor);
    assert!(!ctx.interrupt);
    assert_eq!(ctx.pc(), 0);
    for i in 1..=31 {
        assert_eq!(ctx.x(i), 0);
    }
}

#[test]
fn test_local_context_user() {
    let pc = 0x1000;
    let ctx = LocalContext::user(pc);
    assert!(!ctx.supervisor);
    assert!(ctx.interrupt);
    assert_eq!(ctx.pc(), pc);
}

#[test]
fn test_local_context_thread() {
    let ctx = LocalContext::thread(0x2000, false);
    assert!(ctx.supervisor);
    assert!(!ctx.interrupt);
    assert_eq!(ctx.pc(), 0x2000);
}

#[test]
fn test_local_context_accessors() {
    let mut ctx = LocalContext::empty();
    *ctx.x_mut(1) = 0x11;
    *ctx.sp_mut() = 0x22;
    *ctx.a_mut(0) = 0x33;
    *ctx.a_mut(7) = 0x44;
    *ctx.pc_mut() = 0x55;

    assert_eq!(ctx.ra(), 0x11);
    assert_eq!(ctx.sp(), 0x22);
    assert_eq!(ctx.a(0), 0x33);
    assert_eq!(ctx.x(10), 0x33);
    assert_eq!(ctx.a(7), 0x44);
    assert_eq!(ctx.x(17), 0x44);
    assert_eq!(ctx.pc(), 0x55);
}

#[test]
fn test_move_next_uses_wrapping_add() {
    let mut ctx = LocalContext::empty();
    *ctx.pc_mut() = usize::MAX - 3;
    ctx.move_next();
    assert_eq!(ctx.pc(), 0);
}

#[test]
fn test_clone_is_deep_copy() {
    let mut original = LocalContext::user(0x3000);
    *original.sp_mut() = 0x1234;
    let cloned = original.clone();
    *original.sp_mut() = 0x5678;

    assert_eq!(cloned.pc(), 0x3000);
    assert_eq!(cloned.sp(), 0x1234);
}

#[test]
fn test_local_context_repr_c_size_is_stable() {
    let size = core::mem::size_of::<LocalContext>();
    assert!(size >= 264);
    assert!(size <= 280);
}

#[cfg(not(target_arch = "riscv64"))]
#[test]
#[should_panic(expected = "execute() is only available on RISC-V 64-bit targets")]
fn test_execute_panics_on_non_riscv_host() {
    let mut ctx = LocalContext::user(0x1000);
    unsafe {
        ctx.execute();
    }
}
