//! Event Types

use crate::{sys, Action, Activity, Entitlement, Relationship, User, UserAchievement};

/// On User Achievement Update
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/achievements#onuserachievementupdate)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserAchievementUpdate {
    /// The achievement that was updated
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
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#onactivityjoin)
#[derive(Clone, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct ActivityJoin {
    /// The secret to join the user's game
    pub secret: String,
}

/// On Activity Spectate
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#onactivityspectate)
#[derive(Clone, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct ActivitySpectate {
    /// The secret to join the user's game as a spectator
    pub secret: String,
}

/// On Activity Join Request
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#onactivityjoinrequest)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActivityRequest {
    /// The user asking to join
    pub user: User,
}

impl From<sys::DiscordUser> for ActivityRequest {
    fn from(sys: sys::DiscordUser) -> Self {
        Self { user: sys.into() }
    }
}

/// On Activity Invitation
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#onactivityinvite)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActivityInvite {
    /// Whether the invitation is to play or spectate
    pub action: Action,
    /// The user sending the inviation
    pub user: User,
    /// The inviting user's current activity
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
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onlobbyupdate)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbyUpdate {
    /// ID of the lobby
    pub id: i64,
}

/// On Lobby Deletion
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onlobbydelete)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbyDelete {
    /// ID of the lobby
    pub id: i64,
    /// Reason for deletion (XXX: supposedly a missing enum definition)
    pub reason: u32,
}

/// On Member Connect
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberconnect)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbyMemberConnect {
    /// ID of the lobby
    pub id: i64,
    /// ID of the member
    pub user_id: i64,
}

/// On Member Update
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberupdate)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbyMemberUpdate {
    /// ID of the lobby
    pub id: i64,
    /// ID of the member
    pub user_id: i64,
}

/// On Member Disconnect
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberdisconnect)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbyMemberDisconnect {
    /// ID of the lobby
    pub id: i64,
    /// ID of the member
    pub user_id: i64,
}

/// On Lobby Message
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onlobbymessage)
#[derive(Clone, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbyMessage {
    /// ID of the lobby
    pub id: i64,
    /// ID of the member
    pub user_id: i64,
    /// The contents of the message
    pub buffer: Vec<u8>,
}

/// On Speaking
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onspeaking)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbySpeaking {
    /// ID of the lobby
    pub id: i64,
    /// ID of the member
    pub user_id: i64,
    /// Whether the member is currently transmitting voice
    pub speaking: bool,
}

/// On Lobby Network Message
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onnetworkmessage)
#[derive(Clone, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct LobbyNetworkMessage {
    /// ID of the lobby
    pub id: i64,
    /// ID of the member
    pub user_id: i64,
    /// The channel the message was sent on
    pub chan_id: u8,
    /// The contents of the message
    pub buffer: Vec<u8>,
}

/// On Network Message
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/networking#onmessage)
#[derive(Clone, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct NetworkMessage {
    /// The peer ID of the sender
    pub peer_id: u64,
    /// The channel the message was sent on
    pub chan_id: u8,
    /// The contents of the message
    pub buffer: Vec<u8>,
}

/// On Route Update
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/networking#onrouteupdate)
#[derive(Clone, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct NetworkRouteUpdate {
    /// The new route to the current user
    pub route: String,
}

/// On Overlay Toggle
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/overlay#ontoggle)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct OverlayToggle {
    /// Whether the overlay is open or closed
    pub closed: bool,
}

/// On Relationships Refresh
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/relationships#onrefresh)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct RelationshipsRefresh;

/// On Relationship Update
///
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/relationships#onrelationshipupdate)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RelationshipUpdate {
    /// The relationship that updated
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
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#onentitlementcreate)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StoreEntitlementCreate {
    /// The entitlement that the user was granted
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
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#onentitlementdelete)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StoreEntitlementDelete {
    /// The entitlement that the user has lost
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
/// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/users#oncurrentuserupdate)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct CurrentUserUpdate;

/// On Voice Settings Update
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, derive_more::From)]
pub struct VoiceSettingsUpdate;
