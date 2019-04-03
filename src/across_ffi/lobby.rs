use crate::prelude::*;

pub(crate) extern "C" fn on_lobby_update(event_data: *mut c_void, lobby_id: i64) {}

pub(crate) extern "C" fn on_lobby_delete(event_data: *mut c_void, lobby_id: i64, reason: u32) {}

pub(crate) extern "C" fn on_member_connect(event_data: *mut c_void, lobby_id: i64, user_id: i64) {}

pub(crate) extern "C" fn on_member_update(event_data: *mut c_void, lobby_id: i64, user_id: i64) {}

pub(crate) extern "C" fn on_member_disconnect(
    event_data: *mut c_void,
    lobby_id: i64,
    user_id: i64,
) {
}

pub(crate) extern "C" fn on_lobby_message(
    event_data: *mut c_void,
    lobby_id: i64,
    user_id: i64,
    data: *mut u8,
    data_length: u32,
) {
}

pub(crate) extern "C" fn on_speaking(
    event_data: *mut c_void,
    lobby_id: i64,
    user_id: i64,
    speaking: bool,
) {
}

pub(crate) extern "C" fn on_network_message(
    event_data: *mut c_void,
    lobby_id: i64,
    user_id: i64,
    channel_id: u8,
    data: *mut u8,
    data_length: u32,
) {
}
