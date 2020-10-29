pub mod constants;
pub mod format;
pub mod path;
pub mod port;

use crate::path::executable_path;
use crate::port::{get_free_port, Port};
use async_process::Command;
use notify::{raw_watcher, RecursiveMode, Watcher};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tempdir::TempDir;

pub struct SpawnedTest {
    pub log_dir: PathBuf,
    pub server_addr: String,
    process: async_process::Child,
    app_port: Port,
}

impl Drop for SpawnedTest {
    fn drop(&mut self) {
        // kill the whole tree of subprocesses spawned by the bash entrypoint
        let mut proc_kill_cmd = String::from("kill -- $(pstree -p -g ");
        proc_kill_cmd.push_str(format!("{}", self.process.id()).as_str());
        proc_kill_cmd.push_str(" | head -n1 | awk '{ m=match($0, /library\\(([0-9]+)/, ms); if (m) { print ms[1] } }')");
        std::process::Command::new("bash")
            .arg("-c")
            .arg(proc_kill_cmd)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        std::process::Command::new(executable_path())
            .arg("free_port")
            .arg(format!("{}", &self.app_port))
            .stdout(std::process::Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

pub fn spawn_test_program(
    tmp_dir: &TempDir,
    admin_credentials: Option<(&str, &str)>,
) -> SpawnedTest {
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

    // Workaround for weird lifetime reference on this builder
    macro_rules! server_builder {
        () => {
            Command::new(executable_path())
                .arg("--listen")
                .arg(server_addr_arg)
                .arg("--instance")
                .arg(instance)
                .arg("--dir")
                .arg(app_dir)
                .arg("--signal-file")
                .arg(signal_file)
                .arg("test_server")
        };
    }
    let process = match admin_credentials {
        Some((email, access_token)) => server_builder!()
            .arg("--admin-credentials-for-test")
            .arg(format!("{}::{}", email, access_token))
            .spawn()
            .unwrap(),
        None => server_builder!().spawn().unwrap(),
    };

    let timeout_secs = 10;
    rx.recv_timeout(std::time::Duration::from_secs(timeout_secs))
        .expect(
            format!(
                "Test server took more than {} seconds to signal its start. Aborting.",
                timeout_secs
            )
            .as_str(),
        );
    // Pause for just a bit until the server is _actually_ ready to receive connections
    std::thread::sleep(std::time::Duration::from_secs(1));

    SpawnedTest {
        server_addr,
        log_dir,
        process,
        app_port,
    }
}

pub fn read_snapshot(log_dir: &PathBuf) -> String {
    let entry = log_dir.read_dir().unwrap().next().unwrap().unwrap();
    let mut file = File::open(entry.path()).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}
