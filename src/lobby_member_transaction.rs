use crate::{sys, to_result::ToResult, utils, Result};
use std::collections::HashMap;

/// Lobby Member Transaction
///
/// > [Struct in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#data-models-lobbymembertransaction-struct)
#[derive(Clone, Debug, Default)]
pub struct LobbyMemberTransaction {
    pub(crate) metadata: HashMap<String, Option<String>>,
}

impl LobbyMemberTransaction {
    /// Gets a member update transaction.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getmemberupdatetransaction)
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets metadata value under a given key for the user.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `key` and `value` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbymembertransactionsetmetadata)
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

    /// Deletes metadata value under a given key for the user
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `key` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbymembertransactiondeletemetadata)
    pub fn delete_metadata<S>(&mut self, mut key: String) -> &mut Self {
        if !key.ends_with('\0') {
            key.push('\0')
        };

        let _ = self.metadata.insert(key, None);
        self
    }

    pub(crate) fn process(&self, ptr: *mut sys::IDiscordLobbyMemberTransaction) -> Result<()> {
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
