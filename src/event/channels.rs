use crate::prelude::*;

#[derive(Clone, Debug)]
pub(crate) struct Senders {
    pub(crate) activities_join: Sender<event::activities::Join>,
    pub(crate) activities_spectate: Sender<event::activities::Spectate>,
    pub(crate) activities_request: Sender<event::activities::Request>,
    pub(crate) activities_invite: Sender<event::activities::Invite>,
    pub(crate) lobbies_update: Sender<event::lobbies::Update>,
    pub(crate) lobbies_delete: Sender<event::lobbies::Delete>,
    pub(crate) lobbies_member_connect: Sender<event::lobbies::MemberConnect>,
    pub(crate) lobbies_member_update: Sender<event::lobbies::MemberUpdate>,
    pub(crate) lobbies_member_disconnect: Sender<event::lobbies::MemberDisconnect>,
    pub(crate) lobbies_message: Sender<event::lobbies::Message>,
    pub(crate) lobbies_speaking: Sender<event::lobbies::Speaking>,
    pub(crate) lobbies_network_message: Sender<event::lobbies::NetworkMessage>,
    pub(crate) networking_message: Sender<event::networking::Message>,
    pub(crate) networking_route_update: Sender<event::networking::RouteUpdate>,
    pub(crate) overlay_toggle: Sender<event::overlay::Toggle>,
    pub(crate) relationships_refresh: Sender<event::relationships::Refresh>,
    pub(crate) relationships_update: Sender<event::relationships::Update>,
    pub(crate) store_entitlement_create: Sender<event::store::EntitlementCreate>,
    pub(crate) store_entitlement_delete: Sender<event::store::EntitlementDelete>,
    pub(crate) current_user_update: Sender<event::users::CurrentUserUpdate>,
    pub(crate) voice_settings_update: Sender<event::voice::SettingsUpdate>,
}

#[derive(Clone, Debug)]
pub struct Receivers {
    pub activities_join: Receiver<event::activities::Join>,
    pub activities_spectate: Receiver<event::activities::Spectate>,
    pub activities_request: Receiver<event::activities::Request>,
    pub activities_invite: Receiver<event::activities::Invite>,
    pub lobbies_update: Receiver<event::lobbies::Update>,
    pub lobbies_delete: Receiver<event::lobbies::Delete>,
    pub lobbies_member_connect: Receiver<event::lobbies::MemberConnect>,
    pub lobbies_member_update: Receiver<event::lobbies::MemberUpdate>,
    pub lobbies_member_disconnect: Receiver<event::lobbies::MemberDisconnect>,
    pub lobbies_message: Receiver<event::lobbies::Message>,
    pub lobbies_speaking: Receiver<event::lobbies::Speaking>,
    pub lobbies_network_message: Receiver<event::lobbies::NetworkMessage>,
    pub networking_message: Receiver<event::networking::Message>,
    pub networking_route_update: Receiver<event::networking::RouteUpdate>,
    pub overlay_toggle: Receiver<event::overlay::Toggle>,
    pub relationships_refresh: Receiver<event::relationships::Refresh>,
    pub relationships_update: Receiver<event::relationships::Update>,
    pub store_entitlement_create: Receiver<event::store::EntitlementCreate>,
    pub store_entitlement_delete: Receiver<event::store::EntitlementDelete>,
    pub current_user_update: Receiver<event::users::CurrentUserUpdate>,
    pub voice_settings_update: Receiver<event::voice::SettingsUpdate>,
}

impl Receivers {
    pub fn empty_channels(&self) -> std::result::Result<(), crossbeam_channel::RecvError> {
        loop {
            crossbeam_channel::select! {
                recv(self.activities_join) -> e => { e?; },
                recv(self.activities_spectate) -> e => { e?; },
                recv(self.activities_request) -> e => { e?; },
                recv(self.activities_invite) -> e => { e?; },
                recv(self.lobbies_update) -> e => { e?; },
                recv(self.lobbies_delete) -> e => { e?; },
                recv(self.lobbies_member_connect) -> e => { e?; },
                recv(self.lobbies_member_update) -> e => { e?; },
                recv(self.lobbies_member_disconnect) -> e => { e?; },
                recv(self.lobbies_message) -> e => { e?; },
                recv(self.lobbies_speaking) -> e => { e?; },
                recv(self.lobbies_network_message) -> e => { e?; },
                recv(self.networking_message) -> e => { e?; },
                recv(self.networking_route_update) -> e => { e?; },
                recv(self.overlay_toggle) -> e => { e?; },
                recv(self.relationships_refresh) -> e => { e?; },
                recv(self.relationships_update) -> e => { e?; },
                recv(self.store_entitlement_create) -> e => { e?; },
                recv(self.store_entitlement_delete) -> e => { e?; },
                recv(self.current_user_update) -> e => { e?; },
                recv(self.voice_settings_update) -> e => { e?; },
                default => return Ok(())
            }
        }
    }
}

pub(crate) fn create_channels() -> (Senders, Receivers) {
    use crossbeam_channel::unbounded;

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
