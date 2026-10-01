#![cfg_attr(feature = "no-std", no_std)]

use core::sync::atomic::{AtomicPtr, Ordering};

/// A global handler invoked by `die!` in place of panicking, if one has been set.
#[cfg(not(feature = "no-std"))]
pub type DieHandler = fn(
    &'static core::panic::Location<'static>,
    std::backtrace::Backtrace,
    core::fmt::Arguments,
) -> !;
#[cfg(feature = "no-std")]
pub type DieHandler = fn(&'static core::panic::Location<'static>, core::fmt::Arguments) -> !;

static DIE_HANDLER: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());

/// Sets a global handler that `die!` (and the `or_die` family) will call instead of panicking.
pub fn set_die_handler(handler: DieHandler) {
    DIE_HANDLER.store(handler as *mut (), Ordering::SeqCst);
}

/// Clears the global die handler, reverting `die!` to its default panicking behavior.
pub fn reset_die_handler() {
    DIE_HANDLER.store(core::ptr::null_mut(), Ordering::SeqCst);
}

#[doc(hidden)]
pub fn die_handler() -> Option<DieHandler> {
    let ptr = DIE_HANDLER.load(Ordering::SeqCst);
    // SAFETY: `Option<DieHandler>` is guaranteed null-pointer-optimized, and `ptr` is either
    // null (unset) or a value stored from a `DieHandler` via `set_die_handler`.
    unsafe { core::mem::transmute::<*mut (), Option<DieHandler>>(ptr) }
}

#[macro_export]
macro_rules! die {
    ($($arg:tt)*) => {
        match $crate::die_handler() {
            ::core::option::Option::Some(handler) => handler(
                ::core::panic::Location::caller(),
                #[cfg(not(feature = "no-std"))]
                std::backtrace::Backtrace::capture(),
                ::core::format_args!($($arg)*),
            ),
            ::core::option::Option::None => {
                #[cfg(not(feature = "no-std"))]
                ::core::panic!(
                    "backtrace: {}, {}",
                    std::backtrace::Backtrace::capture(),
                    ::core::format_args!($($arg)*),
                );

                #[cfg(feature = "no-std")]
                ::core::panic!($($arg)*);
            },
        }
    }
}

pub trait OrDie<T> {
    #[track_caller]
    fn or_die(self) -> T;
}

pub trait OrDieWithOnResult<T, FromErrorType, ToErrorType> {
    #[track_caller]
    fn or_die_with<F: FnOnce(FromErrorType) -> ToErrorType>(self, f: F) -> T;
}

pub trait OrDieWithOnOption<T, ToErrorType> {
    #[track_caller]
    fn or_die_with<F: FnOnce() -> ToErrorType>(self, f: F) -> T;
}

pub trait OrDieWithMsg<T> {
    #[track_caller]
    fn or_die_with_msg(self, msg: &str) -> T;
}

impl<T, ErrorType: core::fmt::Debug> OrDie<T> for Result<T, ErrorType> {
    #[track_caller]
    #[inline(always)]
    fn or_die(self) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                die!("internal error: error = '{:?}'", e);
            }
        }
    }
}

impl<T, FromErrorType, ToErrorType: core::fmt::Debug>
    OrDieWithOnResult<T, FromErrorType, ToErrorType> for Result<T, FromErrorType>
{
    #[track_caller]
    #[inline(always)]
    fn or_die_with<F: FnOnce(FromErrorType) -> ToErrorType>(self, f: F) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                die!("internal error: error = '{:?}'", f(e));
            }
        }
    }
}

impl<T, ErrorType: core::fmt::Debug> OrDieWithMsg<T> for Result<T, ErrorType> {
    #[inline(always)]
    #[track_caller]
    fn or_die_with_msg(self, msg: &str) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                die!("internal error: message = '{}', error = '{:?}'", msg, e);
            }
        }
    }
}

impl<T> OrDie<T> for Option<T> {
    #[inline(always)]
    #[track_caller]
    fn or_die(self) -> T {
        match self {
            Some(value) => value,
            None => {
                die!("internal error");
            }
        }
    }
}

impl<T, ToErrorType: core::fmt::Debug> OrDieWithOnOption<T, ToErrorType> for Option<T> {
    #[inline(always)]
    #[track_caller]
    fn or_die_with<F: FnOnce() -> ToErrorType>(self, f: F) -> T {
        match self {
            Some(value) => value,
            None => {
                die!("internal error: error = '{:?}'", f());
            }
        }
    }
}

impl<T> OrDieWithMsg<T> for Option<T> {
    #[inline(always)]
    #[track_caller]
    fn or_die_with_msg(self, msg: &str) -> T {
        match self {
            Some(value) => value,
            None => {
                die!("internal error: message = '{}'", msg);
            }
        }
    }
}
