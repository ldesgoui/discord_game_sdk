use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

/// Complete
pub unsafe extern "C" fn validate_or_exit(
    _: *mut sys::IDiscordApplicationManager,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
    callback.unwrap()(callback_data, sys::DiscordResult_Ok);
}

/// Complete
pub unsafe extern "C" fn get_current_locale(
    _: *mut sys::IDiscordApplicationManager,
    locale: *mut sys::DiscordLocale,
) {
    prevent_unwind!();
    (*locale)[0] = 'e' as i8;
    (*locale)[1] = 'n' as i8;
    (*locale)[2] = '-' as i8;
    (*locale)[3] = 'U' as i8;
    (*locale)[4] = 'S' as i8;
}

/// Complete
pub unsafe extern "C" fn get_current_branch(
    _: *mut sys::IDiscordApplicationManager,
    branch: *mut sys::DiscordBranch,
) {
    prevent_unwind!();
    (*branch)[0] = 'm' as i8;
    (*branch)[1] = 'o' as i8;
    (*branch)[2] = 'c' as i8;
    (*branch)[3] = 'k' as i8;
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
    prevent_unwind!();
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
    prevent_unwind!();
}
