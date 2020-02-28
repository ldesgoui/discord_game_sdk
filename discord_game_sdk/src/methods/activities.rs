use crate::{
    callback, sys, to_result::ToResult, Action, Activity, Discord, RequestReply, Result, UserID,
};
use std::borrow::Cow;

/// # Activities
///
/// Also known as Rich Presence.
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/activities)
impl<'d, E> Discord<'d, E> {
    /// Registers a command by which Discord can launch your game.
    ///
    /// This might be a custom protocol, like `my-awesome-game://`, or a path to an executable.
    /// It also supports any launch parameters that may be needed, like `game.exe --full-screen`.
    ///
    /// On macOS, due to the way Discord registers executables,
    /// your game needs to be bundled for this command to work.
    /// That means it should be a `.app`.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `command` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#registercommand)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord<'_, ()>) -> Result<()> {
    /// discord.register_launch_command("my-awesome-game://run --full-screen")?;
    /// # Ok(()) }
    /// ```
    pub fn register_launch_command<'s>(&self, command: impl Into<Cow<'s, str>>) -> Result<()> {
        let mut command = command.into();

        if !command.ends_with('\0') {
            command.to_mut().push('\0')
        };

        self.with_activity_manager(|mgr| unsafe {
            mgr.register_command.unwrap()(mgr, command.as_ptr())
        })
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
    /// # fn example(discord: Discord<'_, ()>) -> Result<()> {
    /// # let now = 0;
    /// discord.clear_activity(|discord, result| {
    ///     if let Err(error) = result {
    ///         return eprintln!("failed to clear activity: {}", error);
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn register_steam(&self, steam_game_id: u32) -> Result<()> {
        self.with_activity_manager(|mgr| unsafe { mgr.register_steam.unwrap()(mgr, steam_game_id) })
            .to_result()
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
    /// # fn example(discord: Discord<'_, ()>) -> Result<()> {
    /// # let now = 0;
    /// discord.update_activity(
    ///     &Activity::empty()
    ///         .with_state("On Main Menu")
    ///         .with_start_time(now),
    ///     |discord, result| {
    ///         if let Err(error) = result {
    ///             return eprintln!("failed to update activity: {}", error);
    ///         }
    ///     },
    /// );
    /// # Ok(()) }
    /// ```
    pub fn update_activity(
        &self,
        activity: &Activity,
        callback: impl 'd + FnOnce(&Discord<'d, E>, Result<()>),
    ) {
        self.with_activity_manager(|mgr| {
            let (ptr, fun) = callback::one_param(move |res: sys::EDiscordResult| {
                callback(&*self.ref_copy(), res.to_result())
            });

            unsafe {
                mgr.update_activity.unwrap()(
                    mgr,
                    // XXX: *mut should be *const
                    &activity.0 as *const sys::DiscordActivity as *mut sys::DiscordActivity,
                    ptr,
                    fun,
                )
            }
        })
    }

    /// Clears a user's presence in Discord to make it show nothing.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#clearactivity)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord<'_, ()>) -> Result<()> {
    /// # let now = 0;
    /// discord.clear_activity(|discord, result| {
    ///     if let Err(error) = result {
    ///         return eprintln!("failed to clear activity: {}", error);
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn clear_activity(&self, callback: impl 'd + FnOnce(&Discord<'d, E>, Result<()>)) {
        self.with_activity_manager(|mgr| {
            let (ptr, fun) = callback::one_param(move |res: sys::EDiscordResult| {
                callback(&*self.ref_copy(), res.to_result())
            });

            unsafe { mgr.clear_activity.unwrap()(mgr, ptr, fun) }
        })
    }

    /// Sends a reply to an Ask to Join request.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#sendrequestreply)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// struct MyEventHandler;
    ///
    /// impl EventHandler for MyEventHandler {
    ///     fn on_activity_join_request(&mut self, discord: &Discord<'_, Self>, user: &User) {
    ///         println!(
    ///             "received join request from {}#{}",
    ///             user.username(),
    ///             user.discriminator()
    ///         );
    ///
    ///         discord.send_request_reply(user.id(), RequestReply::Yes, |discord, result| {
    ///             if let Err(error) = result {
    ///                 return eprintln!("failed to reply: {}", error);
    ///             }
    ///         });
    ///     }
    /// }
    /// ```
    pub fn send_request_reply(
        &self,
        user_id: UserID,
        reply: RequestReply,
        callback: impl 'd + FnOnce(&Discord<'d, E>, Result<()>),
    ) {
        self.with_activity_manager(|mgr| {
            let (ptr, fun) = callback::one_param(move |res: sys::EDiscordResult| {
                callback(&*self.ref_copy(), res.to_result())
            });

            unsafe { mgr.send_request_reply.unwrap()(mgr, user_id, reply.into(), ptr, fun) }
        })
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
    /// # fn example(discord: Discord<'_, ()>, friend: User) -> Result<()> {
    /// discord.send_invite(
    ///     friend.id(),
    ///     Action::Join,
    ///     "Let's play some Survival!\0",
    ///     |discord, result| {
    ///         if let Err(error) = result {
    ///             return eprintln!("failed to invite: {}", error);
    ///         }
    ///     },
    /// );
    /// # Ok(()) }
    /// ```
    pub fn send_invite<'s>(
        &self,
        user_id: UserID,
        action: Action,
        content: impl Into<Cow<'s, str>>,
        callback: impl 'd + FnOnce(&Discord<'d, E>, Result<()>),
    ) {
        let mut content = content.into();

        if !content.ends_with('\0') {
            content.to_mut().push('\0')
        };

        self.with_activity_manager(|mgr| {
            let (ptr, fun) = callback::one_param(move |res: sys::EDiscordResult| {
                callback(&*self.ref_copy(), res.to_result())
            });

            unsafe {
                mgr.send_invite.unwrap()(mgr, user_id, action.into(), content.as_ptr(), ptr, fun)
            }
        })
    }

    /// Accepts a user's game invitation.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/activities#acceptinvite)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// struct MyEventHandler;
    ///
    /// impl EventHandler for MyEventHandler {
    ///     fn on_activity_invite(
    ///         &mut self,
    ///         discord: &Discord<'_, Self>,
    ///         action: Action,
    ///         user: &User,
    ///         activity: &Activity,
    ///     ) {
    ///         println!(
    ///             "received invitation to {} from {}#{}",
    ///             if action == Action::Join {
    ///                 "join"
    ///             } else {
    ///                 "spectate"
    ///             },
    ///             user.username(),
    ///             user.discriminator()
    ///         );
    ///
    ///         discord.accept_invite(user.id(), |discord, result| {
    ///             if let Err(error) = result {
    ///                 return eprintln!("failed to accept invite: {}", error);
    ///             }
    ///         });
    ///     }
    /// }
    /// ```
    pub fn accept_invite(
        &self,
        user_id: UserID,
        callback: impl 'd + FnOnce(&Discord<'d, E>, Result<()>),
    ) {
        self.with_activity_manager(|mgr| {
            let (ptr, fun) = callback::one_param(move |res: sys::EDiscordResult| {
                callback(&*self.ref_copy(), res.to_result())
            });

            unsafe { mgr.accept_invite.unwrap()(mgr, user_id, ptr, fun) }
        })
    }
}
