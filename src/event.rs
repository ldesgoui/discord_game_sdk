//! Event Types

use crate::{Action, Activity, Entitlement, Relationship, User, UserAchievement};

/// On User Achievement Update
///
/// <https://discordapp.com/developers/docs/game-sdk/achievements#onuserachievementupdate>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserAchievementUpdate {
    pub user_achievement: UserAchievement,
}

/// On Activity Join
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#onactivityjoin>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActivityJoin {
    pub secret: String,
}

/// On Activity Spectate
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#onactivityspectate>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActivitySpectate {
    pub secret: String,
}

/// On Activity Join Request
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#onactivityjoinrequest>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActivityRequest {
    pub user: User,
}

/// On Activity Invitation
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#onactivityinvite>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActivityInvite {
    pub action: Action,
    pub user: User,
    pub activity: Activity,
}

/// On Lobby Update
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onlobbyupdate>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LobbyUpdate {
    pub id: i64,
}

/// On Lobby Deletion
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onlobbydelete>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LobbyDelete {
    pub id: i64,
    pub reason: u32,
}

/// On Member Connect
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberconnect>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LobbyMemberConnect {
    pub id: i64,
    pub user_id: i64,
}

/// On Member Update
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberupdate>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LobbyMemberUpdate {
    pub id: i64,
    pub user_id: i64,
}

/// On Member Disconnect
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberdisconnect>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LobbyMemberDisconnect {
    pub id: i64,
    pub user_id: i64,
}

/// On Lobby Message
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onmessage>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LobbyMessage {
    pub id: i64,
    pub user_id: i64,
    pub buffer: Vec<u8>,
}

/// On Speaking
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onspeaking>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LobbySpeaking {
    pub id: i64,
    pub user_id: i64,
    pub speaking: bool,
}

/// On Lobby Network Message
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onnetworkmessage>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LobbyNetworkMessage {
    pub id: i64,
    pub user_id: i64,
    pub chan_id: u8,
    pub buffer: Vec<u8>,
}

/// On Network Message
///
/// <https://discordapp.com/developers/docs/game-sdk/networking#onmessage>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NetworkMessage {
    pub peer_id: u64,
    pub chan_id: u8,
    pub buffer: Vec<u8>,
}

/// On Route Update
///
/// <https://discordapp.com/developers/docs/game-sdk/networking#onrouteupdate>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NetworkRouteUpdate {
    pub route: String,
}

/// On Overlay Toggle
///
/// <https://discordapp.com/developers/docs/game-sdk/overlay#ontoggle>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OverlayToggle {
    pub opened: bool,
}

/// On Relationships Refresh
///
/// <https://discordapp.com/developers/docs/game-sdk/relationships#onrefresh>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RelationshipsRefresh;

/// On Relationship Update
///
/// <https://discordapp.com/developers/docs/game-sdk/relationships#onrelationshipupdate>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RelationshipUpdate {
    pub relationship: Relationship,
}

/// On Store Entitlement Create
///
/// <https://discordapp.com/developers/docs/game-sdk/store#onentitlementcreate>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StoreEntitlementCreate {
    pub entitlement: Entitlement,
}

/// On Store Entitlement Delete
///
/// <https://discordapp.com/developers/docs/game-sdk/store#onentitlementdelete>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StoreEntitlementDelete {
    pub entitlement: Entitlement,
}

/// On Current User Update
///
/// <https://discordapp.com/developers/docs/game-sdk/users#oncurrentuserupdate>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CurrentUserUpdate;

/// On Voice Settings Update
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct VoiceSettingsUpdate;
