use crate::{
    sys, to_result::ToResult, utils::charbuf_to_str, Collection, Discord, Lobby, LobbyID,
    LobbyMemberTransaction, LobbyTransaction, NetworkChannelID, Reliability, Result, SearchQuery,
    UserID,
};
use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
    mem::size_of,
};

/// # Lobbies
///
/// Provides the ability to group players together and run matchmaking-type searches
/// over the pool of existing groups.
///
/// Some operations must be ran from your game backend:
/// [Reference](https://discordapp.com/developers/docs/game-sdk/lobbies#the-api-way).
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies)
impl Discord {
    /// Create a new lobby. The current user will automatically join and become the owner.
    ///
    /// [`LobbyTransaction::owner`](struct.LobbyTransaction.html#method.owner) *MUST NOT* be called.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#createlobby)
    pub fn create_lobby<'d>(
        &'d self,
        transaction: &LobbyTransaction,
        callback: impl 'd + FnOnce(&Self, Result<&Lobby>),
    ) {
        let mut ptr = std::ptr::null_mut();

        if let Err(e) = unsafe {
            ffi!(self
                .get_lobby_manager()
                .get_lobby_create_transaction(&mut ptr))
            .to_result()
        } {
            return callback(self, Err(e));
        }

        if let Err(e) = unsafe { transaction.process(ptr) } {
            return callback(self, Err(e));
        }

        unsafe {
            ffi!(self.get_lobby_manager().create_lobby(ptr).and_then(
                |res: sys::EDiscordResult, lobby: *mut sys::DiscordLobby| {
                    callback::<Result<&Lobby>>(res.to_result().map(|()| &*(lobby as *mut Lobby)))
                }
            ))
        }
    }

    /// Updates a lobby with data from the given transaction.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#updatelobby)
    pub fn update_lobby<'d>(
        &'d self,
        lobby_id: LobbyID,
        transaction: &LobbyTransaction,
        callback: impl 'd + FnOnce(&Self, Result<()>),
    ) {
        let mut ptr = std::ptr::null_mut();

        if let Err(e) = unsafe {
            ffi!(self
                .get_lobby_manager()
                .get_lobby_update_transaction(lobby_id, &mut ptr))
            .to_result()
        } {
            return callback(self, Err(e));
        }

        if let Err(e) = unsafe { transaction.process(ptr) } {
            return callback(self, Err(e));
        }

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .update_lobby(lobby_id, ptr)
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }

    /// Deletes a given lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#deletelobby)
    pub fn delete_lobby<'d>(
        &'d self,
        lobby_id: LobbyID,
        callback: impl 'd + FnOnce(&Self, Result<()>),
    ) {
        unsafe {
            ffi!(self
                .get_lobby_manager()
                .delete_lobby(lobby_id)
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }

    /// Connects the current user to a given lobby.
    /// You can be connected to up to five lobbies at a time.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `secret` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#connectlobby)
    pub fn connect_lobby<'d, 's>(
        &'d self,
        lobby_id: LobbyID,
        secret: impl Into<Cow<'s, str>>,
        callback: impl 'd + FnOnce(&Self, Result<&Lobby>),
    ) {
        let mut secret = secret.into();

        if !secret.ends_with('\0') {
            secret.to_mut().push('\0')
        };

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .connect_lobby(
                    lobby_id,
                    // XXX: *mut should be *const
                    secret.as_ptr() as *mut u8
                )
                .and_then(|res: sys::EDiscordResult, lobby: *mut sys::DiscordLobby| {
                    callback::<Result<&Lobby>>(res.to_result().map(|()| &*(lobby as *mut Lobby)))
                }))
        }
    }

    /// Connects the current user to a lobby using the special activity secret from the lobby
    /// which is a concatenated lobby ID and its secret.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `activity_secret` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#connectlobbywithactivitysecret)
    pub fn connect_lobby_with_activity_secret<'d, 's>(
        &'d self,
        activity_secret: impl Into<Cow<'s, str>>,
        callback: impl 'd + FnOnce(&Self, Result<&Lobby>),
    ) {
        let mut activity_secret = activity_secret.into();

        if !activity_secret.ends_with('\0') {
            activity_secret.to_mut().push('\0')
        };

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .connect_lobby_with_activity_secret(
                    // XXX: *mut should be *const
                    activity_secret.as_ptr() as *mut u8
                )
                .and_then(|res: sys::EDiscordResult, lobby: *mut sys::DiscordLobby| {
                    callback::<Result<&Lobby>>(res.to_result().map(|()| &*(lobby as *mut Lobby)))
                }))
        }
    }

    /// Disconnects the current user from a lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#disconnectlobby)
    pub fn disconnect_lobby<'d>(
        &'d self,
        lobby_id: LobbyID,
        callback: impl 'd + FnOnce(&Self, Result<()>),
    ) {
        unsafe {
            ffi!(self
                .get_lobby_manager()
                .disconnect_lobby(lobby_id)
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }

    /// Gets the lobby object for a given ID.
    ///
    /// [`lobby_search`](#method.lobby_search) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getlobby)
    pub fn lobby(&self, lobby_id: LobbyID) -> Result<Lobby> {
        let mut lobby = sys::DiscordLobby::default();

        unsafe {
            ffi!(self.get_lobby_manager().get_lobby(lobby_id, &mut lobby)).to_result()?;
        }

        Ok(Lobby::from(lobby))
    }

    /// Gets the activity secret for a given lobby.
    ///
    /// It should be used to populate
    /// [`Activity::with_join_secret`](struct.Activity.html#method.with_join_secret).
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getlobbyactivitysecret)
    pub fn lobby_activity_secret(&self, lobby_id: LobbyID) -> Result<String> {
        let mut secret: sys::DiscordLobbySecret = [0; size_of::<sys::DiscordLobbySecret>()];

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .get_lobby_activity_secret(lobby_id, &mut secret))
            .to_result()?;
        }

        Ok(charbuf_to_str(&secret).to_string())
    }

    /// Returns lobby metadata value for a given key.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `key` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getlobbymetadatavalue)
    pub fn lobby_metadata<'s>(
        &self,
        lobby_id: LobbyID,
        key: impl Into<Cow<'s, str>>,
    ) -> Result<String> {
        let mut value: sys::DiscordMetadataValue = [0; size_of::<sys::DiscordMetadataValue>()];

        let mut key = key.into();

        if !key.ends_with('\0') {
            key.to_mut().push('\0')
        };

        unsafe {
            ffi!(self.get_lobby_manager().get_lobby_metadata_value(
                lobby_id,
                // XXX: *mut should be *const
                key.as_ptr() as *mut u8,
                &mut value
            ))
            .to_result()?;
        }

        Ok(charbuf_to_str(&value).to_string())
    }

    /// Returns the number of metadata key-value pairs available for a given lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbymetadatacount)  
    pub fn lobby_metadata_count(&self, lobby_id: LobbyID) -> Result<u32> {
        let mut count = 0;

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .lobby_metadata_count(lobby_id, &mut count))
            .to_result()?;
        }

        // XXX: i32 should be u32
        Ok(count.try_into().unwrap())
    }

    /// Returns metadata key-value pair at a certain index for a given lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getlobbymetadatakey)  
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getlobbymetadatavalue)
    pub fn lobby_metadata_at(&self, lobby_id: LobbyID, index: u32) -> Result<(String, String)> {
        let mut key: sys::DiscordMetadataKey = [0; size_of::<sys::DiscordMetadataKey>()];
        let mut value: sys::DiscordMetadataValue = [0; size_of::<sys::DiscordMetadataValue>()];

        unsafe {
            ffi!(self.get_lobby_manager().get_lobby_metadata_key(
                lobby_id,
                // XXX: i32 should be u32
                index.try_into().unwrap(),
                &mut key
            ))
            .to_result()?;
        }

        unsafe {
            ffi!(self.get_lobby_manager().get_lobby_metadata_value(
                lobby_id,
                // XXX: *mut should be *const
                key.as_ptr() as *mut u8,
                &mut value
            ))
            .to_result()?;
        }

        Ok((
            charbuf_to_str(&key).to_string(),
            charbuf_to_str(&value).to_string(),
        ))
    }

    /// Returns an `Iterator` over the metadata key-value pairs for a given lobby.
    pub fn iter_lobby_metadata(
        &self,
        lobby_id: LobbyID,
    ) -> Result<Collection<'_, Result<(String, String)>>> {
        Ok(Collection::new(
            self,
            Box::new(move |d, i| d.lobby_metadata_at(lobby_id, i)),
            self.lobby_metadata_count(lobby_id)?,
        ))
    }

    /// Updates lobby member info for a given member of the lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#updatemember)
    pub fn update_member<'d>(
        &'d self,
        lobby_id: LobbyID,
        user_id: UserID,
        transaction: &LobbyMemberTransaction,
        callback: impl 'd + FnOnce(&Self, Result<()>),
    ) {
        let mut ptr = std::ptr::null_mut();

        if let Err(e) = unsafe {
            ffi!(self
                .get_lobby_manager()
                .get_member_update_transaction(lobby_id, user_id, &mut ptr))
            .to_result()
        } {
            return callback(self, Err(e));
        }

        if let Err(e) = unsafe { transaction.process(ptr) } {
            return callback(self, Err(e));
        }

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .update_member(lobby_id, user_id, ptr)
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }

    /// Returns the number of members connected to a lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#membercount)  
    pub fn lobby_member_count(&self, lobby_id: LobbyID) -> Result<u32> {
        let mut count = 0;

        unsafe {
            ffi!(self.get_lobby_manager().member_count(lobby_id, &mut count)).to_result()?;
        }

        // XXX: i32 should be u32
        Ok(count.try_into().unwrap())
    }

    /// Returns the user ID of the lobby member at a certain index.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getmemberuserid)
    pub fn lobby_member_id_at(&self, lobby_id: LobbyID, index: u32) -> Result<UserID> {
        let mut user_id = 0;

        unsafe {
            ffi!(self.get_lobby_manager().get_member_user_id(
                lobby_id,
                // XXX: i32 should be u32
                index.try_into().unwrap(),
                &mut user_id
            ))
            .to_result()?;
        }

        Ok(user_id)
    }

    /// Returns an `Iterator` over the user IDs of the members of a lobby.
    pub fn iter_lobby_member_ids(
        &self,
        lobby_id: LobbyID,
    ) -> Result<Collection<'_, Result<UserID>>> {
        Ok(Collection::new(
            self,
            Box::new(move |d, i| d.lobby_member_id_at(lobby_id, i)),
            self.lobby_member_count(lobby_id)?,
        ))
    }

    /// Returns member metadata value for a given key.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `key` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getmembermetadatavalue)
    pub fn lobby_member_metadata<'s>(
        &self,
        lobby_id: LobbyID,
        user_id: UserID,
        key: impl Into<Cow<'s, str>>,
    ) -> Result<String> {
        let mut value: sys::DiscordMetadataValue = [0; size_of::<sys::DiscordMetadataValue>()];

        let mut key = key.into();

        if !key.ends_with('\0') {
            key.to_mut().push('\0')
        };

        unsafe {
            ffi!(self.get_lobby_manager().get_member_metadata_value(
                lobby_id,
                user_id,
                // XXX: *mut should be *const
                key.as_ptr() as *mut u8,
                &mut value
            ))
            .to_result()?;
        }

        Ok(charbuf_to_str(&value).to_string())
    }

    /// Returns the number of metadata key-value pairs for a given lobby member.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#membermetadatacount)  
    pub fn lobby_member_metadata_count(&self, lobby_id: LobbyID, user_id: UserID) -> Result<u32> {
        let mut count = 0;

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .member_metadata_count(lobby_id, user_id, &mut count))
            .to_result()?;
        }

        // XXX: i32 should be u32
        Ok(count.try_into().unwrap())
    }

    /// Returns the metadata key-value pair at a certain index for a given lobby member.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getmembermetadatakey)
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getmembermetadatavalue)
    pub fn lobby_member_metadata_at(
        &self,
        lobby_id: LobbyID,
        user_id: UserID,
        index: u32,
    ) -> Result<(String, String)> {
        let mut key: sys::DiscordMetadataKey = [0; size_of::<sys::DiscordMetadataKey>()];
        let mut value: sys::DiscordMetadataValue = [0; size_of::<sys::DiscordMetadataValue>()];

        unsafe {
            ffi!(self.get_lobby_manager().get_member_metadata_key(
                lobby_id,
                user_id,
                // XXX: i32 should be u32
                index.try_into().unwrap(),
                &mut key
            ))
            .to_result()?;
        }

        unsafe {
            ffi!(self.get_lobby_manager().get_member_metadata_value(
                lobby_id,
                user_id,
                // XXX: *mut should be *const
                key.as_ptr() as *mut u8,
                &mut value
            ))
            .to_result()?;
        }

        Ok((
            charbuf_to_str(&key).to_string(),
            charbuf_to_str(&value).to_string(),
        ))
    }

    /// Returns an `Iterator` over the metadata key-value pairs of a given lobby member.
    pub fn iter_lobby_member_metadata(
        &self,
        lobby_id: LobbyID,
        user_id: UserID,
    ) -> Result<Collection<'_, Result<(String, String)>>> {
        Ok(Collection::new(
            self,
            Box::new(move |d, i| d.lobby_member_metadata_at(lobby_id, user_id, i)),
            self.lobby_member_metadata_count(lobby_id, user_id)?,
        ))
    }

    /// Sends a message to the lobby on behalf of the current user.
    ///
    /// You must be connected to the lobby you are messaging.
    /// You should use this function for message sending if you are not using
    /// the built in networking layer for the lobby.
    /// If you are, you should use
    /// [`send_lobby_network_message`](#method.send_lobby_network_message) instead.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#sendlobbymessage)
    pub fn send_lobby_message<'d>(
        &'d self,
        lobby_id: LobbyID,
        buffer: impl AsRef<[u8]>,
        callback: impl 'd + FnOnce(&Self, Result<()>),
    ) {
        let buffer = buffer.as_ref();

        debug_assert!(u32::try_from(buffer.len()).is_ok());

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .send_lobby_message(
                    lobby_id,
                    // XXX: *mut should be *const
                    buffer.as_ptr() as *mut u8,
                    // XXX: u32 should be u64
                    buffer.len().try_into().unwrap_or(u32::max_value())
                )
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }

    /// Searches available lobbies based on the search criteria.
    ///
    /// Lobbies that meet the criteria are then globally filtered.
    /// The callback fires when the list of lobbies is stable and ready for iteration.
    /// You do not necessarily need to access the filtered lobbies within the context of the result callback.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#search)
    pub fn lobby_search<'d>(
        &'d self,
        search: &SearchQuery,
        callback: impl 'd + FnOnce(&Self, Result<()>),
    ) {
        let mut ptr = std::ptr::null_mut();

        if let Err(e) =
            unsafe { ffi!(self.get_lobby_manager().get_search_query(&mut ptr)).to_result() }
        {
            return callback(self, Err(e));
        }

        if let Err(e) = unsafe { search.process(ptr) } {
            return callback(self, Err(e));
        }

        unsafe {
            ffi!(self
                .get_lobby_manager()
                .search(ptr)
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }

    /// Returns the number of lobbies found via the search query.
    ///
    /// [`lobby_search`](#method.lobby_search) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbycount)
    pub fn lobby_count(&self) -> u32 {
        let mut count = 0;

        unsafe { ffi!(self.get_lobby_manager().lobby_count(&mut count)) }

        // XXX: i32 should be u32
        count.try_into().unwrap()
    }

    /// Returns the lobby ID at a given index.
    ///
    /// [`lobby_search`](#method.lobby_search) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getlobbyid)
    pub fn lobby_id_at(&self, index: u32) -> Result<LobbyID> {
        let mut lobby_id = 0;

        unsafe {
            ffi!(self.get_lobby_manager().get_lobby_id(
                // XXX: i32 should be u32
                index.try_into().unwrap(),
                &mut lobby_id
            ))
            .to_result()?;
        }

        Ok(lobby_id)
    }

    /// Returns an `Iterator` over the IDs of lobbies found via the lobby search.
    ///
    /// [`lobby_search`](#method.lobby_search) must have completed first.
    pub fn iter_lobbies(&self) -> Collection<'_, Result<LobbyID>> {
        Collection::new(self, Box::new(Self::lobby_id_at), self.lobby_count())
    }

    /// Connects to the voice channel of the current lobby.
    ///
    /// When connected to voice, the user can open their Discord overlay to see a list of other users,
    /// allowing them to mute/deafen themselves as well as mute/adjust the volume of other members.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#connectvoice)
    pub fn connect_lobby_voice<'d>(
        &'d self,
        lobby_id: LobbyID,
        callback: impl 'd + FnOnce(&Self, Result<()>),
    ) {
        unsafe {
            ffi!(self
                .get_lobby_manager()
                .connect_voice(lobby_id)
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }

    /// Disconnects from the voice channel of a given lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#disconnectvoice)
    pub fn disconnect_lobby_voice<'d>(
        &'d self,
        lobby_id: LobbyID,
        callback: impl 'd + FnOnce(&Self, Result<()>),
    ) {
        unsafe {
            ffi!(self
                .get_lobby_manager()
                .disconnect_voice(lobby_id)
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }

    /// Connects to the networking layer for the given lobby ID.
    ///
    /// Call this when connecting to the lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#connectnetwork)
    pub fn connect_lobby_network(&self, lobby_id: LobbyID) -> Result<()> {
        unsafe { ffi!(self.get_lobby_manager().connect_network(lobby_id,)).to_result() }
    }

    /// Disconnects from the networking layer for the given lobby ID.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#disconnectnetwork)
    pub fn disconnect_lobby_network(&self, lobby_id: LobbyID) -> Result<()> {
        unsafe { ffi!(self.get_lobby_manager().disconnect_network(lobby_id,)).to_result() }
    }

    /// Flushes the network. Call this when you're done sending messages.
    ///
    /// This should appear near the end of your game loop.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#flushnetwork)
    pub fn flush_lobby_network(&self) -> Result<()> {
        unsafe { ffi!(self.get_lobby_manager().flush_network()).to_result() }
    }

    /// Opens a network channel to all users in a lobby on the given channel number.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#opennetworkchannel)
    pub fn open_lobby_network_channel(
        &self,
        lobby_id: LobbyID,
        channel_id: NetworkChannelID,
        reliable: Reliability,
    ) -> Result<()> {
        unsafe {
            ffi!(self.get_lobby_manager().open_network_channel(
                lobby_id,
                channel_id,
                reliable.into()
            ))
            .to_result()
        }
    }

    /// Sends a network message.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#sendnetworkmessage)
    pub fn send_lobby_network_message(
        &self,
        lobby_id: LobbyID,
        user_id: UserID,
        channel_id: NetworkChannelID,
        buffer: &[u8],
    ) -> Result<()> {
        debug_assert!(u32::try_from(buffer.len()).is_ok());

        unsafe {
            ffi!(self.get_lobby_manager().send_network_message(
                lobby_id,
                user_id,
                channel_id,
                // XXX: *mut should be *const
                buffer.as_ptr() as *mut u8,
                // XXX: u32 should be u64
                buffer.len().try_into().unwrap_or(u32::max_value()),
            ))
            .to_result()
        }
    }
}
