use crate::prelude::*;

pub(crate) extern "C" fn on_message(
    core_ptr: *mut c_void,
    peer_id: sys::DiscordNetworkPeerId,
    chan_id: sys::DiscordNetworkChannelId,
    data: *mut u8,
    len: u32,
) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    let buffer = unsafe { std::slice::from_raw_parts(data, len as usize) }.to_vec();

    core.network_channel.single_write(event::Network::Message {
        peer_id,
        chan_id,
        buffer,
    })
}

pub(crate) extern "C" fn on_route_update(core_ptr: *mut c_void, route: *const i8) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    let route = unsafe { string_from_cstr(route) };

    core.network_channel
        .single_write(event::Network::RouteUpdate { route })
}
