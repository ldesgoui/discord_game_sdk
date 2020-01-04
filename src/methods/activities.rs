use crate::{
    callbacks::ResultCallback, event, to_result::ToResult, Action, Activity, Discord, RequestReply,
    Result,
};
use std::borrow::Cow;

/// # Activities
///
/// Also known as Rich Presence.
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/activities)
impl<'a> Discord<'a> {
    /// Registers a command by which Discord can launch your game.
    ///
    /// This might be a custom protocol, like `my-awesome-game://`, or a path to an executable.
    /// It also supports any launch parameters that may be needed, like `game.exe --full-screen`.
    ///
    /// On macOS, due to the way Discord registers executables,
    /// your game needs to be bundled for this command to work.
    /// That means it should be a .app.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `command` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#registercommand)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.register_launch_command("my-awesome-game://run --full-screen")?;
    /// # Ok(()) }
    /// ```
    pub fn register_launch_command<'b>(&self, command: impl Into<Cow<'b, str>>) -> Result<()> {
        let mut command = command.into();

        if !command.contains('\0') {
            command.to_mut().push('\0')
        };

        unsafe {
            ffi!(self
                .get_activity_manager()
                .register_command(command.as_ptr()))
        }
        .to_result()
    }

    /// Used if you are distributing this SDK on Steam.
    ///
    /// Registers your game's Steam app ID for the protocol `steam://run-game-id/<id>`.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#registersteam)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// # let now = 0;
    /// discord.clear_activity(|discord, result| {
    ///     if let Err(error) = result {
    ///         eprintln!("failed to clear activity: {}", error);
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn register_steam(&self, steam_game_id: u32) -> Result<()> {
        unsafe { ffi!(self.get_activity_manager().register_steam(steam_game_id)) }.to_result()
    }

    /// Sets a user's presence in Discord to a new Activity.
    ///
    /// This has a rate limit of 5 updates per 20 seconds.
    ///
    /// It is possible for users to hide their presence on Discord (User Settings -> Game Activity).
    /// Presence set through this SDK may not be visible when this setting is toggled off.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#updateactivity)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// # let now = 0;
    /// discord.update_activity(
    ///     &Activity::empty()
    ///         .with_state("On Main Menu")
    ///         .with_start_time(now),
    ///     |discord, result| {
    ///         if let Err(error) = result {
    ///             eprintln!("failed to update activity: {}", error);
    ///         }
    ///     },
    /// );
    /// # Ok(()) }
    /// ```
    pub fn update_activity(
        &self,
        activity: &Activity,
        callback: impl 'a + FnMut(&Discord<'_>, Result<()>),
    ) {
        unsafe {
            ffi!(self
                .get_activity_manager()
                .update_activity(
                    // XXX: *mut should be *const
                    &activity.0 as *const _ as *mut _
                )
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Clears a user's presence in Discord to make it show nothing.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#clearactivity)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// # let now = 0;
    /// discord.clear_activity(|discord, result| {
    ///     if let Err(error) = result {
    ///         eprintln!("failed to clear activity: {}", error);
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn clear_activity(&self, callback: impl 'a + FnMut(&Discord<'_>, Result<()>)) {
        unsafe {
            ffi!(self
                .get_activity_manager()
                .clear_activity()
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Sends a reply to an Ask to Join request.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#sendrequestreply)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// for request in discord.recv_activities_request() {
    ///     println!(
    ///         "received join request from {}#{}",
    ///         request.user.username(),
    ///         request.user.discriminator()
    ///     );
    ///
    ///     discord.send_request_reply(request.user.id(), RequestReply::Yes, |discord, result| {
    ///         if let Err(error) = result {
    ///             eprintln!("failed replying: {}", error);
    ///         }
    ///     });
    /// }
    /// # Ok(()) }
    /// ```
    pub fn send_request_reply(
        &self,
        user_id: i64,
        reply: RequestReply,
        callback: impl 'a + FnMut(&Discord<'_>, Result<()>),
    ) {
        unsafe {
            ffi!(self
                .get_activity_manager()
                .send_request_reply(user_id, reply.into())
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Sends a game invite to a given user.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `content` if one is not present.
    ///
    /// ## Error
    ///
    /// If the [required fields] are missing, this will return an error.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#sendinvite)
    ///
    /// [required fields]: struct.Activity.html
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// # let friend = User::from(discord_game_sdk_sys::DiscordUser::default());
    /// discord.send_invite(
    ///     friend.id(),
    ///     Action::Join,
    ///     "Let's play some Survival!\0",
    ///     |discord, result| {
    ///         if let Err(error) = result {
    ///             eprintln!("failed inviting: {}", error);
    ///         }
    ///     },
    /// );
    /// # Ok(()) }
    /// ```
    pub fn send_invite<'b>(
        &self,
        user_id: i64,
        action: Action,
        content: impl Into<Cow<'b, str>>,
        callback: impl 'a + FnMut(&Discord<'_>, Result<()>),
    ) {
        let mut content = content.into();

        if !content.contains('\0') {
            content.to_mut().push('\0')
        };

        unsafe {
            ffi!(self
                .get_activity_manager()
                .send_invite(user_id, action.into(), content.as_ptr())
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Accepts a user's game invitation.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#acceptinvite)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// for request in discord.recv_activities_invite() {
    ///     println!(
    ///         "received invitation to {} from {}#{}",
    ///         if request.action == Action::Join { "join" } else { "spectate" },
    ///         request.user.username(),
    ///         request.user.discriminator()
    ///     );
    ///
    ///     discord.accept_invite(request.user.id(), |discord, result| {
    ///         if let Err(error) = result {
    ///             eprintln!("failed to accept invite: {}", error);
    ///         }
    ///     });
    /// }
    /// # Ok(()) }
    /// ```
    pub fn accept_invite(&self, user_id: i64, callback: impl 'a + FnMut(&Discord<'_>, Result<()>)) {
        unsafe {
            ffi!(self
                .get_activity_manager()
                .accept_invite(user_id)
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Fires when the current user accepts an invitation to join in chat
    /// or receives confirmation from Asking to Join.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#onactivityjoin)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// if let Some(join) = discord.recv_activities_join().next() {
    ///     println!("joining a game");
    ///
    ///     discord.connect_lobby_with_activity_secret(join.secret, |discord, lobby| {
    ///         match lobby {
    ///             Err(error) => eprintln!("failed connecting to lobby: {}", error),
    ///             Ok(lobby) => {
    ///                 // Update activity, connect to voice and network, etc.
    ///             }
    ///         }
    ///     });
    /// }
    /// # Ok(()) }
    /// ```
    pub fn recv_activities_join(&self) -> impl '_ + Iterator<Item = event::ActivityJoin> {
        self.receivers.activities_join.try_iter()
    }

    /// Fires when the current user accepts an invitation to spectate in chat
    /// or clicks the Spectate button on another user's profile.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#onactivityspectate)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// if let Some(spectate) = discord.recv_activities_spectate().next() {
    ///     println!("spectating a game");
    ///
    ///     discord.connect_lobby_with_activity_secret(spectate.secret, |discord, lobby| {
    ///         match lobby {
    ///             Err(error) => eprintln!("failed connecting to lobby: {}", error),
    ///             Ok(lobby) => {
    ///                 // Update activity, connect to voice and network, etc.
    ///             }
    ///         }
    ///     });
    /// }
    /// # Ok(()) }
    pub fn recv_activities_spectate(&self) -> impl '_ + Iterator<Item = event::ActivitySpectate> {
        self.receivers.activities_spectate.try_iter()
    }

    /// Fires when a user asks to join the game of the current user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#onactivityjoinrequest)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// for request in discord.recv_activities_request() {
    ///     println!(
    ///         "received join request from {}#{}",
    ///         request.user.username(),
    ///         request.user.discriminator()
    ///     );
    ///
    ///     discord.send_request_reply(request.user.id(), RequestReply::Yes, |discord, result| {
    ///         if let Err(error) = result {
    ///             eprintln!("failed replying: {}", error);
    ///         }
    ///     });
    /// }
    /// # Ok(()) }
    /// ```
    pub fn recv_activities_request(&self) -> impl '_ + Iterator<Item = event::ActivityRequest> {
        self.receivers.activities_request.try_iter()
    }

    /// Fires when the current user receives an invitation to join or spectate.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#onactivityinvite)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// for request in discord.recv_activities_invite() {
    ///     println!(
    ///         "received invitation to {} from {}#{}",
    ///         if request.action == Action::Join { "join" } else { "spectate" },
    ///         request.user.username(),
    ///         request.user.discriminator()
    ///     );
    ///
    ///     discord.accept_invite(request.user.id(), |discord, result| {
    ///         if let Err(error) = result {
    ///             eprintln!("failed to accept invite: {}", error);
    ///         }
    ///     });
    /// }
    /// # Ok(()) }
    /// ```
    pub fn recv_activities_invite(&self) -> impl '_ + Iterator<Item = event::ActivityInvite> {
        self.receivers.activities_invite.try_iter()
    }
}
