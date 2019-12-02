use crate::{
    callbacks::ResultCallback, event, sys, to_result::ToResult, Discord, InputMode, Result,
};

/// # Voice
///
/// <https://discordapp.com/developers/docs/game-sdk/discord-voice>
impl<'a> Discord<'a> {
    /// Get the voice input mode for the current user.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/discord-voice#getinputmode>
    pub fn input_mode(&self) -> Result<InputMode> {
        let mut input_mode = InputMode(sys::DiscordInputMode::default());

        unsafe {
            ffi!(self
                .get_voice_manager()
                .get_input_mode(&mut input_mode.0 as *mut _))
        }
        .to_result()?;

        Ok(input_mode)
    }

    /// Sets a new voice input mode for the user.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/discord-voice#setinputmode>
    pub fn set_input_mode(
        &self,
        input_mode: InputMode,
        callback: impl 'a + FnMut(&Discord, Result<()>),
    ) {
        unsafe {
            ffi!(self
                .get_voice_manager()
                .set_input_mode(input_mode.0)
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Whether the current user is muted.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/discord-voice#isselfmute>
    pub fn self_muted(&self) -> Result<bool> {
        let mut muted = false;

        unsafe { ffi!(self.get_voice_manager().is_self_mute(&mut muted as *mut _)) }.to_result()?;

        Ok(muted)
    }

    /// Whether the current used is deafened.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/discord-voice#isselfdeaf>
    pub fn self_deafened(&self) -> Result<bool> {
        let mut deafened = false;

        unsafe {
            ffi!(self
                .get_voice_manager()
                .is_self_deaf(&mut deafened as *mut _))
        }
        .to_result()?;

        Ok(deafened)
    }

    /// Mutes or unmutes the current user.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/discord-voice#setselfmute>
    pub fn set_self_mute(&self, muted: bool) -> Result<()> {
        unsafe { ffi!(self.get_voice_manager().set_self_mute(muted)) }.to_result()
    }

    /// Deafens or undeafens the current user.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/discord-voice#setselfdeaf>
    pub fn set_self_deaf(&self, deafened: bool) -> Result<()> {
        unsafe { ffi!(self.get_voice_manager().set_self_deaf(deafened)) }.to_result()
    }

    /// Whether a given user is locally muted.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/discord-voice#islocalmute>
    pub fn local_muted(&self, user_id: i64) -> Result<bool> {
        let mut muted = false;

        unsafe {
            ffi!(self
                .get_voice_manager()
                .is_local_mute(user_id, &mut muted as *mut _))
        }
        .to_result()?;

        Ok(muted)
    }

    /// Gets the local volume for a given user, in the range `[0..=200]`, `100` being the default.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/discord-voice#getlocalvolume>
    pub fn local_volume(&self, user_id: i64) -> Result<u8> {
        let mut volume = 0;

        unsafe {
            ffi!(self
                .get_voice_manager()
                .get_local_volume(user_id, &mut volume as *mut _))
        }
        .to_result()?;

        Ok(volume)
    }

    /// Locally mutes or unmutes a given user.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/discord-voice#setlocalmute>
    pub fn set_local_mute(&self, user_id: i64, muted: bool) -> Result<()> {
        unsafe { ffi!(self.get_voice_manager().set_local_mute(user_id, muted)) }.to_result()
    }

    /// Sets the local volume for a given user, in the range `[0..=200]`, `100` being the default.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/discord-voice#setlocalvolume>
    pub fn set_local_volume(&self, user_id: i64, volume: u8) -> Result<()> {
        unsafe { ffi!(self.get_voice_manager().set_local_volume(user_id, volume)) }.to_result()
    }

    /// Fires when the current user has updated their voice settings.
    pub fn recv_voice_settings_update(
        &self,
    ) -> impl '_ + Iterator<Item = event::VoiceSettingsUpdate> {
        self.receivers.voice_settings_update.try_iter()
    }
}
