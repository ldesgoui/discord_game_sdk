use crate::{
    callbacks::ResultFromPtrCallback, event, sys, to_result::ToResult, Discord, PremiumKind,
    Result, User, UserFlags,
};

/// # Users
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/users)
impl<'a> Discord<'a> {
    /// Get the current user.
    /// More information can be found through the HTTP API.
    ///
    /// Will return `Err(_)` until [`event::user::CurrentUserUpdate`](struct.CurrentUserUpdate.html).
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/users#getcurrentuser)
    pub fn current_user(&self) -> Result<User> {
        let mut user = User(sys::DiscordUser::default());

        unsafe { ffi!(self.get_user_manager().get_current_user(&mut user.0)) }.to_result()?;

        Ok(user)
    }

    /// Get user information for a given ID.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/users#getuser)
    pub fn user(&self, user_id: i64, callback: impl 'a + FnMut(&Discord<'_>, Result<User>)) {
        unsafe {
            ffi!(self
                .get_user_manager()
                .get_user(user_id)
                .and_then(ResultFromPtrCallback::new(callback)))
        }
    }

    /// Get the Premium type for the currently connected user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/users#getcurrentuserpremiumtype)
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

    /// Fires when the User struct of the currently connected user changes.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/users#oncurrentuserupdate)
    pub fn recv_current_user_update(&self) -> impl '_ + Iterator<Item = event::CurrentUserUpdate> {
        self.receivers.current_user_update.try_iter()
    }
}
