use crate::{
    callbacks::ResultCallback, event, sys, to_result::ToResult, Action, Activity, Discord,
    RequestReply, Result,
};

/// # Activities
/// <https://discordapp.com/developers/docs/game-sdk/activities>
impl<'a> Discord<'a> {
    /// `command` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/activities#registercommand>
    pub fn register_launch_command(&mut self, mut command: String) -> Result<()> {
        command.push('\0');

        unsafe {
            ffi!(self
                .get_activity_manager()
                .register_command(command.as_ptr() as *const _))
        }
        .to_result()
    }

    /// You may find that your status is not updated while trying this feature, make sure that you
    /// have not disabled it in your Discord client settings (User Settings -> Game Activity).
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/activities#updateactivity>
    pub fn update_activity(
        &mut self,
        activity: &Activity,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        let mut activity: sys::DiscordActivity = activity.sys;

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

    /// `content` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/activities#sendinvite>
    pub fn send_invite(
        &mut self,
        user_id: i64,
        action: Action,
        mut content: String,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        content.push('\0');

        unsafe {
            ffi!(self
                .get_activity_manager()
                .send_invite(user_id, action.into(), content.as_ptr() as *const _)
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

    /// <https://discordapp.com/developers/docs/game-sdk/activities#onactivityjoin>
    pub fn recv_activities_join(&'_ self) -> impl '_ + Iterator<Item = event::activities::Join> {
        self.receivers.activities_join.try_iter()
    }

    /// <https://discordapp.com/developers/docs/game-sdk/activities#onactivityspectate>
    pub fn recv_activities_spectate(
        &'_ self,
    ) -> impl '_ + Iterator<Item = event::activities::Spectate> {
        self.receivers.activities_spectate.try_iter()
    }

    /// <https://discordapp.com/developers/docs/game-sdk/activities#onactivityjoinrequest>
    pub fn recv_activities_request(
        &'_ self,
    ) -> impl '_ + Iterator<Item = event::activities::Request> {
        self.receivers.activities_request.try_iter()
    }

    /// <https://discordapp.com/developers/docs/game-sdk/activities#onactivityinvite>
    pub fn recv_activities_invite(
        &'_ self,
    ) -> impl '_ + Iterator<Item = event::activities::Invite> {
        self.receivers.activities_invite.try_iter()
    }
}
