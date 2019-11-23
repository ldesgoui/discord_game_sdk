use crate::{macro_helper::MacroHelper, sys, to_result::ToResult, LobbyKind, Result};
use std::collections::HashMap;

/// Lobby Transaction
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies>
#[derive(Clone, Debug, Default)]
pub struct LobbyTransaction {
    pub(crate) kind: Option<LobbyKind>,
    pub(crate) owner: Option<i64>,
    pub(crate) capacity: Option<u32>,
    pub(crate) locked: Option<bool>,
    pub(crate) metadata: HashMap<String, Option<String>>,
}

impl LobbyTransaction {
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks the lobby as private or public
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsettype>
    pub fn kind(&mut self, kind: LobbyKind) -> &mut Self {
        self.kind = Some(kind);
        self
    }

    /// Sets the ID of the user owning the lobby
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsetowner>
    pub fn owner(&mut self, user_id: i64) -> &mut Self {
        self.owner = Some(user_id);
        self
    }

    /// Sets the maximum amount of players that can join
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsetcapacity>
    pub fn capacity(&mut self, capacity: u32) -> &mut Self {
        self.capacity = Some(capacity);
        self
    }

    /// Set metadata value under a given key for the lobby
    ///
    /// `key` and `value` must not contain any nul bytes, both will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsetmetadata>
    pub fn add_metadata(&mut self, mut key: String, mut value: String) -> &mut Self {
        key.push('\0');
        value.push('\0');
        let _ = self.metadata.insert(key, Some(value));
        self
    }

    /// Deletes metadata value under a given key for the lobby
    ///
    /// `key` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactiondeletemetadata>
    pub fn delete_metadata<S>(&mut self, mut key: String) -> &mut Self {
        key.push('\0');
        let _ = self.metadata.insert(key, None);
        self
    }

    /// Sets whether the lobby is locked or not. When locked, new users cannot join
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsetlocked>
    pub fn locked(&mut self, locked: bool) -> &mut Self {
        self.locked = Some(locked);
        self
    }

    pub(crate) unsafe fn process(&self, ptr: *mut sys::IDiscordLobbyTransaction) -> Result<()> {
        let tx = MacroHelper { core: ptr };

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
                    ffi!(tx.set_metadata(key.as_ptr() as *mut _, value.as_ptr() as *mut _))
                        .to_result()?;
                }

                None => {
                    ffi!(tx.delete_metadata(key.as_ptr() as *mut _)).to_result()?;
                }
            }
        }

        Ok(())
    }
}
