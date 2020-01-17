use crate::{sys, to_result::ToResult, Collection, Discord, Result, Snowflake, UserAchievement};

/// # Achievements
///
/// Achievements are managed in the [Developer Portal](https://discordapp.com/developers/applications).
///
/// Some operations will require an HTTP client, or must be ran from your game backend:
/// [Reference](https://discordapp.com/developers/docs/game-sdk/achievements#the-api-way).
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/achievements)
impl Discord {
    /// Updates the current user's completion for a given achievement.
    ///
    /// `percent_complete` must be in the range `0..=100`.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/achievements#setuserachievement)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # #[derive(Default)] struct GameAchievement { id: Snowflake, progress: u32, completion: u32 }
    /// # fn example(discord: Discord) -> Result<()> {
    /// # let achievement = GameAchievement::default();
    /// discord.set_user_achievement(
    ///     achievement.id,
    ///     (achievement.progress * 100 / achievement.completion) as u8,
    ///     |discord, result| {
    ///         if let Err(error) = result {
    ///             eprintln!("failed setting user achievement: {}", error);
    ///         }
    ///     },
    /// );
    /// # Ok(()) }
    pub fn set_user_achievement(
        &self,
        achievement_id: Snowflake,
        percent_complete: u8,
        callback: impl 'static + FnOnce(&Self, Result<()>),
    ) {
        debug_assert!((0..=100).contains(&percent_complete));

        unsafe {
            ffi!(self
                .get_achievement_manager()
                .set_user_achievement(achievement_id, percent_complete)
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }

    /// Loads the current user's achievements.
    ///
    /// The user achievements will remain loaded after `callback` returns.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/achievements#fetchuserachievements)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.fetch_user_achievements(
    ///     |discord, result| {
    ///         if let Err(error) = result {
    ///             return eprintln!("failed fetching user achievements: {}", error);
    ///         }
    ///
    ///         for achievement in discord.iter_user_achievements() {
    ///             // ...
    ///         }
    ///     },
    /// );
    /// # Ok(()) }
    pub fn fetch_user_achievements(&self, callback: impl 'static + FnOnce(&Self, Result<()>)) {
        unsafe {
            ffi!(self
                .get_achievement_manager()
                .fetch_user_achievements()
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }

    /// Gets the user achievement for the given achievement ID.
    ///
    /// [`fetch_user_achievements`](#method.fetch_user_achievements) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/achievements#getuserachievement)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # const ACHIEVEMENT_ID: Snowflake = 0;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.fetch_user_achievements(
    ///     |discord, result| {
    ///         if let Err(error) = result {
    ///             return eprintln!("failed fetching user achievements: {}", error);
    ///         }
    ///
    ///         let achievement = discord.user_achievement(ACHIEVEMENT_ID);
    ///
    ///         if let Err(error) = achievement {
    ///             return eprintln!("failed getting user achievement: {}", error);
    ///         }
    ///     },
    /// );
    /// # Ok(()) }
    pub fn user_achievement(&self, achievement_id: Snowflake) -> Result<UserAchievement> {
        let mut achievement = UserAchievement(sys::DiscordUserAchievement::default());

        unsafe {
            ffi!(self
                .get_achievement_manager()
                .get_user_achievement(achievement_id, &mut achievement.0))
        }
        .to_result()?;

        Ok(achievement)
    }

    /// Gets the number of user achievements available.
    ///
    /// Prefer using [`iter_user_achievements`](#method.iter_user_achievements).
    ///
    /// [`fetch_user_achievements`](#method.fetch_user_achievements) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/achievements#countuserachievements)  
    pub fn user_achievement_count(&self) -> usize {
        let mut count = 0;

        unsafe {
            ffi!(self
                .get_achievement_manager()
                .count_user_achievements(&mut count))
        }

        // XXX: i32 should be usize
        count as usize
    }

    /// Gets a user achievement by index.
    ///
    /// Prefer using [`iter_user_achievements`](#method.iter_user_achievements).
    ///
    /// [`fetch_user_achievements`](#method.fetch_user_achievements) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/achievements#getuserachievementat)
    pub fn user_achievement_at(&self, index: usize) -> Result<UserAchievement> {
        let mut achievement = UserAchievement(sys::DiscordUserAchievement::default());

        unsafe {
            ffi!(self.get_achievement_manager().get_user_achievement_at(
                // XXX: i32 should be usize
                index as i32,
                &mut achievement.0
            ))
        }
        .to_result()?;

        Ok(achievement)
    }

    /// Returns an `Iterator` over all user achievements available.
    ///
    /// [`fetch_user_achievements`](#method.fetch_user_achievements) must have completed first and must not
    /// be called during the iteration.
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.fetch_user_achievements(
    ///     |discord, result| {
    ///         if let Err(error) = result {
    ///             return eprintln!("failed fetching user achievements: {}", error);
    ///         }
    ///
    ///         for achievement in discord.iter_user_achievements() {
    ///             // ...
    ///         }
    ///     },
    /// );
    /// # Ok(()) }
    pub fn iter_user_achievements(&self) -> Collection<Result<UserAchievement>> {
        Collection::new(
            self,
            Box::new(Self::user_achievement_at),
            self.user_achievement_count(),
        )
    }
}
