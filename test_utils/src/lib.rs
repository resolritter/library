pub mod format;
pub mod path;
pub mod port;

use crate::path::executable_path;
use crate::port::get_free_port;
use async_process::Command;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tempdir::TempDir;

pub struct SpawnedTest {
    pub log_dir: PathBuf,
    pub server_addr: String,
    pub process: async_process::Child,
}

pub fn spawn_test_program() -> SpawnedTest {
    let app_port = get_free_port();
    let tmp_dir = TempDir::new("testing").unwrap();
    let app_dir = tmp_dir.path().to_str().unwrap();
    let instance = tmp_dir.path().extension().unwrap().to_str().unwrap();
    let server_addr = format!("http://localhost:{}", app_port);
    let server_addr_arg = server_addr.clone();
    let log_dir = tmp_dir.path().join("log");

    SpawnedTest {
        server_addr,
        log_dir,
        process: Command::new(executable_path().unwrap())
            .arg("--listen")
            .arg(server_addr_arg)
            .arg("--instance")
            .arg(instance)
            .arg("--dir")
            .arg(app_dir)
            .arg("test_server")
            .spawn()
            .unwrap(),
    }
}

pub fn read_snapshot(log_dir: &PathBuf) -> String {
    let entry = log_dir.read_dir().unwrap().next().unwrap().unwrap();
    let mut file = File::open(entry.path()).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}
