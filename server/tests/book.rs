use entities::{book, user, BookLeaseByTitlePayload, UserCreationPayload};
use insta::assert_snapshot;
use stdext::function_name;
use surf::StatusCode;
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

    const WHOLE_DAY: i64 = 86400;
    let (_, sample_user) = user::create(
        &server_addr,
        &UserCreationPayload {
            email: "user@user.com".to_string(),
            access_level: 0,
        },
    )
    .await;
    let payload = BookLeaseByTitlePayload {
        title: "Rapunzel".to_string(),
        lease_id_req: sample_user.email,
        lease_length: WHOLE_DAY,
    };

    // Lease a book for a whole day
    assert_snapshot!(
        format!("{}_good", test_name),
        book::borrow(&server_addr, &payload)
            .await
            .body_string()
            .await
            .unwrap()
    );

    let bad_borrow = book::do_borrow(&server_addr, &payload).await.unwrap();
    assert!(bad_borrow.status() == StatusCode::Forbidden);

    assert_snapshot!(read_snapshot(&log_dir));
}
