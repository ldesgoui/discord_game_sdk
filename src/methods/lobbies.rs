use crate::prelude::*;

/// # Lobbies
impl<'a> Discord<'a> {
    // TODO: LobbyTransaction<'a> doesn't seem like it's valid, please verify
    pub fn create_lobby_transaction(&mut self) -> Result<LobbyTransaction<'a>> {
        let mut tx: *mut sys::IDiscordLobbyTransaction = std::ptr::null_mut();

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .get_lobby_create_transaction(&mut tx))
        }
        .to_result()?;

        let core = unsafe { tx.as_mut() }.unwrap();

        Ok(LobbyTransaction { core })
    }

    pub fn create_lobby<F>(&mut self, tx: LobbyTransaction<'a>, callback: F)
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

    pub fn update_lobby_transaction(&mut self, lobby_id: i64) -> Result<LobbyTransaction<'a>> {
        let mut tx: *mut sys::IDiscordLobbyTransaction = std::ptr::null_mut();

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .get_lobby_update_transaction(lobby_id, &mut tx))
        }
        .to_result()?;

        let core = unsafe { tx.as_mut() }.unwrap();

        Ok(LobbyTransaction { core })
    }

    pub fn update_lobby<F>(&mut self, lobby_id: i64, tx: LobbyTransaction<'a>, callback: F)
    where
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_lobby_manager().update_lobby(
                lobby_id,
                tx.core,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        }
    }

    pub fn delete_lobby<F>(&mut self, lobby_id: i64, callback: F)
    where
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_lobby_manager().delete_lobby(
                lobby_id,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        }
    }

    pub fn connect_lobby<S, F>(&mut self, lobby_id: i64, secret: S, callback: F)
    where
        S: AsRef<str>,
        F: FnMut(Result<Lobby>),
    {
        let secret = CString::new(secret.as_ref()).unwrap();

        unsafe {
            ffi!(self.get_lobby_manager().connect_lobby(
                lobby_id,
                secret.as_ptr() as *mut _,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result_from_sys::<F, Lobby>)
            ))
        }
    }

    pub fn connect_lobby_with_activity_secret<S, F>(&mut self, activity_secret: S, callback: F)
    where
        S: AsRef<str>,
        F: FnMut(Result<Lobby>),
    {
        let activity_secret = CString::new(activity_secret.as_ref()).unwrap();

        unsafe {
            ffi!(self.get_lobby_manager().connect_lobby_with_activity_secret(
                activity_secret.as_ptr() as *mut _,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result_from_sys::<F, Lobby>)
            ))
        }
    }

    pub fn disconnect_lobby<F>(&mut self, lobby_id: i64, callback: F)
    where
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_lobby_manager().disconnect_lobby(
                lobby_id,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        }
    }

    pub fn lobby(&mut self, lobby_id: i64) -> Result<Lobby> {
        let mut lobby = sys::DiscordLobby::default();

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .get_lobby(lobby_id, &mut lobby as *mut _))
        }
        .to_result()?;

        Ok(Lobby::from_sys(&lobby))
    }

    pub fn lobby_activity_secret(&mut self, lobby_id: i64) -> Result<String> {
        let mut secret: sys::DiscordLobbySecret = [0; size_of::<sys::DiscordLobbySecret>()];

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .get_lobby_activity_secret(lobby_id, &mut secret as *mut _))
        }
        .to_result()?;

        Ok(unsafe { string_from_cstr(&secret as *const _) })
    }

    pub fn lobby_metadata<S>(&mut self, lobby_id: i64, key: S) -> Result<String>
    where
        S: AsRef<str>,
    {
        let key = CString::new(key.as_ref()).unwrap();
        let mut value: sys::DiscordMetadataValue = [0; size_of::<sys::DiscordMetadataValue>()];

        unsafe {
            ffi!(self.get_lobby_manager().get_lobby_metadata_value(
                lobby_id,
                key.as_ptr() as *mut _,
                &mut value as *mut _
            ))
        }
        .to_result()?;

        Ok(unsafe { string_from_cstr(&value as *const _) })
    }

    pub fn all_lobby_metadata(&mut self, lobby_id: i64) -> Result<HashMap<String, String>> {
        let mut count: i32 = 0;

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .lobby_metadata_count(lobby_id, &mut count as *mut _))
        }
        .to_result()?;

        let mut res = HashMap::with_capacity(count as usize);
        let mut key: sys::DiscordMetadataKey = [0; size_of::<sys::DiscordMetadataKey>()];
        let mut value: sys::DiscordMetadataValue = [0; size_of::<sys::DiscordMetadataValue>()];

        for index in 0..count {
            unsafe {
                ffi!(self.get_lobby_manager().get_lobby_metadata_key(
                    lobby_id,
                    index as i32,
                    &mut key as *mut _
                ))
            }
            .to_result()?;

            unsafe {
                ffi!(self.get_lobby_manager().get_lobby_metadata_value(
                    lobby_id,
                    key.as_mut_ptr(),
                    &mut value as *mut _
                ))
            }
            .to_result()?;

            let _ = unsafe {
                res.insert(
                    string_from_cstr(&key as *const _),
                    string_from_cstr(&value as *const _),
                )
            };
        }

        Ok(res)
    }

    pub fn update_member_transaction(
        &mut self,
        lobby_id: i64,
        user_id: i64,
    ) -> Result<LobbyMemberTransaction<'a>> {
        let mut tx: *mut sys::IDiscordLobbyMemberTransaction = std::ptr::null_mut();

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .get_member_update_transaction(lobby_id, user_id, &mut tx))
        }
        .to_result()?;

        let core = unsafe { tx.as_mut() }.unwrap();

        Ok(LobbyMemberTransaction { core })
    }

    pub fn update_member<F>(
        &mut self,
        lobby_id: i64,
        user_id: i64,
        tx: LobbyMemberTransaction<'a>,
        callback: F,
    ) where
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_lobby_manager().update_member(
                lobby_id,
                user_id,
                tx.core,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        }
    }

    pub fn all_lobby_member_ids(&mut self, lobby_id: i64) -> Result<Vec<i64>> {
        let mut count = 0;

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .member_count(lobby_id, &mut count as *mut _))
        }
        .to_result()?;

        let mut result = Vec::with_capacity(count as usize);
        let mut user_id = 0;

        for index in 0..count {
            unsafe {
                ffi!(self.get_lobby_manager().get_member_user_id(
                    lobby_id,
                    index,
                    &mut user_id as *mut _
                ))
            }
            .to_result()?;

            result.push(user_id)
        }

        Ok(result)
    }

    pub fn all_lobby_member_metadata(
        &mut self,
        lobby_id: i64,
        user_id: i64,
    ) -> Result<HashMap<String, String>> {
        let mut count: i32 = 0;

        unsafe {
            ffi!(self.get_lobby_manager().member_metadata_count(
                lobby_id,
                user_id,
                &mut count as *mut _
            ))
        }
        .to_result()?;

        let mut res = HashMap::with_capacity(count as usize);
        let mut key: sys::DiscordMetadataKey = [0; size_of::<sys::DiscordMetadataKey>()];
        let mut value: sys::DiscordMetadataValue = [0; size_of::<sys::DiscordMetadataValue>()];

        for index in 0..count {
            unsafe {
                ffi!(self.get_lobby_manager().get_member_metadata_key(
                    lobby_id,
                    user_id,
                    index as i32,
                    &mut key as *mut _
                ))
            }
            .to_result()?;

            unsafe {
                ffi!(self.get_lobby_manager().get_member_metadata_value(
                    lobby_id,
                    user_id,
                    key.as_mut_ptr(),
                    &mut value as *mut _
                ))
            }
            .to_result()?;

            let _ = unsafe {
                res.insert(
                    string_from_cstr(&key as *const _),
                    string_from_cstr(&value as *const _),
                )
            };
        }

        Ok(res)
    }

    pub fn send_lobby_message<F>(&mut self, lobby_id: i64, buf: &[u8], callback: F)
    where
        F: FnMut(Result<()>),
    {
        assert!(buf.len() <= u32::max_value() as usize);

        unsafe {
            ffi!(self.get_lobby_manager().send_lobby_message(
                lobby_id,
                buf.as_ptr() as *mut _,
                buf.len() as u32,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>),
            ))
        }
    }

    // TODO: blocked because we kinda would like lobby_search to do the aggregation by itself but
    // it can't as it requires &mut self
    //
    // pub fn lobby_search_query(&mut self) -> Result<SearchQuery<'a>> {
    //     let mut query: *mut sys::IDiscordSearchQuery = std::ptr::null_mut();

    //     unsafe {
    //         ffi!(self
    //             .get_lobby_manager()
    //             .get_search_query(&mut query))
    //     }
    //     .to_result()?;

    //     let core = unsafe { query.as_mut() }.unwrap();

    //     Ok(SearchQuery { core })
    // }

    // pub fn lobby_search<F>(&mut self, query: SearchQuery<'a>, callback: F)
    // where
    //     F: FnMut(Result<Vec<i64>>),
    // {
    //     unsafe {
    //         ffi!(self.get_lobby_manager().search(
    //             tx.core,
    //             Box::into_raw(Box::new(callback)) as *mut _,
    //             Some(across_ffi::callbacks::result_from_sys::<F, Lobby>)
    //         ))
    //     }
    // }
    // get_search_query: query: *mut *mut IDiscordLobbySearchQuery) -> EDiscordResult>,
    // search: query: *mut IDiscordLobbySearchQuery, callback_data: *mut c_void, callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: EDiscordResult)>)>,
    // lobby_count: count: *mut i32)>,
    // get_lobby_id: index: i32, lobby_id: *mut u64) -> EDiscordResult>,

    pub fn connect_lobby_voice<F>(&mut self, lobby_id: i64, callback: F)
    where
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_lobby_manager().connect_voice(
                lobby_id,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>),
            ))
        }
    }

    pub fn disconnect_lobby_voice<F>(&mut self, lobby_id: i64, callback: F)
    where
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_lobby_manager().disconnect_voice(
                lobby_id,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>),
            ))
        }
    }

    pub fn connect_lobby_network(&mut self, lobby_id: i64) -> Result<()> {
        unsafe { ffi!(self.get_lobby_manager().connect_network(lobby_id,)) }.to_result()
    }

    pub fn disconnect_lobby_network(&mut self, lobby_id: i64) -> Result<()> {
        unsafe { ffi!(self.get_lobby_manager().disconnect_network(lobby_id,)) }.to_result()
    }

    pub fn flush_lobby_network(&mut self) -> Result<()> {
        unsafe { ffi!(self.get_lobby_manager().flush_network()) }.to_result()
    }

    pub fn open_lobby_network_channel(
        &mut self,
        lobby_id: i64,
        channel_id: u8,
        reliable: bool,
    ) -> Result<()> {
        unsafe {
            ffi!(self
                .get_lobby_manager()
                .open_network_channel(lobby_id, channel_id, reliable))
        }
        .to_result()
    }

    pub fn send_lobby_network_message(
        &mut self,
        lobby_id: i64,
        user_id: i64,
        channel_id: u8,
        buf: &[u8],
    ) -> Result<()> {
        assert!(buf.len() <= u32::max_value() as usize);

        unsafe {
            ffi!(self.get_lobby_manager().send_network_message(
                lobby_id,
                user_id,
                channel_id,
                buf.as_ptr() as *mut _,
                buf.len() as u32
            ))
        }
        .to_result()
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
