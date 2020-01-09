use crate::sys;

/// Client ID of an Application
pub type ClientID = sys::DiscordClientId;

/// Unique ID of a Lobby
pub type LobbyID = sys::DiscordLobbyId;

/// ID of a Network Channel
pub type NetworkChannelID = sys::DiscordNetworkChannelId;

/// ID of a Network Peer
pub type NetworkPeerID = sys::DiscordNetworkPeerId;

/// Unique ID across Discord
///
/// > [Snowflakes in official docs](https://discordapp.com/developers/docs/reference#snowflakes)
pub type Snowflake = sys::DiscordSnowflake;

/// UNIX Timestamp, number of seconds since 1 January 1970
pub type UnixTimestamp = sys::DiscordTimestamp;

/// ID of a User
pub type UserID = sys::DiscordUserId;
