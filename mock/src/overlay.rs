use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

pub(crate) unsafe extern "C" fn is_enabled(
    manager: *mut sys::IDiscordOverlayManager,
    enabled: *mut bool,
) {
}

pub(crate) unsafe extern "C" fn is_locked(
    manager: *mut sys::IDiscordOverlayManager,
    locked: *mut bool,
) {
}

pub(crate) unsafe extern "C" fn set_locked(
    manager: *mut sys::IDiscordOverlayManager,
    locked: bool,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
}

pub(crate) unsafe extern "C" fn open_activity_invite(
    manager: *mut sys::IDiscordOverlayManager,
    type_: sys::EDiscordActivityActionType,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
}

pub(crate) unsafe extern "C" fn open_guild_invite(
    manager: *mut sys::IDiscordOverlayManager,
    code: *const c_char,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
}

pub(crate) unsafe extern "C" fn open_voice_settings(
    manager: *mut sys::IDiscordOverlayManager,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
}
