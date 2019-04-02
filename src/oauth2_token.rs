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

        Ok(Self {
            access_token: from_cstr(&source.access_token as *const _)?.to_string(),
            scopes: from_cstr(&source.scopes as *const _)?
                .to_string()
                .split(' ')
                .map(String::from)
                .collect(),
            expires: chrono::NaiveDateTime::from_timestamp(source.expires, 0),
        })
    }
}
