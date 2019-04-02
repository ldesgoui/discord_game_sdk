use crate::prelude::*;

/// # Voice
impl Discord {
    // TODO: get_input_mode (no idea how im supposed to deal with shortcuts)
    // TODO: set_input_mode

    pub fn is_self_mute(&self) -> Result<bool> {
        let mut muted = false;

        ffi!(self.get_voice_manager().is_self_mute(&mut muted as *mut _))?;

        Ok(muted)
    }

    pub fn is_self_deaf(&self) -> Result<bool> {
        let mut deafened = false;

        ffi!(self
            .get_voice_manager()
            .is_self_deaf(&mut deafened as *mut _))?;

        Ok(deafened)
    }

    pub fn set_self_mute(&self, muted: bool) -> Result<()> {
        ffi!(self.get_voice_manager().set_self_mute(muted))
    }

    pub fn set_self_deaf(&self, deafened: bool) -> Result<()> {
        ffi!(self.get_voice_manager().set_self_deaf(deafened))
    }

    pub fn is_local_mute(&self, user_id: i64) -> Result<bool> {
        let mut muted = false;

        ffi!(self
            .get_voice_manager()
            .is_local_mute(user_id, &mut muted as *mut _))?;

        Ok(muted)
    }

    pub fn get_local_volume(&self, user_id: i64) -> Result<u8> {
        let mut volume = 0u8;

        ffi!(self
            .get_voice_manager()
            .get_local_volume(user_id, &mut volume as *mut _))?;

        Ok(volume)
    }

    pub fn set_local_mute(&self, user_id: i64, muted: bool) -> Result<()> {
        ffi!(self.get_voice_manager().set_local_mute(user_id, muted))
    }

    pub fn set_local_volume(&self, user_id: i64, volume: u8) -> Result<()> {
        ffi!(self.get_voice_manager().set_local_volume(user_id, volume))
    }
}
