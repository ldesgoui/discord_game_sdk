use crate::{
    event,
    panic_messages::{NOT_UTF8, NULL_PTR, SEND_FAIL},
};
use std::ffi::c_void;

pub(crate) extern "C" fn on_lobby_update(senders: *mut c_void, id: i64) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .lobbies_update
        .try_send(event::lobbies::Update { id })
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn on_lobby_delete(senders: *mut c_void, id: i64, reason: u32) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .lobbies_delete
        .try_send(event::lobbies::Delete { id, reason })
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn on_member_connect(senders: *mut c_void, id: i64, user_id: i64) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .lobbies_member_connect
        .try_send(event::lobbies::MemberConnect { id, user_id })
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn on_member_update(senders: *mut c_void, id: i64, user_id: i64) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .lobbies_member_update
        .try_send(event::lobbies::MemberUpdate { id, user_id })
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn on_member_disconnect(senders: *mut c_void, id: i64, user_id: i64) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .lobbies_member_disconnect
        .try_send(event::lobbies::MemberDisconnect { id, user_id })
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn on_lobby_message(
    senders: *mut c_void,
    id: i64,
    user_id: i64,
    data: *mut u8,
    len: u32,
) {
    prevent_unwind!();

    debug_assert!(!data.is_null());

    let buffer = unsafe { std::slice::from_raw_parts(data, len as usize) }.to_vec();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .lobbies_message
        .try_send(event::lobbies::Message {
            id,
            user_id,
            buffer,
        })
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn on_speaking(senders: *mut c_void, id: i64, user_id: i64, speaking: bool) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .lobbies_speaking
        .try_send(event::lobbies::Speaking {
            id,
            user_id,
            speaking,
        })
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn on_network_message(
    senders: *mut c_void,
    id: i64,
    user_id: i64,
    chan_id: u8,
    data: *mut u8,
    len: u32,
) {
    prevent_unwind!();

    debug_assert!(!data.is_null());

    let buffer = unsafe { std::slice::from_raw_parts(data, len as usize) }.to_vec();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .lobbies_network_message
        .try_send(event::lobbies::NetworkMessage {
            id,
            user_id,
            chan_id,
            buffer,
        })
        .expect(SEND_FAIL)
}
