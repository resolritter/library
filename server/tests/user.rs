use entities::access_mask;
use insta::assert_snapshot;
use stdext::function_name;
use surf::StatusCode;
use tempdir::TempDir;
use test_utils::{
    constants::{ADMIN_ACCESS_TOKEN, ADMIN_EMAIL},
    format::format_test_name,
    read_snapshot, spawn_test_program, SpawnedTest,
};

#[async_std::test]
async fn test_create_user_and_login() {
    use entities::{user, UserCreatePayload, UserLoginPayload};

    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        log_dir,
        process: _,
    } = &spawn_test_program(&tmp_dir, None);

    // Fail a login with a user which does _not_ exist
    let bad_request_nonexistent_user = user::do_login(
        &server_addr,
        &UserLoginPayload {
            email: "DOES_NOT_EXIST@user.com".to_string(),
        },
    )
    .await;
    assert!(bad_request_nonexistent_user.status() == StatusCode::NotFound);

    // Creates a simple user
    let (_, user) = user::create(
        &server_addr,
        &UserCreatePayload {
            email: "simple@user.com".to_string(),
            access_mask: access_mask::USER,
            requester_access_token: None,
        },
    )
    .await;

    // Logs in with the created user
    let (_, same_user) = user::login(
        &server_addr,
        &UserLoginPayload {
            email: user.email.to_string(),
        },
    )
    .await;
    assert!(user.email == same_user.email);

    assert_snapshot!(read_snapshot(&log_dir));
}

#[async_std::test]
async fn test_create_user() {
    use entities::{user, UserCreatePayload};

    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        log_dir,
        process: _,
    } = &spawn_test_program(&tmp_dir, Some((ADMIN_EMAIL, ADMIN_ACCESS_TOKEN)));

    user::create(
        &server_addr,
        &UserCreatePayload {
            email: "librarian@user.com".to_string(),
            access_mask: access_mask::LIBRARIAN,
            // Administrators can create librarians
            requester_access_token: Some(ADMIN_ACCESS_TOKEN.to_string()),
        },
    )
    .await;
    let (_, simple_user) = user::create(
        &server_addr,
        &UserCreatePayload {
            email: "simple@user.com".to_string(),
            access_mask: access_mask::USER,
            // Normal user creation does not require any sort of special access
            requester_access_token: None,
        },
    )
    .await;

    // Normal users should not be able to create any sort of special user
    let bad_forbidden_request_admin = user::do_create(
        &server_addr,
        &UserCreatePayload {
            email: "new_ADMIN@user.com".to_string(),
            access_mask: access_mask::ADMIN,
            requester_access_token: Some(simple_user.access_token.clone()),
        },
    )
    .await;
    assert!(bad_forbidden_request_admin.status() == StatusCode::Forbidden);
    let bad_forbidden_request_librarian = user::do_create(
        &server_addr,
        &UserCreatePayload {
            email: "new_LIBRARIAN@user.com".to_string(),
            access_mask: access_mask::LIBRARIAN,
            requester_access_token: Some(simple_user.access_token.clone()),
        },
    )
    .await;
    assert!(bad_forbidden_request_librarian.status() == StatusCode::Forbidden);

    assert_snapshot!(read_snapshot(&log_dir));
}
