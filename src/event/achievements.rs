use crate::Achievement;

/// On User Achievement Update
///
/// <https://discordapp.com/developers/docs/game-sdk/achievements#onuserachievementupdate>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Update {
    pub achievement: Achievement,
}
