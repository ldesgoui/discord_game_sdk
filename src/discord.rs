use crate::event::{ActivityEvent, OverlayEvent, UserEvent, VoiceEvent};
use crate::prelude::*;

pub struct Discord {
    pub(crate) core_ptr: *mut sys::IDiscordCore,
    pub(crate) client_id: i64,
    pub(crate) activity_events: shrev::EventChannel<ActivityEvent>,
    pub(crate) user_events: shrev::EventChannel<UserEvent>,
    pub(crate) overlay_events: shrev::EventChannel<OverlayEvent>,
    pub(crate) voice_events: shrev::EventChannel<VoiceEvent>,
}

impl std::fmt::Debug for Discord {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Discord")
            .field("client_id", &self.client_id)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CreateFlags {
    /// Requires Discord to be running to play the game
    Default,
    /// Does not require Discord to be running, use this on other platforms
    NoRequireDiscord,
}

impl Default for CreateFlags {
    fn default() -> Self {
        CreateFlags::Default
    }
}

impl CreateFlags {
    pub(crate) fn to_sys(self) -> sys::EDiscordCreateFlags {
        match self {
            CreateFlags::Default => sys::DiscordCreateFlags_Default,
            CreateFlags::NoRequireDiscord => sys::DiscordCreateFlags_NoRequireDiscord,
        }
    }
}
