use crate::{sys, Activity, Status};

/// User Presence
///
/// <https://discordapp.com/developers/docs/game-sdk/relationships#data-models-presence-struct>
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Presence {
    status: Status,
    activity: Activity,
}

impl Presence {
    pub fn status(&self) -> Status {
        self.status
    }

    pub fn activity(&self) -> &Activity {
        &self.activity
    }
}

impl From<sys::DiscordPresence> for Presence {
    fn from(sys: sys::DiscordPresence) -> Self {
        Self {
            status: sys.status.into(),
            activity: sys.activity.into(),
        }
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
