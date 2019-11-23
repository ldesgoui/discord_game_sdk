use crate::{
    sys,
    utils::{charbuf_len, charbuf_to_str},
};
use std::iter::FusedIterator;

/// OAuth 2.0 Token
///
/// <https://discordapp.com/developers/docs/game-sdk/applications#data-models-oauth2token-struct>
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct OAuth2Token {
    pub(crate) sys: sys::DiscordOAuth2Token,
    access_token_len: usize,
    scopes_len: usize,
}

impl OAuth2Token {
    /// A bearer token for the current user
    pub fn access_token(&self) -> &str {
        charbuf_to_str(&self.sys.access_token[..self.access_token_len])
    }

    /// The list of OAuth2 scopes
    pub fn scopes<'a>(
        &'a self,
    ) -> impl Iterator<Item = &'a str> + DoubleEndedIterator + FusedIterator + 'a {
        charbuf_to_str(&self.sys.scopes[..self.scopes_len]).split(' ')
    }

    /// When the token exires, in UNIX Time
    pub fn expires(&self) -> i64 {
        self.sys.expires
    }
}

impl From<sys::DiscordOAuth2Token> for OAuth2Token {
    fn from(sys: sys::DiscordOAuth2Token) -> Self {
        Self {
            sys,
            access_token_len: charbuf_len(&sys.access_token[..]),
            scopes_len: charbuf_len(&sys.scopes[..]),
        }
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
