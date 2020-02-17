use crate::{sys, Activity, Status};

/// User Presence
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/relationships#data-models-presence-struct)
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Presence(pub(crate) sys::DiscordPresence);

impl Presence {
    /// The user's current online status
    pub fn status(&self) -> Status {
        self.0.status.into()
    }

    /// The user's current activity
    pub fn activity(&self) -> &Activity {
        unsafe { &*(&self.0.activity as *const sys::DiscordActivity as *const Activity) }
    }
}

impl std::fmt::Debug for Presence {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Presence")
            .field("status", &self.status())
            .field("activity", &self.activity())
            .finish()
    }
}

impl std::fmt::Display for Presence {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}, {}", self.status(), self.activity())
    }
}
