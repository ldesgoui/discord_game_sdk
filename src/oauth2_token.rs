use crate::sys;
use chrono::{offset::TimeZone, DateTime, Utc};
use std::ffi::CStr;
use std::iter::FusedIterator;
use std::mem::transmute;

#[derive(Clone, Copy, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct OAuth2Token(pub(crate) sys::DiscordOAuth2Token);

impl OAuth2Token {
    str_field!(access_token, access_token);

    pub fn scopes(&self) -> impl Iterator + DoubleEndedIterator + FusedIterator {
        CStr::from_bytes_with_nul(unsafe { transmute(&self.0.scopes[..]) })
            .unwrap()
            .to_str()
            .unwrap()
            .split(' ')
    }

    pub fn expires(&self) -> DateTime<Utc> {
        Utc.timestamp(self.0.expires, 0)
    }
}
