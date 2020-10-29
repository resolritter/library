use insta::assert_snapshot;
use stdext::function_name;
use surf::http::mime::JSON;
use surf::StatusCode;
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

    // Check that a book exists and can be retrieved from the API
    let book_route = format!("{}/book/Rapunzel", &server_addr);
    let mut get = surf::get(book_route).await.unwrap();
    assert!(get.status() == 200);
    assert_snapshot!(test_name, get.body_string().await.unwrap());

    assert_snapshot!(read_snapshot(&log_dir));
    process.kill().unwrap();
    Ok(())
}

#[async_std::test]
async fn test_lease() -> std::io::Result<()> {
    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        mut process,
        log_dir,
    } = spawn_test_program(&tmp_dir);

    let book_route = format!("{}/book/Rapunzel", &server_addr);
    let borrow_for_a_day = r#"{ "lease_length": 86400 }"#;

    // Lease a book for a whole day
    let mut good_borrow = surf::patch(&book_route)
        .body(borrow_for_a_day)
        .content_type(JSON)
        .await
        .unwrap();
    let result = good_borrow.body_string().await.unwrap();
    assert!(good_borrow.status() == StatusCode::Ok);
    assert_snapshot!(format!("{}_good", test_name), result);

    // Try again.
    // It should not be allowed since a whole day should have not passed by.
    let mut bad_borrow = surf::patch(&book_route)
        .body(borrow_for_a_day)
        .content_type(JSON)
        .await
        .unwrap();
    let result = bad_borrow.body_string().await.unwrap();
    assert!(bad_borrow.status() == StatusCode::Forbidden);
    assert_snapshot!(format!("{}_bad", test_name), result);

    assert_snapshot!(read_snapshot(&log_dir));
    process.kill().unwrap();
    Ok(())
}
