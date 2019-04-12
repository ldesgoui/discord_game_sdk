use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct LobbyTransaction<'a> {
    pub(crate) kind: Option<LobbyKind>,
    pub(crate) owner: Option<i64>,
    pub(crate) capacity: Option<u32>,
    pub(crate) locked: Option<bool>,
    pub(crate) metadata: Vec<MetadataUpdate<'a>>,
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

    pub fn add_metadata(&'a mut self, key: &'a str, value: &'a str) -> &'a mut Self {
        self.metadata.push(MetadataUpdate::Add(key, value));
        self
    }

    pub fn delete_metadata<S>(&'a mut self, key: &'a str) -> &'a mut Self {
        self.metadata.push(MetadataUpdate::Delete(key));
        self
    }

    pub fn locked(&'a mut self, locked: bool) -> &'a mut Self {
        self.locked = Some(locked);
        self
    }

    pub(crate) unsafe fn process(self, ptr: *mut sys::IDiscordLobbyTransaction) -> Result<()> {
        let tx = MacroHelper { core: ptr };

        if let Some(kind) = self.kind {
            ffi!(tx.set_type(kind.to_sys())).to_result()?;
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
    ) -> Result<()> {
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

#[derive(Clone, Debug)]
pub(crate) enum MetadataUpdate<'a> {
    Add(&'a str, &'a str),
    Delete(&'a str),
}
