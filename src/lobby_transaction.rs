use crate::prelude::*;

pub struct LobbyTransaction<'a> {
    pub(crate) core: &'a mut sys::IDiscordLobbyTransaction,
}

impl<'a> LobbyTransaction<'a> {
    //pub fn set_kind(&mut self, kind: LobbyKind) -> Result<()> {
    //    unsafe { ffi!(self.set_type(kind.to_sys())) }.to_result()
    //}

    //pub fn set_owner(&mut self, user_id: i64) -> Result<()> {
    //    unsafe { ffi!(self.set_owner(user_id)) }.to_result()
    //}

    //pub fn set_capacity(&mut self, capacity: u32) -> Result<()> {
    //    unsafe { ffi!(self.set_capacity(capacity)) }.to_result()
    //}

    //pub fn set_metadata(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<()> {
    //    let key = CString::new(key.as_ref()).unwrap();
    //    let value = CString::new(value.as_ref()).unwrap();

    //    unsafe { ffi!(self.set_metadata(key.as_ptr() as *mut _, value.as_ptr() as *mut _)) }
    //        .to_result()
    //}

    //pub fn delete_metadata<S>(&mut self, key: impl AsRef<str>) -> Result<()> {
    //    let key = CString::new(key.as_ref()).unwrap();

    //    unsafe { ffi!(self.delete_metadata(key.as_ptr() as *mut _)) }.to_result()
    //}

    //pub fn set_locked(&mut self, locked: bool) -> Result<()> {
    //    unsafe { ffi!(self.set_locked(locked)) }.to_result()
    //}
}

pub struct LobbyMemberTransaction<'a> {
    pub(crate) core: &'a mut sys::IDiscordLobbyMemberTransaction,
}

impl<'a> LobbyMemberTransaction<'a> {
    //pub fn set_metadata(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<()> {
    //    let key = CString::new(key.as_ref()).unwrap();
    //    let value = CString::new(value.as_ref()).unwrap();

    //    unsafe { ffi!(self.set_metadata(key.as_ptr() as *mut _, value.as_ptr() as *mut _)) }
    //        .to_result()
    //}

    //pub fn delete_metadata<S>(&mut self, key: impl AsRef<str>) -> Result<()> {
    //    let key = CString::new(key.as_ref()).unwrap();

    //    unsafe { ffi!(self.delete_metadata(key.as_ptr() as *mut _)) }.to_result()
    //}
}
