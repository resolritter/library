use serde::{Deserialize, Serialize};

pub mod access_mask {
    // 0x001 => Lease books
    // 0x011 => Manage books + 0x01
    // 0x111 => Super user
    pub const USER: i32 = 0x001;
    pub const LIBRARIAN: i32 = 0x011;
    pub const ADMIN: i32 = 0x111;
}

type LeaseBookId = String;

structout::generate!(
    pub <> {
        pub id: i32,
        pub title: String,
        pub lease_id: Option<LeaseBookId>,
        pub lease_id_req: LeaseBookId,
        pub lease_until: Option<i64>,
        pub lease_length: i64,
        pub access_token_req: String,
    } => {
        Book => [attr(#[derive(Serialize, Debug)]), omit(lease_length), omit(lease_id_req), omit(access_token_req)],
        BookSeed => [include(title)],
        BookGetByTitlePayload => [attr(#[derive(Serialize, Debug)]), include(title)],
        BookLeaseByTitleRequestBody => [attr(#[derive(Deserialize, Debug)]), include(lease_length), include(lease_id_req)],
        BookEndLoanByTitlePayload => [attr(#[derive(Serialize, Deserialize, Debug)]), include(title), include(lease_id_req), include(access_token_req)],
        BookLeaseByTitlePayload => [attr(#[derive(Serialize, Debug)]), include(title), include(lease_id_req), include(lease_length)]
    }
);

structout::generate!(
    pub <> {
        pub email: LeaseBookId,
        pub access_mask: i32,
        pub access_token: String,
        pub requester_access_token: Option<String>,
    } => {
        UserPublic => [attr(#[derive(Serialize, Deserialize, Debug)]), omit(requester_access_token)],
        UserCreationPayload => [attr(#[derive(Serialize, Deserialize, Debug)]), omit(access_token)],
    }
);
