use entities::{user, UserCreationPayload};
use insta::assert_snapshot;
use stdext::function_name;
use tempdir::TempDir;
use test_utils::{format::format_test_name, read_snapshot, spawn_test_program, SpawnedTest};

#[async_std::test]
async fn test_create_user() {
    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        log_dir,
        process: _,
    } = &spawn_test_program(&tmp_dir);

    let (user_str, _) = user::create(
        &server_addr,
        &UserCreationPayload {
            email: "user@user.com".to_string(),
            access_level: 0 as i32,
        },
    )
    .await;
    assert_snapshot!(test_name, user_str);

    assert_snapshot!(read_snapshot(&log_dir));
}
