use crate::activity::Action;
use crate::prelude::*;

/// # Overlay
impl Discord {
    pub fn is_overlay_enabled(&self) -> Result<bool> {
        let mut enabled = false;

        ffi!(self
            .get_overlay_manager()
            .is_enabled(&mut enabled as *mut _))?;

        Ok(enabled)
    }

    pub fn is_overlay_locked(&self) -> Result<bool> {
        let mut locked = false;

        ffi!(self.get_overlay_manager().is_locked(&mut locked as *mut _))?;

        Ok(locked)
    }

    pub fn set_overlay_locked<F>(&self, locked: bool, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let _ = ffi!(self.get_overlay_manager().set_locked(
            locked,
            &mut callback as *mut _ as *mut _,
            Some(simple_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }

    pub fn open_invite_overlay<F>(&self, action: Action, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let _ = ffi!(self.get_overlay_manager().open_activity_invite(
            action.to_sys(),
            &mut callback as *mut _ as *mut _,
            Some(simple_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }

    pub fn open_guild_invite_overlay<S, F>(&self, code: S, mut callback: F)
    where
        S: AsRef<str>,
        F: FnMut(Result<()>),
    {
        let _ = std::ffi::CString::new(code.as_ref())
            .map_err(DeveloperViolation::from)
            .map_err(Error::from)
            .and_then(|cstring| {
                ffi!(self.get_overlay_manager().open_guild_invite(
                    cstring.as_ptr(),
                    &mut callback as *mut _ as *mut _,
                    Some(simple_callback::<F>)
                ))
            })
            .map_err(|e| callback(Err(e)));
    }

    pub fn open_voice_settings<F>(&self, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let _ = ffi!(self.get_overlay_manager().open_voice_settings(
            &mut callback as *mut _ as *mut _,
            Some(simple_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }
}
