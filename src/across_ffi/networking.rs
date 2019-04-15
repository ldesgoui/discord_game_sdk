use crate::{
    event,
    panic_messages::{NOT_UTF8, NULL_PTR, SEND_FAIL},
    sys,
};
use std::ffi::{c_void, CStr};

pub(crate) extern "C" fn on_message(
    senders: *mut c_void,
    peer_id: sys::DiscordNetworkPeerId,
    chan_id: sys::DiscordNetworkChannelId,
    data: *mut u8,
    len: u32,
) {
    prevent_unwind!();

    debug_assert!(!data.is_null());

    let buffer = unsafe { std::slice::from_raw_parts(data, len as usize) }.to_vec();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .networking_message
        .try_send(event::networking::Message {
            peer_id,
            chan_id,
            buffer,
        })
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn on_route_update(senders: *mut c_void, route: *const i8) {
    prevent_unwind!();

    debug_assert!(!route.is_null());

    let route = unsafe { CStr::from_ptr(route) }
        .to_str()
        .expect(NOT_UTF8)
        .to_string();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .networking_route_update
        .try_send(event::networking::RouteUpdate { route })
        .expect(SEND_FAIL)
}
