use std::fmt::Debug;

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
