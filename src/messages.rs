use crate::entities::Book;
use crate::logging::Loggable;

#[derive(strum_macros::AsRefStr, strum_macros::ToString)]
pub enum ActorGroups {
    Input,
    Book,
    User,
}

resource_messaging::generate!(User, [(Creation, String)]);
resource_messaging::generate!(Book, [(LeaseByTitle, String), (GetByTitle, Book)]);

impl Loggable for BookMsg {
    fn to_log(&self) -> String {
        match self {
            BookMsg::GetByTitle(msg) => format!("{:#?}", msg.payload),
            BookMsg::LeaseByTitle(msg) => format!("{:#?}", msg.payload),
        }
    }
}
