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

## Example
```
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
