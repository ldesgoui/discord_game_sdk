use crate::{sys, Activity, Status};

#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Presence(pub(crate) sys::DiscordPresence);

impl Presence {
    pub fn status(&self) -> Status {
        self.0.status.into()
    }

    pub fn activity(&self) -> &Activity {
        unsafe { &*(&self.0.activity as *const _ as *const Activity) }
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
