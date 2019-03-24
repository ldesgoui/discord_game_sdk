use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

pub(crate) unsafe extern "C" fn fetch_skus(
    manager: *mut sys::IDiscordStoreManager,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
}

pub(crate) unsafe extern "C" fn count_skus(
    manager: *mut sys::IDiscordStoreManager,
    count: *mut i32,
) {
}

pub(crate) unsafe extern "C" fn get_sku(
    manager: *mut sys::IDiscordStoreManager,
    sku_id: sys::DiscordSnowflake,
    sku: *mut sys::DiscordSku,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub(crate) unsafe extern "C" fn get_sku_at(
    manager: *mut sys::IDiscordStoreManager,
    index: i32,
    sku: *mut sys::DiscordSku,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub(crate) unsafe extern "C" fn fetch_entitlements(
    manager: *mut sys::IDiscordStoreManager,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
}

pub(crate) unsafe extern "C" fn count_entitlements(
    manager: *mut sys::IDiscordStoreManager,
    count: *mut i32,
) {
}

pub(crate) unsafe extern "C" fn get_entitlement(
    manager: *mut sys::IDiscordStoreManager,
    entitlement_id: sys::DiscordSnowflake,
    entitlement: *mut sys::DiscordEntitlement,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub(crate) unsafe extern "C" fn get_entitlement_at(
    manager: *mut sys::IDiscordStoreManager,
    index: i32,
    entitlement: *mut sys::DiscordEntitlement,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub(crate) unsafe extern "C" fn has_sku_entitlement(
    manager: *mut sys::IDiscordStoreManager,
    sku_id: sys::DiscordSnowflake,
    has_entitlement: *mut bool,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub(crate) unsafe extern "C" fn start_purchase(
    manager: *mut sys::IDiscordStoreManager,
    sku_id: sys::DiscordSnowflake,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
}
