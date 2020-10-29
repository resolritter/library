use serde::{Deserialize, Serialize};

pub mod access_mask {
    // 0x001 => Borrow books
    // 0x011 => Manage books + 0x01
    // 0x111 => Super user
    pub const USER: i32 = 0x001;
    pub const LIBRARIAN: i32 = 0x011;
    pub const ADMIN: i32 = 0x111;
}

type BorrowBookId = String;
type BookBorrowLength = i64;

structout::generate!(
    #[derive(Serialize, Deserialize, Debug)]
    pub {
        pub title: String,
        pub borrow_id: Option<BorrowBookId>,
        pub borrow_until: Option<BookBorrowLength>,
        pub borrow_length: BookBorrowLength,
    } => {
        Book => [omit(borrow_length)],
        // 'borrow_id' refers to the current borrower of the book;
        // of course, it should be hidden for the general public
        BookPublic => [omit(borrow_id, borrow_length)],
        BookGetByTitlePayload => [include(title)],
        BookBorrowByTitleRequestBody => [include(borrow_length), upsert(pub borrow_id: BorrowBookId)],
        BookEndBorrowByTitlePayload => [include(title), upsert(pub access_token: String)],
        BookCreatePayloadRequestBody => [include(title)],
        BookCreatePayload => [include(title), upsert(pub access_token: String)],
        BookBorrowByTitlePayload => [include(title, borrow_length), upsert(pub borrow_id: BorrowBookId)]
    }
);
#[derive(Serialize, Deserialize, Debug)]
pub struct BookPublicListPayload {
    pub query: Option<String>,
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct BookOkResponse {
    pub Ok: Book,
}

structout::generate!(
    #[derive(Serialize, Deserialize, Debug)]
    pub {
        pub email: BorrowBookId,
        pub access_mask: i32,
        pub access_token: String,
    } => {
        User => [],
        UserCreatePayload => [omit(access_token), upsert(pub requester_access_token: Option<String>)],
        // FIXME auth should support password as well
        UserLoginPayload => [include(email)],
    }
);
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct UserOkResponse {
    pub Ok: User,
}
