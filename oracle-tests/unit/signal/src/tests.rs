extern crate std;

use super::{Signal, SignalAction, SignalNo, SignalResult, MAX_SIG};
use kernel_context::LocalContext;
use std::boxed::Box;

struct DummySignal {
    mask: usize,
}

impl Signal for DummySignal {
    fn from_fork(&mut self) -> Box<dyn Signal> {
        Box::new(Self { mask: self.mask })
    }

    fn clear(&mut self) {
        self.mask = 0;
    }

    fn add_signal(&mut self, _signal: SignalNo) {}

    fn is_handling_signal(&self) -> bool {
        false
    }

    fn set_action(&mut self, _signum: SignalNo, _action: &SignalAction) -> bool {
        true
    }

    fn get_action_ref(&self, _signum: SignalNo) -> Option<SignalAction> {
        Some(SignalAction::default())
    }

    fn update_mask(&mut self, mask: usize) -> usize {
        let old = self.mask;
        self.mask = mask;
        old
    }

    fn handle_signals(&mut self, _current_context: &mut LocalContext) -> SignalResult {
        SignalResult::NoSignal
    }

    fn sig_return(&mut self, _current_context: &mut LocalContext) -> bool {
        false
    }
}

#[test]
fn test_signal_result_variants_are_constructible() {
    assert!(matches!(SignalResult::NoSignal, SignalResult::NoSignal));
    assert!(matches!(SignalResult::IsHandlingSignal, SignalResult::IsHandlingSignal));
    assert!(matches!(SignalResult::Ignored, SignalResult::Ignored));
    assert!(matches!(SignalResult::Handled, SignalResult::Handled));
    assert!(matches!(SignalResult::ProcessKilled(-9), SignalResult::ProcessKilled(-9)));
    assert!(matches!(SignalResult::ProcessSuspended, SignalResult::ProcessSuspended));
}

#[test]
fn test_signal_action_and_constants_match_contract() {
    let action = SignalAction {
        handler: 0x1000,
        mask: 0x2000,
    };
    assert_eq!(action.handler, 0x1000);
    assert_eq!(action.mask, 0x2000);
    assert_eq!(SignalAction::default().handler, 0);
    assert_eq!(SignalAction::default().mask, 0);
    assert_eq!(SignalNo::SIGINT as u8, 2);
    assert_eq!(SignalNo::SIGKILL as u8, 9);
    assert_eq!(MAX_SIG, 31);
}

#[test]
fn test_signal_trait_object_updates_mask_and_forks() {
    let mut sig: Box<dyn Signal> = Box::new(DummySignal { mask: 0x12 });
    assert_eq!(sig.update_mask(0x34), 0x12);
    let mut child = sig.from_fork();
    assert_eq!(child.update_mask(0x56), 0x34);
}
