/// Result of examining pending signals for a task.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignalResult {
    /// No signal work is required and execution may proceed as normal.
    NoSignal,
    /// The task is already handling a signal and should not re-enter.
    IsHandlingSignal,
    /// Pending signals were ignored due to disposition or mask.
    Ignored,
    /// At least one signal has been delivered and handled successfully.
    Handled,
    /// Signal handling requests that the process terminates with the given code.
    ProcessKilled(i32),
    /// Signal handling requests that the process transitions into a suspended state.
    ProcessSuspended,
}
