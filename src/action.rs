use crate::prelude::*;

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Join,
    Spectate,
}

impl Action {
    pub(crate) fn to_sys(self) -> sys::EDiscordActivityActionType {
        match self {
            Action::Join => sys::DiscordActivityActionType_Join,
            Action::Spectate => sys::DiscordActivityActionType_Spectate,
        }
    }
}
