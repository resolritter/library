use serde::{Deserialize, Serialize};

pub mod access_mask {
    // 0x001 => Borrow books
    // 0x011 => Manage books + 0x01
    // 0x111 => Super user
    pub const USER: i32 = 0x001;
    pub const LIBRARIAN: i32 = 0x011;
    pub const ADMIN: i32 = 0x111;
}

type BookBorrowLength = i64;
type BookTitle = String;
type UserEmail = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub title: String,
    pub borrow_id: Option<UserEmail>,
    pub borrow_until: Option<BookBorrowLength>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BookPublic {
    pub title: String,
    pub borrow_until: Option<BookBorrowLength>,
}
#[derive(Deserialize, Debug)]
pub struct BookGetByTitlePayload {
    pub title: BookTitle,
}
#[derive(Deserialize, Debug)]
pub struct BookBorrowByTitleRequestBody {
    pub borrow_length: BookBorrowLength,
    pub borrow_id: String,
}
#[derive(Serialize, Debug)]
pub struct BookEndBorrowByTitlePayload {
    pub title: BookTitle,
    pub access_token: String,
}
#[derive(Deserialize, Debug)]
pub struct BookCreatePayloadRequestBody {
    pub title: BookTitle,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BookCreatePayload {
    pub title: BookTitle,
    pub access_token: String,
}
#[derive(Serialize, Debug)]
pub struct BookBorrowByTitlePayload {
    pub title: BookTitle,
    pub borrow_id: String,
    pub borrow_length: BookBorrowLength,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BookPublicListPayload {
    pub query: Option<String>,
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct BookOkResponse {
    pub Ok: Book,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub email: String,
    pub access_mask: i32,
    pub access_token: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreatePayload {
    pub email: String,
    pub access_mask: i32,
    pub requester_access_token: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginPayload {
    pub email: String,
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct UserOkResponse {
    pub Ok: User,
}
