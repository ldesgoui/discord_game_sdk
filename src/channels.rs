use crate::event;
use crossbeam_channel::{Receiver, Sender};

#[derive(Clone, Debug, derive_more::AsRef)]
pub(crate) struct Senders {
    pub(crate) achievements_update: Sender<event::UserAchievementUpdate>,
    pub(crate) activities_join: Sender<event::ActivityJoin>,
    pub(crate) activities_spectate: Sender<event::ActivitySpectate>,
    pub(crate) activities_request: Sender<event::ActivityRequest>,
    pub(crate) activities_invite: Sender<event::ActivityInvite>,
    pub(crate) lobbies_update: Sender<event::LobbyUpdate>,
    pub(crate) lobbies_delete: Sender<event::LobbyDelete>,
    pub(crate) lobbies_member_connect: Sender<event::LobbyMemberConnect>,
    pub(crate) lobbies_member_update: Sender<event::LobbyMemberUpdate>,
    pub(crate) lobbies_member_disconnect: Sender<event::LobbyMemberDisconnect>,
    pub(crate) lobbies_message: Sender<event::LobbyMessage>,
    pub(crate) lobbies_speaking: Sender<event::LobbySpeaking>,
    pub(crate) lobbies_network_message: Sender<event::LobbyNetworkMessage>,
    pub(crate) networking_message: Sender<event::NetworkMessage>,
    pub(crate) networking_route_update: Sender<event::NetworkRouteUpdate>,
    pub(crate) overlay_toggle: Sender<event::OverlayToggle>,
    pub(crate) relationships_refresh: Sender<event::RelationshipsRefresh>,
    pub(crate) relationships_update: Sender<event::RelationshipUpdate>,
    pub(crate) store_entitlement_create: Sender<event::StoreEntitlementCreate>,
    pub(crate) store_entitlement_delete: Sender<event::StoreEntitlementDelete>,
    pub(crate) current_user_update: Sender<event::CurrentUserUpdate>,
    pub(crate) voice_settings_update: Sender<event::VoiceSettingsUpdate>,
}

#[derive(Clone, Debug)]
pub(crate) struct Receivers {
    pub(crate) achievements_update: Receiver<event::UserAchievementUpdate>,
    pub(crate) activities_join: Receiver<event::ActivityJoin>,
    pub(crate) activities_spectate: Receiver<event::ActivitySpectate>,
    pub(crate) activities_request: Receiver<event::ActivityRequest>,
    pub(crate) activities_invite: Receiver<event::ActivityInvite>,
    pub(crate) lobbies_update: Receiver<event::LobbyUpdate>,
    pub(crate) lobbies_delete: Receiver<event::LobbyDelete>,
    pub(crate) lobbies_member_connect: Receiver<event::LobbyMemberConnect>,
    pub(crate) lobbies_member_update: Receiver<event::LobbyMemberUpdate>,
    pub(crate) lobbies_member_disconnect: Receiver<event::LobbyMemberDisconnect>,
    pub(crate) lobbies_message: Receiver<event::LobbyMessage>,
    pub(crate) lobbies_speaking: Receiver<event::LobbySpeaking>,
    pub(crate) lobbies_network_message: Receiver<event::LobbyNetworkMessage>,
    pub(crate) networking_message: Receiver<event::NetworkMessage>,
    pub(crate) networking_route_update: Receiver<event::NetworkRouteUpdate>,
    pub(crate) overlay_toggle: Receiver<event::OverlayToggle>,
    pub(crate) relationships_refresh: Receiver<event::RelationshipsRefresh>,
    pub(crate) relationships_update: Receiver<event::RelationshipUpdate>,
    pub(crate) store_entitlement_create: Receiver<event::StoreEntitlementCreate>,
    pub(crate) store_entitlement_delete: Receiver<event::StoreEntitlementDelete>,
    pub(crate) current_user_update: Receiver<event::CurrentUserUpdate>,
    pub(crate) voice_settings_update: Receiver<event::VoiceSettingsUpdate>,
}

impl Receivers {
    pub(crate) fn empty_channels(&self) {
        self.achievements_update.try_iter().for_each(drop);
        self.activities_join.try_iter().for_each(drop);
        self.activities_spectate.try_iter().for_each(drop);
        self.activities_request.try_iter().for_each(drop);
        self.activities_invite.try_iter().for_each(drop);
        self.lobbies_update.try_iter().for_each(drop);
        self.lobbies_delete.try_iter().for_each(drop);
        self.lobbies_member_connect.try_iter().for_each(drop);
        self.lobbies_member_update.try_iter().for_each(drop);
        self.lobbies_member_disconnect.try_iter().for_each(drop);
        self.lobbies_message.try_iter().for_each(drop);
        self.lobbies_speaking.try_iter().for_each(drop);
        self.lobbies_network_message.try_iter().for_each(drop);
        self.networking_message.try_iter().for_each(drop);
        self.networking_route_update.try_iter().for_each(drop);
        self.overlay_toggle.try_iter().for_each(drop);
        self.relationships_refresh.try_iter().for_each(drop);
        self.relationships_update.try_iter().for_each(drop);
        self.store_entitlement_create.try_iter().for_each(drop);
        self.store_entitlement_delete.try_iter().for_each(drop);
        self.current_user_update.try_iter().for_each(drop);
        self.voice_settings_update.try_iter().for_each(drop);
    }
}

pub(crate) fn create_channels() -> (Senders, Receivers) {
    use crossbeam_channel::unbounded;

    let achievements_update = unbounded();
    let activities_join = unbounded();
    let activities_spectate = unbounded();
    let activities_request = unbounded();
    let activities_invite = unbounded();
    let lobbies_update = unbounded();
    let lobbies_delete = unbounded();
    let lobbies_member_connect = unbounded();
    let lobbies_member_update = unbounded();
    let lobbies_member_disconnect = unbounded();
    let lobbies_message = unbounded();
    let lobbies_speaking = unbounded();
    let lobbies_network_message = unbounded();
    let networking_message = unbounded();
    let networking_route_update = unbounded();
    let overlay_toggle = unbounded();
    let relationships_refresh = unbounded();
    let relationships_update = unbounded();
    let store_entitlement_create = unbounded();
    let store_entitlement_delete = unbounded();
    let current_user_update = unbounded();
    let voice_settings_update = unbounded();

    (
        Senders {
            achievements_update: achievements_update.0,
            activities_join: activities_join.0,
            activities_spectate: activities_spectate.0,
            activities_request: activities_request.0,
            activities_invite: activities_invite.0,
            lobbies_update: lobbies_update.0,
            lobbies_delete: lobbies_delete.0,
            lobbies_member_connect: lobbies_member_connect.0,
            lobbies_member_update: lobbies_member_update.0,
            lobbies_member_disconnect: lobbies_member_disconnect.0,
            lobbies_message: lobbies_message.0,
            lobbies_speaking: lobbies_speaking.0,
            lobbies_network_message: lobbies_network_message.0,
            networking_message: networking_message.0,
            networking_route_update: networking_route_update.0,
            overlay_toggle: overlay_toggle.0,
            relationships_refresh: relationships_refresh.0,
            relationships_update: relationships_update.0,
            store_entitlement_create: store_entitlement_create.0,
            store_entitlement_delete: store_entitlement_delete.0,
            current_user_update: current_user_update.0,
            voice_settings_update: voice_settings_update.0,
        },
        Receivers {
            achievements_update: achievements_update.1,
            activities_join: activities_join.1,
            activities_spectate: activities_spectate.1,
            activities_request: activities_request.1,
            activities_invite: activities_invite.1,
            lobbies_update: lobbies_update.1,
            lobbies_delete: lobbies_delete.1,
            lobbies_member_connect: lobbies_member_connect.1,
            lobbies_member_update: lobbies_member_update.1,
            lobbies_member_disconnect: lobbies_member_disconnect.1,
            lobbies_message: lobbies_message.1,
            lobbies_speaking: lobbies_speaking.1,
            lobbies_network_message: lobbies_network_message.1,
            networking_message: networking_message.1,
            networking_route_update: networking_route_update.1,
            overlay_toggle: overlay_toggle.1,
            relationships_refresh: relationships_refresh.1,
            relationships_update: relationships_update.1,
            store_entitlement_create: store_entitlement_create.1,
            store_entitlement_delete: store_entitlement_delete.1,
            current_user_update: current_user_update.1,
            voice_settings_update: voice_settings_update.1,
        },
    )
}
