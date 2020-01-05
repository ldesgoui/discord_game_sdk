use crate::{callbacks::AnyCallback, channels, sys};
use std::cell::UnsafeCell;

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
pub struct Discord<'a> {
    pub(crate) core: *mut sys::IDiscordCore,
    pub(crate) client_id: i64,
    #[allow(dead_code)]
    pub(crate) senders: Box<channels::Senders>,
    pub(crate) receivers: channels::Receivers,
    pub(crate) callbacks: UnsafeCell<Vec<Box<dyn AnyCallback + 'a>>>,
}

impl<'a> Discord<'a> {
    pub(crate) fn register_callback(&self, callback: impl AnyCallback + 'a) {
        let callbacks = unsafe { &mut *self.callbacks.get() };

        callbacks.push(Box::new(callback))
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
