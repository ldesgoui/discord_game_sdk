use crate::{callbacks::ResultCallback, sys, to_result::ToResult, Achievement, Discord, Result};

/// # Achievements
///
/// <https://discordapp.com/developers/docs/game-sdk/achievements>
impl<'a> Discord<'a> {
    /// `percent_complete` must be [0..=100]
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/achievements#setuserachievement>
    pub fn set_achievement(
        &'a mut self,
        achievement_id: i64,
        percent_complete: i64,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        unsafe {
            ffi!(self
                .get_achievement_manager()
                .set_user_achievement(achievement_id, percent_complete)
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// <https://discordapp.com/developers/docs/game-sdk/achievements#fetchuserachievements>
    pub fn fetch_achievements(&'a mut self, callback: impl FnMut(&mut Discord, Result<()>) + 'a) {
        unsafe {
            ffi!(self
                .get_achievement_manager()
                .fetch_user_achievements()
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// <https://discordapp.com/developers/docs/game-sdk/achievements#getuserachievement>
    pub fn achievement(&'a mut self, achievement_id: i64) -> Result<Achievement> {
        let mut achievement = Achievement(sys::DiscordUserAchievement::default());

        unsafe {
            ffi!(self
                .get_achievement_manager()
                .get_user_achievement(achievement_id, &mut achievement.0))
        }
        .to_result()?;

        Ok(achievement)
    }

    /// <https://discordapp.com/developers/docs/game-sdk/achievements#countuserachievements>  
    /// <https://discordapp.com/developers/docs/game-sdk/achievements#getuserachievementat>
    pub fn all_achievements(&'a mut self) -> Result<Vec<Achievement>> {
        let mut count: i32 = 0;

        unsafe {
            ffi!(self
                .get_achievement_manager()
                .count_user_achievements(&mut count))
        }

        let mut result = Vec::with_capacity(count as usize);
        let mut achievement = Achievement(sys::DiscordUserAchievement::default());

        for index in 0..count {
            unsafe {
                ffi!(self
                    .get_achievement_manager()
                    .get_user_achievement_at(index, &mut achievement.0))
            }
            .to_result()?;

            result.push(achievement);
        }

        Ok(result)
    }
}
