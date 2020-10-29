use entities::access_level;
use insta::assert_snapshot;
use stdext::function_name;
use surf::StatusCode;
use tempdir::TempDir;
use test_utils::{format::format_test_name, read_snapshot, spawn_test_program, SpawnedTest};

#[async_std::test]
async fn test_get() {
    use entities::book;

    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        log_dir,
        process: _,
    } = &spawn_test_program(&tmp_dir, None);

    let mut get = book::get(&server_addr, "Rapunzel").await;
    assert_snapshot!(test_name, get.body_string().await.unwrap());

    assert_snapshot!(read_snapshot(&log_dir));
}

#[async_std::test]
async fn test_lease() {
    use entities::{book, user, BookLeaseByTitlePayload, UserCreationPayload, UserPublic};

    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        log_dir,
        process: _,
    } = &spawn_test_program(&tmp_dir, None);

    const WHOLE_DAY: i64 = 86400;
    let (_, sample_user) = user::create(
        &server_addr,
        &UserCreationPayload {
            email: "simple@user.com".to_string(),
            access_level: access_level::USER,
            requester_access_token: None,
        },
    )
    .await;
    let payload = BookLeaseByTitlePayload {
        title: "Rapunzel".to_string(),
        lease_id_req: sample_user.email.clone(),
        lease_length: WHOLE_DAY,
    };

    // The route should be protected against guest users
    let guest_user = book::do_borrow(&server_addr, None, "DOES_NOT_MATTER", &payload)
        .await
        .unwrap();
    assert!(guest_user.status() == StatusCode::Forbidden);

    // The route should be protected against ficticious users
    let bad_ficticious_user = book::do_borrow(
        &server_addr,
        Some(&UserPublic {
            email: "DOES_NOT_EXIST".to_string(),
            access_level: access_level::USER,
            access_token: "DOES_NOT_MATTER".to_string(),
        }),
        &sample_user.access_token,
        &payload,
    )
    .await
    .unwrap();
    assert!(bad_ficticious_user.status() == StatusCode::Forbidden);

    // The route should be protected against users with invalid tokens
    let bad_invalid_token_user =
        book::do_borrow(&server_addr, Some(&sample_user), "INVALID_TOKEN", &payload)
            .await
            .unwrap();
    assert!(bad_invalid_token_user.status() == StatusCode::Forbidden);

    // Lease a book for a whole day
    assert_snapshot!(
        format!("{}_good", test_name),
        book::borrow(
            &server_addr,
            Some(&sample_user),
            &sample_user.access_token,
            &payload
        )
        .await
        .body_string()
        .await
        .unwrap()
    );

    // The book has been leased successfully, thus the following should not work
    let bad_borrow = book::do_borrow(
        &server_addr,
        Some(&sample_user),
        &sample_user.access_token,
        &payload,
    )
    .await
    .unwrap();
    assert!(bad_borrow.status() == StatusCode::Forbidden);

    assert_snapshot!(read_snapshot(&log_dir));
}
