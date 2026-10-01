use or_die::{OrDie, OrDieWithMsg, OrDieWithOnOption, OrDieWithOnResult, die};

// Guards the tests below since the die handler is process-global state shared across
// test threads.
static DIE_HANDLER_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
static DIE_HANDLER_CALLED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

fn test_die_handler(
    location: &'static core::panic::Location<'static>,
    #[cfg(not(feature = "no-std"))] backtrace: std::backtrace::Backtrace,
    args: core::fmt::Arguments,
) -> ! {
    DIE_HANDLER_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);

    #[cfg(feature = "no-std")]
    panic!("location: {}, {}", location, args);
    #[cfg(not(feature = "no-std"))]
    panic!("location: {}, backtrace: {}\n{}", location, backtrace, args);
}

#[test]
fn set_die_handler_is_invoked_by_die() {
    let _guard = DIE_HANDLER_TEST_LOCK.lock().unwrap();
    DIE_HANDLER_CALLED.store(false, std::sync::atomic::Ordering::SeqCst);

    or_die::set_die_handler(test_die_handler);
    let result = std::panic::catch_unwind(|| {
        die!("my error message");
    });

    assert!(result.is_err());
    assert!(DIE_HANDLER_CALLED.load(std::sync::atomic::Ordering::SeqCst));
}

#[test]
fn set_die_handler_closure_is_invoked_by_die() {
    let _guard = DIE_HANDLER_TEST_LOCK.lock().unwrap();
    DIE_HANDLER_CALLED.store(false, std::sync::atomic::Ordering::SeqCst);

    or_die::set_die_handler(
        |location, #[cfg(not(feature = "no-std"))] backtrace, args| {
            test_die_handler(
                location,
                #[cfg(not(feature = "no-std"))]
                backtrace,
                args,
            )
        },
    );

    let result = std::panic::catch_unwind(|| {
        die!("my error message");
    });

    assert!(result.is_err());
    assert!(DIE_HANDLER_CALLED.load(std::sync::atomic::Ordering::SeqCst));
}

#[test]
fn reset_die_handler_restores_default_panic_behavior() {
    let _guard = DIE_HANDLER_TEST_LOCK.lock().unwrap();
    or_die::set_die_handler(test_die_handler);
    or_die::reset_die_handler();
    DIE_HANDLER_CALLED.store(false, std::sync::atomic::Ordering::SeqCst);

    let result = std::panic::catch_unwind(|| {
        die!("my error message");
    });

    assert!(result.is_err());
    assert!(!DIE_HANDLER_CALLED.load(std::sync::atomic::Ordering::SeqCst));
}

#[test]
#[should_panic]
fn file_handling() {
    fn read_file_to_string(file_path: impl AsRef<std::path::Path>) -> String {
        std::fs::read_to_string(file_path.as_ref()).or_die_with(|e| {
            format!(
                "could not read file, path = {:?}, error = {e:?}",
                file_path.as_ref()
            )
        })
    }

    read_file_to_string("non-existing-file");
}

#[test]
#[should_panic]
fn die() {
    die!("my error message");
}

#[test]
fn or_die_on_ok() {
    let value = Ok::<&str, u64>("foobar").or_die();
    assert_eq!(value, "foobar");
}

#[test]
#[should_panic]
fn or_die_on_err() {
    Err::<&str, u64>(42).or_die();
}

#[test]
fn or_die_on_some() {
    let value = Some::<&str>("foobar").or_die();
    assert_eq!(value, "foobar");
}

#[test]
#[should_panic]
fn or_die_on_none() {
    None::<&str>.or_die();
}

#[test]
fn or_die_with_on_ok() {
    let value =
        Ok::<&str, u64>("foobar").or_die_with(|e| format!("my error message, error = '{e}'"));
    assert_eq!(value, "foobar");
}

#[test]
#[should_panic]
fn or_die_with_on_err() {
    Err::<&str, u64>(42).or_die_with(|e| format!("my error message, error = '{e}'"));
}

#[test]
fn or_die_with_on_some() {
    let value = Some::<&str>("foobar").or_die_with(|| "my error message".to_string());
    assert_eq!(value, "foobar");
}

#[test]
#[should_panic]
fn or_die_with_on_none() {
    None::<&str>.or_die_with(|| "my error message".to_string());
}

#[test]
fn or_die_with_msg_on_ok() {
    let value = Ok::<&str, u64>("foobar").or_die_with_msg("my error message");
    assert_eq!(value, "foobar");
}

#[test]
#[should_panic]
fn or_die_with_msg_on_err() {
    Err::<&str, u64>(42).or_die_with_msg("my error message");
}

#[test]
fn or_die_with_msg_on_some() {
    let value = Some::<&str>("foobar").or_die_with_msg("my error message");
    assert_eq!(value, "foobar");
}

#[test]
#[should_panic]
fn or_die_with_msg_on_none() {
    None::<&str>.or_die_with_msg("my error message");
}
