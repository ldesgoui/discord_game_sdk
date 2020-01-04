use crate::sys;

/// Lobby Type
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#data-models-lobbytype-enum)
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum LobbyKind {
    /// Lobby is public
    Public,
    /// Lobby is private (cannot be joined through matchmaking)
    Private,
    /// Safety net for missing definitions
    Undefined(sys::EDiscordLobbyType),
}

impl From<sys::EDiscordLobbyType> for LobbyKind {
    fn from(source: sys::EDiscordLobbyType) -> Self {
        match source {
            sys::DiscordLobbyType_Public => Self::Public,
            sys::DiscordLobbyType_Private => Self::Private,
            _ => Self::Undefined(source),
        }
    }
}

impl Into<sys::EDiscordLobbyType> for LobbyKind {
    fn into(self) -> sys::EDiscordLobbyType {
        match self {
            Self::Public => sys::DiscordLobbyType_Public,
            Self::Private => sys::DiscordLobbyType_Private,
            Self::Undefined(n) => n,
        }
    }
}
