use crate::{macro_helper::MacroHelper, sys, to_result::ToResult, LobbyKind, Result};
use std::collections::HashMap;
use std::ffi::CStr;

/// Lobby Transaction
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies>
#[derive(Clone, Debug, Default)]
pub struct LobbyTransaction<'a> {
    pub(crate) kind: Option<LobbyKind>,
    pub(crate) owner: Option<i64>,
    pub(crate) capacity: Option<u32>,
    pub(crate) locked: Option<bool>,
    pub(crate) metadata: HashMap<&'a CStr, Option<&'a CStr>>,
}

impl<'a> LobbyTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsettype>
    pub fn kind(&mut self, kind: LobbyKind) -> &mut Self {
        self.kind = Some(kind);
        self
    }

    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsetowner>
    pub fn owner(&mut self, user_id: i64) -> &mut Self {
        self.owner = Some(user_id);
        self
    }

    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsetcapacity>
    pub fn capacity(&mut self, capacity: u32) -> &mut Self {
        self.capacity = Some(capacity);
        self
    }

    /// `key` and `value` must also be valid UTF-8
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactionsetmetadata>
    pub fn add_metadata(&mut self, key: &'a CStr, value: &'a CStr) -> &mut Self {
        let _ = self.metadata.insert(key, Some(value));
        self
    }

    /// `key` must also be valid UTF-8
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbytransactiondeletemetadata>
    pub fn delete_metadata<S>(&mut self, key: &'a CStr) -> &mut Self {
        let _ = self.metadata.insert(key, None);
        self
    }

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
