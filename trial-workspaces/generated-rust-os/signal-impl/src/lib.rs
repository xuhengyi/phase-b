#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core::mem;
use kernel_context::LocalContext;
use signal::{Signal, SignalAction, SignalNo, SignalResult};

mod default_action;
mod signal_set;

use default_action::{default_action, is_unmaskable, DefaultAction};
use signal_set::SignalSet;

const SIGNAL_CAPACITY: usize = SignalNo::SIGRT31 as usize + 1;

#[derive(Clone)]
pub struct HandlingSignal {
    signal: SignalNo,
    saved_context: LocalContext,
    saved_mask: usize,
}

impl HandlingSignal {
    fn new(signal: SignalNo, ctx: LocalContext, mask: usize) -> Self {
        Self {
            signal,
            saved_context: ctx,
            saved_mask: mask,
        }
    }

    pub fn signal(&self) -> SignalNo {
        self.signal
    }

    pub fn saved_context(&self) -> &LocalContext {
        &self.saved_context
    }

    pub fn saved_mask(&self) -> usize {
        self.saved_mask
    }
}

#[derive(Clone)]
enum HandlingState {
    None,
    User(HandlingSignal),
    Suspended,
}

impl HandlingState {
    fn take_user(&mut self) -> Option<HandlingSignal> {
        match self {
            HandlingState::User(_) => match mem::replace(self, HandlingState::None) {
                HandlingState::User(signal) => Some(signal),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Default for HandlingState {
    fn default() -> Self {
        HandlingState::None
    }
}

pub struct SignalImpl {
    pending: SignalSet,
    mask: usize,
    actions: [SignalAction; SIGNAL_CAPACITY],
    handling: HandlingState,
}

impl SignalImpl {
    pub fn new() -> Self {
        Self {
            pending: SignalSet::default(),
            mask: 0,
            actions: [SignalAction::default(); SIGNAL_CAPACITY],
            handling: HandlingState::None,
        }
    }

    fn signal_index(signal: SignalNo) -> Option<usize> {
        match signal {
            SignalNo::ERR => None,
            _ => Some(signal as usize),
        }
    }

    fn mask_bit(signal: SignalNo) -> usize {
        1usize << (signal as usize)
    }

    fn is_masked(&self, signal: SignalNo) -> bool {
        !is_unmaskable(signal) && (self.mask & Self::mask_bit(signal)) != 0
    }

    fn next_pending(&mut self) -> Option<SignalNo> {
        for raw in 1..SIGNAL_CAPACITY {
            let signal = SignalNo::from(raw);
            if signal == SignalNo::ERR {
                continue;
            }
            if self.pending.contains(signal) && !self.is_masked(signal) {
                self.pending.remove(signal);
                return Some(signal);
            }
        }
        None
    }

    fn deliver_default_action(&mut self, signal: SignalNo) -> SignalResult {
        match default_action(signal) {
            DefaultAction::Ignore => SignalResult::Ignored,
            DefaultAction::Terminate => SignalResult::ProcessKilled(-(signal as i32)),
            DefaultAction::Stop => {
                self.handling = HandlingState::Suspended;
                SignalResult::ProcessSuspended
            }
            DefaultAction::Continue => SignalResult::Handled,
        }
    }

    fn deliver_user_handler(
        &mut self,
        signal: SignalNo,
        action: &SignalAction,
        ctx: &mut LocalContext,
    ) -> SignalResult {
        let saved_context = ctx.clone();
        let saved_mask = self.mask;
        let additional_mask = action.mask as usize | Self::mask_bit(signal);
        self.mask |= additional_mask;
        self.handling = HandlingState::User(HandlingSignal::new(signal, saved_context, saved_mask));

        *ctx.pc_mut() = action.handler;
        *ctx.a_mut(0) = signal as usize;

        SignalResult::Handled
    }

    fn process_pending_signal(&mut self, ctx: &mut LocalContext) -> SignalResult {
        let Some(signal) = self.next_pending() else {
            return SignalResult::NoSignal;
        };

        if let Some(index) = Self::signal_index(signal) {
            let action = self.actions[index];
            if action.handler != 0 {
                return self.deliver_user_handler(signal, &action, ctx);
            }
        }

        self.deliver_default_action(signal)
    }

    fn handle_stopped_state(&mut self) -> SignalResult {
        if self.pending.remove(SignalNo::SIGCONT) {
            self.handling = HandlingState::None;
            SignalResult::Handled
        } else {
            SignalResult::ProcessSuspended
        }
    }
}

impl Default for SignalImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Signal for SignalImpl {
    fn from_fork(&mut self) -> Box<dyn Signal> {
        Box::new(SignalImpl {
            pending: SignalSet::default(),
            mask: self.mask,
            actions: self.actions,
            handling: HandlingState::None,
        })
    }

    fn clear(&mut self) {
        self.pending.clear();
        self.actions = [SignalAction::default(); SIGNAL_CAPACITY];
        self.handling = HandlingState::None;
    }

    fn add_signal(&mut self, signal: SignalNo) {
        self.pending.add(signal);
    }

    fn is_handling_signal(&self) -> bool {
        !matches!(self.handling, HandlingState::None)
    }

    fn set_action(&mut self, signum: SignalNo, action: &SignalAction) -> bool {
        if matches!(
            signum,
            SignalNo::ERR | SignalNo::SIGKILL | SignalNo::SIGSTOP
        ) {
            return false;
        }
        if let Some(index) = Self::signal_index(signum) {
            self.actions[index] = *action;
            true
        } else {
            false
        }
    }

    fn get_action_ref(&self, signum: SignalNo) -> Option<SignalAction> {
        Self::signal_index(signum).map(|index| self.actions[index])
    }

    fn update_mask(&mut self, mask: usize) -> usize {
        let old = self.mask;
        self.mask = mask;
        old
    }

    fn handle_signals(&mut self, current_context: &mut LocalContext) -> SignalResult {
        match self.handling {
            HandlingState::User(_) => return SignalResult::IsHandlingSignal,
            HandlingState::Suspended => return self.handle_stopped_state(),
            HandlingState::None => {}
        }
        self.process_pending_signal(current_context)
    }

    fn sig_return(&mut self, current_context: &mut LocalContext) -> bool {
        if let Some(saved) = self.handling.take_user() {
            *current_context = saved.saved_context;
            self.mask = saved.saved_mask;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests;
