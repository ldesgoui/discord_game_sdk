use crate::{callbacks::ResultCallback, Action, Discord, Result};
use std::ffi::CStr;

/// # Overlay
/// https://discordapp.com/developers/docs/game-sdk/overlay
impl<'a> Discord<'a> {
    // tested in terminal, was returning false
    // kinda inconclusive
    pub fn overlay_enabled(&mut self) -> bool {
        let mut enabled = false;

        unsafe {
            ffi!(self
                .get_overlay_manager()
                .is_enabled(&mut enabled as *mut _))
        }

        enabled
    }

    // tested in terminal, was returning true until event::overlay::Toggled
    // kinda inconclusive
    pub fn overlay_opened(&mut self) -> bool {
        let mut locked = false;

        unsafe { ffi!(self.get_overlay_manager().is_locked(&mut locked as *mut _)) }

        !locked
    }

    pub fn set_overlay_opened<F>(&mut self, opened: bool, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'a,
    {
        unsafe {
            ffi!(self
                .get_overlay_manager()
                .set_locked(!opened)
                .and_then(ResultCallback::new(callback)))
        }
    }

    pub fn open_invite_overlay<F>(&mut self, action: Action, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'a,
    {
        unsafe {
            ffi!(self
                .get_overlay_manager()
                .open_activity_invite(action.into())
                .and_then(ResultCallback::new(callback)))
        }
    }

    // tested
    pub fn open_guild_invite_overlay<F>(&mut self, code: impl AsRef<CStr>, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'a,
    {
        unsafe {
            ffi!(self
                .get_overlay_manager()
                .open_guild_invite(code.as_ref().as_ptr())
                .and_then(ResultCallback::new(callback)))
        }
    }

    // tested
    pub fn open_voice_settings<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'a,
    {
        unsafe {
            ffi!(self
                .get_overlay_manager()
                .open_voice_settings()
                .and_then(ResultCallback::new(callback)))
        }
    }
}
