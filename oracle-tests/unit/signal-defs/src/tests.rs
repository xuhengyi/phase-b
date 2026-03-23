extern crate std;

use super::*;
use std::fmt;

#[test]
fn test_signal_action_basic_traits() {
    let action = SignalAction {
        handler: 0x1000,
        mask: 0x2000,
    };
    assert_eq!(action.handler, 0x1000);
    assert_eq!(action.mask, 0x2000);
    assert_eq!(SignalAction::default().handler, 0);
    assert_eq!(SignalAction::default().mask, 0);
    assert!(fmt::format(format_args!("{action:?}")).contains("SignalAction"));
}

#[test]
fn test_max_sig_and_signal_ranges() {
    assert_eq!(MAX_SIG, 31);
    assert_eq!(SignalNo::ERR as u8, 0);
    assert_eq!(SignalNo::SIGSYS as u8, 31);
    assert_eq!(SignalNo::SIGRTMIN as u8, 32);
    assert_eq!(SignalNo::SIGRT31 as u8, 63);
}

#[test]
fn test_signal_no_from_usize() {
    assert_eq!(SignalNo::from(0), SignalNo::ERR);
    assert_eq!(SignalNo::from(1), SignalNo::SIGHUP);
    assert_eq!(SignalNo::from(9), SignalNo::SIGKILL);
    assert_eq!(SignalNo::from(15), SignalNo::SIGTERM);
    assert_eq!(SignalNo::from(31), SignalNo::SIGSYS);
    assert_eq!(SignalNo::from(32), SignalNo::SIGRTMIN);
    assert_eq!(SignalNo::from(63), SignalNo::SIGRT31);
    assert_eq!(SignalNo::from(64), SignalNo::ERR);
    assert_eq!(SignalNo::from(255), SignalNo::ERR);
}

#[test]
fn test_signal_no_ordering_and_debug() {
    assert!((SignalNo::ERR as u8) < (SignalNo::SIGHUP as u8));
    assert!((SignalNo::SIGSYS as u8) < (SignalNo::SIGRTMIN as u8));
    assert!(fmt::format(format_args!("{:?}", SignalNo::SIGHUP)).contains("SIGHUP"));
}
