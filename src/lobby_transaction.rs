use crate::{sys, to_result::ToResult, utils::MacroHelper, LobbyKind, Result};
use std::collections::HashMap;

/// Lobby Transaction
///
/// > [Struct in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#data-models-lobbytransaction-struct)
#[derive(Clone, Debug, Default)]
pub struct LobbyTransaction {
    pub(crate) kind: Option<LobbyKind>,
    pub(crate) owner: Option<i64>,
    pub(crate) capacity: Option<u32>,
    pub(crate) locked: Option<bool>,
    pub(crate) metadata: HashMap<String, Option<String>>,
}

impl LobbyTransaction {
    /// Gets a Lobby transaction used for creating or updating a new lobby.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getlobbycreatetransaction)  
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getlobbyupdatetransaction)
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
    pub fn owner(&mut self, user_id: i64) -> &mut Self {
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
    /// A nul byte will be appended to `key` and `value` if necessary.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsetmetadata)
    pub fn add_metadata(&mut self, mut key: String, mut value: String) -> &mut Self {
        if !key.contains('\0') {
            key.push('\0')
        };

        if !value.contains('\0') {
            value.push('\0')
        };

        let _ = self.metadata.insert(key, Some(value));
        self
    }

    /// Deletes metadata value under a given key for the lobby
    ///
    /// A nul byte will be appended to `key` if necessary.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactiondeletemetadata)
    pub fn delete_metadata<S>(&mut self, mut key: String) -> &mut Self {
        if !key.contains('\0') {
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

    pub(crate) unsafe fn process(&self, ptr: *mut sys::IDiscordLobbyTransaction) -> Result<()> {
        let tx = MacroHelper::new(ptr);

        if let Some(kind) = self.kind {
            ffi!(tx.set_type(kind.into())).to_result()?;
        }

        if let Some(user_id) = self.owner {
            ffi!(tx.set_owner(user_id)).to_result()?;
        }

        if let Some(capacity) = self.capacity {
            ffi!(tx.set_capacity(capacity)).to_result()?;
        }

        if let Some(locked) = self.locked {
            ffi!(tx.set_locked(locked)).to_result()?;
        }

        for (key, value) in &self.metadata {
            match value {
                Some(value) => {
                    ffi!(tx.set_metadata(
                        // XXX: *mut should be *const
                        key.as_ptr() as *mut _,
                        // XXX: *mut should be *const
                        value.as_ptr() as *mut _
                    ))
                    .to_result()?;
                }

                None => {
                    ffi!(tx.delete_metadata(
                        // XXX: *mut should be *const
                        key.as_ptr() as *mut _
                    ))
                    .to_result()?;
                }
            }
        }

        Ok(())
    }
}
