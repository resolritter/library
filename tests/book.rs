use insta::assert_snapshot;
use stdext::function_name;
use tempdir::TempDir;
use test_utils::{format::format_test_name, read_snapshot, spawn_test_program, SpawnedTest};

#[async_std::test]
async fn test_get() -> std::io::Result<()> {
    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        mut process,
        log_dir,
    } = spawn_test_program(&tmp_dir);

    let book_route = format!("{}/book/Rapunzel", &server_addr);
    let mut get = surf::get(book_route).await.unwrap();
    assert!(get.status() == 200);
    assert_snapshot!(test_name, get.body_string().await.unwrap());

    assert_snapshot!(read_snapshot(&log_dir));
    process.kill().unwrap();
    Ok(())
}
