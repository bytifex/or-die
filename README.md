# or-die
The aim of this package is to add methods for `Option` and `Result` in the style of `or_die(..)`. These methods unwraps the value, or panics on `Err`s and `None`s.


# Macro
A macro (`die!`) is introduced. Currently it just calls `panic!`.


## Methods
All methods are in the form of `or_die..(..)`. The following methods can be used:
- Options
  - `Option::or_die()`
  - `Option::or_die_with(f: impl FnOnce() -> ToErrorType)`
  - `Option::or_die_with_msg(msg: &str)`
- Results
  - `Result::or_die()`
  - `Result::or_die_with(f: impl FnOnce(FromErrorType) -> ToErrorType)`
  - `Result::or_die_with_msg(msg: &str)`


### Example
```rust
use or_die::OrDieWith;

fn read_file_to_string(file_path: impl AsRef<std::path::Path>) -> String {
    std::fs::read_to_string(file_path.as_ref()).or_die_with(|e| {
        format!(
            "could not read file, path = {:?}, error = {e:?}",
            file_path.as_ref()
        )
    })
}
```


## DieHandler
A handler can be set, that is called when one of the `die` methods are called. Also, this handler can be reset.

Enable the `no-std` feature to build the crate without linking `std`:

```toml
or-die = { version = "*", features = ["no-std"] }
```

In `std` mode, the die handler has three parameters:
- `&'static core::panic::Location<'static>`
- `std::backtrace::Backtrace`
- `core::fmt::Arguments`

In `no-std` mode, the die handler does not have the backtrace parameter.

The following functions can be used to setting and resetting the handler that is called when one of the `die` methods are called:
- `set_die_handler(..)`
- `reset_die_handler()`


### Examples (`std`)

**Using a function**
```rust
use or_die::die;

fn die_handler(
    location: &'static core::panic::Location<'static>,
    backtrace: std::backtrace::Backtrace,
    args: core::fmt::Arguments,
) -> ! {
    eprintln!("location: {}, backtrace: {}\n{}", location, backtrace, args);

    std::process::exit(0);
}

fn main() {
    or_die::set_die_handler(die_handler);
    die!("my error message");
}
```

**Using a closure**
```rust
use or_die::die;

fn main() {
    or_die::set_die_handler(|location, backtrace, args| {
        eprintln!("location: {}, backtrace: {}\n{}", location, backtrace, args);
        std::process::exit(0);
    });
    die!("my error message");
}
```
