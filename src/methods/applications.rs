use crate::{
    callbacks::{ResultCallback, ResultFromPtrCallback, ResultStringCallback},
    sys,
    utils::charbuf_to_str,
    Discord, OAuth2Token, Result,
};
use std::mem::size_of;

/// # Applications
///
/// Authentication and various helper functions
///
/// <https://discordapp.com/developers/docs/game-sdk/applications>
impl<'a> Discord<'a> {
    /// The locale that was set by the current user in their Discord settings.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/applications#getcurrentlocale>
    pub fn current_locale(&self) -> String {
        let mut locale: sys::DiscordLocale = [0; size_of::<sys::DiscordLocale>()];

        unsafe {
            ffi!(self
                .get_application_manager()
                .get_current_locale(&mut locale as *mut _))
        }

        charbuf_to_str(&locale).to_string()
    }

    /// Get the name of pushed branch on which the game is running.
    /// These are branches that you created and pushed using
    /// [Dispatch](https://discordapp.com/developers/docs/dispatch/dispatch-and-you).
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/applications#getcurrentbranch>
    pub fn current_branch(&self) -> String {
        let mut branch: sys::DiscordBranch = [0; size_of::<sys::DiscordBranch>()];

        unsafe {
            ffi!(self
                .get_application_manager()
                .get_current_branch(&mut branch as *mut _))
        }

        charbuf_to_str(&branch).to_string()
    }

    /// Checks if the current user has the entitlement to run this game.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/applications#validateorexit>
    pub fn validate_or_exit(&mut self, callback: impl FnMut(&mut Discord, Result<()>) + 'a) {
        unsafe {
            ffi!(self
                .get_application_manager()
                .validate_or_exit()
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Retrieve an OAuth 2.0 Bearer token for the current user.
    /// If your game was launched from Discord and you call this function,
    /// you will automatically receive the token.
    /// If the game was not launched from Discord and this method is called,
    /// Discord will focus itself and prompt the user for authorization.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/applications#getoauth2token>
    pub fn oauth2_token(&mut self, callback: impl FnMut(&mut Discord, Result<OAuth2Token>) + 'a) {
        unsafe {
            ffi!(self
                .get_application_manager()
                .get_oauth2_token()
                .and_then(ResultFromPtrCallback::new(callback)))
        }
    }

    /// Get the signed app ticket for the current user.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/applications#getticket>
    pub fn app_ticket(&mut self, callback: impl FnMut(&mut Discord, Result<String>) + 'a) {
        unsafe {
            ffi!(self
                .get_application_manager()
                .get_ticket()
                .and_then(ResultStringCallback::new(callback)))
        }
    }
}
