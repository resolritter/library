use insta::assert_snapshot;
use stdext::function_name;
use surf::http::mime::JSON;
use surf::StatusCode;
use tempdir::TempDir;
use test_utils::{
    format::format_test_name, port::get_free_port, read_snapshot, spawn_test_program, SpawnedTest,
};

#[async_std::test]
async fn test_create_user() -> std::io::Result<()> {
    let app_port = get_free_port();
    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        log_dir,
        process: _,
    } = &spawn_test_program(&tmp_dir, app_port);

    let user_route = format!("{}/user", &server_addr);
    let mut should_work = surf::post(&user_route)
        .body(r#"{ "email": "user@user.com", "access_level": 0 }"#)
        .content_type(JSON)
        .await
        .unwrap();
    assert!(should_work.status() == StatusCode::Created);
    assert_snapshot!(test_name, should_work.body_string().await.unwrap());

    assert_snapshot!(read_snapshot(&log_dir));
    Ok(())
}
