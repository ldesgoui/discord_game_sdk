use crate::{
    callbacks::ResultFromPtrCallback, sys, to_result::ToResult, Discord, PremiumKind, Result, User,
    UserFlags,
};

/// # Users
///
/// <https://discordapp.com/developers/docs/game-sdk/users>
impl<'a> Discord<'a> {
    /// Get Current User
    ///
    /// Will return Err(_) until event::user::CurrentUserUpdate
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/users#getcurrentuser>
    pub fn current_user(&mut self) -> Result<User> {
        let mut user = sys::DiscordUser::default();

        unsafe {
            ffi!(self
                .get_user_manager()
                .get_current_user(&mut user as *mut _))
        }
        .to_result()?;

        Ok(user.into())
    }

    /// <https://discordapp.com/developers/docs/game-sdk/users#getuser>
    pub fn user(&mut self, user_id: i64, callback: impl FnMut(&mut Discord, Result<User>) + 'a) {
        unsafe {
            ffi!(self
                .get_user_manager()
                .get_user(user_id)
                .and_then(ResultFromPtrCallback::new(callback)))
        }
    }

    /// <https://discordapp.com/developers/docs/game-sdk/users#getcurrentuserpremiumtype>
    pub fn current_user_premium_kind(&mut self) -> Result<PremiumKind> {
        let mut premium_type = sys::EDiscordPremiumType::default();

        unsafe {
            ffi!(self
                .get_user_manager()
                .get_current_user_premium_type(&mut premium_type as *mut _))
        }
        .to_result()?;

        Ok(PremiumKind::from(premium_type))
    }

    /// <https://discordapp.com/developers/docs/game-sdk/users#currentuserhasflag>
    pub fn current_user_flags(&mut self) -> Result<UserFlags> {
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
                    .current_user_has_flag(flag.bits(), &mut contains as *mut _))
            }
            .to_result()?;

            flags.set(*flag, contains);
        }

        Ok(flags)
    }
}
