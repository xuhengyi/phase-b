use signal::SignalNo;

/// Default behaviour for signals that have no user-installed action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefaultAction {
    Ignore,
    Terminate,
    Stop,
    Continue,
}

pub fn default_action(signal: SignalNo) -> DefaultAction {
    match signal {
        SignalNo::SIGCHLD | SignalNo::SIGURG | SignalNo::SIGWINCH => DefaultAction::Ignore,
        SignalNo::SIGSTOP | SignalNo::SIGTSTP | SignalNo::SIGTTIN | SignalNo::SIGTTOU => {
            DefaultAction::Stop
        }
        SignalNo::SIGCONT => DefaultAction::Continue,
        SignalNo::ERR => DefaultAction::Ignore,
        _ => DefaultAction::Terminate,
    }
}

pub fn is_unmaskable(signal: SignalNo) -> bool {
    matches!(signal, SignalNo::SIGKILL | SignalNo::SIGSTOP)
}
