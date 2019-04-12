use crate::prelude::*;

/// # Lobbies
impl Discord {
    // tested
    pub fn create_lobby<F>(&mut self, builder: LobbyTransaction, mut callback: F)
    where
        F: FnMut(&mut Discord, Result<Lobby>) + 'static,
    {
        let mut ptr = std::ptr::null_mut();

        if let Err(e) = unsafe {
            ffi!(self
                .get_lobby_manager()
                .get_lobby_create_transaction(&mut ptr))
            .to_result()
        } {
            callback(self, Err(e));
            return;
        }

        if let Err(e) = unsafe { builder.process(ptr) } {
            callback(self, Err(e));
            return;
        }

        unsafe {
            ffi!(self.get_lobby_manager().create_lobby(ptr)(
                ResultFromSysPtrCallback::new(callback)
            ))
        }
    }

    // tested
    pub fn update_lobby<F>(&mut self, lobby_id: i64, builder: LobbyTransaction, mut callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'static,
    {
        let mut ptr = std::ptr::null_mut();

        if let Err(e) = unsafe {
            ffi!(self
                .get_lobby_manager()
                .get_lobby_update_transaction(lobby_id, &mut ptr))
            .to_result()
        } {
            callback(self, Err(e));
            return;
        }

        if let Err(e) = unsafe { builder.process(ptr) } {
            callback(self, Err(e));
            return;
        }

        unsafe {
            ffi!(self.get_lobby_manager().update_lobby(lobby_id, ptr)(
                ResultCallback::new(callback)
            ))
        }
    }

    pub fn delete_lobby<F>(&mut self, lobby_id: i64, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'static,
    {
        unsafe {
            ffi!(self.get_lobby_manager().delete_lobby(lobby_id)(
                ResultCallback::new(callback)
            ))
        }
    }

    pub fn connect_lobby<F>(&mut self, lobby_id: i64, secret: impl AsRef<str>, callback: F)
    where
        F: FnMut(&mut Discord, Result<Lobby>) + 'static,
    {
        let secret = CString::new(secret.as_ref()).unwrap();

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .connect_lobby(lobby_id, secret.as_ptr() as *mut _)(
                ResultFromSysPtrCallback::new(callback)
            ))
        }
    }

    pub fn connect_lobby_with_activity_secret<F>(
        &mut self,
        activity_secret: impl AsRef<str>,
        callback: F,
    ) where
        F: FnMut(&mut Discord, Result<Lobby>) + 'static,
    {
        let activity_secret = CString::new(activity_secret.as_ref()).unwrap();

        unsafe {
            ffi!(self.get_lobby_manager().connect_lobby_with_activity_secret(
                activity_secret.as_ptr() as *mut _
            )(ResultFromSysPtrCallback::new(callback)))
        }
    }

    pub fn disconnect_lobby<F>(&mut self, lobby_id: i64, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'static,
    {
        unsafe {
            ffi!(self.get_lobby_manager().disconnect_lobby(lobby_id)(
                ResultCallback::new(callback)
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

    // tested
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

    // tested
    pub fn lobby_metadata(&mut self, lobby_id: i64, key: impl AsRef<str>) -> Result<String> {
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

    // tested
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

    pub fn update_member<F>(
        &mut self,
        lobby_id: i64,
        user_id: i64,
        builder: LobbyMemberTransaction,
        mut callback: F,
    ) where
        F: FnMut(&mut Discord, Result<()>) + 'static,
    {
        let mut ptr = std::ptr::null_mut();

        if let Err(e) = unsafe {
            ffi!(self
                .get_lobby_manager()
                .get_member_update_transaction(lobby_id, user_id, &mut ptr))
            .to_result()
        } {
            callback(self, Err(e));
            return;
        }

        if let Err(e) = unsafe { builder.process(ptr) } {
            callback(self, Err(e));
            return;
        }

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .update_member(lobby_id, user_id, ptr)(
                ResultCallback::new(callback)
            ))
        }
    }

    // tested
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
        F: FnMut(&mut Discord, Result<()>) + 'static,
    {
        assert!(buf.len() <= u32::max_value() as usize);

        unsafe {
            ffi!(self.get_lobby_manager().send_lobby_message(
                lobby_id,
                buf.as_ptr() as *mut _,
                buf.len() as u32
            )(ResultCallback::new(callback)))
        }
    }

    pub fn lobby_search<F>(&mut self, builder: SearchQuery, mut callback: F)
    where
        F: FnMut(&mut Discord, Result<Vec<i64>>) + 'static,
    {
        let mut ptr = std::ptr::null_mut();

        if let Err(e) =
            unsafe { ffi!(self.get_lobby_manager().get_search_query(&mut ptr)).to_result() }
        {
            callback(self, Err(e));
            return;
        }

        if let Err(e) = unsafe { builder.process(ptr) } {
            callback(self, Err(e));
            return;
        }

        let inner = move |gsdk: &mut Discord, res: Result<()>| {
            if let Err(e) = res {
                return callback(gsdk, Err(e));
            }

            let mut count = 0;

            unsafe { ffi!(gsdk.get_lobby_manager().lobby_count(&mut count)) }

            let mut vec = Vec::with_capacity(count as usize);
            let mut lobby_id = 0;

            for index in 0..count {
                let res =
                    unsafe { ffi!(gsdk.get_lobby_manager().get_lobby_id(index, &mut lobby_id)) }
                        .to_result();

                if let Err(e) = res {
                    return callback(gsdk, Err(e));
                }

                vec.push(lobby_id);
            }

            callback(gsdk, Ok(vec))
        };

        unsafe {
            ffi!(self.get_lobby_manager().search(ptr)(ResultCallback::new(
                inner
            )))
        }
    }

    // tested
    pub fn connect_lobby_voice<F>(&mut self, lobby_id: i64, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'static,
    {
        unsafe {
            ffi!(self.get_lobby_manager().connect_voice(lobby_id)(
                ResultCallback::new(callback)
            ))
        }
    }

    pub fn disconnect_lobby_voice<F>(&mut self, lobby_id: i64, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'static,
    {
        unsafe {
            ffi!(self.get_lobby_manager().disconnect_voice(lobby_id)(
                ResultCallback::new(callback)
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
}
