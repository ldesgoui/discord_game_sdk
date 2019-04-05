use crate::prelude::*;

/// # Application
impl<'a> Discord<'a> {
    pub fn current_locale(&mut self) -> String {
        let mut locale: sys::DiscordLocale = [0; 128];

        unsafe {
            ffi!(self
                .get_application_manager()
                .get_current_locale(&mut locale as *mut _))
        }

        unsafe { string_from_cstr(&locale as *const _) }
    }

    pub fn current_branch(&mut self) -> String {
        let mut branch: sys::DiscordBranch = [0; 4096];

        unsafe {
            ffi!(self
                .get_application_manager()
                .get_current_branch(&mut branch as *mut _))
        }

        unsafe { string_from_cstr(&branch as *const _) }
    }

    pub fn validate_or_exit<F>(&mut self, callback: F)
    where
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_application_manager().validate_or_exit(
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        }
    }

    pub fn oauth2_token<F>(&mut self, callback: F)
    where
        F: FnMut(Result<OAuth2Token>),
    {
        unsafe {
            ffi!(self.get_application_manager().get_oauth2_token(
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result_from_sys::<F, OAuth2Token>)
            ))
        }
    }
}