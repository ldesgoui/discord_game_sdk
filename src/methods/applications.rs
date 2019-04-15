use crate::{
    callbacks::{ResultCallback, ResultFromPtrCallback, ResultStringCallback},
    sys,
    utils::cstr_to_str,
    Discord, DiscordResult, OAuth2Token,
};
use std::mem::size_of;

/// # Application
impl<'a> Discord<'a> {
    // tested, returns "en-US" and similar
    pub fn current_locale(&mut self) -> String {
        let mut locale: sys::DiscordLocale = [0; size_of::<sys::DiscordLocale>()];

        unsafe {
            ffi!(self
                .get_application_manager()
                .get_current_locale(&mut locale as *mut _))
        }

        cstr_to_str(&locale[..]).to_string()
    }

    // tested, returns "master" or whichever `dispatch` branch is in use
    pub fn current_branch(&mut self) -> String {
        let mut branch: sys::DiscordBranch = [0; size_of::<sys::DiscordBranch>()];

        unsafe {
            ffi!(self
                .get_application_manager()
                .get_current_branch(&mut branch as *mut _))
        }

        cstr_to_str(&branch[..]).to_string()
    }

    // tested, hasn't failed yet
    pub fn validate_or_exit<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Discord, DiscordResult<()>) + 'a,
    {
        unsafe {
            ffi!(self
                .get_application_manager()
                .validate_or_exit()
                .and_then(ResultCallback::new(callback)))
        }
    }

    // tested
    pub fn oauth2_token<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Discord, DiscordResult<OAuth2Token>) + 'a,
    {
        unsafe {
            ffi!(self
                .get_application_manager()
                .get_oauth2_token()
                .and_then(ResultFromPtrCallback::new(callback)))
        }
    }

    // tested
    pub fn app_ticket<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Discord, DiscordResult<String>) + 'a,
    {
        unsafe {
            ffi!(self
                .get_application_manager()
                .get_ticket()
                .and_then(ResultStringCallback::new(callback)))
        }
    }
}
