use crate::{
    callback, sys, to_result::ToResult, utils, Collection, Discord, Lobby, LobbyID,
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
        callback: impl 'd + FnOnce(Result<&Lobby>),
    ) {
        let mut tx = std::ptr::null_mut();

        let create = self
            .with_lobby_manager(|mgr| unsafe {
                mgr.get_lobby_create_transaction.unwrap()(mgr, &mut tx)
            })
            .to_result();
        if let Err(e) = create {
            return callback(Err(e));
        }

        if let Err(e) = transaction.process(tx) {
            return callback(Err(e));
        }

        self.with_lobby_manager(|mgr| {
            let (ptr, fun) =
                callback::two_params(|res: sys::EDiscordResult, lobby: *mut sys::DiscordLobby| {
                    callback(res.to_result().map(|()| unsafe { &*(lobby as *mut Lobby) }))
                });

            unsafe { mgr.create_lobby.unwrap()(mgr, tx, ptr, fun) }
        })
    }

    /// Updates a lobby with data from the given transaction.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#updatelobby)
    pub fn update_lobby<'d>(
        &'d self,
        lobby_id: LobbyID,
        transaction: &LobbyTransaction,
        callback: impl 'd + FnOnce(Result<()>),
    ) {
        let mut tx = std::ptr::null_mut();

        let create = self
            .with_lobby_manager(|mgr| unsafe {
                mgr.get_lobby_update_transaction.unwrap()(mgr, lobby_id, &mut tx)
            })
            .to_result();
        if let Err(e) = create {
            return callback(Err(e));
        }

        if let Err(e) = transaction.process(tx) {
            return callback(Err(e));
        }

        self.with_lobby_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(res.to_result()));

            unsafe { mgr.update_lobby.unwrap()(mgr, lobby_id, tx, ptr, fun) }
        })
    }

    /// Deletes a given lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#deletelobby)
    pub fn delete_lobby<'d>(&'d self, lobby_id: LobbyID, callback: impl 'd + FnOnce(Result<()>)) {
        self.with_lobby_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(res.to_result()));

            unsafe { mgr.delete_lobby.unwrap()(mgr, lobby_id, ptr, fun) }
        })
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
        callback: impl 'd + FnOnce(Result<&Lobby>),
    ) {
        let mut secret = secret.into();

        if !secret.ends_with('\0') {
            secret.to_mut().push('\0')
        };

        self.with_lobby_manager(|mgr| {
            let (ptr, fun) =
                callback::two_params(|res: sys::EDiscordResult, lobby: *mut sys::DiscordLobby| {
                    callback(res.to_result().map(|()| unsafe { &*(lobby as *mut Lobby) }))
                });

            unsafe {
                mgr.connect_lobby.unwrap()(
                    mgr,
                    lobby_id,
                    // XXX: *mut should be *const
                    secret.as_ptr() as *mut u8,
                    ptr,
                    fun,
                )
            }
        })
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
        callback: impl 'd + FnOnce(Result<&Lobby>),
    ) {
        let mut activity_secret = activity_secret.into();

        if !activity_secret.ends_with('\0') {
            activity_secret.to_mut().push('\0')
        };

        self.with_lobby_manager(|mgr| {
            let (ptr, fun) =
                callback::two_params(|res: sys::EDiscordResult, lobby: *mut sys::DiscordLobby| {
                    callback(res.to_result().map(|()| unsafe { &*(lobby as *mut Lobby) }))
                });

            unsafe {
                mgr.connect_lobby_with_activity_secret.unwrap()(
                    mgr,
                    // XXX: *mut should be *const
                    activity_secret.as_ptr() as *mut u8,
                    ptr,
                    fun,
                )
            }
        })
    }

    /// Disconnects the current user from a lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#disconnectlobby)
    pub fn disconnect_lobby<'d>(
        &'d self,
        lobby_id: LobbyID,
        callback: impl 'd + FnOnce(Result<()>),
    ) {
        self.with_lobby_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(res.to_result()));

            unsafe { mgr.disconnect_lobby.unwrap()(mgr, lobby_id, ptr, fun) }
        })
    }

    /// Gets the lobby object for a given ID.
    ///
    /// [`lobby_search`](#method.lobby_search) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getlobby)
    pub fn lobby(&self, lobby_id: LobbyID) -> Result<Lobby> {
        let mut lobby = Lobby(sys::DiscordLobby::default());

        self.with_lobby_manager(|mgr| unsafe {
            mgr.get_lobby.unwrap()(mgr, lobby_id, &mut lobby.0)
        })
        .to_result()?;

        Ok(lobby)
    }

    /// Gets the activity secret for a given lobby.
    ///
    /// It should be used to populate
    /// [`Activity::with_join_secret`](struct.Activity.html#method.with_join_secret).
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getlobbyactivitysecret)
    pub fn lobby_activity_secret(&self, lobby_id: LobbyID) -> Result<String> {
        let mut secret: sys::DiscordLobbySecret = [0; size_of::<sys::DiscordLobbySecret>()];

        self.with_lobby_manager(|mgr| unsafe {
            mgr.get_lobby_activity_secret.unwrap()(mgr, lobby_id, &mut secret)
        })
        .to_result()?;

        Ok(utils::charbuf_to_str(&secret).to_string())
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

        self.with_lobby_manager(|mgr| unsafe {
            mgr.get_lobby_metadata_value.unwrap()(
                mgr,
                lobby_id,
                // XXX: *mut should be *const
                key.as_ptr() as *mut u8,
                &mut value,
            )
        })
        .to_result()?;

        Ok(utils::charbuf_to_str(&value).to_string())
    }

    /// Returns the number of metadata key-value pairs available for a given lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbymetadatacount)  
    pub fn lobby_metadata_count(&self, lobby_id: LobbyID) -> Result<u32> {
        let mut count = 0;

        self.with_lobby_manager(|mgr| unsafe {
            mgr.lobby_metadata_count.unwrap()(mgr, lobby_id, &mut count)
        })
        .to_result()?;

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

        self.with_lobby_manager(|mgr| unsafe {
            mgr.get_lobby_metadata_key.unwrap()(
                mgr,
                lobby_id,
                // XXX: i32 should be u32
                index.try_into().unwrap(),
                &mut key,
            )
        })
        .to_result()?;

        self.with_lobby_manager(|mgr| unsafe {
            mgr.get_lobby_metadata_value.unwrap()(
                mgr,
                lobby_id,
                // XXX: *mut should be *const
                key.as_ptr() as *mut u8,
                &mut value,
            )
        })
        .to_result()?;

        Ok((
            utils::charbuf_to_str(&key).to_string(),
            utils::charbuf_to_str(&value).to_string(),
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
        callback: impl 'd + FnOnce(Result<()>),
    ) {
        let mut tx = std::ptr::null_mut();

        let create = self
            .with_lobby_manager(|mgr| unsafe {
                mgr.get_member_update_transaction.unwrap()(mgr, lobby_id, user_id, &mut tx)
            })
            .to_result();
        if let Err(e) = create {
            return callback(Err(e));
        }

        if let Err(e) = transaction.process(tx) {
            return callback(Err(e));
        }

        self.with_lobby_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(res.to_result()));

            unsafe { mgr.update_member.unwrap()(mgr, lobby_id, user_id, tx, ptr, fun) }
        })
    }

    /// Returns the number of members connected to a lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#membercount)  
    pub fn lobby_member_count(&self, lobby_id: LobbyID) -> Result<u32> {
        let mut count = 0;

        self.with_lobby_manager(|mgr| unsafe {
            mgr.member_count.unwrap()(mgr, lobby_id, &mut count)
        })
        .to_result()?;

        // XXX: i32 should be u32
        Ok(count.try_into().unwrap())
    }

    /// Returns the user ID of the lobby member at a certain index.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getmemberuserid)
    pub fn lobby_member_id_at(&self, lobby_id: LobbyID, index: u32) -> Result<UserID> {
        let mut user_id = 0;

        self.with_lobby_manager(|mgr| unsafe {
            mgr.get_member_user_id.unwrap()(
                mgr,
                lobby_id,
                // XXX: i32 should be u32
                index.try_into().unwrap(),
                &mut user_id,
            )
        })
        .to_result()?;

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

        self.with_lobby_manager(|mgr| unsafe {
            mgr.get_member_metadata_value.unwrap()(
                mgr,
                lobby_id,
                user_id,
                // XXX: *mut should be *const
                key.as_ptr() as *mut u8,
                &mut value,
            )
        })
        .to_result()?;

        Ok(utils::charbuf_to_str(&value).to_string())
    }

    /// Returns the number of metadata key-value pairs for a given lobby member.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#membermetadatacount)  
    pub fn lobby_member_metadata_count(&self, lobby_id: LobbyID, user_id: UserID) -> Result<u32> {
        let mut count = 0;

        self.with_lobby_manager(|mgr| unsafe {
            mgr.member_metadata_count.unwrap()(mgr, lobby_id, user_id, &mut count)
        })
        .to_result()?;

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

        self.with_lobby_manager(|mgr| unsafe {
            mgr.get_member_metadata_key.unwrap()(
                mgr,
                lobby_id,
                user_id,
                // XXX: i32 should be u32
                index.try_into().unwrap(),
                &mut key,
            )
        })
        .to_result()?;

        self.with_lobby_manager(|mgr| unsafe {
            mgr.get_member_metadata_value.unwrap()(
                mgr,
                lobby_id,
                user_id,
                // XXX: *mut should be *const
                key.as_ptr() as *mut u8,
                &mut value,
            )
        })
        .to_result()?;

        Ok((
            utils::charbuf_to_str(&key).to_string(),
            utils::charbuf_to_str(&value).to_string(),
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
        callback: impl 'd + FnOnce(Result<()>),
    ) {
        let buffer = buffer.as_ref();

        debug_assert!(u32::try_from(buffer.len()).is_ok());

        self.with_lobby_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(res.to_result()));

            unsafe {
                mgr.send_lobby_message.unwrap()(
                    mgr,
                    lobby_id,
                    // XXX: *mut should be *const
                    buffer.as_ptr() as *mut u8,
                    // XXX: u32 should be u64
                    buffer.len().try_into().unwrap_or(u32::max_value()),
                    ptr,
                    fun,
                )
            }
        })
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
        callback: impl 'd + FnOnce(Result<()>),
    ) {
        let mut tx = std::ptr::null_mut();

        let create = self
            .with_lobby_manager(|mgr| unsafe { mgr.get_search_query.unwrap()(mgr, &mut tx) })
            .to_result();
        if let Err(e) = create {
            return callback(Err(e));
        }

        if let Err(e) = search.process(tx) {
            return callback(Err(e));
        }

        self.with_lobby_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(res.to_result()));

            unsafe { mgr.search.unwrap()(mgr, tx, ptr, fun) }
        })
    }

    /// Returns the number of lobbies found via the search query.
    ///
    /// [`lobby_search`](#method.lobby_search) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbycount)
    pub fn lobby_count(&self) -> u32 {
        let mut count = 0;

        self.with_lobby_manager(|mgr| unsafe { mgr.lobby_count.unwrap()(mgr, &mut count) });

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

        self.with_lobby_manager(|mgr| unsafe {
            mgr.get_lobby_id.unwrap()(
                mgr,
                // XXX: i32 should be u32
                index.try_into().unwrap(),
                &mut lobby_id,
            )
        })
        .to_result()?;

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
        callback: impl 'd + FnOnce(Result<()>),
    ) {
        self.with_lobby_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(res.to_result()));

            unsafe { mgr.connect_voice.unwrap()(mgr, lobby_id, ptr, fun) }
        })
    }

    /// Disconnects from the voice channel of a given lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#disconnectvoice)
    pub fn disconnect_lobby_voice<'d>(
        &'d self,
        lobby_id: LobbyID,
        callback: impl 'd + FnOnce(Result<()>),
    ) {
        self.with_lobby_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(res.to_result()));

            unsafe { mgr.disconnect_voice.unwrap()(mgr, lobby_id, ptr, fun) }
        })
    }

    /// Connects to the networking layer for the given lobby ID.
    ///
    /// Call this when connecting to the lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#connectnetwork)
    pub fn connect_lobby_network(&self, lobby_id: LobbyID) -> Result<()> {
        self.with_lobby_manager(|mgr| unsafe { mgr.connect_network.unwrap()(mgr, lobby_id) })
            .to_result()
    }

    /// Disconnects from the networking layer for the given lobby ID.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#disconnectnetwork)
    pub fn disconnect_lobby_network(&self, lobby_id: LobbyID) -> Result<()> {
        self.with_lobby_manager(|mgr| unsafe { mgr.disconnect_network.unwrap()(mgr, lobby_id) })
            .to_result()
    }

    /// Flushes the network. Call this when you're done sending messages.
    ///
    /// This should appear near the end of your game loop.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#flushnetwork)
    pub fn flush_lobby_network(&self) -> Result<()> {
        self.with_lobby_manager(|mgr| unsafe { mgr.flush_network.unwrap()(mgr) })
            .to_result()
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
        self.with_lobby_manager(|mgr| unsafe {
            mgr.open_network_channel.unwrap()(mgr, lobby_id, channel_id, reliable.into())
        })
        .to_result()
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

        self.with_lobby_manager(|mgr| unsafe {
            mgr.send_network_message.unwrap()(
                mgr,
                lobby_id,
                user_id,
                channel_id,
                // XXX: *mut should be *const
                buffer.as_ptr() as *mut u8,
                // XXX: u32 should be u64
                buffer.len().try_into().unwrap_or(u32::max_value()),
            )
        })
        .to_result()
    }
}
