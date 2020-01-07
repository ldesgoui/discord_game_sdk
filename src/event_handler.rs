use crate::{
    Action, Activity, Discord, Entitlement, LobbyID, LogLevel, NetworkChannelID, NetworkPeerID,
    Relationship, User, UserAchievement, UserID,
};

pub trait EventHandler {
    fn on_log_message(&mut self, _discord: &Discord, _level: LogLevel, _message: &str) {}

    fn on_user_achievement_update(
        &mut self,
        _discord: &Discord,
        _user_achievement: &UserAchievement,
    ) {
    }

    fn on_activity_join(&mut self, _discord: &Discord, _secret: &str) {}
    fn on_activity_spectate(&mut self, _discord: &Discord, _secret: &str) {}
    fn on_activity_join_request(&mut self, _discord: &Discord, _user: &User) {}
    fn on_activity_invite(
        &mut self,
        _discord: &Discord,
        _kind: Action,
        _user: &User,
        _activity: &Activity,
    ) {
    }

    fn on_lobby_update(&mut self, _discord: &Discord, _lobby_id: LobbyID) {}
    fn on_lobby_delete(&mut self, _discord: &Discord, _lobby_id: LobbyID, _reason: u32) {}
    fn on_member_connect(&mut self, _discord: &Discord, _lobby_id: LobbyID, _member_id: UserID) {}
    fn on_member_update(&mut self, _discord: &Discord, _lobby_id: LobbyID, _member_id: UserID) {}
    fn on_member_disconnect(&mut self, _discord: &Discord, _lobby_id: LobbyID, _member_id: UserID) {
    }
    fn on_lobby_message(
        &mut self,
        _discord: &Discord,
        _lobby_id: LobbyID,
        _member_id: UserID,
        _data: &[u8],
    ) {
    }
    fn on_speaking(
        &mut self,
        _discord: &Discord,
        _lobby_id: LobbyID,
        _member_id: UserID,
        _speaking: bool,
    ) {
    }
    fn on_lobby_network_message(
        &mut self,
        _discord: &Discord,
        _lobby_id: LobbyID,
        _member_id: UserID,
        _channel_id: NetworkChannelID,
        _data: &[u8],
    ) {
    }

    fn on_network_message(
        &mut self,
        _discord: &Discord,
        _peer_id: NetworkPeerID,
        _channel_id: NetworkChannelID,
        _data: &[u8],
    ) {
    }
    fn on_network_route_update(&mut self, _discord: &Discord, _route: &str) {}

    fn on_overlay_toggle(&mut self, _discord: &Discord, _closed: bool) {}

    fn on_relationships_refresh(&mut self, _discord: &Discord) {}
    fn on_relationship_update(&mut self, _discord: &Discord, _relationship: &Relationship) {}

    fn on_entitlement_create(&mut self, _discord: &Discord, _entitlement: &Entitlement) {}
    fn on_entitlement_delete(&mut self, _discord: &Discord, _entitlement: &Entitlement) {}

    fn on_current_user_update(&mut self, _discord: &Discord) {}

    fn on_voice_settings_update(&mut self, _discord: &Discord) {}
}

#[derive(Debug, Default)]
pub(crate) struct VoidEvents;

impl EventHandler for VoidEvents {}
