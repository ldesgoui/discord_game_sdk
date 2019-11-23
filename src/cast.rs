use crate::sys;

/// Lobby Search Cast
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#data-models-lobbysearchcast-enum>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Cast {
    Number,
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
