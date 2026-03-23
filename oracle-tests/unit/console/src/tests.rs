extern crate std;

use super::{init_console, set_log_level, test_log, Console};
use std::sync::{Arc, Mutex, Once};
use std::{boxed::Box, vec::Vec};

struct TestConsole {
    output: Arc<Mutex<Vec<u8>>>,
}

impl Console for TestConsole {
    fn put_char(&self, c: u8) {
        self.output.lock().unwrap().push(c);
    }

    fn put_str(&self, s: &str) {
        self.output.lock().unwrap().extend_from_slice(s.as_bytes());
    }
}

static SHARED_OUTPUT: Mutex<Option<Arc<Mutex<Vec<u8>>>>> = Mutex::new(None);
static SHARED_CONSOLE_INIT: Once = Once::new();
static TEST_SERIAL: Mutex<()> = Mutex::new(());

fn lock_test() -> std::sync::MutexGuard<'static, ()> {
    TEST_SERIAL.lock().unwrap()
}

fn get_shared_output() -> Arc<Mutex<Vec<u8>>> {
    {
        let guard = SHARED_OUTPUT.lock().unwrap();
        if let Some(ref output) = *guard {
            return output.clone();
        }
    }

    SHARED_CONSOLE_INIT.call_once(|| {
        let output = Arc::new(Mutex::new(Vec::new()));
        let console = Box::leak(Box::new(TestConsole {
            output: output.clone(),
        }));
        init_console(console);
        *SHARED_OUTPUT.lock().unwrap() = Some(output);
    });

    SHARED_OUTPUT.lock().unwrap().as_ref().unwrap().clone()
}

fn clear_output() {
    get_shared_output().lock().unwrap().clear();
}

fn get_output() -> Vec<u8> {
    get_shared_output().lock().unwrap().clone()
}

#[test]
fn test_console_trait_basic() {
    let _guard = lock_test();
    let output = Arc::new(Mutex::new(Vec::new()));
    let console = TestConsole {
        output: output.clone(),
    };

    console.put_char(b'A');
    console.put_char(b'B');
    assert_eq!(output.lock().unwrap().as_slice(), b"AB");
}

#[test]
fn test_console_put_str() {
    let _guard = lock_test();
    let output = Arc::new(Mutex::new(Vec::new()));
    let console = TestConsole {
        output: output.clone(),
    };

    console.put_str("hello");
    assert_eq!(output.lock().unwrap().as_slice(), b"hello");
}

#[test]
fn test_console_init_and_print_macros() {
    let _guard = lock_test();
    clear_output();
    let _ = get_shared_output();

    crate::print!("test");
    assert_eq!(get_output().as_slice(), b"test");

    clear_output();
    crate::println!("hello {}", "world");
    let bytes = get_output();
    let output = std::str::from_utf8(&bytes).unwrap();
    assert!(output.contains("hello"));
    assert!(output.contains("world"));
    assert!(output.ends_with('\n'));
}

#[test]
fn test_console_set_log_level() {
    let _guard = lock_test();
    set_log_level(None);
    set_log_level(Some("info"));
    set_log_level(Some("debug"));
    set_log_level(Some("trace"));
    set_log_level(Some("warn"));
    set_log_level(Some("error"));
    set_log_level(Some("invalid"));
}

#[test]
fn test_log_integration_and_test_log() {
    let _guard = lock_test();
    clear_output();
    set_log_level(Some("trace"));

    log::trace!("trace message");
    log::debug!("debug message");
    log::info!("info message");
    log::warn!("warn message");
    log::error!("error message");

    let bytes = get_output();
    let output = std::str::from_utf8(&bytes).unwrap();
    for needle in ["trace message", "debug message", "info message", "warn message", "error message"] {
        assert!(output.contains(needle), "missing log output: {needle}");
    }

    clear_output();
    test_log();
    let bytes = get_output();
    let output = std::str::from_utf8(&bytes).unwrap();
    assert!(output.contains("____") || output.contains("LOG TEST"));
}

#[test]
fn test_console_sync() {
    let _guard = lock_test();
    let output = Arc::new(Mutex::new(Vec::new()));
    let console = Arc::new(TestConsole {
        output: output.clone(),
    });

    let console_clone = console.clone();
    std::thread::spawn(move || {
        console_clone.put_char(b'X');
    })
    .join()
    .unwrap();

    assert_eq!(output.lock().unwrap().as_slice(), b"X");
}
