use crate::prelude::*;

// get_lobby_update_transaction: lobby_id: u64, transaction: *mut *mut IDiscordLobbyTransaction) -> EDiscordResult>,
// get_member_update_transaction: lobby_id: u64, user_id: u64, transaction: *mut *mut IDiscordLobbyMemberTransaction) -> EDiscordResult>,
// update_lobby: lobby_id: u64, transaction: *mut IDiscordLobbyTransaction, callback_data: *mut c_void, callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: EDiscordResult)>)>,
// delete_lobby: lobby_id: u64, callback_data: *mut c_void, callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: EDiscordResult)>)>,

// connect_lobby: lobby_id: u64, secret: *mut c_char, callback_data: *mut c_void, callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: EDiscordResult, lobby: *mut DiscordLobby)>)>,
// connect_lobby_with_activity_secret: activity_secret: *mut c_char, callback_data: *mut c_void, callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: EDiscordResult, lobby: *mut DiscordLobby)>)>,
// disconnect_lobby: lobby_id: u64, callback_data: *mut c_void, callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: EDiscordResult)>)>,
// get_lobby: lobby_id: u64, lobby: *mut DiscordLobby) -> EDiscordResult>,
// get_lobby_activity_secret: lobby_id: u64, secret: *mut DiscordLobbySecret) -> EDiscordResult>,
// get_lobby_metadata_value: lobby_id: u64, key: *mut c_char, value: *mut DiscordMetadataValue) -> EDiscordResult>,
// get_lobby_metadata_key: lobby_id: u64, index: i32, key: *mut DiscordMetadataKey) -> EDiscordResult>,
// lobby_metadata_count: lobby_id: u64, count: *mut i32) -> EDiscordResult>,
// member_count: lobby_id: u64, count: *mut i32) -> EDiscordResult>,
// get_member_user_id: lobby_id: u64, index: i32, user_id: *mut u64) -> EDiscordResult>,
// get_member_user: lobby_id: u64, user_id: u64, user: *mut DiscordUser) -> EDiscordResult>,
// get_member_metadata_value: lobby_id: u64, user_id: u64, key: *mut c_char, value: *mut DiscordMetadataValue) -> EDiscordResult>,
// get_member_metadata_key: lobby_id: u64, user_id: u64, index: i32, key: *mut DiscordMetadataKey) -> EDiscordResult>,
// member_metadata_count: lobby_id: u64, user_id: u64, count: *mut i32) -> EDiscordResult>,
// update_member: lobby_id: u64, user_id: u64, transaction: *mut IDiscordLobbyMemberTransaction, callback_data: *mut c_void, callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: EDiscordResult)>)>,
// send_lobby_message: lobby_id: u64, data: *mut u8, data_length: u32, callback_data: *mut c_void, callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: EDiscordResult)>)>,
// get_search_query: query: *mut *mut IDiscordLobbySearchQuery) -> EDiscordResult>,
// search: query: *mut IDiscordLobbySearchQuery, callback_data: *mut c_void, callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: EDiscordResult)>)>,
// lobby_count: count: *mut i32)>,
// get_lobby_id: index: i32, lobby_id: *mut u64) -> EDiscordResult>,
// connect_voice: lobby_id: u64, callback_data: *mut c_void, callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: EDiscordResult)>)>,
// disconnect_voice: lobby_id: u64, callback_data: *mut c_void, callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: EDiscordResult)>)>,
// connect_network: lobby_id: u64) -> EDiscordResult>,
// disconnect_network: lobby_id: u64) -> EDiscordResult>,
// flush_network: Option<unsafe extern "C" fn(manager: *mut IDiscordLobbyManager) -> EDiscordResult>,
// open_network_channel: lobby_id: u64, channel_id: u8, reliable: bool) -> EDiscordResult>,
// send_network_message: lobby_id: u64, user_id: u64, channel_id: u8, data: *mut u8, data_length: u32) -> EDiscordResult>,<Paste>

/// # Lobbies
impl<'a> Discord<'a> {
    pub fn create_lobby_transaction(&mut self) -> Result<LobbyCreateTransaction<'a>> {
        let mut tx: *mut sys::IDiscordLobbyTransaction = std::ptr::null_mut();

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .get_lobby_create_transaction(&mut tx))
        }
        .to_result()?;

        let core = unsafe { tx.as_mut() }.unwrap();

        Ok(LobbyCreateTransaction { core })
    }

    pub fn create_lobby<F>(&mut self, tx: LobbyCreateTransaction<'a>, callback: F)
    where
        F: FnMut(Result<Lobby>),
    {
        unsafe {
            ffi!(self.get_lobby_manager().create_lobby(
                tx.core,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result_from_sys::<F, Lobby>)
            ))
        }
    }

    pub fn lobby_events_reader(&mut self) -> shrev::ReaderId<event::Lobby> {
        self.lobby_channel.register_reader()
    }

    pub fn lobby_events(
        &self,
        reader: &mut shrev::ReaderId<event::Lobby>,
    ) -> shrev::EventIterator<event::Lobby> {
        self.lobby_channel.read(reader)
    }
}

pub struct LobbyCreateTransaction<'a> {
    pub(crate) core: &'a mut sys::IDiscordLobbyTransaction,
}

impl<'a> LobbyCreateTransaction<'a> {
    pub fn set_kind(&mut self, kind: LobbyKind) -> Result<()> {
        unsafe { ffi!(self.set_type(kind.to_sys())) }.to_result()
    }

    pub fn set_owner(&mut self, user_id: i64) -> Result<()> {
        unsafe { ffi!(self.set_owner(user_id)) }.to_result()
    }

    pub fn set_capacity(&mut self, capacity: u32) -> Result<()> {
        unsafe { ffi!(self.set_capacity(capacity)) }.to_result()
    }

    pub fn set_metadata<S1, S2>(&mut self, key: S1, value: S2) -> Result<()>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let key = CString::new(key.as_ref()).unwrap();
        let value = CString::new(value.as_ref()).unwrap();

        unsafe { ffi!(self.set_metadata(key.as_ptr() as *mut _, value.as_ptr() as *mut _)) }
            .to_result()
    }

    pub fn delete_metadata<S>(&mut self, key: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let key = CString::new(key.as_ref()).unwrap();

        unsafe { ffi!(self.delete_metadata(key.as_ptr() as *mut _)) }.to_result()
    }

    pub fn set_locked(&mut self, locked: bool) -> Result<()> {
        unsafe { ffi!(self.set_locked(locked)) }.to_result()
    }
}
