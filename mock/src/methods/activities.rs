use crate::prelude::*;

/// Complete
pub unsafe extern "C" fn register_command(
    _: *mut sys::IDiscordActivityManager,
    _: *const i8,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

/// Complete
pub unsafe extern "C" fn register_steam(
    _: *mut sys::IDiscordActivityManager,
    _: u32,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn update_activity(
    manager: *mut sys::IDiscordActivityManager,
    activity: *mut sys::DiscordActivity,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
    // TODO: save activity state
    // TODO: calc rate limit (5/20s)
}

pub unsafe extern "C" fn clear_activity(
    manager: *mut sys::IDiscordActivityManager,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn send_request_reply(
    manager: *mut sys::IDiscordActivityManager,
    user_id: sys::DiscordUserId,
    reply: sys::EDiscordActivityJoinRequestReply,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn send_invite(
    manager: *mut sys::IDiscordActivityManager,
    user_id: sys::DiscordUserId,
    type_: sys::EDiscordActivityActionType,
    content: *const i8,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn accept_invite(
    manager: *mut sys::IDiscordActivityManager,
    user_id: sys::DiscordUserId,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}
