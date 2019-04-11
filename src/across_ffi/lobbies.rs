use crate::prelude::*;

pub(crate) extern "C" fn on_lobby_update(senders: *mut c_void, id: i64) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .lobbies_update
        .try_send(event::lobbies::Update { id })
        .unwrap()
}

pub(crate) extern "C" fn on_lobby_delete(senders: *mut c_void, id: i64, reason: u32) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .lobbies_delete
        .try_send(event::lobbies::Delete { id, reason })
        .unwrap()
}

pub(crate) extern "C" fn on_member_connect(senders: *mut c_void, id: i64, user_id: i64) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .lobbies_member_connect
        .try_send(event::lobbies::MemberConnect { id, user_id })
        .unwrap()
}

pub(crate) extern "C" fn on_member_update(senders: *mut c_void, id: i64, user_id: i64) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .lobbies_member_update
        .try_send(event::lobbies::MemberUpdate { id, user_id })
        .unwrap()
}

pub(crate) extern "C" fn on_member_disconnect(senders: *mut c_void, id: i64, user_id: i64) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .lobbies_member_disconnect
        .try_send(event::lobbies::MemberDisconnect { id, user_id })
        .unwrap()
}

pub(crate) extern "C" fn on_lobby_message(
    senders: *mut c_void,
    id: i64,
    user_id: i64,
    data: *mut u8,
    len: u32,
) {
    prevent_unwind!();

    let buffer = unsafe { std::slice::from_raw_parts(data, len as usize) }.to_vec();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .lobbies_message
        .try_send(event::lobbies::Message {
            id,
            user_id,
            buffer,
        })
        .unwrap()
}

pub(crate) extern "C" fn on_speaking(senders: *mut c_void, id: i64, user_id: i64, speaking: bool) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .lobbies_speaking
        .try_send(event::lobbies::Speaking {
            id,
            user_id,
            speaking,
        })
        .unwrap()
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

    let buffer = unsafe { std::slice::from_raw_parts(data, len as usize) }.to_vec();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .lobbies_network_message
        .try_send(event::lobbies::NetworkMessage {
            id,
            user_id,
            chan_id,
            buffer,
        })
        .unwrap()
}
