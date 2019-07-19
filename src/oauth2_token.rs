use crate::{sys, utils::cstr_to_str};
use chrono::{offset::TimeZone, DateTime, Utc};
use std::iter::FusedIterator;

/// OAuth2 Token
///
/// <https://discordapp.com/developers/docs/game-sdk/applications#data-models-oauth2token-struct>
#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct OAuth2Token(pub(crate) sys::DiscordOAuth2Token);

impl OAuth2Token {
    get_str!(
        "A bearer token for the current user",
        access_token,
        access_token
    );

    /// The list of oauth2 scopes
    pub fn scopes<'a>(
        &'a self,
    ) -> impl Iterator<Item = &'a str> + DoubleEndedIterator + FusedIterator + 'a {
        cstr_to_str(&self.0.scopes[..]).split(' ')
    }

    /// The timestamp at which the token expires
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
