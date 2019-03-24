use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

pub unsafe extern "C" fn register_command(
    manager: *mut sys::IDiscordActivityManager,
    command: *const c_char,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn register_steam(
    manager: *mut sys::IDiscordActivityManager,
    steam_id: u32,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn update_activity(
    manager: *mut sys::IDiscordActivityManager,
    activity: *mut sys::DiscordActivity,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
}

pub unsafe extern "C" fn clear_activity(
    manager: *mut sys::IDiscordActivityManager,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
}

pub unsafe extern "C" fn send_request_reply(
    manager: *mut sys::IDiscordActivityManager,
    user_id: sys::DiscordUserId,
    reply: sys::EDiscordActivityJoinRequestReply,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
}

pub unsafe extern "C" fn send_invite(
    manager: *mut sys::IDiscordActivityManager,
    user_id: sys::DiscordUserId,
    type_: sys::EDiscordActivityActionType,
    content: *const c_char,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
}

pub unsafe extern "C" fn accept_invite(
    manager: *mut sys::IDiscordActivityManager,
    user_id: sys::DiscordUserId,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
}
