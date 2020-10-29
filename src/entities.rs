use serde::Serialize;

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
        pub title: String
    } => {
        Book => [attr(#[derive(Serialize, Debug, Default)])],
        BookSeed => [omit(id), omit(lease_id), omit(leased_until)],
        GetBookByTitlePayload => [attr(#[derive(Serialize, Debug)]), omit(id), omit(lease_id), omit(leased_until)]
    }
);
