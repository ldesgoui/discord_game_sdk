use crate::{sys, utils::charbuf_to_str, UnixTimestamp};

/// OAuth 2.0 Token
///
/// > [Struct in official docs](https://discordapp.com/developers/docs/game-sdk/applications#data-models-oauth2token-struct)
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct OAuth2Token(pub(crate) sys::DiscordOAuth2Token);

impl OAuth2Token {
    /// A bearer token for the current user
    pub fn access_token(&self) -> &str {
        charbuf_to_str(&self.0.access_token)
    }

    /// The list of `OAuth2` scopes separated by spaces
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(token: OAuth2Token) -> Result<()> {
    /// for scope in token.scopes().split(' ') {
    ///     println!("we have access to: {}", scope);
    /// }
    /// # Ok(()) }
    /// ```
    pub fn scopes(&self) -> &str {
        charbuf_to_str(&self.0.scopes)
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
            .field("scopes", &self.scopes())
            .field("expires", &self.expires())
            .finish()
    }
}
