use crate::{sys, LobbyKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Lobby(pub(crate) sys::DiscordLobby);

impl Lobby {
    pub fn id(&self) -> i64 {
        self.0.id
    }

    pub fn kind(&self) -> LobbyKind {
        self.0.type_.into()
    }

    pub fn owner_id(&self) -> i64 {
        self.0.owner_id
    }

    get_str!(secret, secret);

    pub fn capacity(&self) -> u32 {
        self.0.capacity
    }

    pub fn locked(&self) -> bool {
        self.0.locked
    }
}
