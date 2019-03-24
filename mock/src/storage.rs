use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

pub(crate) unsafe extern "C" fn read(
    manager: *mut sys::IDiscordStorageManager,
    name: *const c_char,
    data: *mut u8,
    data_length: u32,
    read: *mut u32,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub(crate) unsafe extern "C" fn read_async(
    manager: *mut sys::IDiscordStorageManager,
    name: *const c_char,
    callback_data: *mut c_void,
    callback: Option<
        unsafe extern "C" fn(
            callback_data: *mut c_void,
            result: sys::EDiscordResult,
            data: *mut u8,
            data_length: u32,
        ),
    >,
) {
}

pub(crate) unsafe extern "C" fn read_async_partial(
    manager: *mut sys::IDiscordStorageManager,
    name: *const c_char,
    offset: u64,
    length: u64,
    callback_data: *mut c_void,
    callback: Option<
        unsafe extern "C" fn(
            callback_data: *mut c_void,
            result: sys::EDiscordResult,
            data: *mut u8,
            data_length: u32,
        ),
    >,
) {
}

pub(crate) unsafe extern "C" fn write(
    manager: *mut sys::IDiscordStorageManager,
    name: *const c_char,
    data: *mut u8,
    data_length: u32,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub(crate) unsafe extern "C" fn write_async(
    manager: *mut sys::IDiscordStorageManager,
    name: *const c_char,
    data: *mut u8,
    data_length: u32,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
}

pub(crate) unsafe extern "C" fn delete_(
    manager: *mut sys::IDiscordStorageManager,
    name: *const c_char,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub(crate) unsafe extern "C" fn exists(
    manager: *mut sys::IDiscordStorageManager,
    name: *const c_char,
    exists: *mut bool,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub(crate) unsafe extern "C" fn count(manager: *mut sys::IDiscordStorageManager, count: *mut i32) {}

pub(crate) unsafe extern "C" fn stat(
    manager: *mut sys::IDiscordStorageManager,
    name: *const c_char,
    stat: *mut sys::DiscordFileStat,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub(crate) unsafe extern "C" fn stat_at(
    manager: *mut sys::IDiscordStorageManager,
    index: i32,
    stat: *mut sys::DiscordFileStat,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub(crate) unsafe extern "C" fn get_path(
    manager: *mut sys::IDiscordStorageManager,
    path: *mut sys::DiscordPath,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}
