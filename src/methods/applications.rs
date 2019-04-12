use crate::prelude::*;

/// # Application
impl Discord {
    // tested, returns "en-US" and similar
    pub fn current_locale(&mut self) -> String {
        let mut locale: sys::DiscordLocale = [0; size_of::<sys::DiscordLocale>()];

        unsafe {
            ffi!(self
                .get_application_manager()
                .get_current_locale(&mut locale as *mut _))
        }

        unsafe { string_from_cstr(&locale as *const _) }
    }

    // tested, returns "master" or whichever `dispatch` branch is in use
    pub fn current_branch(&mut self) -> String {
        let mut branch: sys::DiscordBranch = [0; size_of::<sys::DiscordBranch>()];

        unsafe {
            ffi!(self
                .get_application_manager()
                .get_current_branch(&mut branch as *mut _))
        }

        unsafe { string_from_cstr(&branch as *const _) }
    }

    // tested, hasn't failed yet
    pub fn validate_or_exit<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'static,
    {
        unsafe {
            ffi!(self.get_application_manager().validate_or_exit()(
                ResultCallback::new(callback)
            ))
        }
    }

    // tested
    pub fn oauth2_token<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Discord, Result<OAuth2Token>) + 'static,
    {
        unsafe {
            ffi!(self.get_application_manager().get_oauth2_token()(
                ResultFromSysPtrCallback::new(callback)
            ))
        }
    }

    // tested
    pub fn app_ticket<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Discord, Result<String>) + 'static,
    {
        unsafe {
            ffi!(self.get_application_manager().get_ticket()(
                ResultStringCallback::new(callback)
            ))
        }
    }
}
