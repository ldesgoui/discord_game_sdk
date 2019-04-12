use crate::prelude::*;

/// # Overlay
impl Discord {
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
        F: FnMut(&mut Discord, Result<()>) + 'static,
    {
        unsafe {
            ffi!(self.get_overlay_manager().set_locked(!opened)(
                ResultCallback::new(callback)
            ))
        }
    }

    pub fn open_invite_overlay<F>(&mut self, action: Action, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'static,
    {
        unsafe {
            ffi!(self
                .get_overlay_manager()
                .open_activity_invite(action.to_sys())(
                ResultCallback::new(callback)
            ))
        }
    }

    // tested
    pub fn open_guild_invite_overlay<F>(&mut self, code: impl AsRef<str>, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'static,
    {
        let code = std::ffi::CString::new(code.as_ref()).unwrap();

        unsafe {
            ffi!(self.get_overlay_manager().open_guild_invite(code.as_ptr())(
                ResultCallback::new(callback)
            ))
        }
    }

    // tested
    pub fn open_voice_settings<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'static,
    {
        unsafe {
            ffi!(self.get_overlay_manager().open_voice_settings()(
                ResultCallback::new(callback)
            ))
        }
    }
}
