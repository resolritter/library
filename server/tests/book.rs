use entities::access_mask;
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
    let (_, first_user) = user::create(
        &server_addr,
        &UserCreationPayload {
            email: "simple@user.com".to_string(),
            access_mask: access_mask::USER,
            requester_access_token: None,
        },
    )
    .await;
    let payload = BookLeaseByTitlePayload {
        title: "Rapunzel".to_string(),
        lease_id_req: first_user.email.clone(),
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
            access_mask: access_mask::USER,
            access_token: "DOES_NOT_MATTER".to_string(),
        }),
        &first_user.access_token,
        &payload,
    )
    .await
    .unwrap();
    assert!(bad_ficticious_user.status() == StatusCode::Forbidden);

    // The route should be protected against users with invalid tokens
    let bad_invalid_token_user =
        book::do_borrow(&server_addr, Some(&first_user), "INVALID_TOKEN", &payload)
            .await
            .unwrap();
    assert!(bad_invalid_token_user.status() == StatusCode::Forbidden);

    // Lease a book for a whole day
    assert_snapshot!(
        format!("{}_good", test_name),
        book::borrow(
            &server_addr,
            Some(&first_user),
            &first_user.access_token,
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
        Some(&first_user),
        &first_user.access_token,
        &payload,
    )
    .await
    .unwrap();
    assert!(bad_borrow.status() == StatusCode::Forbidden);

    assert_snapshot!(read_snapshot(&log_dir));
}

#[async_std::test]
async fn test_end_loan() {
    use entities::{
        book, user, BookEndLoanByTitlePayload, BookLeaseByTitlePayload, UserCreationPayload,
    };

    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
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

    const WHOLE_DAY: i64 = 86400;
    let (_, first_user) = user::create(
        &server_addr,
        &UserCreationPayload {
            email: "first@user.com".to_string(),
            access_mask: access_mask::USER,
            requester_access_token: None,
        },
    )
    .await;
    let (_, second_user) = user::create(
        &server_addr,
        &UserCreationPayload {
            email: "second@user.com".to_string(),
            access_mask: access_mask::USER,
            requester_access_token: None,
        },
    )
    .await;
    let mut borrow_payload = BookLeaseByTitlePayload {
        title: "Rapunzel".to_string(),
        lease_id_req: first_user.email.clone(),
        lease_length: WHOLE_DAY,
    };

    // Borrow a book with the first user
    book::borrow(
        &server_addr,
        Some(&first_user),
        &first_user.access_token,
        &borrow_payload,
    )
    .await;

    // The second user won't be able to end the loan on behalf of the first
    let mut end_loan_payload = BookEndLoanByTitlePayload {
        title: borrow_payload.title.clone(),
        lease_id_req: borrow_payload.lease_id_req.clone(),
        access_token_req: second_user.access_token.to_string(),
    };
    let bad_forbidden_end_loan =
        book::do_end_loan(&server_addr, &second_user.email, &end_loan_payload)
            .await
            .unwrap();
    assert!(bad_forbidden_end_loan.status() == StatusCode::Forbidden);

    // The first user can end the loan on its own
    end_loan_payload = BookEndLoanByTitlePayload {
        access_token_req: first_user.access_token.to_string(),
        ..end_loan_payload
    };
    book::end_loan(&server_addr, &first_user.email, &end_loan_payload).await;

    // Now the book is free and the second user will be able to borrow it
    borrow_payload = BookLeaseByTitlePayload {
        lease_id_req: second_user.email.clone(),
        ..borrow_payload
    };
    book::borrow(
        &server_addr,
        Some(&second_user),
        &second_user.access_token,
        &borrow_payload,
    )
    .await;

    // Librarians will be able to end the loan on the behalf of normal users
    let (_, librarian_user) = user::create(
        &server_addr,
        &UserCreationPayload {
            email: "librarian@user.com".to_string(),
            access_mask: access_mask::LIBRARIAN,
            requester_access_token: Some(admin_access_token.to_string()),
        },
    )
    .await;
    book::end_loan(
        &server_addr,
        &librarian_user.email,
        &BookEndLoanByTitlePayload {
            title: borrow_payload.title.clone(),
            lease_id_req: borrow_payload.lease_id_req.clone(),
            access_token_req: librarian_user.access_token.to_string(),
        },
    )
    .await;

    assert_snapshot!(read_snapshot(&log_dir));
}
