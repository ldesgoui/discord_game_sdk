use crate::{macro_helper::MacroHelper, sys, to_result::ToResult, Result};
use std::collections::HashMap;
use std::ffi::CStr;

#[derive(Clone, Debug, Default)]
pub struct LobbyMemberTransaction<'a> {
    pub(crate) metadata: HashMap<&'a CStr, Option<&'a CStr>>,
}

impl<'a> LobbyMemberTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_metadata(&'a mut self, key: &'a CStr, value: &'a CStr) -> &'a mut Self {
        let _ = self.metadata.insert(key, Some(value));
        self
    }

    pub fn delete_metadata<S>(&'a mut self, key: &'a CStr) -> &'a mut Self {
        let _ = self.metadata.insert(key, None);
        self
    }

    pub(crate) unsafe fn process(
        self,
        ptr: *mut sys::IDiscordLobbyMemberTransaction,
    ) -> Result<()> {
        let tx = MacroHelper { core: ptr };

        for (key, value) in self.metadata {
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
