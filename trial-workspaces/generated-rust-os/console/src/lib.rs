#![no_std]

use core::fmt::{self, Write};
use core::sync::atomic::{AtomicUsize, Ordering};

use log::{LevelFilter, Metadata, Record};
use spin::{Mutex, Once};

/// Minimal interface that concrete console drivers must implement.
pub trait Console: Sync {
    /// Output a single byte to the console.
    fn put_char(&self, c: u8);

    /// Output a UTF-8 string to the console.
    fn put_str(&self, s: &str) {
        for &b in s.as_bytes() {
            self.put_char(b);
        }
    }
}

static CONSOLE: Mutex<Option<&'static dyn Console>> = Mutex::new(None);

/// Install a global console instance used by `print!`, `println!` and log output.
pub fn init_console(console: &'static dyn Console) {
    *CONSOLE.lock() = Some(console);
}

fn current_console() -> Option<&'static dyn Console> {
    CONSOLE.lock().clone()
}

struct ConsoleWriter {
    inner: &'static dyn Console,
}

impl Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.inner.put_str(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments<'_>) {
    if let Some(console) = current_console() {
        let mut writer = ConsoleWriter { inner: console };
        let _ = writer.write_fmt(args);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::_print(core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::_print(core::format_args!("\n"));
    }};
    ($($arg:tt)*) => {{
        $crate::_print(core::format_args!($($arg)*));
        $crate::_print(core::format_args!("\n"));
    }};
}

struct ConsoleLogger;

static LOGGER: ConsoleLogger = ConsoleLogger;
static LOGGER_INIT: Once<()> = Once::new();
static LOG_LEVEL: AtomicUsize = AtomicUsize::new(LevelFilter::Off as usize);

fn ensure_logger() {
    LOGGER_INIT.call_once(|| {
        log::set_logger(&LOGGER).expect("logger already initialized");
        log::set_max_level(LevelFilter::Off);
    });
}

fn set_level_internal(filter: LevelFilter) {
    LOG_LEVEL.store(filter as usize, Ordering::Relaxed);
    log::set_max_level(filter);
}

fn current_level() -> LevelFilter {
    match LOG_LEVEL.load(Ordering::Relaxed) {
        x if x == LevelFilter::Error as usize => LevelFilter::Error,
        x if x == LevelFilter::Warn as usize => LevelFilter::Warn,
        x if x == LevelFilter::Info as usize => LevelFilter::Info,
        x if x == LevelFilter::Debug as usize => LevelFilter::Debug,
        x if x == LevelFilter::Trace as usize => LevelFilter::Trace,
        _ => LevelFilter::Off,
    }
}

fn parse_level(level: &str) -> Option<LevelFilter> {
    if level.eq_ignore_ascii_case("off") {
        Some(LevelFilter::Off)
    } else if level.eq_ignore_ascii_case("error") {
        Some(LevelFilter::Error)
    } else if level.eq_ignore_ascii_case("warn") || level.eq_ignore_ascii_case("warning") {
        Some(LevelFilter::Warn)
    } else if level.eq_ignore_ascii_case("info") {
        Some(LevelFilter::Info)
    } else if level.eq_ignore_ascii_case("debug") {
        Some(LevelFilter::Debug)
    } else if level.eq_ignore_ascii_case("trace") {
        Some(LevelFilter::Trace)
    } else {
        None
    }
}

/// Adjust the maximum log level honored by the console logger.
pub fn set_log_level(level: Option<&str>) {
    ensure_logger();
    match level {
        None => set_level_internal(LevelFilter::Off),
        Some(name) => {
            if let Some(filter) = parse_level(name) {
                set_level_internal(filter);
            }
        }
    }
}

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= current_level()
    }

    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }
        if let Some(console) = current_console() {
            let mut writer = ConsoleWriter { inner: console };
            let _ = writer.write_fmt(format_args!("[{}] {}\n", record.level(), record.args()));
        }
    }

    fn flush(&self) {}
}

/// Emit a simple log line for smoke testing purposes.
pub fn test_log() {
    ensure_logger();
    log::info!("____ LOG TEST ____");
}

#[cfg(test)]
mod tests;
