use crate::prelude::*;

pub(crate) extern "C" fn on_message(
    event_data: *mut c_void,
    peer_id: sys::DiscordNetworkPeerId,
    channel_id: sys::DiscordNetworkChannelId,
    data: *mut u8,
    data_length: u32,
) {
}

pub(crate) extern "C" fn on_route_update(event_data: *mut c_void, route_data: *const c_char) {}
