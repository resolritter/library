use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Global {
    pub db_pool: &'static sqlx::PgPool,
}

#[derive(Debug, Clone)]
pub struct ServerState {
    pub global: &'static Global,
}

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
        GetBookByTitlePayload => [attr(#[derive(Serialize, Debug)]), include(title)],
        LeaseBookByTitleRequestBody => [attr(#[derive(Deserialize, Debug)]), include(lease_length)],
        LeaseBookByTitlePayload => [attr(#[derive(Serialize, Debug)]), include(title), include(lease_length)]
    }
);
