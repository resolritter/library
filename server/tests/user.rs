use entities::{access_level, user, UserCreationPayload};
use insta::assert_snapshot;
use stdext::function_name;
use surf::StatusCode;
use tempdir::TempDir;
use test_utils::{format::format_test_name, read_snapshot, spawn_test_program, SpawnedTest};

#[async_std::test]
async fn test_create_user() {
    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();

    // Calling the test server with those credentials will create a superuser at the start
    let admin_access_token = "ADMIN";
    let admin_email = "admin@admin.com";
    let SpawnedTest {
        server_addr,
        log_dir,
        process: _,
    } = &spawn_test_program(
        &tmp_dir,
        Some(format!("{}::{}", admin_email, admin_access_token)),
    );

    user::create(
        &server_addr,
        &UserCreationPayload {
            email: "librarian@user.com".to_string(),
            access_level: access_level::LIBRARIAN,
            // Administrators can create librarians
            requester_access_token: Some(admin_access_token.to_string()),
        },
    )
    .await;
    let (_, simple_user) = user::create(
        &server_addr,
        &UserCreationPayload {
            email: "simple@user.com".to_string(),
            access_level: access_level::USER,
            // Normal user creation does not require any sort of special access
            requester_access_token: None,
        },
    )
    .await;

    // Normal users should not be able to create any sort of special user
    let bad_forbidden_request_admin = user::do_create(
        &server_addr,
        &UserCreationPayload {
            email: "new_ADMIN@user.com".to_string(),
            access_level: access_level::ADMIN,
            requester_access_token: Some(simple_user.access_token.clone()),
        },
    )
    .await;
    assert!(bad_forbidden_request_admin.status() == StatusCode::Forbidden);
    let bad_forbidden_request_librarian = user::do_create(
        &server_addr,
        &UserCreationPayload {
            email: "new_LIBRARIAN@user.com".to_string(),
            access_level: access_level::LIBRARIAN,
            requester_access_token: Some(simple_user.access_token.clone()),
        },
    )
    .await;
    assert!(bad_forbidden_request_librarian.status() == StatusCode::Forbidden);

    assert_snapshot!(read_snapshot(&log_dir));
}
