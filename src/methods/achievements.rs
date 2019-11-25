use crate::{
    callbacks::ResultCallback, event, iter, sys, to_result::ToResult, Discord, Result,
    UserAchievement,
};

/// # Achievements
///
/// Achievements are managed in the [Developer Portal](https://discordapp.com/developers/applications).
///
/// Some operations will require an http client, or must be ran from your game backend:
/// [Reference](https://discordapp.com/developers/docs/game-sdk/achievements#the-api-way).
///
/// <https://discordapp.com/developers/docs/game-sdk/achievements>
impl<'a> Discord<'a> {
    /// Updates the current user's completion for a given achievement.
    ///
    /// `percent_complete` must be in the range `0..=100`
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/achievements#setuserachievement>
    pub fn set_user_achievement(
        &mut self,
        achievement_id: i64,
        percent_complete: u8,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        unsafe {
            ffi!(self
                .get_achievement_manager()
                .set_user_achievement(achievement_id, percent_complete)
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Loads the current user's achievements.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/achievements#fetchuserachievements>
    pub fn fetch_user_achievements(&mut self, callback: impl FnMut(&mut Discord, Result<()>) + 'a) {
        unsafe {
            ffi!(self
                .get_achievement_manager()
                .fetch_user_achievements()
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Gets the user achievement for the given achievement ID.
    ///
    /// [`fetch_achievements`](#method.fetch_achievements) must have completed first.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/achievements#getuserachievement>
    pub fn user_achievement(&self, achievement_id: i64) -> Result<UserAchievement> {
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
    /// [`fetch_achievements`](#method.fetch_achievements) must have completed first.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/achievements#countuserachievements>  
    pub fn user_achievement_count(&self) -> i32 {
        let mut count = 0;

        unsafe {
            ffi!(self
                .get_achievement_manager()
                .count_user_achievements(&mut count))
        }

        count
    }

    /// Gets a user achievement by index.
    ///
    /// [`fetch_achievements`](#method.fetch_achievements) must have completed first.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/achievements#getuserachievementat>
    pub fn user_achievement_at(&self, index: i32) -> Result<UserAchievement> {
        let mut achievement = UserAchievement(sys::DiscordUserAchievement::default());

        unsafe {
            ffi!(self
                .get_achievement_manager()
                .get_user_achievement_at(index, &mut achievement.0))
        }
        .to_result()?;

        Ok(achievement)
    }

    /// Returns an `Iterator` over all user achievements available.
    ///
    /// [`fetch_achievements`](#method.fetch_achievements) must have completed first and must not
    /// be called during the iteration.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/achievements#countuserachievements>  
    /// <https://discordapp.com/developers/docs/game-sdk/achievements#getuserachievementat>
    pub fn iter_user_achievements<'b>(
        &'b self,
    ) -> iter::GenericIter<'a, 'b, Result<UserAchievement>> {
        let count = self.user_achievement_count();

        iter::GenericIter::new(self, Box::new(|d, i| d.user_achievement_at(i)), count)
    }

    /// Fires when an achievement is updated for the current user.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/achievements#onuserachievementupdate>
    pub fn recv_achievements_update(
        &self,
    ) -> impl '_ + Iterator<Item = event::achievements::Update> {
        self.receivers.achievements_update.try_iter()
    }
}
