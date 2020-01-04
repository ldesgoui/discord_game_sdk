use crate::sys;

/// Activity Action
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#data-models-activityactiontype-enum>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Action {
    /// Invite to join a game
    Join,
    /// Invite to spectate a game
    Spectate,
    /// Safety net for missing definitions
    Undefined(sys::EDiscordActivityActionType),
}

impl From<sys::EDiscordActivityActionType> for Action {
    fn from(source: sys::EDiscordActivityActionType) -> Self {
        match source {
            sys::DiscordActivityActionType_Join => Self::Join,
            sys::DiscordActivityActionType_Spectate => Self::Spectate,
            _ => Self::Undefined(source),
        }
    }
}

impl Into<sys::EDiscordActivityActionType> for Action {
    fn into(self) -> sys::EDiscordActivityActionType {
        match self {
            Self::Join => sys::DiscordActivityActionType_Join,
            Self::Spectate => sys::DiscordActivityActionType_Spectate,
            Self::Undefined(n) => n,
        }
    }
}
