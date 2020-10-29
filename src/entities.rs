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

#[derive(Serialize, Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
}

#[derive(Debug)]
pub struct BookGet {
    pub title: String,
}

#[derive(Debug)]
pub struct BookGetMessage {
    pub reply: crossbeam_channel::Sender<Option<Book>>,
    pub payload: BookGet,
    pub db_pool: &'static sqlx::PgPool,
}
