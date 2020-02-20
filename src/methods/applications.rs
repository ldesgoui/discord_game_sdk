use crate::{callback, sys, to_result::ToResult, utils, Discord, OAuth2Token, Result};
use std::mem::size_of;

/// # Applications
///
/// Authentication and various helper functions
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/applications)
impl Discord {
    /// The locale that was set by the current user in their Discord settings.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/applications#getcurrentlocale)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// println!("current locale is {}", discord.current_locale());
    /// # Ok(()) }
    pub fn current_locale(&self) -> String {
        let mut locale: sys::DiscordLocale = [0; size_of::<sys::DiscordLocale>()];

        self.with_application_manager(|mgr| unsafe {
            mgr.get_current_locale.unwrap()(mgr, &mut locale)
        });

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
    /// # fn example(discord: Discord) -> Result<()> {
    /// println!("current branch is {}", discord.current_branch());
    /// # Ok(()) }
    pub fn current_branch(&self) -> String {
        let mut branch: sys::DiscordBranch = [0; size_of::<sys::DiscordBranch>()];

        self.with_application_manager(|mgr| unsafe {
            mgr.get_current_branch.unwrap()(mgr, &mut branch);
        });

        utils::charbuf_to_str(&branch).to_string()
    }

    /// Checks if the current user has the entitlement to run this game.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/applications#validateorexit)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.validate_or_exit(|discord, result| {
    ///     // ...
    /// });
    /// # Ok(()) }
    pub fn validate_or_exit<'d>(&'d self, callback: impl 'd + FnOnce(&Self, Result<()>)) {
        self.with_application_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(self, res.to_result()));

            unsafe { mgr.validate_or_exit.unwrap()(mgr, ptr, fun) };
        });
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
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.oauth2_token(|discord, token| {
    ///     match token {
    ///         Ok(token) => {
    ///             //...
    ///         },
    ///         Err(error) => eprintln!("failed to retrieve OAuth2 token: {}", error),
    ///     }
    /// });
    /// # Ok(()) }
    pub fn oauth2_token<'d>(&'d self, callback: impl 'd + FnOnce(&Self, Result<&OAuth2Token>)) {
        self.with_application_manager(|mgr| {
            let (ptr, fun) = callback::two_params(
                |res: sys::EDiscordResult, token: *mut sys::DiscordOAuth2Token| {
                    callback(
                        self,
                        res.to_result()
                            .map(|()| unsafe { &*(token as *mut OAuth2Token) }),
                    )
                },
            );

            unsafe { mgr.get_oauth2_token.unwrap()(mgr, ptr, fun) };
        });
    }

    /// Get the signed app ticket for the current user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/applications#getticket)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.app_ticket(|discord, ticket| {
    ///     match ticket {
    ///         Ok(ticket) => {
    ///             //...
    ///         },
    ///         Err(error) => eprintln!("failed to retrieve signed app ticket: {}", error),
    ///     }
    /// });
    /// # Ok(()) }
    pub fn app_ticket<'d>(&'d self, callback: impl 'd + FnOnce(&Self, Result<&str>)) {
        self.with_application_manager(|mgr| {
            let (ptr, fun) = callback::two_params(|res: sys::EDiscordResult, string: *const u8| {
                callback(
                    self,
                    res.to_result()
                        .map(|()| unsafe { utils::charptr_to_str(string) }),
                )
            });

            unsafe { mgr.get_ticket.unwrap()(mgr, ptr, fun) };
        });
    }
}
