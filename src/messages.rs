use crate::entities::BookGetMessage;

use once_cell::sync::OnceCell;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;

pub static mut BOOK: OnceCell<
  &'static Arc<&'static Mutex<Option<crossbeam_channel::Sender<BookGetMessage>>>>,
> = OnceCell::new();

#[derive(strum_macros::AsRefStr, strum_macros::ToString)]
pub enum ActorGroups {
  Input,
  Book,
}

#[derive(Debug)]
pub struct Message<'a, T, M>
where
  T: Debug,
  M: Debug,
{
  channel: crossbeam_channel::Sender<T>,
  payload: M,
  db_pool: &'a sqlx::PgPool,
}
