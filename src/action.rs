use crate::{panic_messages::INVALID_ENUM, sys};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Action {
    Join,
    Spectate,
}

impl From<sys::EDiscordActivityActionType> for Action {
    fn from(source: sys::EDiscordActivityActionType) -> Self {
        match source {
            sys::DiscordActivityActionType_Join => Action::Join,
            sys::DiscordActivityActionType_Spectate => Action::Spectate,
            _ => panic!(INVALID_ENUM),
        }
    }
}

impl Into<sys::EDiscordActivityActionType> for Action {
    fn into(self) -> sys::EDiscordActivityActionType {
        match self {
            Action::Join => sys::DiscordActivityActionType_Join,
            Action::Spectate => sys::DiscordActivityActionType_Spectate,
        }
    }
}
