use crate::{
    callbacks::ResultCallback, sys, to_result::ToResult, Action, Activity, ActivityKind, Discord,
    RequestReply, Result,
};
use std::ffi::CStr;

/// # Activities
/// <https://discordapp.com/developers/docs/game-sdk/activities>
impl<'a> Discord<'a> {
    /// `command` must also be valid UTF-8
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/activities#registercommand>
    pub fn register_launch_command(&mut self, command: impl AsRef<CStr>) -> Result<()> {
        unsafe {
            ffi!(self
                .get_activity_manager()
                .register_command(command.as_ref().as_ptr()))
        }
        .to_result()
    }

    /// <https://discordapp.com/developers/docs/game-sdk/activities#updateactivity>
    pub fn update_activity(
        &mut self,
        activity: &Activity,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        let mut activity: sys::DiscordActivity = activity.0;

        // Unsure if this is required
        activity.type_ = ActivityKind::Playing.into();
        activity.application_id = self.client_id;

        unsafe {
            ffi!(self
                .get_activity_manager()
                .update_activity(&mut activity as *mut _)
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// <https://discordapp.com/developers/docs/game-sdk/activities#clearactivity>
    pub fn clear_activity(&mut self, callback: impl FnMut(&mut Discord, Result<()>) + 'a) {
        unsafe {
            ffi!(self
                .get_activity_manager()
                .clear_activity()
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// <https://discordapp.com/developers/docs/game-sdk/activities#sendrequestreply>
    pub fn send_request_reply(
        &mut self,
        user_id: i64,
        reply: RequestReply,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        unsafe {
            ffi!(self
                .get_activity_manager()
                .send_request_reply(user_id, reply.into())
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// `content` must also be valid UTF-8
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/activities#sendinvite>
    pub fn send_invite(
        &mut self,
        user_id: i64,
        action: Action,
        content: impl AsRef<CStr>,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        unsafe {
            ffi!(self
                .get_activity_manager()
                .send_invite(user_id, action.into(), content.as_ref().as_ptr())
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// <https://discordapp.com/developers/docs/game-sdk/activities#acceptinvite>
    pub fn accept_invite(
        &mut self,
        user_id: i64,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        unsafe {
            ffi!(self
                .get_activity_manager()
                .accept_invite(user_id)
                .and_then(ResultCallback::new(callback)))
        }
    }
}
