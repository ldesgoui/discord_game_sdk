use crate::sys;

/// Action to take when invited or inviting to an [`Activity`]
///
/// [`Activity`]: struct.Activity.html
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Action {
    Join,
    Spectate,
    Undefined(sys::EDiscordActivityActionType),
}

#[doc(hidden)]
impl From<sys::EDiscordActivityActionType> for Action {
    fn from(source: sys::EDiscordActivityActionType) -> Self {
        match source {
            sys::DiscordActivityActionType_Join => Self::Join,
            sys::DiscordActivityActionType_Spectate => Self::Spectate,
            _ => Self::Undefined(source),
        }
    }
}

#[doc(hidden)]
impl Into<sys::EDiscordActivityActionType> for Action {
    fn into(self) -> sys::EDiscordActivityActionType {
        match self {
            Self::Join => sys::DiscordActivityActionType_Join,
            Self::Spectate => sys::DiscordActivityActionType_Spectate,
            Self::Undefined(n) => n,
        }
    }
}
