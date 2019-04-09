use crate::prelude::*;

/// Complete
pub unsafe extern "C" fn is_enabled(_: *mut sys::IDiscordOverlayManager, enabled: *mut bool) {
    prevent_unwind!();

    *enabled = true;
}

/// Complete
pub unsafe extern "C" fn is_locked(manager: *mut sys::IDiscordOverlayManager, locked: *mut bool) {
    prevent_unwind!();
    let inst = Instance::from_overlay(manager);

    *locked = inst.state.overlay_opened;
}

/// Complete
pub unsafe extern "C" fn set_locked(
    manager: *mut sys::IDiscordOverlayManager,
    locked: bool,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
    let inst = Instance::from_overlay(manager);

    inst.state.overlay_opened = locked;

    inst.state.queue(1, move |i| {
        log::info!("WOW IM HNERE");
        callback.unwrap()(callback_data, sys::DiscordResult_Ok);
    })
}

/// Complete
pub unsafe extern "C" fn open_activity_invite(
    _: *mut sys::IDiscordOverlayManager,
    _: sys::EDiscordActivityActionType,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();

    callback.unwrap()(callback_data, sys::DiscordResult_Ok);
}

/// Complete
pub unsafe extern "C" fn open_guild_invite(
    _: *mut sys::IDiscordOverlayManager,
    _: *const i8,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();

    callback.unwrap()(callback_data, sys::DiscordResult_Ok);
}

/// Complete
pub unsafe extern "C" fn open_voice_settings(
    _: *mut sys::IDiscordOverlayManager,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();

    callback.unwrap()(callback_data, sys::DiscordResult_Ok);
}
