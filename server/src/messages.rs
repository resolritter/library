use crate::logging::Loggable;
use entities::{Book, BookPublic, User};

#[derive(strum_macros::AsRefStr, strum_macros::ToString)]
pub enum ActorGroups {
    Input,
    Book,
    User,
}

actor_msg_resources::generate!(User, [(Create, Result<User, String>), (Login, User)]);
actor_msg_resources::generate!(
    Book,
    [
        (BorrowByTitle, String),
        (GetByTitle, Book),
        (EndBorrowByTitle, ()),
        (Create, Result<Book, String>),
        (PublicList, Vec<BookPublic>),
    ]
);

impl Loggable for BookMsg {
    fn to_log(&self) -> String {
        match self {
            BookMsg::GetByTitle(msg) => format!("{:#?}", msg.payload),
            BookMsg::BorrowByTitle(msg) => format!("{:#?}", msg.payload),
            BookMsg::EndBorrowByTitle(msg) => format!("{:#?}", msg.payload),
            BookMsg::Create(msg) => format!("{:#?}", msg.payload),
            BookMsg::PublicList(msg) => format!("{:#?}", msg.payload),
        }
    }
}

impl Loggable for UserMsg {
    fn to_log(&self) -> String {
        match self {
            UserMsg::Create(msg) => format!("{:#?}", msg.payload),
            UserMsg::Login(msg) => format!("{:#?}", msg.payload),
        }
    }
}
