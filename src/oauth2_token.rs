use crate::{sys, utils::charbuf_to_str, UnixTimestamp};

/// OAuth 2.0 Token
///
/// > [Struct in official docs](https://discordapp.com/developers/docs/game-sdk/applications#data-models-oauth2token-struct)
#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
#[repr(transparent)]
pub struct OAuth2Token(pub(crate) sys::DiscordOAuth2Token);

impl OAuth2Token {
    /// A bearer token for the current user
    pub fn access_token(&self) -> &str {
        charbuf_to_str(&self.0.access_token)
    }

    /// The list of `OAuth2` scopes
    pub fn scopes(&self) -> std::str::Split<'_, char> {
        charbuf_to_str(&self.0.scopes).split(' ')
    }

    /// When the token exires, in UNIX Time
    pub fn expires(&self) -> UnixTimestamp {
        self.0.expires
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
