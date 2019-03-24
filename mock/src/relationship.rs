use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

pub unsafe extern "C" fn filter(
    manager: *mut sys::IDiscordRelationshipManager,
    filter_data: *mut c_void,
    filter: Option<
        unsafe extern "C" fn(
            filter_data: *mut c_void,
            relationship: *mut sys::DiscordRelationship,
        ) -> bool,
    >,
) {
}

pub unsafe extern "C" fn count(
    manager: *mut sys::IDiscordRelationshipManager,
    count: *mut i32,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get(
    manager: *mut sys::IDiscordRelationshipManager,
    user_id: sys::DiscordUserId,
    relationship: *mut sys::DiscordRelationship,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_at(
    manager: *mut sys::IDiscordRelationshipManager,
    index: u32,
    relationship: *mut sys::DiscordRelationship,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}
