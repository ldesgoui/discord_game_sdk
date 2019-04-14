use crate::{sys, utils::{CStrExt,slice_i8_to_u8}};
use chrono::{offset::TimeZone, DateTime, Utc};
use std::ffi::CStr;
use std::iter::FusedIterator;

#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct OAuth2Token(pub(crate) sys::DiscordOAuth2Token);

impl OAuth2Token {
    get_str!(access_token, access_token);

    pub fn scopes<'a>(
        &'a self,
    ) -> impl Iterator<Item = &'a str> + DoubleEndedIterator + FusedIterator + 'a {
        CStr::from_bytes(slice_i8_to_u8(&self.0.scopes[..]))
            .unwrap()
            .to_str()
            .unwrap()
            .split(' ')
    }

    pub fn expires(&self) -> DateTime<Utc> {
        Utc.timestamp(self.0.expires, 0)
    }
}

impl std::fmt::Debug for OAuth2Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OAuth2Token")
            .field("access_token", &self.access_token())
            .field("scopes", &self.scopes().collect::<Vec<_>>())
            .field("expires", &self.expires())
            .finish()
    }
}
