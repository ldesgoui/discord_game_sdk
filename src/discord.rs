use crate::{sys, EventHandler};

/// Main interface with SDK
///
/// The Discord Game SDK is not thread-safe, this struct should only be made `Send`/`Sync` with
/// appropriate safety measures, and not as-is.
///
/// As opposed to the general structure of the Discord Game SDK, and to help with memory and thread
/// safety, the methods of the Manager "classes" are part of this struct.
///
/// ## Table of Contents
///
/// - [Core](#core)
/// - [Achievements](#achievements)
/// - [Activities](#activities)
/// - [Applications](#applications)
/// - [Images](#images)
/// - [Lobbies](#lobbies)
/// - [Networking](#networking)
/// - [Overlay](#overlay)
/// - [Relationships](#relationships)
/// - [Storage](#storage)
/// - [Store](#store)
/// - [Users](#users)
/// - [Voice](#voice)
#[derive(Debug)]
#[repr(transparent)]
pub struct Discord(pub(crate) Box<DiscordInner>);

impl Drop for Discord {
    fn drop(&mut self) {
        unsafe { ffi!(self.destroy()) }
    }
}

pub(crate) struct DiscordInner {
    pub(crate) core: *mut sys::IDiscordCore,
    pub(crate) client_id: sys::DiscordClientId,
    pub(crate) event_handler: Box<dyn EventHandler>,
}

impl std::fmt::Debug for DiscordInner {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("DiscordInner")
            .field("ffi_ptr", &self.core)
            .field("client_id", &self.client_id)
            .field("event_handler", &(..))
            .finish()
    }
}
