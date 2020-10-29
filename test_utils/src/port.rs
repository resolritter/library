use crate::path::{executable_path, tests_lock_path};
use std::process::{Command, Stdio};

pub type Port = u16;

pub fn get_free_port() -> Port {
    String::from_utf8_lossy(
        &Command::new("flock")
            .arg("-x")
            .arg(tests_lock_path())
            .arg("-c")
            .arg(format!("{} get_port_sync | tail -n +2", executable_path()))
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
