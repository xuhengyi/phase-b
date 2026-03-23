extern crate std;

use super::SignalImpl;
use kernel_context::LocalContext;
use signal::{Signal, SignalAction, SignalNo, SignalResult};

#[test]
fn test_from_fork_inherits_mask_and_actions_without_pending_state() {
    let mut signal_impl = SignalImpl::new();
    let action = SignalAction {
        handler: 0x1000,
        mask: 0x2000,
    };
    signal_impl.set_action(SignalNo::SIGUSR1, &action);
    signal_impl.update_mask(0x55aa);
    signal_impl.add_signal(SignalNo::SIGTERM);

    let mut child = signal_impl.from_fork();
    let child_action = child.get_action_ref(SignalNo::SIGUSR1).unwrap();
    assert_eq!(child_action.handler, action.handler);
    assert_eq!(child_action.mask, action.mask);
    assert_eq!(child.update_mask(0x33), 0x55aa);

    let mut ctx = LocalContext::user(0x8000);
    assert_eq!(child.handle_signals(&mut ctx), SignalResult::NoSignal);
}

#[test]
fn test_clear_removes_installed_actions() {
    let mut signal_impl = SignalImpl::new();
    signal_impl.set_action(
        SignalNo::SIGINT,
        &SignalAction {
            handler: 0x1234,
            mask: 0x5678,
        },
    );
    signal_impl.clear();
    let cleared = signal_impl.get_action_ref(SignalNo::SIGINT).unwrap();
    assert_eq!(cleared.handler, 0);
    assert_eq!(cleared.mask, 0);
}

#[test]
fn test_user_handler_overrides_context_and_sigreturn_restores_it() {
    let mut signal_impl = SignalImpl::new();
    let action = SignalAction {
        handler: 0x4000,
        mask: 0,
    };
    let mut ctx = LocalContext::user(0x2000);
    *ctx.a_mut(0) = 0xdead_beef;
    let original = ctx.clone();

    signal_impl.set_action(SignalNo::SIGUSR1, &action);
    signal_impl.add_signal(SignalNo::SIGUSR1);

    assert_eq!(signal_impl.handle_signals(&mut ctx), SignalResult::Handled);
    assert!(signal_impl.is_handling_signal());
    assert_eq!(ctx.pc(), 0x4000);
    assert_eq!(ctx.a(0), SignalNo::SIGUSR1 as usize);

    assert!(signal_impl.sig_return(&mut ctx));
    assert!(!signal_impl.is_handling_signal());
    assert_eq!(ctx.pc(), original.pc());
    assert_eq!(ctx.a(0), original.a(0));
}

#[test]
fn test_default_actions_match_current_contract() {
    let mut signal_impl = SignalImpl::new();
    let mut ctx = LocalContext::user(0x3000);

    signal_impl.add_signal(SignalNo::SIGCHLD);
    assert_eq!(signal_impl.handle_signals(&mut ctx), SignalResult::Ignored);

    signal_impl.add_signal(SignalNo::SIGTERM);
    assert_eq!(
        signal_impl.handle_signals(&mut ctx),
        SignalResult::ProcessKilled(-(SignalNo::SIGTERM as i32))
    );
}

#[test]
fn test_stop_then_continue_flow_matches_contract() {
    let mut signal_impl = SignalImpl::new();
    let mut ctx = LocalContext::user(0x5000);

    signal_impl.add_signal(SignalNo::SIGSTOP);
    assert_eq!(signal_impl.handle_signals(&mut ctx), SignalResult::ProcessSuspended);
    assert!(signal_impl.is_handling_signal());
    assert_eq!(signal_impl.handle_signals(&mut ctx), SignalResult::ProcessSuspended);

    signal_impl.add_signal(SignalNo::SIGCONT);
    assert_eq!(signal_impl.handle_signals(&mut ctx), SignalResult::Handled);
    assert!(!signal_impl.is_handling_signal());
}
