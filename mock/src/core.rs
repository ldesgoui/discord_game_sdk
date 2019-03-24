use crate::INTERFACES;
use discord_game_sdk_sys as sys;
use log::{debug, error, info, log, trace, warn};
use std::os::raw::{c_char, c_void};

pub(crate) unsafe extern "C" fn destroy(_: *mut sys::IDiscordCore) {}

pub(crate) unsafe extern "C" fn run_callbacks(_: *mut sys::IDiscordCore) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub(crate) unsafe extern "C" fn set_log_hook(
    _: *mut sys::IDiscordCore,
    min_level: sys::EDiscordLogLevel,
    hook_data: *mut c_void,
    hook: Option<
        unsafe extern "C" fn(
            hook_data: *mut c_void,
            level: sys::EDiscordLogLevel,
            message: *const c_char,
        ),
    >,
) {
    [
        sys::DiscordLogLevel_Debug,
        sys::DiscordLogLevel_Info,
        sys::DiscordLogLevel_Warn,
        sys::DiscordLogLevel_Error,
    ]
    .iter()
    .filter(|&&l| l <= min_level)
    .for_each(|&l| {
        let c_str =
            std::ffi::CString::new(format!("testing log message with level {:?}", l)).unwrap();
        hook.unwrap()(hook_data, l, c_str.as_ptr())
    });
}

pub(crate) unsafe extern "C" fn get_application_manager(
    _: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordApplicationManager {
    &mut INTERFACES.application as *mut _
}

pub(crate) unsafe extern "C" fn get_user_manager(
    _: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordUserManager {
    &mut INTERFACES.user as *mut _
}

pub(crate) unsafe extern "C" fn get_image_manager(
    _: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordImageManager {
    &mut INTERFACES.image as *mut _
}

pub(crate) unsafe extern "C" fn get_activity_manager(
    _: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordActivityManager {
    &mut INTERFACES.activity as *mut _
}

pub(crate) unsafe extern "C" fn get_relationship_manager(
    _: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordRelationshipManager {
    &mut INTERFACES.relationship as *mut _
}

pub(crate) unsafe extern "C" fn get_lobby_manager(
    _: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordLobbyManager {
    &mut INTERFACES.lobby as *mut _
}

pub(crate) unsafe extern "C" fn get_network_manager(
    _: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordNetworkManager {
    &mut INTERFACES.network as *mut _
}

pub(crate) unsafe extern "C" fn get_overlay_manager(
    _: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordOverlayManager {
    &mut INTERFACES.overlay as *mut _
}

pub(crate) unsafe extern "C" fn get_storage_manager(
    _: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordStorageManager {
    &mut INTERFACES.storage as *mut _
}

pub(crate) unsafe extern "C" fn get_store_manager(
    _: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordStoreManager {
    &mut INTERFACES.store as *mut _
}

pub(crate) unsafe extern "C" fn get_voice_manager(
    _: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordVoiceManager {
    &mut INTERFACES.voice as *mut _
}

pub(crate) unsafe extern "C" fn get_achievement_manager(
    _: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordAchievementManager {
    &mut INTERFACES.achievement as *mut _
}
