#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use discord_game_sdk_sys as sys;
use log::{debug, error, info, log, trace, warn};

//

mod core;

//

#[no_mangle]
unsafe extern "C" fn DiscordCreate(
    version: sys::DiscordVersion,
    params: *mut sys::DiscordCreateParams,
    result: *mut *mut sys::IDiscordCore,
) -> sys::EDiscordResult {
    *result = &mut INTERFACES.core as *mut _;

    sys::DiscordResult_Ok
}

//

pub(crate) struct Interfaces {
    core: sys::IDiscordCore,
    application: sys::IDiscordApplicationManager,
    user: sys::IDiscordUserManager,
    image: sys::IDiscordImageManager,
    activity: sys::IDiscordActivityManager,
    relationship: sys::IDiscordRelationshipManager,
    lobby: sys::IDiscordLobbyManager,
    network: sys::IDiscordNetworkManager,
    overlay: sys::IDiscordOverlayManager,
    storage: sys::IDiscordStorageManager,
    store: sys::IDiscordStoreManager,
    voice: sys::IDiscordVoiceManager,
    achievement: sys::IDiscordAchievementManager,
}

pub(crate) static mut INTERFACES: Interfaces = Interfaces {
    core: sys::IDiscordCore {
        destroy: Some(core::destroy),
        run_callbacks: Some(core::run_callbacks),
        set_log_hook: Some(core::set_log_hook),
        get_application_manager: Some(core::get_application_manager),
        get_user_manager: Some(core::get_user_manager),
        get_image_manager: Some(core::get_image_manager),
        get_activity_manager: Some(core::get_activity_manager),
        get_relationship_manager: Some(core::get_relationship_manager),
        get_lobby_manager: Some(core::get_lobby_manager),
        get_network_manager: Some(core::get_network_manager),
        get_overlay_manager: Some(core::get_overlay_manager),
        get_storage_manager: Some(core::get_storage_manager),
        get_store_manager: Some(core::get_store_manager),
        get_voice_manager: Some(core::get_voice_manager),
        get_achievement_manager: Some(core::get_achievement_manager),
    },
    application: sys::IDiscordApplicationManager {
        validate_or_exit: None,
        get_current_locale: None,
        get_current_branch: None,
        get_oauth2_token: None,
        get_ticket: None,
    },
    user: sys::IDiscordUserManager {
        get_current_user: None,
        get_user: None,
        get_current_user_premium_type: None,
        current_user_has_flag: None,
    },
    image: sys::IDiscordImageManager {
        fetch: None,
        get_dimensions: None,
        get_data: None,
    },
    activity: sys::IDiscordActivityManager {
        register_command: None,
        register_steam: None,
        update_activity: None,
        clear_activity: None,
        send_request_reply: None,
        send_invite: None,
        accept_invite: None,
    },
    relationship: sys::IDiscordRelationshipManager {
        filter: None,
        count: None,
        get: None,
        get_at: None,
    },
    lobby: sys::IDiscordLobbyManager {
        get_lobby_create_transaction: None,
        get_lobby_update_transaction: None,
        get_member_update_transaction: None,
        create_lobby: None,
        update_lobby: None,
        delete_lobby: None,
        connect_lobby: None,
        connect_lobby_with_activity_secret: None,
        disconnect_lobby: None,
        get_lobby: None,
        get_lobby_activity_secret: None,
        get_lobby_metadata_value: None,
        get_lobby_metadata_key: None,
        lobby_metadata_count: None,
        member_count: None,
        get_member_user_id: None,
        get_member_user: None,
        get_member_metadata_value: None,
        get_member_metadata_key: None,
        member_metadata_count: None,
        update_member: None,
        send_lobby_message: None,
        get_search_query: None,
        search: None,
        lobby_count: None,
        get_lobby_id: None,
        connect_voice: None,
        disconnect_voice: None,
        connect_network: None,
        disconnect_network: None,
        flush_network: None,
        open_network_channel: None,
        send_network_message: None,
    },
    network: sys::IDiscordNetworkManager {
        get_peer_id: None,
        flush: None,
        open_peer: None,
        update_peer: None,
        close_peer: None,
        open_channel: None,
        close_channel: None,
        send_message: None,
    },
    overlay: sys::IDiscordOverlayManager {
        is_enabled: None,
        is_locked: None,
        set_locked: None,
        open_activity_invite: None,
        open_guild_invite: None,
        open_voice_settings: None,
    },
    storage: sys::IDiscordStorageManager {
        read: None,
        read_async: None,
        read_async_partial: None,
        write: None,
        write_async: None,
        delete_: None,
        exists: None,
        count: None,
        stat: None,
        stat_at: None,
        get_path: None,
    },
    store: sys::IDiscordStoreManager {
        fetch_skus: None,
        count_skus: None,
        get_sku: None,
        get_sku_at: None,
        fetch_entitlements: None,
        count_entitlements: None,
        get_entitlement: None,
        get_entitlement_at: None,
        has_sku_entitlement: None,
        start_purchase: None,
    },
    voice: sys::IDiscordVoiceManager {
        get_input_mode: None,
        set_input_mode: None,
        is_self_mute: None,
        set_self_mute: None,
        is_self_deaf: None,
        set_self_deaf: None,
        is_local_mute: None,
        set_local_mute: None,
        get_local_volume: None,
        set_local_volume: None,
    },
    achievement: sys::IDiscordAchievementManager {
        set_user_achievement: None,
        fetch_user_achievements: None,
        count_user_achievements: None,
        get_user_achievement: None,
        get_user_achievement_at: None,
    },
};
