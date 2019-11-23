use crate::{
    sys,
    utils::{charbuf_len, charbuf_to_str},
    LobbyKind,
};

/// Lobby
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#data-models-lobby-struct>
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Lobby {
    pub(crate) sys: sys::DiscordLobby,
    secret_len: usize,
}

impl Lobby {
    /// The unique ID of the lobby
    pub fn id(&self) -> i64 {
        self.sys.id
    }

    /// What sort of lobby it is
    pub fn kind(&self) -> LobbyKind {
        self.sys.type_.into()
    }

    /// The unique ID of the user owning the lobby
    pub fn owner_id(&self) -> i64 {
        self.sys.owner_id
    }

    /// The password to the lobby
    pub fn secret(&self) -> &str {
        charbuf_to_str(&self.sys.secret[..self.secret_len])
    }

    /// The maximum number of players that can join
    pub fn capacity(&self) -> u32 {
        self.sys.capacity
    }

    /// Whether the lobby can be joined or not
    pub fn locked(&self) -> bool {
        self.sys.locked
    }
}

impl From<sys::DiscordLobby> for Lobby {
    fn from(sys: sys::DiscordLobby) -> Self {
        Self {
            sys,
            secret_len: charbuf_len(&sys.secret),
        }
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
