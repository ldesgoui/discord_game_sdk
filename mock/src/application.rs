use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

/// Complete
pub unsafe extern "C" fn validate_or_exit(
    _: *mut sys::IDiscordApplicationManager,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    callback.unwrap()(callback_data, sys::DiscordResult_Ok);
}

/// Complete
pub unsafe extern "C" fn get_current_locale(
    _: *mut sys::IDiscordApplicationManager,
    locale: *mut sys::DiscordLocale,
) {
    let src = std::ffi::CString::new("en-US").unwrap();
    (*locale).clone_from_slice(std::mem::transmute(src.as_bytes_with_nul()));
}

/// Complete
pub unsafe extern "C" fn get_current_branch(
    _: *mut sys::IDiscordApplicationManager,
    branch: *mut sys::DiscordBranch,
) {
    let src = std::ffi::CString::new("mocking_test").unwrap();
    (*branch).clone_from_slice(std::mem::transmute(src.as_bytes_with_nul()));
}

pub unsafe extern "C" fn get_oauth2_token(
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

pub unsafe extern "C" fn get_ticket(
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
