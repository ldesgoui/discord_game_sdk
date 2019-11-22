use crate::{callbacks::ResultCallback, event, Action, Discord, Result};

/// # Overlay
///
/// <https://discordapp.com/developers/docs/game-sdk/overlay>
impl<'a> Discord<'a> {
    /// <https://discordapp.com/developers/docs/game-sdk/overlay#isenabled>
    pub fn overlay_enabled(&mut self) -> bool {
        let mut enabled = false;

        unsafe {
            ffi!(self
                .get_overlay_manager()
                .is_enabled(&mut enabled as *mut _))
        }

        enabled
    }

    /// <https://discordapp.com/developers/docs/game-sdk/overlay#islocked>
    pub fn overlay_opened(&mut self) -> bool {
        let mut locked = false;

        unsafe { ffi!(self.get_overlay_manager().is_locked(&mut locked as *mut _)) }

        !locked
    }

    /// <https://discordapp.com/developers/docs/game-sdk/overlay#setlocked>
    pub fn set_overlay_opened(
        &mut self,
        opened: bool,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        unsafe {
            ffi!(self
                .get_overlay_manager()
                .set_locked(!opened)
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// <https://discordapp.com/developers/docs/game-sdk/overlay#openactivityinvite>
    pub fn open_invite_overlay(
        &mut self,
        action: Action,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        unsafe {
            ffi!(self
                .get_overlay_manager()
                .open_activity_invite(action.into())
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// `code` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/overlay#openguildinvite>
    pub fn open_guild_invite_overlay(
        &mut self,
        mut code: String,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        code.push('\0');

        unsafe {
            ffi!(self
                .get_overlay_manager()
                .open_guild_invite(code.as_ptr() as *const _)
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// <https://discordapp.com/developers/docs/game-sdk/overlay#openvoicesettings>
    pub fn open_voice_settings(&mut self, callback: impl FnMut(&mut Discord, Result<()>) + 'a) {
        unsafe {
            ffi!(self
                .get_overlay_manager()
                .open_voice_settings()
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// <https://discordapp.com/developers/docs/game-sdk/overlay#ontoggle>
    pub fn recv_overlay_toggle(&'_ self) -> impl '_ + Iterator<Item = event::overlay::Toggle> {
        self.receivers.overlay_toggle.try_iter()
    }
}
