use crate::entities::{Book, UserPublic};
use crate::logging::Loggable;

#[derive(strum_macros::AsRefStr, strum_macros::ToString)]
pub enum ActorGroups {
    Input,
    Book,
    User,
}

resource_messaging::generate!(User, [(Creation, UserPublic)]);
resource_messaging::generate!(Book, [(LeaseByTitle, String), (GetByTitle, Book)]);

impl Loggable for BookMsg {
    fn to_log(&self) -> String {
        match self {
            BookMsg::GetByTitle(msg) => format!("{:#?}", msg.payload),
            BookMsg::LeaseByTitle(msg) => format!("{:#?}", msg.payload),
        }
    }
}

impl Loggable for UserMsg {
    fn to_log(&self) -> String {
        match self {
            UserMsg::Creation(msg) => format!("{:#?}", msg.payload),
        }
    }
}