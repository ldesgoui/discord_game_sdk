use crate::prelude::*;

#[derive(Debug)]
pub struct OAuth2Token {
    pub access_token: String,
    pub scopes: Vec<String>,
    pub expires: chrono::NaiveDateTime,
}

impl FromSys for OAuth2Token {
    type Source = sys::DiscordOAuth2Token;

    fn from_sys(source: &Self::Source) -> Result<Self> {
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
