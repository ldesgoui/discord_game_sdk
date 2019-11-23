use crate::{callbacks::ResultCallback, event, Action, Discord, Result};

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
/// <https://discordapp.com/developers/docs/game-sdk/overlay>
impl<'a> Discord<'a> {
    /// Check whether the user has the overlay enabled or disabled.
    /// If the overlay is disabled, all the functionality in this manager will still work.
    /// The calls will instead focus the Discord client and show the modal there instead.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/overlay#isenabled>
    pub fn overlay_enabled(&mut self) -> bool {
        let mut enabled = false;

        unsafe {
            ffi!(self
                .get_overlay_manager()
                .is_enabled(&mut enabled as *mut _))
        }

        enabled
    }

    /// Whether the overlay is appearing and has taken focus.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/overlay#islocked>
    pub fn overlay_opened(&mut self) -> bool {
        let mut locked = false;

        unsafe { ffi!(self.get_overlay_manager().is_locked(&mut locked as *mut _)) }

        !locked
    }

    /// Open or close the overlay.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/overlay#setlocked>
    pub fn set_overlay_opened(
        &mut self,
        opened: bool,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        unsafe {
            ffi!(self
                .get_overlay_manager()
                .set_locked(!opened)
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Opens the overlay modal for sending game invitations to users, channels, and servers.
    /// If you do not have a valid activity with all the required fields, this call will error.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/overlay#openactivityinvite>
    pub fn open_invite_overlay(
        &mut self,
        action: Action,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        unsafe {
            ffi!(self
                .get_overlay_manager()
                .open_activity_invite(action.into())
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Opens the overlay modal for joining a Discord guild, given its invite code
    /// (e.g.: `ABCDEF` in `https://discord.gg/ABCDEF` or `https://discordapp.com/invite/ABCDEF`).
    ///
    /// Receiving `Ok(())` does not necessarily mean that the user has joined the guild.
    ///
    /// `code` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/overlay#openguildinvite>
    pub fn open_guild_invite_overlay(
        &mut self,
        mut code: String,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        code.push('\0');

        unsafe {
            ffi!(self
                .get_overlay_manager()
                .open_guild_invite(code.as_ptr() as *const _)
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Opens the overlay widget for voice settings for the currently connected application.
    /// These settings are unique to each user within the context of your application.
    /// That means that a user can have different favorite voice settings for each of their games.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/overlay#openvoicesettings>
    pub fn open_voice_settings(&mut self, callback: impl FnMut(&mut Discord, Result<()>) + 'a) {
        unsafe {
            ffi!(self
                .get_overlay_manager()
                .open_voice_settings()
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Fires when the overlay is opened or closed.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/overlay#ontoggle>
    pub fn recv_overlay_toggle(&self) -> impl '_ + Iterator<Item = event::overlay::Toggle> {
        self.receivers.overlay_toggle.try_iter()
    }
}
