use serde::{Deserialize, Serialize};

structout::generate!(
    pub <> {
        pub id: i32,
        pub title: String,
        pub lease_id: Option<i32>,
        pub lease_until: Option<i64>,
        pub lease_length: i64
    } => {
        Book => [attr(#[derive(Serialize, Debug)]), omit(lease_length)],
        BookSeed => [include(title)],
        BookGetByTitlePayload => [attr(#[derive(Serialize, Debug)]), include(title)],
        BookLeaseByTitleRequestBody => [attr(#[derive(Deserialize, Debug)]), include(lease_length)],
        BookLeaseByTitlePayload => [attr(#[derive(Serialize, Debug)]), include(title), include(lease_length)]
    }
);

structout::generate!(
    pub <> {
        pub email: String,
        pub access_level: i32
    } => {
        UserPublic => [attr(#[derive(Serialize, Debug)])],
        UserCreationPayload => [attr(#[derive(Deserialize, Debug)])],
    }
);
