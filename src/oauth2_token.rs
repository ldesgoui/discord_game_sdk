use crate::prelude::*;

#[derive(Debug)]
pub struct OAuth2Token {
    pub access_token: String,
    pub scopes: Vec<String>,
    pub expires: chrono::NaiveDateTime,
}

impl OAuth2Token {
    pub(crate) fn from_sys(ptr: *const sys::DiscordOAuth2Token) -> Result<Self> {
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
