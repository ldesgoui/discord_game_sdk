//! COMPLETE COMPLETE COMPLETE

use crate::Instance;
use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

/// Complete
pub unsafe extern "C" fn is_enabled(_: *mut sys::IDiscordOverlayManager, enabled: *mut bool) {
    *enabled = true;
}

/// Complete
pub unsafe extern "C" fn is_locked(manager: *mut sys::IDiscordOverlayManager, locked: *mut bool) {
    let inst = Instance::from_overlay(manager);

    *locked = inst.state.overlay_locked;
}

/// Complete
pub unsafe extern "C" fn set_locked(
    manager: *mut sys::IDiscordOverlayManager,
    locked: bool,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    let inst = Instance::from_overlay(manager);

    inst.state.overlay_locked = locked;

    callback.unwrap()(callback_data, sys::DiscordResult_Ok);
}

/// Complete
pub unsafe extern "C" fn open_activity_invite(
    manager: *mut sys::IDiscordOverlayManager,
    _: sys::EDiscordActivityActionType,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    let inst = Instance::from_overlay(manager);

    callback.unwrap()(
        callback_data,
        if inst.state.overlay_locked {
            sys::DiscordResult_InvalidCommand
        } else {
            sys::DiscordResult_Ok
        },
    );
}

/// Complete
pub unsafe extern "C" fn open_guild_invite(
    manager: *mut sys::IDiscordOverlayManager,
    _: *const c_char,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    let inst = Instance::from_overlay(manager);

    callback.unwrap()(
        callback_data,
        if inst.state.overlay_locked {
            sys::DiscordResult_InvalidCommand
        } else {
            sys::DiscordResult_Ok
        },
    );
}

/// Complete
pub unsafe extern "C" fn open_voice_settings(
    manager: *mut sys::IDiscordOverlayManager,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    let inst = Instance::from_overlay(manager);

    callback.unwrap()(
        callback_data,
        if inst.state.overlay_locked {
            sys::DiscordResult_InvalidCommand
        } else {
            sys::DiscordResult_Ok
        },
    );
}
