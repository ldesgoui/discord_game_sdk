use crate::{callbacks::ResultCallback, sys, to_result::ToResult, Discord, InputMode, Result};

/// # Voice
/// <https://discordapp.com/developers/docs/game-sdk/voice>
impl<'a> Discord<'a> {
    pub fn input_mode(&mut self) -> Result<InputMode> {
        let mut input_mode = InputMode(sys::DiscordInputMode::default());

        unsafe {
            ffi!(self
                .get_voice_manager()
                .get_input_mode(&mut input_mode.0 as *mut _))
        }
        .to_result()?;

        Ok(input_mode)
    }

    pub fn set_input_mode(
        &mut self,
        input_mode: InputMode,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        unsafe {
            ffi!(self
                .get_voice_manager()
                .set_input_mode(input_mode.0)
                .and_then(ResultCallback::new(callback)))
        }
    }

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
