use crate::{
    callbacks::ResultCallback, event, sys, to_result::ToResult, Achievement, Discord, Result,
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
    pub fn set_achievement(
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
    pub fn fetch_achievements(&mut self, callback: impl FnMut(&mut Discord, Result<()>) + 'a) {
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
    pub fn achievement(&mut self, achievement_id: i64) -> Result<Achievement> {
        let mut achievement = sys::DiscordUserAchievement::default();

        unsafe {
            ffi!(self
                .get_achievement_manager()
                .get_user_achievement(achievement_id, &mut achievement))
        }
        .to_result()?;

        Ok(achievement.into())
    }

    /// Gets the user's list of achievements
    /// [`fetch_achievements`](#method.fetch_achievements) must be called before.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/achievements#countuserachievements>  
    /// <https://discordapp.com/developers/docs/game-sdk/achievements#getuserachievementat>
    pub fn all_achievements(&mut self) -> Result<Vec<Achievement>> {
        let mut count: i32 = 0;

        unsafe {
            ffi!(self
                .get_achievement_manager()
                .count_user_achievements(&mut count))
        }

        let mut result = Vec::with_capacity(count as usize);
        let mut achievement = sys::DiscordUserAchievement::default();

        for index in 0..count {
            unsafe {
                ffi!(self
                    .get_achievement_manager()
                    .get_user_achievement_at(index, &mut achievement))
            }
            .to_result()?;

            result.push(achievement.into());
        }

        Ok(result)
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
