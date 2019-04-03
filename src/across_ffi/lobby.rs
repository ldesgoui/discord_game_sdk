use crate::prelude::*;

pub(crate) extern "C" fn on_lobby_update(core_ptr: *mut c_void, id: i64) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    core.lobby_channel.single_write(event::Lobby::Update { id })
}

pub(crate) extern "C" fn on_lobby_delete(core_ptr: *mut c_void, id: i64, reason: u32) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    core.lobby_channel
        .single_write(event::Lobby::Delete { id, reason })
}

pub(crate) extern "C" fn on_member_connect(core_ptr: *mut c_void, id: i64, user_id: i64) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    core.lobby_channel
        .single_write(event::Lobby::MemberConnect { id, user_id })
}

pub(crate) extern "C" fn on_member_update(core_ptr: *mut c_void, id: i64, user_id: i64) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    core.lobby_channel
        .single_write(event::Lobby::MemberUpdate { id, user_id })
}

pub(crate) extern "C" fn on_member_disconnect(core_ptr: *mut c_void, id: i64, user_id: i64) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    core.lobby_channel
        .single_write(event::Lobby::MemberDisconnect { id, user_id })
}

pub(crate) extern "C" fn on_lobby_message(
    core_ptr: *mut c_void,
    id: i64,
    user_id: i64,
    data: *mut u8,
    len: u32,
) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    let buffer = unsafe { std::slice::from_raw_parts(data, len as usize) }.to_vec();

    core.lobby_channel.single_write(event::Lobby::Message {
        id,
        user_id,
        buffer,
    })
}

pub(crate) extern "C" fn on_speaking(core_ptr: *mut c_void, id: i64, user_id: i64, speaking: bool) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    core.lobby_channel.single_write(event::Lobby::Speaking {
        id,
        user_id,
        speaking,
    })
}

pub(crate) extern "C" fn on_network_message(
    core_ptr: *mut c_void,
    id: i64,
    user_id: i64,
    chan_id: u8,
    data: *mut u8,
    len: u32,
) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    let buffer = unsafe { std::slice::from_raw_parts(data, len as usize) }.to_vec();

    core.lobby_channel
        .single_write(event::Lobby::NetworkMessage {
            id,
            user_id,
            chan_id,
            buffer,
        })
}
