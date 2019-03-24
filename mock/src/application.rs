use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

pub(crate) unsafe extern "C" fn validate_or_exit(
    manager: *mut sys::IDiscordApplicationManager,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
}

pub(crate) unsafe extern "C" fn get_current_locale(
    manager: *mut sys::IDiscordApplicationManager,
    locale: *mut sys::DiscordLocale,
) {
}

pub(crate) unsafe extern "C" fn get_current_branch(
    manager: *mut sys::IDiscordApplicationManager,
    branch: *mut sys::DiscordBranch,
) {
}

pub(crate) unsafe extern "C" fn get_oauth2_token(
    manager: *mut sys::IDiscordApplicationManager,
    callback_data: *mut c_void,
    callback: Option<
        unsafe extern "C" fn(
            callback_data: *mut c_void,
            result: sys::EDiscordResult,
            oauth2_token: *mut sys::DiscordOAuth2Token,
        ),
    >,
) {
}

pub(crate) unsafe extern "C" fn get_ticket(
    manager: *mut sys::IDiscordApplicationManager,
    callback_data: *mut c_void,
    callback: Option<
        unsafe extern "C" fn(
            callback_data: *mut c_void,
            result: sys::EDiscordResult,
            data: *const c_char,
        ),
    >,
) {
}
