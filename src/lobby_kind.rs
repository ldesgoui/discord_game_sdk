use crate::{panic_messages::INVALID_ENUM, sys};

/// Lobby Type
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#data-models-lobbytype-enum>
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum LobbyKind {
    Public,
    Private,
}

#[doc(hidden)]
impl From<sys::EDiscordLobbyType> for LobbyKind {
    fn from(source: sys::EDiscordLobbyType) -> Self {
        match source {
            sys::DiscordLobbyType_Public => LobbyKind::Public,
            sys::DiscordLobbyType_Private => LobbyKind::Private,
            _ => panic!(INVALID_ENUM),
        }
    }
}

#[doc(hidden)]
impl Into<sys::EDiscordLobbyType> for LobbyKind {
    fn into(self) -> sys::EDiscordLobbyType {
        match self {
            LobbyKind::Public => sys::DiscordLobbyType_Public,
            LobbyKind::Private => sys::DiscordLobbyType_Private,
        }
    }
}
