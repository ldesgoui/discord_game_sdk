#![allow(dead_code)]
#![allow(unused_variables)]

use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

mod achievement;
mod activity;
mod application;
mod core;
mod image;
mod lobby;
mod network;
mod overlay;
mod relationship;
mod storage;
mod store;
mod user;
mod voice;

#[no_mangle]
unsafe extern "C" fn DiscordCreate(
    version: sys::DiscordVersion,
    params: *mut sys::DiscordCreateParams,
    result: *mut *mut sys::IDiscordCore,
) -> sys::EDiscordResult {
    let inst = Instance {
        interfaces: INTERFACES,
        state: State {
            version,
            log_min_level: 0,
            log_hook_data: std::ptr::null_mut(),
            log_hook: None,
            overlay_locked: false,
            voice_input_mode: sys::DiscordInputMode {
                type_: sys::DiscordInputModeType_VoiceActivity,
                shortcut: [0; 256],
            },
            voice_self_mute: false,
            voice_self_deaf: false,
        },
    };

    *result = Box::into_raw(Box::new(inst)) as *mut _;

    log::trace!(
        "Instance at {:p} is {} bytes wide",
        *result,
        std::mem::size_of::<Instance>()
    );

    sys::DiscordResult_Ok
}

pub struct Instance {
    pub interfaces: Interfaces,
    pub state: State,
}

pub struct State {
    pub version: sys::DiscordVersion,
    pub log_min_level: sys::EDiscordLogLevel,
    pub log_hook_data: *mut c_void,
    pub log_hook: Option<
        unsafe extern "C" fn(
            hook_data: *mut c_void,
            level: sys::EDiscordLogLevel,
            message: *const c_char,
        ),
    >,
    pub overlay_locked: bool,
    pub voice_input_mode: sys::DiscordInputMode,
    pub voice_self_mute: bool,
    pub voice_self_deaf: bool,
}

pub struct Interfaces {
    pub core: sys::IDiscordCore,
    pub application: sys::IDiscordApplicationManager,
    pub user: sys::IDiscordUserManager,
    pub image: sys::IDiscordImageManager,
    pub activity: sys::IDiscordActivityManager,
    pub relationship: sys::IDiscordRelationshipManager,
    pub lobby: sys::IDiscordLobbyManager,
    pub network: sys::IDiscordNetworkManager,
    pub overlay: sys::IDiscordOverlayManager,
    pub storage: sys::IDiscordStorageManager,
    pub store: sys::IDiscordStoreManager,
    pub voice: sys::IDiscordVoiceManager,
    pub achievement: sys::IDiscordAchievementManager,
}

impl Instance {
    fn log(&self, message: &str, level: sys::EDiscordLogLevel) {
        log::log!(
            match level {
                1 => log::Level::Error,
                2 => log::Level::Warn,
                3 => log::Level::Info,
                4 => log::Level::Debug,
                _ => log::Level::Trace,
            },
            "{}",
            message
        );

        if self.state.log_hook.is_none() || level > self.state.log_min_level {
            return;
        }

        let c_str = std::ffi::CString::new(message).unwrap();

        unsafe {
            self.state.log_hook.unwrap()(self.state.log_hook_data, level, c_str.as_ptr());
        }
    }
}

pub const INTERFACES: Interfaces = Interfaces {
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
        validate_or_exit: Some(application::validate_or_exit),
        get_current_locale: Some(application::get_current_locale),
        get_current_branch: Some(application::get_current_branch),
        get_oauth2_token: Some(application::get_oauth2_token),
        get_ticket: Some(application::get_ticket),
    },
    user: sys::IDiscordUserManager {
        get_current_user: Some(user::get_current_user),
        get_user: Some(user::get_user),
        get_current_user_premium_type: Some(user::get_current_user_premium_type),
        current_user_has_flag: Some(user::current_user_has_flag),
    },
    image: sys::IDiscordImageManager {
        fetch: Some(image::fetch),
        get_dimensions: Some(image::get_dimensions),
        get_data: Some(image::get_data),
    },
    activity: sys::IDiscordActivityManager {
        register_command: Some(activity::register_command),
        register_steam: Some(activity::register_steam),
        update_activity: Some(activity::update_activity),
        clear_activity: Some(activity::clear_activity),
        send_request_reply: Some(activity::send_request_reply),
        send_invite: Some(activity::send_invite),
        accept_invite: Some(activity::accept_invite),
    },
    relationship: sys::IDiscordRelationshipManager {
        filter: Some(relationship::filter),
        count: Some(relationship::count),
        get: Some(relationship::get),
        get_at: Some(relationship::get_at),
    },
    lobby: sys::IDiscordLobbyManager {
        get_lobby_create_transaction: Some(lobby::get_lobby_create_transaction),
        get_lobby_update_transaction: Some(lobby::get_lobby_update_transaction),
        get_member_update_transaction: Some(lobby::get_member_update_transaction),
        create_lobby: Some(lobby::create_lobby),
        update_lobby: Some(lobby::update_lobby),
        delete_lobby: Some(lobby::delete_lobby),
        connect_lobby: Some(lobby::connect_lobby),
        connect_lobby_with_activity_secret: Some(lobby::connect_lobby_with_activity_secret),
        disconnect_lobby: Some(lobby::disconnect_lobby),
        get_lobby: Some(lobby::get_lobby),
        get_lobby_activity_secret: Some(lobby::get_lobby_activity_secret),
        get_lobby_metadata_value: Some(lobby::get_lobby_metadata_value),
        get_lobby_metadata_key: Some(lobby::get_lobby_metadata_key),
        lobby_metadata_count: Some(lobby::lobby_metadata_count),
        member_count: Some(lobby::member_count),
        get_member_user_id: Some(lobby::get_member_user_id),
        get_member_user: Some(lobby::get_member_user),
        get_member_metadata_value: Some(lobby::get_member_metadata_value),
        get_member_metadata_key: Some(lobby::get_member_metadata_key),
        member_metadata_count: Some(lobby::member_metadata_count),
        update_member: Some(lobby::update_member),
        send_lobby_message: Some(lobby::send_lobby_message),
        get_search_query: Some(lobby::get_search_query),
        search: Some(lobby::search),
        lobby_count: Some(lobby::lobby_count),
        get_lobby_id: Some(lobby::get_lobby_id),
        connect_voice: Some(lobby::connect_voice),
        disconnect_voice: Some(lobby::disconnect_voice),
        connect_network: Some(lobby::connect_network),
        disconnect_network: Some(lobby::disconnect_network),
        flush_network: Some(lobby::flush_network),
        open_network_channel: Some(lobby::open_network_channel),
        send_network_message: Some(lobby::send_network_message),
    },
    network: sys::IDiscordNetworkManager {
        get_peer_id: Some(network::get_peer_id),
        flush: Some(network::flush),
        open_peer: Some(network::open_peer),
        update_peer: Some(network::update_peer),
        close_peer: Some(network::close_peer),
        open_channel: Some(network::open_channel),
        close_channel: Some(network::close_channel),
        send_message: Some(network::send_message),
    },
    overlay: sys::IDiscordOverlayManager {
        is_enabled: Some(overlay::is_enabled),
        is_locked: Some(overlay::is_locked),
        set_locked: Some(overlay::set_locked),
        open_activity_invite: Some(overlay::open_activity_invite),
        open_guild_invite: Some(overlay::open_guild_invite),
        open_voice_settings: Some(overlay::open_voice_settings),
    },
    storage: sys::IDiscordStorageManager {
        read: Some(storage::read),
        read_async: Some(storage::read_async),
        read_async_partial: Some(storage::read_async_partial),
        write: Some(storage::write),
        write_async: Some(storage::write_async),
        delete_: Some(storage::delete_),
        exists: Some(storage::exists),
        count: Some(storage::count),
        stat: Some(storage::stat),
        stat_at: Some(storage::stat_at),
        get_path: Some(storage::get_path),
    },
    store: sys::IDiscordStoreManager {
        fetch_skus: Some(store::fetch_skus),
        count_skus: Some(store::count_skus),
        get_sku: Some(store::get_sku),
        get_sku_at: Some(store::get_sku_at),
        fetch_entitlements: Some(store::fetch_entitlements),
        count_entitlements: Some(store::count_entitlements),
        get_entitlement: Some(store::get_entitlement),
        get_entitlement_at: Some(store::get_entitlement_at),
        has_sku_entitlement: Some(store::has_sku_entitlement),
        start_purchase: Some(store::start_purchase),
    },
    voice: sys::IDiscordVoiceManager {
        get_input_mode: Some(voice::get_input_mode),
        set_input_mode: Some(voice::set_input_mode),
        is_self_mute: Some(voice::is_self_mute),
        set_self_mute: Some(voice::set_self_mute),
        is_self_deaf: Some(voice::is_self_deaf),
        set_self_deaf: Some(voice::set_self_deaf),
        is_local_mute: Some(voice::is_local_mute),
        set_local_mute: Some(voice::set_local_mute),
        get_local_volume: Some(voice::get_local_volume),
        set_local_volume: Some(voice::set_local_volume),
    },
    achievement: sys::IDiscordAchievementManager {
        set_user_achievement: Some(achievement::set_user_achievement),
        fetch_user_achievements: Some(achievement::fetch_user_achievements),
        count_user_achievements: Some(achievement::count_user_achievements),
        get_user_achievement: Some(achievement::get_user_achievement),
        get_user_achievement_at: Some(achievement::get_user_achievement_at),
    },
};

macro_rules! from_ptr {
    ($name:ident, $typ:path, $($field:tt)+) => {
        unsafe fn $name<'a>(ptr: *mut $typ) -> &'a mut Self {
            &mut *(ptr.sub(memoffset::offset_of!(Self, $($field)+)) as *mut _)
        }
    };
}

#[rustfmt::skip]
impl Instance {
    from_ptr!(from_core, sys::IDiscordCore, interfaces.core);
    from_ptr!(from_application, sys::IDiscordApplicationManager, interfaces.application);
    from_ptr!(from_user, sys::IDiscordUserManager, interfaces.user);
    from_ptr!(from_image, sys::IDiscordImageManager, interfaces.image);
    from_ptr!(from_activity, sys::IDiscordActivityManager, interfaces.activity);
    from_ptr!(from_relationship, sys::IDiscordRelationshipManager, interfaces.relationship);
    from_ptr!(from_lobby, sys::IDiscordLobbyManager, interfaces.lobby);
    from_ptr!(from_network, sys::IDiscordNetworkManager, interfaces.network);
    from_ptr!(from_overlay, sys::IDiscordOverlayManager, interfaces.overlay);
    from_ptr!(from_storage, sys::IDiscordStorageManager, interfaces.storage);
    from_ptr!(from_store, sys::IDiscordStoreManager, interfaces.store);
    from_ptr!(from_voice, sys::IDiscordVoiceManager, interfaces.voice);
    from_ptr!(from_achievement, sys::IDiscordAchievementManager, interfaces.achievement);
}
