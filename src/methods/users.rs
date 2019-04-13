use crate::{
    callbacks::ResultFromPtrCallback, sys, to_result::ToResult, Discord, DiscordResult,
    PremiumKind, User,
};

/// # Users
impl<'a> Discord<'a> {
    // tested, returned Err(_) until event::user::CurrentUserUpdate
    pub fn current_user(&mut self) -> DiscordResult<User> {
        let mut user = sys::DiscordUser::default();

        unsafe {
            ffi!(self
                .get_user_manager()
                .get_current_user(&mut user as *mut _))
        }
        .to_result()?;

        Ok(User::from(user))
    }

    // tested
    pub fn user<F>(&mut self, user_id: i64, callback: F)
    where
        F: FnMut(&mut Discord, DiscordResult<User>) + 'a,
    {
        unsafe {
            ffi!(self.get_user_manager().get_user(user_id)(
                ResultFromPtrCallback::new(callback)
            ))
        }
    }

    // tested
    pub fn current_user_premium_kind(&mut self) -> DiscordResult<PremiumKind> {
        let mut premium_type = sys::EDiscordPremiumType::default();

        unsafe {
            ffi!(self
                .get_user_manager()
                .get_current_user_premium_type(&mut premium_type as *mut _))
        }
        .to_result()?;

        Ok(PremiumKind::from(premium_type))
    }
}
