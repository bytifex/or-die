#![no_std]

#[macro_export]
macro_rules! die {
    ($($arg:tt)*) => {
        ::core::panic!($($arg)*)
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
                die!("internal error: {:?}", e);
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
                die!("internal error: {:?}", f(e));
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
                die!("internal error: '{}', {:?}", msg, e);
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
                die!("internal error: {:?}", f());
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
                die!("internal error: '{}'", msg);
            }
        }
    }
}
