#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use kernel_context::LocalContext;

pub use signal_defs::{SignalAction, SignalNo, MAX_SIG};

mod signal_result;

pub use signal_result::SignalResult;

/// Abstract interface that the tutorial kernel uses to manage per-process
/// signal state. Concrete implementations may store arbitrary internal
/// structures, but they MUST honour the behavioural contract documented in
/// the specification.
pub trait Signal {
    /// Create a forked copy of the signal state.
    ///
    /// The returned object MUST inherit the mask and registered actions from
    /// the parent, so that signal disposition remains consistent after fork.
    fn from_fork(&mut self) -> Box<dyn Signal>;

    /// Clear all pending signals.
    fn clear(&mut self);

    /// Record that a signal should be delivered in the future.
    fn add_signal(&mut self, signal: SignalNo);

    /// Whether the implementation is currently inside a handler.
    fn is_handling_signal(&self) -> bool;

    /// Update the registered action for the given signal number.
    ///
    /// Returns `true` when the action change succeeds and `false` otherwise.
    fn set_action(&mut self, signum: SignalNo, action: &SignalAction) -> bool;

    /// Retrieve the stored action for a signal, if any.
    fn get_action_ref(&self, signum: SignalNo) -> Option<SignalAction>;

    /// Update the signal mask, returning the previous mask.
    fn update_mask(&mut self, mask: usize) -> usize;

    /// Attempt to deliver pending signals to the provided execution context.
    fn handle_signals(&mut self, current_context: &mut LocalContext) -> SignalResult;

    /// Finalise a signal handler return.
    fn sig_return(&mut self, current_context: &mut LocalContext) -> bool;
}

#[cfg(test)]
mod tests;
