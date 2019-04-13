use crate::{sys, Activity, Status};

#[derive(Clone, Copy, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Presence(pub(crate) sys::DiscordPresence);

impl Presence {
    pub fn status(&self) -> Status {
        self.0.status.into()
    }

    pub fn activity(&self) -> &Activity {
        unsafe { std::mem::transmute(&self.0.activity) }
    }
}
