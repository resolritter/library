pub mod format;
pub mod path;
pub mod port;

use crate::path::executable_path;
use crate::port::get_free_port;
use async_process::Command;
use notify::{raw_watcher, RecursiveMode, Watcher};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tempdir::TempDir;

pub struct SpawnedTest {
    pub log_dir: PathBuf,
    pub server_addr: String,
    pub process: async_process::Child,
}

pub fn spawn_test_program(tmp_dir: &TempDir) -> SpawnedTest {
    let app_port = get_free_port();
    let app_dir = tmp_dir.path().to_str().unwrap();
    let instance = tmp_dir.path().extension().unwrap().to_str().unwrap();
    let server_addr = format!("http://localhost:{}", app_port);
    let server_addr_arg = server_addr.clone();
    let log_dir = tmp_dir.path().join("log");

    let signal_file = tmp_dir.path().join("is_ready");
    File::create(&signal_file).unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = raw_watcher(tx).unwrap();
    watcher
        .watch(&signal_file, RecursiveMode::NonRecursive)
        .unwrap();

    let process = Command::new(executable_path().unwrap())
        .arg("--listen")
        .arg(server_addr_arg)
        .arg("--instance")
        .arg(instance)
        .arg("--dir")
        .arg(app_dir)
        .arg("--signal-file")
        .arg(signal_file)
        .arg("test_server")
        .spawn()
        .unwrap();

    let timeout_secs = 10;
    rx.recv_timeout(std::time::Duration::from_secs(timeout_secs))
        .expect(
            format!(
                "Test server took more than {} seconds to signal its start. Aborting.",
                timeout_secs
            )
            .as_str(),
        );

    SpawnedTest {
        server_addr,
        log_dir,
        process,
    }
}

pub fn read_snapshot(log_dir: &PathBuf) -> String {
    let entry = log_dir.read_dir().unwrap().next().unwrap().unwrap();
    let mut file = File::open(entry.path()).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}