use crate::prelude::*;

pub unsafe extern "C" fn fetch(
    manager: *mut sys::IDiscordImageManager,
    handle: sys::DiscordImageHandle,
    refresh: bool,
    callback_data: *mut c_void,
    callback: Option<
        unsafe extern "C" fn(
            callback_data: *mut c_void,
            result: sys::EDiscordResult,
            handle_result: sys::DiscordImageHandle,
        ),
    >,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn get_dimensions(
    manager: *mut sys::IDiscordImageManager,
    handle: sys::DiscordImageHandle,
    dimensions: *mut sys::DiscordImageDimensions,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_data(
    manager: *mut sys::IDiscordImageManager,
    handle: sys::DiscordImageHandle,
    data: *mut u8,
    data_length: u32,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}
