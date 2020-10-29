use crate::logging::Loggable;
use entities::{Book, BookPublic, User};

#[derive(strum_macros::AsRefStr, strum_macros::ToString)]
pub enum ActorGroups {
    Input,
    Book,
    User,
}

resource_messaging::generate!(User, [(Creation, User), (Login, User)]);
resource_messaging::generate!(
    Book,
    [
        (BorrowByTitle, String),
        (GetByTitle, Book),
        (EndBorrowByTitle, ()),
        (Creation, Book),
        (PublicList, Vec<BookPublic>),
    ]
);

impl Loggable for BookMsg {
    fn to_log(&self) -> String {
        match self {
            BookMsg::GetByTitle(msg) => format!("{:#?}", msg.payload),
            BookMsg::BorrowByTitle(msg) => format!("{:#?}", msg.payload),
            BookMsg::EndBorrowByTitle(msg) => format!("{:#?}", msg.payload),
            BookMsg::Creation(msg) => format!("{:#?}", msg.payload),
            BookMsg::PublicList(msg) => format!("{:#?}", msg.payload),
        }
    }
}

impl Loggable for UserMsg {
    fn to_log(&self) -> String {
        match self {
            UserMsg::Creation(msg) => format!("{:#?}", msg.payload),
            UserMsg::Login(msg) => format!("{:#?}", msg.payload),
        }
    }
}
