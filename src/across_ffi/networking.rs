use crate::prelude::*;

pub(crate) extern "C" fn on_message(
    senders: *mut c_void,
    peer_id: sys::DiscordNetworkPeerId,
    chan_id: sys::DiscordNetworkChannelId,
    data: *mut u8,
    len: u32,
) {
    let buffer = unsafe { std::slice::from_raw_parts(data, len as usize) }.to_vec();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .networking_message
        .try_send(event::networking::Message {
            peer_id,
            chan_id,
            buffer,
        })
        .unwrap()
}

pub(crate) extern "C" fn on_route_update(senders: *mut c_void, route: *const i8) {
    let route = unsafe { string_from_cstr(route) };

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .networking_route_update
        .try_send(event::networking::RouteUpdate { route })
        .unwrap()
}
