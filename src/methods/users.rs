use crate::prelude::*;

/// # Users
impl<'a> Discord<'a> {
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

    pub fn user<F>(&mut self, user_id: i64, callback: F)
    where
        F: FnMut(Result<User>),
    {
        unsafe {
            ffi!(self.get_user_manager().get_user(
                user_id,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result_from_sys::<F, User>)
            ))
        }
    }

    pub fn current_user_premium_type(&mut self) -> Result<PremiumType> {
        let mut premium_type = sys::EDiscordPremiumType::default();

        unsafe {
            ffi!(self
                .get_user_manager()
                .get_current_user_premium_type(&mut premium_type as *mut _))
        }
        .to_result()?;

        Ok(PremiumType::from_sys(&premium_type))
    }

    pub fn user_events_reader(&mut self) -> shrev::ReaderId<event::User> {
        self.user_channel.register_reader()
    }

    pub fn user_events(
        &mut self,
        reader: &mut shrev::ReaderId<event::User>,
    ) -> shrev::EventIterator<event::User> {
        self.user_channel.read(reader)
    }
}
