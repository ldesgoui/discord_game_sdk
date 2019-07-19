use crate::sys;

#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Achievement(pub(crate) sys::DiscordUserAchievement);

impl Achievement {
    pub fn user_id(&self) -> i64 {
        self.0.user_id
    }

    pub fn achievement_id(&self) -> i64 {
        self.0.achievement_id
    }

    pub fn percent_complete(&self) -> u8 {
        self.0.percent_complete
    }

    get_str!(unlocked_at, unlocked_at);
}

impl std::fmt::Debug for Achievement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Achievement")
            .field("user_id", &self.user_id())
            .field("achievement_id", &self.achievement_id())
            .field("percent_complete", &self.percent_complete())
            .field("unlocked_at", &self.unlocked_at())
            .finish()
    }
}
