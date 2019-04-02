use crate::event::{ActivityEvent, UserEvent};
use crate::prelude::*;

pub struct Discord {
    pub(crate) core_ptr: *mut sys::IDiscordCore,
    pub(crate) client_id: i64,
    pub(crate) activity_events: shrev::EventChannel<ActivityEvent>,
    pub(crate) user_events: shrev::EventChannel<UserEvent>,
}

impl std::fmt::Debug for Discord {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Discord")
            .field("client_id", &self.client_id)
            .finish()
    }
}
