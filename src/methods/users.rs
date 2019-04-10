use crate::prelude::*;

/// # Users
impl<'a> Discord<'a> {
    // tested, returned Err(_) until event::user::CurrentUserUpdate
    pub fn current_user(&mut self) -> Result<User> {
        let mut user = sys::DiscordUser::default();

        unsafe {
            ffi!(self
                .get_user_manager()
                .get_current_user(&mut user as *mut _))
        }
        .to_result()?;

        Ok(User::from_sys(&user))
    }

    // tested
    pub fn user<F>(&mut self, user_id: i64, callback: F)
    where
        F: FnMut(&mut Discord, Result<User>),
    {
        unsafe {
            ffi!(self.get_user_manager().get_user(
                user_id,
                self.wrap_callback(callback),
                Some(callbacks::result_from_sys_ptr::<F, User>)
            ))
        }
    }

    // tested
    pub fn current_user_premium_kind(&mut self) -> Result<PremiumKind> {
        let mut premium_type = sys::EDiscordPremiumType::default();

        unsafe {
            ffi!(self
                .get_user_manager()
                .get_current_user_premium_type(&mut premium_type as *mut _))
        }
        .to_result()?;

        Ok(PremiumKind::from_sys(&premium_type))
    }
}
