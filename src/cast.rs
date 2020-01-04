use crate::sys;

/// Lobby Search Cast
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#data-models-lobbysearchcast-enum)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Cast {
    /// Cast the value as a number
    Number,
    /// Cast the value as a number
    String,
}

impl Into<sys::EDiscordLobbySearchCast> for Cast {
    fn into(self) -> sys::EDiscordLobbySearchCast {
        match self {
            Self::String => sys::DiscordLobbySearchCast_String,
            Self::Number => sys::DiscordLobbySearchCast_Number,
        }
    }
}
