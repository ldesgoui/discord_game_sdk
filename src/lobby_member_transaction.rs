use crate::{macro_helper::MacroHelper, sys, to_result::ToResult, Result};
use std::collections::HashMap;

/// Lobby Member Transaction
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies>
#[derive(Clone, Debug, Default)]
pub struct LobbyMemberTransaction {
    pub(crate) metadata: HashMap<String, Option<String>>,
}

impl LobbyMemberTransaction {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets metadata value under a given key for the current user
    ///
    /// `key` and `value` must not contain any nul bytes, both will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbymembertransactionsetmetadata>
    pub fn add_metadata(&mut self, mut key: String, mut value: String) -> &mut Self {
        key.push('\0');
        value.push('\0');

        let _ = self.metadata.insert(key, Some(value));

        self
    }

    /// Deletes metadata value under a given key for the current user
    ///
    /// `key` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbymembertransactiondeletemetadata>
    pub fn delete_metadata<S>(&mut self, mut key: String) -> &mut Self {
        key.push('\0');
        let _ = self.metadata.insert(key, None);
        self
    }

    pub(crate) unsafe fn process(
        &self,
        ptr: *mut sys::IDiscordLobbyMemberTransaction,
    ) -> Result<()> {
        let tx = MacroHelper { core: ptr };

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
