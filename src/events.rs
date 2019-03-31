use crate::Discord;
use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

//

pub(crate) const USER: sys::IDiscordUserEvents = sys::IDiscordUserEvents {
    on_current_user_update: Some(on_current_user_update),
};

extern "C" fn on_current_user_update(event_data: *mut c_void) {
    let core = event_data as *mut Discord;
}

//

pub(crate) const ACTIVITY: sys::IDiscordActivityEvents = sys::IDiscordActivityEvents {
    on_activity_join: Some(on_activity_join),
    on_activity_spectate: Some(on_activity_spectate),
    on_activity_join_request: Some(on_activity_join_request),
    on_activity_invite: Some(on_activity_invite),
};

extern "C" fn on_activity_join(event_data: *mut c_void, secret: *const c_char) {}

extern "C" fn on_activity_spectate(event_data: *mut c_void, secret: *const c_char) {}

extern "C" fn on_activity_join_request(event_data: *mut c_void, user: *mut sys::DiscordUser) {}

extern "C" fn on_activity_invite(
    event_data: *mut c_void,
    ty: sys::EDiscordActivityActionType,
    user: *mut sys::DiscordUser,
    activity: *mut sys::DiscordActivity,
) {
}

//

pub(crate) const RELATIONSHIP: sys::IDiscordRelationshipEvents = sys::IDiscordRelationshipEvents {
    on_refresh: Some(on_refresh),
    on_relationship_update: Some(on_relationship_update),
};

extern "C" fn on_refresh(event_data: *mut c_void) {}

extern "C" fn on_relationship_update(
    event_data: *mut c_void,
    relationship: *mut sys::DiscordRelationship,
) {
}

//

pub(crate) const LOBBY: sys::IDiscordLobbyEvents = sys::IDiscordLobbyEvents {
    on_lobby_update: Some(on_lobby_update),
    on_lobby_delete: Some(on_lobby_delete),
    on_member_connect: Some(on_member_connect),
    on_member_update: Some(on_member_update),
    on_member_disconnect: Some(on_member_disconnect),
    on_lobby_message: Some(on_lobby_message),
    on_speaking: Some(on_speaking),
    on_network_message: Some(on_network_message),
};

extern "C" fn on_lobby_update(event_data: *mut c_void, lobby_id: i64) {}

extern "C" fn on_lobby_delete(event_data: *mut c_void, lobby_id: i64, reason: u32) {}

extern "C" fn on_member_connect(event_data: *mut c_void, lobby_id: i64, user_id: i64) {}

extern "C" fn on_member_update(event_data: *mut c_void, lobby_id: i64, user_id: i64) {}

extern "C" fn on_member_disconnect(event_data: *mut c_void, lobby_id: i64, user_id: i64) {}

extern "C" fn on_lobby_message(
    event_data: *mut c_void,
    lobby_id: i64,
    user_id: i64,
    data: *mut u8,
    data_length: u32,
) {
}

extern "C" fn on_speaking(event_data: *mut c_void, lobby_id: i64, user_id: i64, speaking: bool) {}

extern "C" fn on_network_message(
    event_data: *mut c_void,
    lobby_id: i64,
    user_id: i64,
    channel_id: u8,
    data: *mut u8,
    data_length: u32,
) {
}

//

pub(crate) const NETWORK: sys::IDiscordNetworkEvents = sys::IDiscordNetworkEvents {
    on_message: Some(on_message),
    on_route_update: Some(on_route_update),
};

extern "C" fn on_message(
    event_data: *mut c_void,
    peer_id: sys::DiscordNetworkPeerId,
    channel_id: sys::DiscordNetworkChannelId,
    data: *mut u8,
    data_length: u32,
) {
}

extern "C" fn on_route_update(event_data: *mut c_void, route_data: *const c_char) {}

//

pub(crate) const OVERLAY: sys::IDiscordOverlayEvents = sys::IDiscordOverlayEvents {
    on_toggle: Some(on_toggle),
};

extern "C" fn on_toggle(event_data: *mut c_void, locked: bool) {}

//

pub(crate) const STORE: sys::IDiscordStoreEvents = sys::IDiscordStoreEvents {
    on_entitlement_create: Some(on_entitlement_create),
    on_entitlement_delete: Some(on_entitlement_delete),
};

extern "C" fn on_entitlement_create(
    event_data: *mut c_void,
    entitlement: *mut sys::DiscordEntitlement,
) {
}

extern "C" fn on_entitlement_delete(
    event_data: *mut c_void,
    entitlement: *mut sys::DiscordEntitlement,
) {
}

//

pub(crate) const VOICE: sys::IDiscordVoiceEvents = sys::IDiscordVoiceEvents {
    on_settings_update: Some(on_settings_update),
};

extern "C" fn on_settings_update(event_data: *mut c_void) {}

//

pub(crate) const ACHIEVEMENT: sys::IDiscordAchievementEvents = sys::IDiscordAchievementEvents {
    on_user_achievement_update: Some(on_user_achievement_update),
};

extern "C" fn on_user_achievement_update(
    event_data: *mut c_void,
    achievement: *mut sys::DiscordUserAchievement,
) {
}
