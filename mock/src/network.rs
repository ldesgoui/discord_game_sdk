use discord_game_sdk_sys as sys;
use std::os::raw::c_char;

pub unsafe extern "C" fn get_peer_id(
    manager: *mut sys::IDiscordNetworkManager,
    peer_id: *mut sys::DiscordNetworkPeerId,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn flush(manager: *mut sys::IDiscordNetworkManager) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn open_peer(
    manager: *mut sys::IDiscordNetworkManager,
    peer_id: sys::DiscordNetworkPeerId,
    route_data: *const c_char,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn update_peer(
    manager: *mut sys::IDiscordNetworkManager,
    peer_id: sys::DiscordNetworkPeerId,
    route_data: *const c_char,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn close_peer(
    manager: *mut sys::IDiscordNetworkManager,
    peer_id: sys::DiscordNetworkPeerId,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn open_channel(
    manager: *mut sys::IDiscordNetworkManager,
    peer_id: sys::DiscordNetworkPeerId,
    channel_id: sys::DiscordNetworkChannelId,
    reliable: bool,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn close_channel(
    manager: *mut sys::IDiscordNetworkManager,
    peer_id: sys::DiscordNetworkPeerId,
    channel_id: sys::DiscordNetworkChannelId,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn send_message(
    manager: *mut sys::IDiscordNetworkManager,
    peer_id: sys::DiscordNetworkPeerId,
    channel_id: sys::DiscordNetworkChannelId,
    data: *mut u8,
    data_length: u32,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}
