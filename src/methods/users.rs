use crate::{sys, to_result::ToResult, Discord, PremiumKind, Result, User, UserFlags, UserID};

/// # Users
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/users)
impl Discord {
    /// Get the current user.
    ///
    /// More information can be found through the HTTP API.
    ///
    /// ## Errors
    ///
    /// Until the event [`CurrentUserUpdate`] is fired, this method will return an error.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/users#getcurrentuser)
    ///
    /// [`CurrentUserUpdate`]: event/struct.CurrentUserUpdate.html
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// let current_user = discord.current_user()?;
    /// # Ok(()) }
    /// ```
    pub fn current_user(&self) -> Result<User> {
        let mut user = User(sys::DiscordUser::default());

        unsafe { ffi!(self.get_user_manager().get_current_user(&mut user.0)) }.to_result()?;

        Ok(user)
    }

    /// Get user information for a given ID.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/users#getuser)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// # let id_to_lookup = 0;
    /// discord.user(id_to_lookup, |discord, result| {
    ///     match result {
    ///         Ok(user) => {
    ///             // ...
    ///         }
    ///         Err(error) => eprintln!("failed to fetch user: {}", error),
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn user<'d>(&'d self, user_id: UserID, callback: impl 'd + FnOnce(&Self, Result<User>)) {
        unsafe {
            ffi!(self.get_user_manager().get_user(user_id).and_then(
                |res: sys::EDiscordResult, user: *mut sys::DiscordUser| {
                    callback::<Result<&User>>(res.to_result().map(|()| &*(user as *mut User)))
                }
            ))
        }
    }

    /// Get the Premium type for the currently connected user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/users#getcurrentuserpremiumtype)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// let premium = discord.current_user_premium_kind()?;
    /// # Ok(()) }
    /// ```
    pub fn current_user_premium_kind(&self) -> Result<PremiumKind> {
        let mut premium_type = sys::EDiscordPremiumType::default();

        unsafe {
            ffi!(self
                .get_user_manager()
                .get_current_user_premium_type(&mut premium_type))
        }
        .to_result()?;

        Ok(PremiumKind::from(premium_type))
    }

    /// Return a bitfield of all flags set for the current user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/users#currentuserhasflag)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// let flags = discord.current_user_flags()?;
    /// # Ok(()) }
    /// ```
    pub fn current_user_flags(&self) -> Result<UserFlags> {
        let mut flags = UserFlags::empty();

        for flag in &[
            UserFlags::PARTNER,
            UserFlags::HYPE_SQUAD_EVENTS,
            UserFlags::HYPE_SQUAD_HOUSE_1,
            UserFlags::HYPE_SQUAD_HOUSE_2,
            UserFlags::HYPE_SQUAD_HOUSE_3,
        ] {
            let mut contains = false;

            unsafe {
                ffi!(self
                    .get_user_manager()
                    .current_user_has_flag(flag.bits(), &mut contains))
            }
            .to_result()?;

            flags.set(*flag, contains);
        }

        Ok(flags)
    }
}
