use crate::{
    Action, Activity, Discord, Entitlement, LobbyID, NetworkChannelID, NetworkPeerID, Relationship,
    User, UserAchievement, UserID,
};

#[allow(unused_variables)]
/// Trait providing callbacks for the SDK.
///
/// All methods have a default empty implementation.
pub trait EventHandler: Sized {
    /// Fired when an User Achievement is updated
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/achievements#onuserachievementupdate)
    fn on_user_achievement_update(
        &mut self,
        discord: &Discord<Self>,
        user_achievement: &UserAchievement,
    ) {
    }

    /// Fired when the current user accepts an invitation to join in chat or receives confirmation from Asking to Join.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#onactivityjoin)
    fn on_activity_join(&mut self, discord: &Discord<Self>, secret: &str) {}

    /// Fired when the current user accepts an invitation to spectate in chat
    /// or clicks the Spectate button on another user's profile.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#onactivityspectate)
    fn on_activity_spectate(&mut self, discord: &Discord<Self>, secret: &str) {}

    /// Fires when a user asks to join the game of the current user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#onactivityjoinrequest)
    fn on_activity_join_request(&mut self, discord: &Discord<Self>, user: &User) {}

    /// Fires when the current user receives an invitation to join or spectate.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#onactivityinvite)
    fn on_activity_invite(
        &mut self,
        discord: &Discord<Self>,
        kind: Action,
        user: &User,
        activity: &Activity,
    ) {
    }

    /// Fires when a lobby is updated.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onlobbyupdate)
    fn on_lobby_update(&mut self, discord: &Discord<Self>, lobby_id: LobbyID) {}

    /// Fired when a lobby is deleted.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onlobbydelete)
    fn on_lobby_delete(&mut self, discord: &Discord<Self>, lobby_id: LobbyID, reason: u32) {}

    /// Fires when a member joins the lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberconnect)
    fn on_member_connect(&mut self, discord: &Discord<Self>, lobby_id: LobbyID, member_id: UserID) {
    }

    /// Fires when data for a lobby member is updated.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberupdate)
    fn on_member_update(&mut self, discord: &Discord<Self>, lobby_id: LobbyID, member_id: UserID) {}

    /// Fires when a member leaves the lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onmemberdisconnect)
    fn on_member_disconnect(
        &mut self,
        discord: &Discord<Self>,
        lobby_id: LobbyID,
        member_id: UserID,
    ) {
    }

    /// Fires when a message is sent to the lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onlobbymessage)
    fn on_lobby_message(
        &mut self,
        discord: &Discord<Self>,
        lobby_id: LobbyID,
        member_id: UserID,
        data: &[u8],
    ) {
    }

    /// Fires when a user connected to voice starts or stops speaking.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onspeaking)
    fn on_speaking(
        &mut self,
        discord: &Discord<Self>,
        lobby_id: LobbyID,
        member_id: UserID,
        speaking: bool,
    ) {
    }

    /// Fires when the user receives a message from the lobby's networking layer.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#onnetworkmessage)
    fn on_lobby_network_message(
        &mut self,
        discord: &Discord<Self>,
        lobby_id: LobbyID,
        member_id: UserID,
        channel_id: NetworkChannelID,
        data: &[u8],
    ) {
    }

    /// Fires when you receive data from another user.
    ///
    /// This callback will only fire if you already have an open channel with the user sending you data.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/networking#onmessage)
    fn on_network_message(
        &mut self,
        discord: &Discord<Self>,
        peer_id: NetworkPeerID,
        channel_id: NetworkChannelID,
        data: &[u8],
    ) {
    }

    /// Fires when your networking route has changed.
    ///
    /// You should broadcast this change to other users.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/networking#onrouteupdate)
    fn on_network_route_update(&mut self, discord: &Discord<Self>, route: &str) {}

    /// Fires when the overlay is opened or closed.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/overlay#ontoggle)
    fn on_overlay_toggle(&mut self, discord: &Discord<Self>, closed: bool) {}

    /// Fires at initialization when Discord<Self> has cached a snapshot of all your relationships.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/relationships#onrefresh)
    fn on_relationships_refresh(&mut self, discord: &Discord<Self>) {}

    /// Fires when a relationship in the filtered list changes, like an updated presence or user attribute.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/relationships#onrelationshipupdate)
    fn on_relationship_update(&mut self, discord: &Discord<Self>, relationship: &Relationship) {}

    /// Fires when the connected user receives a new entitlement, either through purchase or through a developer grant.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#onentitlementcreate)
    fn on_entitlement_create(&mut self, discord: &Discord<Self>, entitlement: &Entitlement) {}

    /// Fires when the connected user loses an entitlement, either by expiration, revocation,
    /// or consumption in the case of consumable entitlements.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#onentitlementdelete)
    fn on_entitlement_delete(&mut self, discord: &Discord<Self>, entitlement: &Entitlement) {}

    /// Fires when the User struct of the currently connected user changes.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/users#oncurrentuserupdate)
    fn on_current_user_update(&mut self, discord: &Discord<Self>) {}

    /// Fires when the current user has updated their voice settings.
    fn on_voice_settings_update(&mut self, discord: &Discord<Self>) {}
}

/// Empty implementation
impl EventHandler for () {}
