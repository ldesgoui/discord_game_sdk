/// On Lobby Update
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onlobbyupdate>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Update {
    pub id: i64,
}

/// On Lobby Deletion
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onlobbydelete>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Delete {
    pub id: i64,
    pub reason: u32,
}

/// On Member Connect
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberconnect>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MemberConnect {
    pub id: i64,
    pub user_id: i64,
}

/// On Member Update
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberupdate>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MemberUpdate {
    pub id: i64,
    pub user_id: i64,
}

/// On Member Disconnect
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberdisconnect>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MemberDisconnect {
    pub id: i64,
    pub user_id: i64,
}

/// On Lobby Message
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onmessage>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Message {
    pub id: i64,
    pub user_id: i64,
    pub buffer: Vec<u8>,
}

/// On Speaking
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onspeaking>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Speaking {
    pub id: i64,
    pub user_id: i64,
    pub speaking: bool,
}

/// On Lobby Network Message
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onnetworkmessage>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NetworkMessage {
    pub id: i64,
    pub user_id: i64,
    pub chan_id: u8,
    pub buffer: Vec<u8>,
}
