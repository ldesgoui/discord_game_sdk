use crate::{sys, to_result::ToResult, utils, LobbyKind, Result, UserID};
use std::collections::HashMap;

/// Lobby Transaction
///
/// > [Struct in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#data-models-lobbytransaction-struct)
#[derive(Clone, Debug, Default)]
pub struct LobbyTransaction {
    pub(crate) kind: Option<LobbyKind>,
    pub(crate) owner: Option<UserID>,
    pub(crate) capacity: Option<u32>,
    pub(crate) locked: Option<bool>,
    pub(crate) metadata: HashMap<String, Option<String>>,
}

impl LobbyTransaction {
    /// Gets a Lobby transaction used for creating or updating a new lobby.
    ///
    /// > [`GetLobbyCreateTransaction` in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getlobbycreatetransaction)  
    /// > [`GetLobbyUpdateTransaction` in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getlobbyupdatetransaction)
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks the lobby as private or public
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsettype)
    pub fn kind(&mut self, kind: LobbyKind) -> &mut Self {
        self.kind = Some(kind);
        self
    }

    /// Sets the ID of the user owning the lobby
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsetowner)
    pub fn owner(&mut self, user_id: UserID) -> &mut Self {
        self.owner = Some(user_id);
        self
    }

    /// Sets the maximum amount of players that can join
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsetcapacity)
    pub fn capacity(&mut self, capacity: u32) -> &mut Self {
        self.capacity = Some(capacity);
        self
    }

    /// Set metadata value under a given key for the lobby
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `key` and `value` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsetmetadata)
    pub fn add_metadata(&mut self, mut key: String, mut value: String) -> &mut Self {
        if !key.ends_with('\0') {
            key.push('\0')
        };

        if !value.ends_with('\0') {
            value.push('\0')
        };

        let _ = self.metadata.insert(key, Some(value));
        self
    }

    /// Deletes metadata value under a given key for the lobby
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `key` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactiondeletemetadata)
    pub fn delete_metadata<S>(&mut self, mut key: String) -> &mut Self {
        if !key.ends_with('\0') {
            key.push('\0')
        };

        let _ = self.metadata.insert(key, None);
        self
    }

    /// Sets whether the lobby is locked or not. When locked, new users cannot join
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsetlocked)
    pub fn locked(&mut self, locked: bool) -> &mut Self {
        self.locked = Some(locked);
        self
    }

    pub(crate) fn process(&self, ptr: *mut sys::IDiscordLobbyTransaction) -> Result<()> {
        if let Some(kind) = self.kind {
            utils::with_tx(ptr, |tx| unsafe { tx.set_type.unwrap()(tx, kind.into()) })
                .to_result()?;
        }

        if let Some(user_id) = self.owner {
            utils::with_tx(ptr, |tx| unsafe { tx.set_owner.unwrap()(tx, user_id) }).to_result()?;
        }

        if let Some(capacity) = self.capacity {
            utils::with_tx(ptr, |tx| unsafe { tx.set_capacity.unwrap()(tx, capacity) })
                .to_result()?;
        }

        if let Some(locked) = self.locked {
            utils::with_tx(ptr, |tx| unsafe { tx.set_locked.unwrap()(tx, locked) }).to_result()?;
        }

        for (key, value) in &self.metadata {
            match value {
                Some(value) => {
                    utils::with_tx(ptr, |tx| unsafe {
                        tx.set_metadata.unwrap()(
                            tx,
                            // XXX: *mut should be *const
                            key.as_ptr() as *mut u8,
                            // XXX: *mut should be *const
                            value.as_ptr() as *mut u8,
                        )
                    })
                    .to_result()?;
                }

                None => {
                    utils::with_tx(ptr, |tx| unsafe {
                        tx.delete_metadata.unwrap()(
                            tx,
                            // XXX: *mut should be *const
                            key.as_ptr() as *mut u8,
                        )
                    })
                    .to_result()?;
                }
            }
        }

        Ok(())
    }
}
