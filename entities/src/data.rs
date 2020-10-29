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
    #[derive(Serialize, Deserialize, Debug)]
    pub {
        pub id: i32,
        pub title: String,
        pub lease_id: Option<LeaseBookId>,
        pub lease_until: Option<i64>,
        pub lease_length: i64,
    } => {
        Book => [omit(lease_length)],
        BookGetByTitlePayload => [include(title)],
        BookLeaseByTitleRequestBody => [include(lease_length), upsert(pub lease_id: LeaseBookId)],
        BookEndLoanByTitlePayload => [include(title), upsert(pub lease_id: LeaseBookId, pub access_token: String)],
        BookLeaseByTitlePayload => [include(title, lease_length), upsert(pub lease_id: LeaseBookId)]
    }
);

structout::generate!(
    #[derive(Serialize, Deserialize, Debug)]
    pub {
        pub email: LeaseBookId,
        pub access_mask: i32,
        pub access_token: String,
    } => {
        UserPublic => [],
        UserCreationPayload => [omit(access_token), upsert(pub requester_access_token: Option<String>)],
    }
);
