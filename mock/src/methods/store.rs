use crate::prelude::*;

pub unsafe extern "C" fn fetch_skus(
    manager: *mut sys::IDiscordStoreManager,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn count_skus(manager: *mut sys::IDiscordStoreManager, count: *mut i32) {}

pub unsafe extern "C" fn get_sku(
    manager: *mut sys::IDiscordStoreManager,
    sku_id: sys::DiscordSnowflake,
    sku: *mut sys::DiscordSku,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_sku_at(
    manager: *mut sys::IDiscordStoreManager,
    index: i32,
    sku: *mut sys::DiscordSku,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn fetch_entitlements(
    manager: *mut sys::IDiscordStoreManager,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn count_entitlements(
    manager: *mut sys::IDiscordStoreManager,
    count: *mut i32,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn get_entitlement(
    manager: *mut sys::IDiscordStoreManager,
    entitlement_id: sys::DiscordSnowflake,
    entitlement: *mut sys::DiscordEntitlement,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_entitlement_at(
    manager: *mut sys::IDiscordStoreManager,
    index: i32,
    entitlement: *mut sys::DiscordEntitlement,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn has_sku_entitlement(
    manager: *mut sys::IDiscordStoreManager,
    sku_id: sys::DiscordSnowflake,
    has_entitlement: *mut bool,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

/// Complete
pub unsafe extern "C" fn start_purchase(
    _: *mut sys::IDiscordStoreManager,
    _: sys::DiscordSnowflake,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();

    callback.unwrap()(callback_data, sys::DiscordResult_Ok);
}
