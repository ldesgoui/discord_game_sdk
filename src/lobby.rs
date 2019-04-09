use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lobby {
    pub id: i64,
    pub kind: LobbyKind,
    pub owner_id: i64,
    pub secret: String,
    pub capacity: u32,
    pub locked: bool,
}

impl FromSys for Lobby {
    type Source = sys::DiscordLobby;

    fn from_sys(source: &Self::Source) -> Self {
        Self {
            id: source.id,
            kind: LobbyKind::from_sys(&source.type_),
            owner_id: source.owner_id,
            secret: unsafe { string_from_cstr(&source.secret as *const _) },
            capacity: source.capacity,
            locked: source.locked,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LobbyKind {
    Public,
    Private,
}

impl FromSys for LobbyKind {
    type Source = sys::EDiscordLobbyType;

    fn from_sys(source: &Self::Source) -> Self {
        match *source {
            sys::DiscordLobbyType_Public => LobbyKind::Public,
            sys::DiscordLobbyType_Private => LobbyKind::Private,
            _ => panic!("enum"),
        }
    }
}

impl LobbyKind {
    pub(crate) fn to_sys(self) -> sys::EDiscordLobbyType {
        match self {
            LobbyKind::Public => sys::DiscordLobbyType_Public,
            LobbyKind::Private => sys::DiscordLobbyType_Private,
        }
    }
}
