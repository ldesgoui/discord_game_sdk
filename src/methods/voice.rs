use crate::{to_result::ToResult, Discord, Result};

/// # Voice
/// https://discordapp.com/developers/docs/game-sdk/voice
///
/// ## SDK issues
/// Currently, the SDK seems to crash after running the "voice settings updated" event handler
/// with a panic inside panic. This has beem reported. For the meantime, the following functions
/// are not usable.
impl<'a> Discord<'a> {
    // TODO: get_input_mode (no idea how im supposed to deal with shortcuts)
    // TODO: set_input_mode

    pub fn self_muted(&mut self) -> Result<bool> {
        let mut muted = false;

        unsafe { ffi!(self.get_voice_manager().is_self_mute(&mut muted as *mut _)) }.to_result()?;

        Ok(muted)
    }

    pub fn self_deafened(&mut self) -> Result<bool> {
        let mut deafened = false;

        unsafe {
            ffi!(self
                .get_voice_manager()
                .is_self_deaf(&mut deafened as *mut _))
        }
        .to_result()?;

        Ok(deafened)
    }

    pub fn set_self_mute(&mut self, muted: bool) -> Result<()> {
        unsafe { ffi!(self.get_voice_manager().set_self_mute(muted)) }.to_result()
    }

    pub fn set_self_deaf(&mut self, deafened: bool) -> Result<()> {
        unsafe { ffi!(self.get_voice_manager().set_self_deaf(deafened)) }.to_result()
    }

    pub fn local_muted(&mut self, user_id: i64) -> Result<bool> {
        let mut muted = false;

        unsafe {
            ffi!(self
                .get_voice_manager()
                .is_local_mute(user_id, &mut muted as *mut _))
        }
        .to_result()?;

        Ok(muted)
    }

    pub fn local_volume(&mut self, user_id: i64) -> Result<u8> {
        let mut volume = 0;

        unsafe {
            ffi!(self
                .get_voice_manager()
                .get_local_volume(user_id, &mut volume as *mut _))
        }
        .to_result()?;

        Ok(volume)
    }

    pub fn set_local_mute(&mut self, user_id: i64, muted: bool) -> Result<()> {
        unsafe { ffi!(self.get_voice_manager().set_local_mute(user_id, muted)) }.to_result()
    }

    pub fn set_local_volume(&mut self, user_id: i64, volume: u8) -> Result<()> {
        unsafe { ffi!(self.get_voice_manager().set_local_volume(user_id, volume)) }.to_result()
    }
}
