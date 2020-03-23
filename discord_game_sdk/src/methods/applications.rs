use crate::{sys, to_result::ToResult, utils, Discord, OAuth2Token, Result};
use std::mem::size_of;

/// # Applications
///
/// Authentication and various helper functions
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/applications)
impl<'d, E> Discord<'d, E> {
    /// The locale that was set by the current user in their Discord settings.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/applications#getcurrentlocale)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord<'_, ()>) -> Result<()> {
    /// println!("current locale is {}", discord.current_locale());
    /// # Ok(()) }
    /// ```
    pub fn current_locale(&self) -> String {
        let mut locale: sys::DiscordLocale = [0; size_of::<sys::DiscordLocale>()];

        unsafe {
            let mgr = self.application_manager();

            (*mgr).get_current_locale.unwrap()(mgr, &mut locale)
        }

        utils::charbuf_to_str(&locale).to_string()
    }

    /// Get the name of pushed branch on which the game is running.
    ///
    /// These are branches that you created and pushed using
    /// [Dispatch](https://discordapp.com/developers/docs/dispatch/dispatch-and-you).
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/applications#getcurrentbranch)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord<'_, ()>) -> Result<()> {
    /// println!("current branch is {}", discord.current_branch());
    /// # Ok(()) }
    /// ```
    pub fn current_branch(&self) -> String {
        let mut branch: sys::DiscordBranch = [0; size_of::<sys::DiscordBranch>()];

        unsafe {
            let mgr = self.application_manager();

            (*mgr).get_current_branch.unwrap()(mgr, &mut branch);
        }

        utils::charbuf_to_str(&branch).to_string()
    }

    /// Checks if the current user has the entitlement to run this game.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/applications#validateorexit)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord<'_, ()>) -> Result<()> {
    /// discord.validate_or_exit(|discord, result| {
    ///     // ...
    /// });
    /// # Ok(()) }
    /// ```
    pub fn validate_or_exit(&self, callback: impl 'd + FnOnce(&Discord<'d, E>, Result<()>)) {
        let (ptr, fun) = self
            .one_param(move |discord, res: sys::EDiscordResult| callback(discord, res.to_result()));

        unsafe {
            let mgr = self.application_manager();

            (*mgr).validate_or_exit.unwrap()(mgr, ptr, fun)
        }
    }

    /// Retrieve an OAuth 2.0 Bearer token for the current user.
    ///
    /// If your game was launched from Discord and you call this function,
    /// you will automatically receive the token.
    /// If the game was not launched from Discord and this method is called,
    /// Discord will focus itself and prompt the user for authorization.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/applications#getoauth2token)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord<'_, ()>) -> Result<()> {
    /// discord.oauth2_token(|discord, token| {
    ///     match token {
    ///         Ok(token) => {
    ///             //...
    ///         },
    ///         Err(error) => eprintln!("failed to retrieve OAuth2 token: {}", error),
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn oauth2_token(&self, callback: impl 'd + FnOnce(&Discord<'d, E>, Result<&OAuth2Token>)) {
        let (ptr, fun) = self.two_params(
            move |discord, res: sys::EDiscordResult, token: *mut sys::DiscordOAuth2Token| {
                callback(
                    discord,
                    res.to_result()
                        .map(|()| unsafe { &*(token as *mut OAuth2Token) }),
                )
            },
        );

        unsafe {
            let mgr = self.application_manager();

            (*mgr).get_oauth2_token.unwrap()(mgr, ptr, fun)
        }
    }

    /// Get the signed app ticket for the current user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/applications#getticket)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord<'_, ()>) -> Result<()> {
    /// discord.app_ticket(|discord, ticket| {
    ///     match ticket {
    ///         Ok(ticket) => {
    ///             //...
    ///         },
    ///         Err(error) => eprintln!("failed to retrieve signed app ticket: {}", error),
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn app_ticket(&self, callback: impl 'd + FnOnce(&Discord<'d, E>, Result<&str>)) {
        let (ptr, fun) = self.two_params(
            move |discord, res: sys::EDiscordResult, string: *const u8| {
                callback(
                    discord,
                    res.to_result()
                        .map(|()| unsafe { utils::charptr_to_str(string) }),
                )
            },
        );

        unsafe {
            let mgr = self.application_manager();

            (*mgr).get_ticket.unwrap()(mgr, ptr, fun)
        }
    }
}
