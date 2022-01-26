use crate::path::executable_path;
use std::process::{Command, Stdio};

pub type Port = u16;

pub fn get_free_port() -> Port {
    String::from_utf8_lossy(
        &Command::new(executable_path())
            .arg("get_port")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap()
            .stdout,
    )
    .trim()
    .parse::<u16>()
    .unwrap()
}
