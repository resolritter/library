use path_clean::PathClean;
use std::env;
use std::path::{Path, PathBuf};

pub fn absolute_path(path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }
    .clean();

    Ok(absolute_path)
}

pub fn root_path() -> PathBuf {
    absolute_path(format!("{}/../../../..", file!())).unwrap()
}

pub fn executable_path() -> String {
    format!("{}", root_path().join("run.sh").display())
}

pub fn tests_lock_path() -> String {
    format!("{}", root_path().join("server/tests/.tests.lock").display())
}
