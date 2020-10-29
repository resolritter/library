use bastion::prelude::BastionContext;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone)]
pub struct App {
  pub db_pool: PgPool,
  pub bastion: &'static BastionContext,
}
impl std::fmt::Debug for App {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(())
  }
}

#[derive(Clone)]
pub struct ServerState {
  pub app: &'static App,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
  pub id: String,
  pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct BookGet {
  pub title: String,
}

#[derive(Debug)]
pub struct BookGetMessage {
  pub channel: crossbeam_channel::Sender<Book>,
  pub payload: BookGet,
  pub app: &'static App,
}
