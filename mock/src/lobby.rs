use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

pub unsafe extern "C" fn get_lobby_create_transaction(
    manager: *mut sys::IDiscordLobbyManager,
    transaction: *mut *mut sys::IDiscordLobbyTransaction,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_lobby_update_transaction(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    transaction: *mut *mut sys::IDiscordLobbyTransaction,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_member_update_transaction(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    user_id: sys::DiscordUserId,
    transaction: *mut *mut sys::IDiscordLobbyMemberTransaction,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn create_lobby(
    manager: *mut sys::IDiscordLobbyManager,
    transaction: *mut sys::IDiscordLobbyTransaction,
    callback_data: *mut c_void,
    callback: Option<
        unsafe extern "C" fn(
            callback_data: *mut c_void,
            result: sys::EDiscordResult,
            lobby: *mut sys::DiscordLobby,
        ),
    >,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn update_lobby(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    transaction: *mut sys::IDiscordLobbyTransaction,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn delete_lobby(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn connect_lobby(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    secret: *mut c_char,
    callback_data: *mut c_void,
    callback: Option<
        unsafe extern "C" fn(
            callback_data: *mut c_void,
            result: sys::EDiscordResult,
            lobby: *mut sys::DiscordLobby,
        ),
    >,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn connect_lobby_with_activity_secret(
    manager: *mut sys::IDiscordLobbyManager,
    activity_secret: *mut c_char,
    callback_data: *mut c_void,
    callback: Option<
        unsafe extern "C" fn(
            callback_data: *mut c_void,
            result: sys::EDiscordResult,
            lobby: *mut sys::DiscordLobby,
        ),
    >,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn disconnect_lobby(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn get_lobby(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    lobby: *mut sys::DiscordLobby,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_lobby_activity_secret(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    secret: *mut sys::DiscordLobbySecret,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_lobby_metadata_value(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    key: *mut c_char,
    value: *mut sys::DiscordMetadataValue,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_lobby_metadata_key(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    index: i32,
    key: *mut sys::DiscordMetadataKey,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn lobby_metadata_count(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    count: *mut i32,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn member_count(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    count: *mut i32,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_member_user_id(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    index: i32,
    user_id: *mut sys::DiscordUserId,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_member_user(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    user_id: sys::DiscordUserId,
    user: *mut sys::DiscordUser,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_member_metadata_value(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    user_id: sys::DiscordUserId,
    key: *mut c_char,
    value: *mut sys::DiscordMetadataValue,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_member_metadata_key(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    user_id: sys::DiscordUserId,
    index: i32,
    key: *mut sys::DiscordMetadataKey,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn member_metadata_count(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    user_id: sys::DiscordUserId,
    count: *mut i32,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn update_member(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    user_id: sys::DiscordUserId,
    transaction: *mut sys::IDiscordLobbyMemberTransaction,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn send_lobby_message(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    data: *mut u8,
    data_length: u32,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn get_search_query(
    manager: *mut sys::IDiscordLobbyManager,
    query: *mut *mut sys::IDiscordLobbySearchQuery,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn search(
    manager: *mut sys::IDiscordLobbyManager,
    query: *mut sys::IDiscordLobbySearchQuery,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn lobby_count(manager: *mut sys::IDiscordLobbyManager, count: *mut i32) {}

pub unsafe extern "C" fn get_lobby_id(
    manager: *mut sys::IDiscordLobbyManager,
    index: i32,
    lobby_id: *mut sys::DiscordLobbyId,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn connect_voice(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn disconnect_voice(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn connect_network(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn disconnect_network(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn flush_network(
    manager: *mut sys::IDiscordLobbyManager,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn open_network_channel(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    channel_id: u8,
    reliable: bool,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn send_network_message(
    manager: *mut sys::IDiscordLobbyManager,
    lobby_id: sys::DiscordLobbyId,
    user_id: sys::DiscordUserId,
    channel_id: u8,
    data: *mut u8,
    data_length: u32,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}
