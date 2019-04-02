use crate::event::UserEvent;
use crate::prelude::*;
use crate::premium_type::PremiumType;

/// Activities
impl Discord {
    pub fn get_current_user(&self) -> Result<User> {
        let mut user = sys::DiscordUser::default();

        ffi!(self
            .get_user_manager()
            .get_current_user(&mut user as *mut _))?;

        User::from_sys(&user)
    }

    pub fn get_user<F>(&self, user_id: i64, mut callback: F)
    where
        F: FnMut(Result<User>),
    {
        let _ = ffi!(self.get_user_manager().get_user(
            user_id,
            &callback as *const _ as *mut _,
            Some(get_user_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }

    pub fn get_current_user_premium_type(&self) -> Result<PremiumType> {
        let mut premium_type = sys::EDiscordPremiumType::default();

        ffi!(self
            .get_user_manager()
            .get_current_user_premium_type(&mut premium_type as *mut _))?;

        PremiumType::from_sys(premium_type)
    }

    pub fn user_events_reader(&mut self) -> shrev::ReaderId<UserEvent> {
        self.user_events.register_reader()
    }

    pub fn user_events(
        &self,
        reader: &mut shrev::ReaderId<UserEvent>,
    ) -> shrev::EventIterator<UserEvent> {
        self.user_events.read(reader)
    }
}

extern "C" fn get_user_callback<F>(
    data: *mut c_void,
    res: sys::EDiscordResult,
    user: *mut sys::DiscordUser,
) where
    F: FnMut(Result<User>),
{
    if data.is_null() {
        log::error!("SDK invoked callback with null");
        return;
    }
    let callback: &mut F = unsafe { &mut *(data as *mut _) };

    callback(
        res.to_result()
            .and_then(|_| unsafe { user.as_ref() }.ok_or(BindingsViolation::NullPointer.into()))
            .and_then(|user| User::from_sys(user)),
    )
}
