use crate::{sys, utils::charbuf_to_str, LobbyID, LobbyKind, UserID};

/// Lobby
///
/// > [Struct in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#data-models-lobby-struct)
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Lobby(pub(crate) sys::DiscordLobby);

impl Lobby {
    /// The unique ID of the lobby
    pub fn id(&self) -> LobbyID {
        self.0.id
    }

    /// What sort of lobby it is
    pub fn kind(&self) -> LobbyKind {
        self.0.type_.into()
    }

    /// The unique ID of the user owning the lobby
    pub fn owner_id(&self) -> UserID {
        self.0.owner_id
    }

    /// The password to the lobby
    pub fn secret(&self) -> &str {
        charbuf_to_str(&self.0.secret)
    }

    /// The maximum number of players that can join
    pub fn capacity(&self) -> u32 {
        self.0.capacity
    }

    /// Whether the lobby can be joined or not
    pub fn locked(&self) -> bool {
        self.0.locked
    }
}

impl std::fmt::Debug for Lobby {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Lobby")
            .field("id", &self.id())
            .field("kind", &self.kind())
            .field("owner_id", &self.owner_id())
            .field("secret", &self.secret())
            .field("capacity", &self.capacity())
            .field("locked", &self.locked())
            .finish()
    }
}
