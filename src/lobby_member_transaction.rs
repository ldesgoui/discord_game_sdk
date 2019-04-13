use crate::{
    macro_helper::MacroHelper, metadata_update::MetadataUpdate, sys, to_result::ToResult,
    DiscordResult,
};
use std::ffi::CString;

#[derive(Clone, Debug, Default)]
pub struct LobbyMemberTransaction<'a> {
    pub(crate) metadata: Vec<MetadataUpdate<'a>>,
}

impl<'a> LobbyMemberTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_metadata(&'a mut self, key: &'a str, value: &'a str) -> &'a mut Self {
        self.metadata.push(MetadataUpdate::Add(key, value));
        self
    }

    pub fn delete_metadata<S>(&'a mut self, key: &'a str) -> &'a mut Self {
        self.metadata.push(MetadataUpdate::Delete(key));
        self
    }

    pub(crate) unsafe fn process(
        self,
        ptr: *mut sys::IDiscordLobbyMemberTransaction,
    ) -> DiscordResult<()> {
        let tx = MacroHelper { core: ptr };

        for meta in self.metadata {
            match meta {
                MetadataUpdate::Add(key, value) => {
                    let key = CString::new(key).unwrap();
                    let value = CString::new(value).unwrap();
                    ffi!(tx.set_metadata(key.as_ptr() as *mut _, value.as_ptr() as *mut _))
                        .to_result()?;
                }
                MetadataUpdate::Delete(key) => {
                    let key = CString::new(key).unwrap();
                    ffi!(tx.delete_metadata(key.as_ptr() as *mut _)).to_result()?;
                }
            }
        }

        Ok(())
    }
}
