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
    /// `percent_complete` must be [0..=100]
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

    /// Loads a stable list of the current user's achievements to iterate over.
    /// Do your iteration within the callback of this function.
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

    /// Gets the user achievement for the given achievement id.
    /// [`fetch_achievements`](#method.fetch_achievements) must be called before.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/achievements#getuserachievement>
    pub fn user_achievement(&mut self, achievement_id: i64) -> Result<UserAchievement> {
        let mut achievement = sys::DiscordUserAchievement::default();

        unsafe {
            ffi!(self
                .get_achievement_manager()
                .get_user_achievement(achievement_id, &mut achievement))
        }
        .to_result()?;

        Ok(achievement.into())
    }

    /// <https://discordapp.com/developers/docs/game-sdk/achievements#countuserachievements>  
    pub fn user_achievement_count(&mut self) -> i32 {
        let mut count = 0;

        unsafe {
            ffi!(self
                .get_achievement_manager()
                .count_user_achievements(&mut count))
        }

        count
    }

    /// <https://discordapp.com/developers/docs/game-sdk/achievements#getuserachievementat>
    pub fn user_achievement_at(&mut self, index: i32) -> Result<UserAchievement> {
        let mut achievement = sys::DiscordUserAchievement::default();

        unsafe {
            ffi!(self
                .get_achievement_manager()
                .get_user_achievement_at(index, &mut achievement))
        }
        .to_result()?;

        Ok(achievement.into())
    }

    pub fn iter_user_achievements<'b>(
        &'b mut self,
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
