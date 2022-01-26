use entities::access_mask;
use insta::assert_snapshot;
use stdext::function_name;
use surf::StatusCode;
use tempdir::TempDir;
use test_utils::{
    constants::ADMIN_ACCESS_TOKEN, format::format_test_name, read_snapshot, spawn_test_program,
    SpawnedTest,
};

#[async_std::test]
async fn test_create_and_get() {
    use entities::{book, user, BookCreatePayload, UserCreatePayload};

    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        log_dir,
        ..
    } = &spawn_test_program(&tmp_dir);

    let (_, normal_user) = user::create(
        server_addr,
        &UserCreatePayload {
            email: "normal@user.com".to_string(),
            access_mask: access_mask::USER,
            requester_access_token: None,
        },
    )
    .await;
    // A normal user is not able to create books
    let bad_unauthorized_creation = book::do_create(
        server_addr,
        &BookCreatePayload {
            access_token: normal_user.access_token.to_string(),
            title: "TEST".to_string(),
        },
    )
    .await
    .unwrap();
    assert!(bad_unauthorized_creation.status() == StatusCode::Forbidden);

    // Create a LIBRARIAN type of user for book creation
    let (_, librarian) = user::create(
        server_addr,
        &UserCreatePayload {
            email: "librarian@user.com".to_string(),
            access_mask: access_mask::LIBRARIAN,
            requester_access_token: Some(ADMIN_ACCESS_TOKEN.to_string()),
        },
    )
    .await;
    let (_, new_book) = book::create(
        server_addr,
        &BookCreatePayload {
            access_token: librarian.access_token.to_string(),
            title: "TEST".to_string(),
        },
    )
    .await;
    book::get(server_addr, &new_book.title).await;

    assert_snapshot!(read_snapshot(log_dir));
}

#[async_std::test]
async fn test_borrow() {
    use entities::{book, user, BookBorrowByTitlePayload, BookCreatePayload, UserCreatePayload};

    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        log_dir,
        ..
    } = &spawn_test_program(&tmp_dir);

    const WHOLE_DAY: i64 = 86400;
    let (_, book) = book::create(
        server_addr,
        &BookCreatePayload {
            access_token: ADMIN_ACCESS_TOKEN.to_string(),
            title: "Cinderella".to_string(),
        },
    )
    .await;
    let (_, first_user) = user::create(
        server_addr,
        &UserCreatePayload {
            email: "simple@user.com".to_string(),
            access_mask: access_mask::USER,
            requester_access_token: None,
        },
    )
    .await;
    let payload = BookBorrowByTitlePayload {
        title: book.title.to_string(),
        borrow_id: first_user.email.clone(),
        borrow_length: WHOLE_DAY,
    };

    // The route should be protected against invalid tokens
    let guest_user = book::do_borrow(server_addr, "INVALID_TOKEN", &payload)
        .await
        .unwrap();
    assert!(guest_user.status() == StatusCode::Forbidden);

    // Borrow a book for a whole day
    assert_snapshot!(
        format!("{}_good", test_name),
        book::borrow(server_addr, &first_user.access_token, &payload)
            .await
            .body_string()
            .await
            .unwrap()
    );

    // The book is already borrowed to somebody, thus the following should not work
    let bad_borrow = book::do_borrow(server_addr, &first_user.access_token, &payload)
        .await
        .unwrap();
    assert!(bad_borrow.status() == StatusCode::Forbidden);

    assert_snapshot!(read_snapshot(log_dir));
}

#[async_std::test]
async fn test_end_borrow() {
    use entities::{
        book, user, BookBorrowByTitlePayload, BookCreatePayload, BookEndBorrowByTitlePayload,
        UserCreatePayload,
    };

    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        log_dir,
        ..
    } = &spawn_test_program(&tmp_dir);

    const WHOLE_DAY: i64 = 86400;
    let (_, book) = book::create(
        server_addr,
        &BookCreatePayload {
            access_token: ADMIN_ACCESS_TOKEN.to_string(),
            title: "Cinderella".to_string(),
        },
    )
    .await;
    let (_, first_user) = user::create(
        server_addr,
        &UserCreatePayload {
            email: "first@user.com".to_string(),
            access_mask: access_mask::USER,
            requester_access_token: None,
        },
    )
    .await;
    let (_, second_user) = user::create(
        server_addr,
        &UserCreatePayload {
            email: "second@user.com".to_string(),
            access_mask: access_mask::USER,
            requester_access_token: None,
        },
    )
    .await;
    let mut borrow_payload = BookBorrowByTitlePayload {
        title: book.title.to_string(),
        borrow_id: first_user.email.clone(),
        borrow_length: WHOLE_DAY,
    };

    // Borrow a book with the first user
    book::borrow(server_addr, &first_user.access_token, &borrow_payload).await;

    // The second user won't be able to end the borrow on behalf of the first
    let mut end_borrow_payload = BookEndBorrowByTitlePayload {
        title: borrow_payload.title.clone(),
        access_token: second_user.access_token.to_string(),
    };
    let bad_forbidden_end_borrow = book::do_end_borrow(server_addr, &end_borrow_payload)
        .await
        .unwrap();
    assert!(bad_forbidden_end_borrow.status() == StatusCode::Forbidden);

    // The first user can end the borrow on its own
    end_borrow_payload = BookEndBorrowByTitlePayload {
        access_token: first_user.access_token.to_string(),
        ..end_borrow_payload
    };
    book::end_borrow(server_addr, &end_borrow_payload).await;

    // Now the book is free and the second user will be able to borrow it
    borrow_payload = BookBorrowByTitlePayload {
        borrow_id: second_user.email.clone(),
        ..borrow_payload
    };
    book::borrow(server_addr, &second_user.access_token, &borrow_payload).await;

    // Librarians will be able to end the borrow on the behalf of normal users
    let (_, librarian_user) = user::create(
        server_addr,
        &UserCreatePayload {
            email: "librarian@user.com".to_string(),
            access_mask: access_mask::LIBRARIAN,
            requester_access_token: Some(ADMIN_ACCESS_TOKEN.to_string()),
        },
    )
    .await;
    book::end_borrow(
        server_addr,
        &BookEndBorrowByTitlePayload {
            title: borrow_payload.title.clone(),
            access_token: librarian_user.access_token.to_string(),
        },
    )
    .await;

    assert_snapshot!(read_snapshot(log_dir));
}

#[async_std::test]
async fn test_list() {
    use entities::{book, BookCreatePayload};

    let test_name = format_test_name(function_name!());
    let tmp_dir = TempDir::new(&test_name).unwrap();
    let SpawnedTest {
        server_addr,
        log_dir,
        ..
    } = &spawn_test_program(&tmp_dir);

    let (_, book) = book::create(
        server_addr,
        &BookCreatePayload {
            access_token: ADMIN_ACCESS_TOKEN.to_string(),
            title: "Cinderella".to_string(),
        },
    )
    .await;
    let (_, books) = book::list(server_addr, None).await;
    assert!(books.len() == 1);

    let (_, filtered_books) = book::list(server_addr, Some(&book.title)).await;
    assert!(filtered_books.len() == 1);
    for book in filtered_books {
        assert!(book.title.contains(&book.title));
    }

    let (_, no_matching_books) = book::list(server_addr, Some("DOES_NOT_EXIST")).await;
    assert!(no_matching_books.is_empty());

    assert_snapshot!(read_snapshot(log_dir));
}
