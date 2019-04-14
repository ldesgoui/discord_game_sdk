use crate::{macro_helper::MacroHelper, sys, to_result::ToResult, DiscordResult, LobbyKind};
use std::collections::HashMap;
use std::ffi::CStr;

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

    pub fn kind(&'a mut self, kind: LobbyKind) -> &'a mut Self {
        self.kind = Some(kind);
        self
    }

    pub fn owner(&'a mut self, user_id: i64) -> &'a mut Self {
        self.owner = Some(user_id);
        self
    }

    pub fn capacity(&'a mut self, capacity: u32) -> &'a mut Self {
        self.capacity = Some(capacity);
        self
    }

    pub fn add_metadata(&'a mut self, key: &'a CStr, value: &'a CStr) -> &'a mut Self {
        let _ = self.metadata.insert(key, Some(value));
        self
    }

    pub fn delete_metadata<S>(&'a mut self, key: &'a CStr) -> &'a mut Self {
        let _ = self.metadata.insert(key, None);
        self
    }

    pub fn locked(&'a mut self, locked: bool) -> &'a mut Self {
        self.locked = Some(locked);
        self
    }

    pub(crate) unsafe fn process(
        self,
        ptr: *mut sys::IDiscordLobbyTransaction,
    ) -> DiscordResult<()> {
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
