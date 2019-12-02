use crate::{callbacks::AnyCallback, channels, sys};

/// Main interface with SDK
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
pub struct Discord<'a> {
    pub(crate) core: *mut sys::IDiscordCore,
    pub(crate) client_id: i64,
    #[allow(dead_code)]
    pub(crate) senders: Box<channels::Senders>,
    pub(crate) receivers: channels::Receivers,
    pub(crate) callbacks: std::cell::UnsafeCell<Vec<Box<dyn AnyCallback + 'a>>>,
}

impl<'a> Discord<'a> {
    pub(crate) fn register_callback(&self, callback: impl AnyCallback + 'a) {
        unsafe { &mut *self.callbacks.get() }.push(Box::new(callback))
    }
}

impl<'a> std::fmt::Debug for Discord<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Discord")
            .field("ffi_ptr", &self.core)
            .field("client_id", &self.client_id)
            .finish()
    }
}
