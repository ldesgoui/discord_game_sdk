use crate::sys;
use std::cell::UnsafeCell;

/// Main interface with SDK
///
/// The Discord Game SDK is not thread-safe, this struct should only be made `Send`/`Sync` with
/// appropriate safety measures, and not as-is.
///
/// As opposed to the general structure of the Discord Game SDK, and to help with memory and thread
/// safety, the methods of the Manager "classes" are part of this struct.
///
/// > All `callback`s will be called with `Err(TransactionAborted)` when the instance is dropped
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
pub struct Discord<E>(pub(crate) Box<DiscordInner<E>>);

pub(crate) struct DiscordInner<E> {
    pub(crate) core: *mut sys::IDiscordCore,
    pub(crate) client_id: sys::DiscordClientId,
    pub(crate) event_handler: UnsafeCell<Option<E>>,

    pub(crate) achievement_events: sys::IDiscordAchievementEvents,
    pub(crate) activity_events: sys::IDiscordActivityEvents,
    pub(crate) lobby_events: sys::IDiscordLobbyEvents,
    pub(crate) network_events: sys::IDiscordNetworkEvents,
    pub(crate) overlay_events: sys::IDiscordOverlayEvents,
    pub(crate) relationship_events: sys::IDiscordRelationshipEvents,
    pub(crate) store_events: sys::IDiscordStoreEvents,
    pub(crate) user_events: sys::IDiscordUserEvents,
    pub(crate) voice_events: sys::IDiscordVoiceEvents,
}

impl<E> DiscordInner<E> {
    pub(crate) fn event_handler(&self) -> &Option<E> {
        unsafe { &*self.event_handler.get() }
    }

    pub(crate) fn event_handler_mut(&mut self) -> &mut Option<E> {
        unsafe { &mut *self.event_handler.get() }
    }
}

impl<E> Drop for DiscordInner<E> {
    fn drop(&mut self) {
        unsafe { (*self.core).destroy.unwrap()(self.core) }
    }
}

impl<E: std::fmt::Debug> std::fmt::Debug for DiscordInner<E> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("DiscordInner")
            .field("ffi_ptr", &self.core)
            .field("client_id", &self.client_id)
            .field("event_handler", self.event_handler())
            .finish()
    }
}
