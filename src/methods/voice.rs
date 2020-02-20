use crate::{callback, sys, to_result::ToResult, Discord, InputMode, Result, UserID};

/// # Voice
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/discord-voice)
impl Discord {
    /// Get the voice input mode for the current user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/discord-voice#getinputmode)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// let input_mode = discord.input_mode()?;
    /// # Ok(()) }
    pub fn input_mode(&self) -> Result<InputMode> {
        let mut input_mode = InputMode(sys::DiscordInputMode::default());

        self.with_voice_manager(|mgr| unsafe {
            mgr.get_input_mode.unwrap()(mgr, &mut input_mode.0)
        })
        .to_result()?;

        Ok(input_mode)
    }

    /// Sets a new voice input mode for the user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/discord-voice#setinputmode)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.set_input_mode(
    ///     InputMode::push_to_talk("caps lock"),
    ///     |result| {
    ///         if let Err(error) = result {
    ///             return eprintln!("failed to set voice input mode: {}", error);
    ///         }
    ///     },
    /// );
    /// # Ok(()) }
    pub fn set_input_mode<'d>(
        &'d self,
        input_mode: InputMode,
        callback: impl 'd + FnOnce(Result<()>),
    ) {
        self.with_voice_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(res.to_result()));
            unsafe { mgr.set_input_mode.unwrap()(mgr, input_mode.0, ptr, fun) }
        })
    }

    /// Whether the current user is muted.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/discord-voice#isselfmute)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// if discord.self_muted()? {
    ///     // ...
    /// }
    /// # Ok(()) }
    pub fn self_muted(&self) -> Result<bool> {
        let mut muted = false;

        self.with_voice_manager(|mgr| unsafe { mgr.is_self_mute.unwrap()(mgr, &mut muted) })
            .to_result()?;

        Ok(muted)
    }

    /// Whether the current used is deafened.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/discord-voice#isselfdeaf)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// if discord.self_deafened()? {
    ///     // ...
    /// }
    /// # Ok(()) }
    pub fn self_deafened(&self) -> Result<bool> {
        let mut deafened = false;

        self.with_voice_manager(|mgr| unsafe { mgr.is_self_deaf.unwrap()(mgr, &mut deafened) })
            .to_result()?;

        Ok(deafened)
    }

    /// Mutes or unmutes the current user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/discord-voice#setselfmute)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.set_self_mute(false)?;
    /// # Ok(()) }
    pub fn set_self_mute(&self, muted: bool) -> Result<()> {
        self.with_voice_manager(|mgr| unsafe { mgr.set_self_mute.unwrap()(mgr, muted) })
            .to_result()
    }

    /// Deafens or undeafens the current user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/discord-voice#setselfdeaf)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.set_self_deaf(false)?;
    /// # Ok(()) }
    pub fn set_self_deaf(&self, deafened: bool) -> Result<()> {
        self.with_voice_manager(|mgr| unsafe { mgr.set_self_deaf.unwrap()(mgr, deafened) })
            .to_result()
    }

    /// Whether a given user is locally muted.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/discord-voice#islocalmute)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord, user: User) -> Result<()> {
    /// if discord.local_muted(user.id())? {
    ///     // ...
    /// }
    /// # Ok(()) }
    pub fn local_muted(&self, user_id: UserID) -> Result<bool> {
        let mut muted = false;

        self.with_voice_manager(|mgr| unsafe {
            mgr.is_local_mute.unwrap()(mgr, user_id, &mut muted)
        })
        .to_result()?;

        Ok(muted)
    }

    /// Gets the local volume for a given user, in the range `[0..=200]`, `100` being the default.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/discord-voice#getlocalvolume)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord, user: User) -> Result<()> {
    /// discord.set_local_volume(user.id(), discord.local_volume(user.id())? + 10)?;
    /// # Ok(()) }
    pub fn local_volume(&self, user_id: UserID) -> Result<u8> {
        let mut volume = 0;

        self.with_voice_manager(|mgr| unsafe {
            mgr.get_local_volume.unwrap()(mgr, user_id, &mut volume)
        })
        .to_result()?;

        debug_assert!((0..=200).contains(&volume));

        Ok(volume)
    }

    /// Locally mutes or unmutes a given user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/discord-voice#setlocalmute)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord, user: User) -> Result<()> {
    /// discord.set_local_mute(user.id(), true)?;
    /// # Ok(()) }
    pub fn set_local_mute(&self, user_id: UserID, muted: bool) -> Result<()> {
        self.with_voice_manager(|mgr| unsafe { mgr.set_local_mute.unwrap()(mgr, user_id, muted) })
            .to_result()
    }

    /// Sets the local volume for a given user.
    ///
    /// In the range `[0..=200]`, `100` being the default.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/discord-voice#setlocalvolume)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord, user: User) -> Result<()> {
    /// discord.set_local_volume(user.id(), discord.local_volume(user.id())? + 10)?;
    /// # Ok(()) }
    pub fn set_local_volume(&self, user_id: UserID, volume: u8) -> Result<()> {
        debug_assert!((0..=200).contains(&volume));

        self.with_voice_manager(|mgr| unsafe {
            mgr.set_local_volume.unwrap()(mgr, user_id, volume)
        })
        .to_result()
    }
}
