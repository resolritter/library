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
        pub title: String,
    } => {
        Book => [attr(#[derive(Serialize, Debug, Default)])],
        BookSeed => [include(title)],
        GetBookByTitlePayload => [attr(#[derive(Serialize, Debug)]), include(title)]
    }
);
