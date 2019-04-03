use crate::oauth2_token::OAuth2Token;
use crate::prelude::*;

/// # Application
impl Discord {
    pub fn get_current_locale(&self) -> Result<String> {
        let mut locale: sys::DiscordLocale = [0; 128];

        ffi!(self
            .get_application_manager()
            .get_current_locale(&mut locale as *mut _))?;

        Ok(from_cstr(&locale as *const _)?.to_string())
    }

    pub fn get_current_branch(&self) -> Result<String> {
        let mut branch: sys::DiscordBranch = [0; 4096];

        ffi!(self
            .get_application_manager()
            .get_current_branch(&mut branch as *mut _))?;

        Ok(from_cstr(&branch as *const _)?.to_string())
    }

    pub fn validate_or_exit<F>(&self, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let _ = ffi!(self
            .get_application_manager()
            .validate_or_exit(&callback as *const _ as *mut _, Some(simple_callback::<F>)))
        .map_err(|e| callback(Err(e)));
    }

    pub fn get_oauth2_token<F>(&self, mut callback: F)
    where
        F: FnMut(Result<OAuth2Token>),
    {
        let _ = ffi!(self.get_application_manager().get_oauth2_token(
            &callback as *const _ as *mut _,
            Some(get_oauth2_token_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }
}

extern "C" fn get_oauth2_token_callback<F>(
    data: *mut c_void,
    res: sys::EDiscordResult,
    token: *mut sys::DiscordOAuth2Token,
) where
    F: FnMut(Result<OAuth2Token>) + Sized,
{
    if data.is_null() {
        log::error!("SDK invoked callback with null");
        return;
    }
    let callback: &mut F = unsafe { &mut *(data as *mut _) };

    callback(
        res.to_result()
            .and_then(|_| OAuth2Token::from_sys_ptr(token)),
    )
}
