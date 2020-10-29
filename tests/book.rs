use async_process::Command;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::{thread, time};
use tempdir::TempDir;

fn get_free_port() -> u16 {
    loop {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
        if let Ok(listener) = TcpListener::bind(addr) {
            if let Ok(bind) = listener.local_addr() {
                return bind.port();
            }
        }
    }
}

#[async_std::test]
async fn my_test() -> std::io::Result<()> {
    let db_port = get_free_port();
    let app_port = get_free_port();
    let tmp_dir = TempDir::new("my_test").unwrap();
    let app_dir = tmp_dir.path().to_str().unwrap();
    let instance = tmp_dir.path().extension().unwrap().to_str().unwrap();
    println!("{}", app_dir);

    let server_addr = format!("http://localhost:{}", app_port);

    let child = Command::new("/home/reaysawa/rs/library/run.sh")
        .arg("--db-port")
        .arg(format!("{}", db_port))
        .arg("--listen")
        .arg(&server_addr)
        .arg("--instance")
        .arg(instance)
        .arg("--dir")
        .arg(app_dir)
        .arg("--test")
        .spawn()
        .unwrap();

    thread::sleep(time::Duration::from_secs(10));
    let book_route = format!("{}/book/Rapunzel", &server_addr);
    surf::get(book_route).recv_string().await.unwrap();

    let log_dir = tmp_dir.path().join("log");
    for entry in log_dir.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let file = File::open(entry.path()).unwrap();
            let buf = BufReader::new(file);
            for line in buf.lines() {
                if let Ok(l) = line {
                    println!("Loggin! {}", l);
                }
            }
        }
    }

    Ok(())
}
