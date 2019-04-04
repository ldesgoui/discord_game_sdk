use crate::prelude::*;

/// # Voice
impl<'a> Discord<'a> {
    // TODO: get_input_mode (no idea how im supposed to deal with shortcuts)
    // TODO: set_input_mode

    pub fn self_mute(&mut self) -> Result<bool> {
        let mut muted = false;

        unsafe { ffi!(self.get_voice_manager().is_self_mute(&mut muted as *mut _)) }.to_result()?;

        Ok(muted)
    }

    pub fn self_deaf(&mut self) -> Result<bool> {
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

    pub fn local_mute(&mut self, user_id: i64) -> Result<bool> {
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
        let mut volume = 0u8;

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

    pub fn voice_events_reader(&mut self) -> shrev::ReaderId<event::Voice> {
        self.voice_channel.register_reader()
    }

    pub fn voice_events(
        &self,
        reader: &mut shrev::ReaderId<event::Voice>,
    ) -> shrev::EventIterator<event::Voice> {
        self.voice_channel.read(reader)
    }
}
