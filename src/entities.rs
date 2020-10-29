use bastion::prelude::BastionContext;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

pub struct App {
    pub db_pool: &'static Arc<&'static PgPool>,
    pub bastion: &'static Arc<&'static BastionContext>,
}
impl std::fmt::Debug for App {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Config {
    pub db_url: String,
}

#[derive(Clone)]
pub struct ServerState {
    pub app: &'static Arc<&'static App>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct BookGet {
    pub title: String,
}

#[derive(Debug)]
pub struct BookGetMessage {
    pub channel: crossbeam_channel::Sender<Option<Book>>,
    pub payload: BookGet,
    pub app: &'static App,
}
