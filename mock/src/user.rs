use discord_game_sdk_sys as sys;
use std::os::raw::c_void;

pub unsafe extern "C" fn get_current_user(
    manager: *mut sys::IDiscordUserManager,
    current_user: *mut sys::DiscordUser,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_user(
    manager: *mut sys::IDiscordUserManager,
    user_id: sys::DiscordUserId,
    callback_data: *mut c_void,
    callback: Option<
        unsafe extern "C" fn(
            callback_data: *mut c_void,
            result: sys::EDiscordResult,
            user: *mut sys::DiscordUser,
        ),
    >,
) {
    prevent_unwind!();
}

/// Complete
pub unsafe extern "C" fn get_current_user_premium_type(
    _: *mut sys::IDiscordUserManager,
    premium_type: *mut sys::EDiscordPremiumType,
) -> sys::EDiscordResult {
    prevent_unwind!();
    *premium_type = sys::DiscordPremiumType_None;

    sys::DiscordResult_Ok
}

/// Complete
pub unsafe extern "C" fn current_user_has_flag(
    _: *mut sys::IDiscordUserManager,
    _: sys::EDiscordUserFlag,
    has_flag: *mut bool,
) -> sys::EDiscordResult {
    prevent_unwind!();
    *has_flag = false;

    sys::DiscordResult_Ok
}
