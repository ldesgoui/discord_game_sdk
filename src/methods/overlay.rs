use crate::{sys, to_result::ToResult, Action, Discord, Result};
use std::borrow::Cow;

/// # Overlay
///
/// The terminology employed by the Game SDK is confusing, this crate employs the terms "opened"
/// and "closed" instead:
///
/// |                                          | Game SDK | `discord_game_sdk` |
/// |------------------------------------------|----------|--------------------|
/// | Overlay is appearing and has taken focus | unlocked | opened             |
/// | Overlay is hidden                        | locked   | closed             |
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/overlay)
impl Discord {
    /// Check whether the user has the overlay enabled or disabled.
    ///
    /// If the overlay is disabled, all the functionality in this manager will still work.
    /// The calls will instead focus the Discord client and show the modal there instead.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/overlay#isenabled)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// if discord.overlay_enabled() {
    ///     // ...
    /// }
    /// # Ok(()) }
    /// ```
    pub fn overlay_enabled(&self) -> bool {
        let mut enabled = false;

        unsafe { ffi!(self.get_overlay_manager().is_enabled(&mut enabled)) }

        enabled
    }

    /// Whether the overlay is appearing and has taken focus.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/overlay#islocked)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// if discord.overlay_opened() {
    ///     // ...
    /// }
    /// # Ok(()) }
    /// ```
    pub fn overlay_opened(&self) -> bool {
        let mut locked = false;

        unsafe { ffi!(self.get_overlay_manager().is_locked(&mut locked)) }

        !locked
    }

    /// Open or close the overlay.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/overlay#setlocked)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.set_overlay_opened(false, |discord, result| {
    ///     if let Err(error) = result {
    ///         return eprintln!("failed to set overlay open: {}", error);
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn set_overlay_opened<'d>(
        &'d self,
        opened: bool,
        callback: impl 'd + FnOnce(&Self, Result<()>),
    ) {
        unsafe {
            ffi!(self
                .get_overlay_manager()
                .set_locked(!opened)
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }

    /// Opens the overlay modal for sending game invitations to users, channels, and servers.
    /// If you do not have a valid activity with all the required fields, this call will error.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/overlay#openactivityinvite)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.open_invite_overlay(Action::Join, |discord, result| {
    ///     if let Err(error) = result {
    ///         return eprintln!("failed open invite overlay: {}", error);
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn open_invite_overlay<'d>(
        &'d self,
        action: Action,
        callback: impl 'd + FnOnce(&Self, Result<()>),
    ) {
        unsafe {
            ffi!(self
                .get_overlay_manager()
                .open_activity_invite(action.into())
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }

    /// Opens the overlay modal for joining a Discord guild, given its invite code
    /// (e.g.: `ABCDEF` in `https://discord.gg/ABCDEF` or `https://discordapp.com/invite/ABCDEF`).
    ///
    /// Receiving `Ok(())` does not necessarily mean that the user has joined the guild.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `code` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/overlay#openguildinvite)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.open_guild_invite_overlay("discord-gamesdk\0", |discord, result| {
    ///     if let Err(error) = result {
    ///         return eprintln!("failed open guild invite overlay: {}", error);
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn open_guild_invite_overlay<'d, 'b>(
        &'d self,
        code: impl Into<Cow<'b, str>>,
        callback: impl 'd + FnOnce(&Self, Result<()>),
    ) {
        let mut code = code.into();

        if !code.ends_with('\0') {
            code.to_mut().push('\0')
        };

        unsafe {
            ffi!(self
                .get_overlay_manager()
                .open_guild_invite(code.as_ptr())
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }

    /// Opens the overlay widget for voice settings for the currently connected application.
    /// These settings are unique to each user within the context of your application.
    /// That means that a user can have different favorite voice settings for each of their games.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/overlay#openvoicesettings)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.open_voice_settings(|discord, result| {
    ///     if let Err(error) = result {
    ///         return eprintln!("failed open voice settings overlay: {}", error);
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn open_voice_settings<'d>(&'d self, callback: impl 'd + FnOnce(&Self, Result<()>)) {
        unsafe {
            ffi!(self
                .get_overlay_manager()
                .open_voice_settings()
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }
}
