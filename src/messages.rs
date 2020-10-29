use crate::entities::{Book, GetBookByTitlePayload};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::fmt::Debug;

#[derive(strum_macros::AsRefStr, strum_macros::ToString)]
pub enum ActorGroups {
    Input,
    Book,
}

macro_rules! define_message {
    ($name: ident, $reply:ty, $payload: ty) => {
        #[derive(Debug)]
        pub struct $name {
            pub reply: crossbeam_channel::Sender<$reply>,
            pub payload: $payload,
            pub db_pool: &'static sqlx::PgPool,
        }
    };
}
define_message!(GetBookByTitleMsg, Option<Book>, GetBookByTitlePayload);
pub enum BookMsg {
    GetByTitle(GetBookByTitleMsg),
}

pub static mut BOOK: OnceCell<&'static RwLock<Option<crossbeam_channel::Sender<BookMsg>>>> =
    OnceCell::new();
