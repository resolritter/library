use serde::Serialize;

#[derive(Debug)]
pub struct Global {
    pub db_url: &'static str,
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
        Book => [attr(#[derive(Serialize, Debug)])],
        GetBookByTitlePayload => [omit(id), attr(#[derive(Serialize, Debug)])]
    }
);
