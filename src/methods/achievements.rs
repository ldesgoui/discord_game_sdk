use crate::{
    callback, iter, sys, to_result::ToResult, Discord, Result, Snowflake, UserAchievement,
};
use std::convert::TryInto;

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
    pub fn set_user_achievement<'d>(
        &'d self,
        achievement_id: Snowflake,
        percent_complete: u8,
        callback: impl 'd + FnOnce(&Self, Result<()>),
    ) {
        debug_assert!((0..=100).contains(&percent_complete));

        self.with_achievement_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(self, res.to_result()));

            unsafe {
                mgr.set_user_achievement.unwrap()(mgr, achievement_id, percent_complete, ptr, fun)
            }
        });
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
    pub fn fetch_user_achievements<'d>(&'d self, callback: impl 'd + FnOnce(&Self, Result<()>)) {
        self.with_achievement_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(self, res.to_result()));

            unsafe { mgr.fetch_user_achievements.unwrap()(mgr, ptr, fun) }
        });
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

        self.with_achievement_manager(|mgr| unsafe {
            mgr.get_user_achievement.unwrap()(mgr, achievement_id, &mut achievement.0)
        })
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
    pub fn user_achievement_count(&self) -> u32 {
        let mut count = 0;

        self.with_achievement_manager(|mgr| unsafe {
            mgr.count_user_achievements.unwrap()(mgr, &mut count)
        });

        // XXX: i32 should be u32
        count.try_into().unwrap()
    }

    /// Gets a user achievement by index.
    ///
    /// Prefer using [`iter_user_achievements`](#method.iter_user_achievements).
    ///
    /// [`fetch_user_achievements`](#method.fetch_user_achievements) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/achievements#getuserachievementat)
    pub fn user_achievement_at(&self, index: u32) -> Result<UserAchievement> {
        let mut achievement = UserAchievement(sys::DiscordUserAchievement::default());

        self.with_achievement_manager(|mgr| unsafe {
            mgr.get_user_achievement_at.unwrap()(
                mgr,
                // XXX: i32 should be u32
                index.try_into().unwrap(),
                &mut achievement.0,
            )
        })
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
    pub fn iter_user_achievements<'d>(
        &'d self,
    ) -> impl 'd
           + Iterator<Item = Result<UserAchievement>>
           + DoubleEndedIterator
           + ExactSizeIterator
           + std::iter::FusedIterator
           + std::fmt::Debug {
        iter::Collection::new(
            self,
            Self::user_achievement_at,
            self.user_achievement_count(),
        )
    }
}
