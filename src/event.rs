//! Event Types

use crate::{sys, Action, Activity, Entitlement, Relationship, User, UserAchievement};

/// On User Achievement Update
///
/// <https://discordapp.com/developers/docs/game-sdk/achievements#onuserachievementupdate>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserAchievementUpdate {
    pub user_achievement: UserAchievement,
}

impl From<sys::DiscordUserAchievement> for UserAchievementUpdate {
    fn from(sys: sys::DiscordUserAchievement) -> Self {
        Self {
            user_achievement: sys.into(),
        }
    }
}

/// On Activity Join
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#onactivityjoin>
#[derive(Clone, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct ActivityJoin {
    pub secret: String,
}

/// On Activity Spectate
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#onactivityspectate>
#[derive(Clone, Debug, Eq, Hash, PartialEq, derive_more::From)]
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

impl From<sys::DiscordUser> for ActivityRequest {
    fn from(sys: sys::DiscordUser) -> Self {
        Self { user: sys.into() }
    }
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

type SysActivityInvite = (
    sys::EDiscordActivityActionType,
    sys::DiscordUser,
    sys::DiscordActivity,
);

impl From<SysActivityInvite> for ActivityInvite {
    fn from((action, user, activity): SysActivityInvite) -> Self {
        Self {
            action: action.into(),
            user: user.into(),
            activity: activity.into(),
        }
    }
}

/// On Lobby Update
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onlobbyupdate>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbyUpdate {
    pub id: i64,
}

/// On Lobby Deletion
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onlobbydelete>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbyDelete {
    pub id: i64,
    pub reason: u32,
}

/// On Member Connect
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberconnect>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbyMemberConnect {
    pub id: i64,
    pub user_id: i64,
}

/// On Member Update
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberupdate>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbyMemberUpdate {
    pub id: i64,
    pub user_id: i64,
}

/// On Member Disconnect
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberdisconnect>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbyMemberDisconnect {
    pub id: i64,
    pub user_id: i64,
}

/// On Lobby Message
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onmessage>
#[derive(Clone, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbyMessage {
    pub id: i64,
    pub user_id: i64,
    pub buffer: Vec<u8>,
}

/// On Speaking
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onspeaking>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbySpeaking {
    pub id: i64,
    pub user_id: i64,
    pub speaking: bool,
}

/// On Lobby Network Message
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#onnetworkmessage>
#[derive(Clone, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbyNetworkMessage {
    pub id: i64,
    pub user_id: i64,
    pub chan_id: u8,
    pub buffer: Vec<u8>,
}

/// On Network Message
///
/// <https://discordapp.com/developers/docs/game-sdk/networking#onmessage>
#[derive(Clone, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct NetworkMessage {
    pub peer_id: u64,
    pub chan_id: u8,
    pub buffer: Vec<u8>,
}

/// On Route Update
///
/// <https://discordapp.com/developers/docs/game-sdk/networking#onrouteupdate>
#[derive(Clone, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct NetworkRouteUpdate {
    pub route: String,
}

/// On Overlay Toggle
///
/// <https://discordapp.com/developers/docs/game-sdk/overlay#ontoggle>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct OverlayToggle {
    pub opened: bool,
}

/// On Relationships Refresh
///
/// <https://discordapp.com/developers/docs/game-sdk/relationships#onrefresh>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct RelationshipsRefresh;

/// On Relationship Update
///
/// <https://discordapp.com/developers/docs/game-sdk/relationships#onrelationshipupdate>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RelationshipUpdate {
    pub relationship: Relationship,
}

impl From<sys::DiscordRelationship> for RelationshipUpdate {
    fn from(sys: sys::DiscordRelationship) -> Self {
        Self {
            relationship: sys.into(),
        }
    }
}

/// On Store Entitlement Create
///
/// <https://discordapp.com/developers/docs/game-sdk/store#onentitlementcreate>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StoreEntitlementCreate {
    pub entitlement: Entitlement,
}

impl From<sys::DiscordEntitlement> for StoreEntitlementCreate {
    fn from(sys: sys::DiscordEntitlement) -> Self {
        Self {
            entitlement: sys.into(),
        }
    }
}

/// On Store Entitlement Delete
///
/// <https://discordapp.com/developers/docs/game-sdk/store#onentitlementdelete>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StoreEntitlementDelete {
    pub entitlement: Entitlement,
}

impl From<sys::DiscordEntitlement> for StoreEntitlementDelete {
    fn from(sys: sys::DiscordEntitlement) -> Self {
        Self {
            entitlement: sys.into(),
        }
    }
}

/// On Current User Update
///
/// <https://discordapp.com/developers/docs/game-sdk/users#oncurrentuserupdate>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct CurrentUserUpdate;

/// On Voice Settings Update
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct VoiceSettingsUpdate;
