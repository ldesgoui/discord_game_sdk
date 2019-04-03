use crate::prelude::*;

pub struct Discord<'a> {
    pub(crate) core: &'a mut sys::IDiscordCore,
    pub(crate) client_id: i64,
    pub(crate) activity_channel: shrev::EventChannel<event::Activity>,
    pub(crate) lobby_channel: shrev::EventChannel<event::Lobby>,
    pub(crate) network_channel: shrev::EventChannel<event::Network>,
    pub(crate) overlay_channel: shrev::EventChannel<event::Overlay>,
    pub(crate) relationship_channel: shrev::EventChannel<event::Relationship>,
    pub(crate) store_channel: shrev::EventChannel<event::Store>,
    pub(crate) user_channel: shrev::EventChannel<event::User>,
    pub(crate) voice_channel: shrev::EventChannel<event::Voice>,
}

impl<'a> std::fmt::Debug for Discord<'a> {
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
