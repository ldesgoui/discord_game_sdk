use crate::{
    callbacks::ResultCallback, sys, to_result::ToResult, Action, Activity, ActivityKind, Discord,
    DiscordResult, RequestReply,
};
use std::ffi::CStr;

/// # Activities
impl<'a> Discord<'a> {
    pub fn register_launch_command(&mut self, command: impl AsRef<CStr>) -> DiscordResult<()> {
        unsafe {
            ffi!(self
                .get_activity_manager()
                .register_command(command.as_ref().as_ptr()))
        }
        .to_result()
    }

    pub fn update_activity<F>(&mut self, activity: &Activity, callback: F)
    where
        F: FnMut(&mut Discord, DiscordResult<()>) + 'a,
    {
        let mut activity: sys::DiscordActivity = activity.0;

        // Unsure if this is required
        activity.type_ = ActivityKind::Playing.into();
        activity.application_id = self.client_id;

        unsafe {
            ffi!(self
                .get_activity_manager()
                .update_activity(&mut activity as *mut _)(
                ResultCallback::new(callback)
            ))
        }
    }

    pub fn clear_activity<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Discord, DiscordResult<()>) + 'a,
    {
        unsafe {
            ffi!(self.get_activity_manager().clear_activity()(
                ResultCallback::new(callback)
            ))
        }
    }

    pub fn send_request_reply<F>(&mut self, user_id: i64, reply: RequestReply, callback: F)
    where
        F: FnMut(&mut Discord, DiscordResult<()>) + 'a,
    {
        unsafe {
            ffi!(self
                .get_activity_manager()
                .send_request_reply(user_id, reply.into())(
                ResultCallback::new(callback)
            ))
        }
    }

    pub fn send_invite<F>(
        &mut self,
        user_id: i64,
        action: Action,
        content: impl AsRef<CStr>,
        callback: F,
    ) where
        F: FnMut(&mut Discord, DiscordResult<()>) + 'a,
    {
        unsafe {
            ffi!(self.get_activity_manager().send_invite(
                user_id,
                action.into(),
                content.as_ref().as_ptr()
            )(ResultCallback::new(callback)))
        }
    }

    pub fn accept_invite<F>(&mut self, user_id: i64, callback: F)
    where
        F: FnMut(&mut Discord, DiscordResult<()>) + 'a,
    {
        unsafe {
            ffi!(self.get_activity_manager().accept_invite(user_id)(
                ResultCallback::new(callback)
            ))
        }
    }
}
