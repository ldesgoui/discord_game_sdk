/// On Network Message
///
/// <https://discordapp.com/developers/docs/game-sdk/networking#onmessage>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Message {
    pub peer_id: u64,
    pub chan_id: u8,
    pub buffer: Vec<u8>,
}

/// On Route Update
///
/// <https://discordapp.com/developers/docs/game-sdk/networking#onrouteupdate>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RouteUpdate {
    pub route: String,
}
