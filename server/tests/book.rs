use entities::book;
use insta::assert_snapshot;
use stdext::function_name;
use tempdir::TempDir;
use test_utils::{format::format_test_name, read_snapshot, spawn_test_program, SpawnedTest};

#[async_std::test]
async fn test_get() {
    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        log_dir,
        process: _,
    } = &spawn_test_program(&tmp_dir);

    let mut get = book::get(&server_addr, "Rapunzel").await;
    assert_snapshot!(test_name, get.body_string().await.unwrap());

    assert_snapshot!(read_snapshot(&log_dir));
}

#[async_std::test]
async fn test_lease() {
    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        log_dir,
        process: _,
    } = &spawn_test_program(&tmp_dir);

    // Lease a book for a whole day
    let book = "Rapunzel";
    assert_snapshot!(
        format!("{}_good", test_name),
        book::borrow(&server_addr, book)
            .await
            .body_string()
            .await
            .unwrap()
    );

    // This time it should not be allowed since the book will still be borrowed.
    assert_snapshot!(
        format!("{}_bad", test_name),
        book::bad_borrow(&server_addr, book)
            .await
            .body_string()
            .await
            .unwrap()
    );

    assert_snapshot!(read_snapshot(&log_dir));
}
