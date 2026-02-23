use or_die::{OrDie, OrDieWithMsg, OrDieWithOnOption, OrDieWithOnResult, die};

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
