use entities::user;
use insta::assert_snapshot;
use stdext::function_name;
use tempdir::TempDir;
use test_utils::{
    format::format_test_name, read_snapshot, spawn_test_program, SpawnedTest,
};

#[async_std::test]
async fn test_create_user() {
    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        log_dir,
        process: _,
    } = &spawn_test_program(&tmp_dir);

    assert_snapshot!(
        test_name,
        user::create(&server_addr)
            .await
            .body_string()
            .await
            .unwrap()
    );

    assert_snapshot!(read_snapshot(&log_dir));
}
