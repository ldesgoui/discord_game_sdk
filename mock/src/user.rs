use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

pub(crate) unsafe extern "C" fn get_current_user(
    manager: *mut sys::IDiscordUserManager,
    current_user: *mut sys::DiscordUser,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub(crate) unsafe extern "C" fn get_user(
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
}

pub(crate) unsafe extern "C" fn get_current_user_premium_type(
    manager: *mut sys::IDiscordUserManager,
    premium_type: *mut sys::EDiscordPremiumType,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub(crate) unsafe extern "C" fn current_user_has_flag(
    manager: *mut sys::IDiscordUserManager,
    flag: sys::EDiscordUserFlag,
    has_flag: *mut bool,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}
