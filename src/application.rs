use crate::error::*;
use crate::utils::*;
use crate::Discord;
use discord_game_sdk_sys as sys;
use std::os::raw::c_void;

/// Application
impl Discord {
    pub fn get_current_locale(&self) -> Result<String> {
        let &mut mut locale: &mut sys::DiscordLocale = &mut [0; 128];

        ffi!(self
            .get_application_manager()
            .get_current_locale(&mut locale as *mut _))?;

        Ok(
            unsafe { std::ffi::CStr::from_ptr(&locale as *const _ as *const _) }
                .to_str()
                .map_err(BindingsViolation::from)?
                .to_string(),
        )
    }

    pub fn get_current_branch(&self) -> Result<String> {
        let &mut mut branch: &mut sys::DiscordBranch = &mut [0; 4096];

        ffi!(self
            .get_application_manager()
            .get_current_branch(&mut branch as *mut _))?;

        Ok(
            unsafe { std::ffi::CStr::from_ptr(&branch as *const _ as *const _) }
                .to_str()
                .map_err(BindingsViolation::from)?
                .to_string(),
        )
    }

    pub fn validate_or_exit<F>(&self, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        // TODO: catch ffi! errors and send them to callback
        if let Err(err) = ffi!(self
            .get_application_manager()
            .validate_or_exit(&callback as *const _ as *mut _, Some(simple_callback::<F>)))
        {
            callback(Err(err))
        }
    }

    pub fn get_oauth2_token<F>(&self, callback: F)
    where
        F: FnMut(Result<DiscordOAuth2Token>),
    {
        // TODO: catch ffi! errors and send them to callback
        let _ = ffi!(self
            .get_application_manager()
            .get_oauth2_token(&callback as *const _ as *mut _, Some(get_oauth2_token::<F>)));
    }
}

extern "C" fn get_oauth2_token<F>(
    data: *mut c_void,
    res: sys::EDiscordResult,
    token: *mut sys::DiscordOAuth2Token,
) where
    F: FnMut(Result<DiscordOAuth2Token>) + Sized,
{
    if data.is_null() {
        log::error!("SDK invoked callback with null");
        return;
    }
    let callback: &mut F = unsafe { &mut *(data as *mut _) };

    match res.to_result() {
        Err(err) => callback(Err(err)),
        Ok(()) => callback(DiscordOAuth2Token::from_sys(token)),
    }
}

#[derive(Debug)]
pub struct DiscordOAuth2Token {
    pub access_token: String,
    pub scopes: Vec<String>,
    pub expires: chrono::NaiveDateTime,
}

impl DiscordOAuth2Token {
    fn from_sys(ptr: *const sys::DiscordOAuth2Token) -> Result<Self> {
        let source = unsafe { ptr.as_ref() }.ok_or(BindingsViolation::NullPointer)?;

        let access_token = unsafe { std::ffi::CStr::from_ptr(&source.access_token as *const _) }
            .to_str()
            .map_err(BindingsViolation::from)?
            .to_string();

        let scopes = unsafe { std::ffi::CStr::from_ptr(&source.scopes as *const _) }
            .to_str()
            .map_err(BindingsViolation::from)?
            .split(' ')
            .map(String::from)
            .collect();

        let expires = chrono::NaiveDateTime::from_timestamp(source.expires, 0);

        Ok(Self {
            access_token,
            scopes,
            expires,
        })
    }
}
