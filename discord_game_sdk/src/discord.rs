use crate::{sys, ClientID};
use std::{cell::UnsafeCell, marker::PhantomData, mem::ManuallyDrop};

/// Main interface with SDK
///
/// The Discord Game SDK is not thread-safe, this struct should only be made `Send`/`Sync` with
/// appropriate safety measures, and not as-is.
///
/// As opposed to the general structure of the Discord Game SDK, and to help with memory and thread
/// safety, the methods of the Manager "classes" are part of this struct.
///
/// ### Callbacks
///
/// All `callback`s will be called with `Err(TransactionAborted)` when the instance is dropped
///
/// ```rust,compile_fail
/// // Static test to verify callbacks exhibit proper ownership
/// # use discord_game_sdk::*;
/// # fn example(discord: Discord<'_, ()>) -> Result<()>
/// {
///     let illegal = "hey".to_string();
///     discord.validate_or_exit(move |_, _| {
///         dbg!(&illegal); // moved here, will outlive outer block
///     });
///     dbg!(&illegal); // but borrowed here, illegal
/// #   Ok(())
/// }
/// ```
///
/// ### Iterators
///
/// ```rust,compile_fail
/// // Static test to verify `Iterator`s depend on `Discord`'s lifetime
/// # use discord_game_sdk::*;
/// # fn example(discord: Discord<'_, ()>) -> Result<()> {
/// let mut iter = discord.iter_user_achievements();
/// drop(discord); // dropped while a live reference exists, illegal
/// for achievement in iter {
///     // ...
/// }
/// # Ok(()) }
/// ```
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
pub struct Discord<'d, E>(pub(crate) *mut DiscordInner<'d, E>);

impl<E> Drop for Discord<'_, E> {
    fn drop(&mut self) {
        unsafe {
            let core = (*self.0).core;
            if !core.is_null() {
                (*core).destroy.unwrap()(core);
            }

            drop(Box::from_raw(self.0));
        }
    }
}

impl<'d, E> Discord<'d, E> {
    /// The Client ID that was supplied during creation
    pub fn client_id(&self) -> ClientID {
        self.inner().client_id
    }

    /// The [`EventHandler`](trait.EventHandler.html)
    pub fn event_handler(&self) -> &Option<E> {
        self.inner().event_handler()
    }

    /// The [`EventHandler`](trait.EventHandler.html)
    pub fn event_handler_mut(&mut self) -> &mut Option<E> {
        self.inner_mut().event_handler_mut()
    }

    pub(crate) fn inner(&self) -> &DiscordInner<'d, E> {
        unsafe { &*self.0 }
    }

    pub(crate) fn inner_mut(&mut self) -> &mut DiscordInner<'d, E> {
        unsafe { &mut *self.0 }
    }

    pub(crate) fn ref_copy(&self) -> DiscordRef<'d, E> {
        DiscordRef(ManuallyDrop::new(Discord(self.0)))
    }
}

impl<E: std::fmt::Debug> std::fmt::Debug for Discord<'_, E> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_tuple("Discord").field(&self.inner()).finish()
    }
}

pub(crate) struct DiscordInner<'d, E> {
    pub(crate) _invariant_lifetime: PhantomData<*mut &'d ()>,

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

impl<E> DiscordInner<'_, E> {
    pub(crate) fn event_handler(&self) -> &Option<E> {
        unsafe { &*self.event_handler.get() }
    }

    pub(crate) fn event_handler_mut(&mut self) -> &mut Option<E> {
        unsafe { &mut *self.event_handler.get() }
    }
}

impl<E: std::fmt::Debug> std::fmt::Debug for DiscordInner<'_, E> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("DiscordInner")
            .field("ffi_ptr", &self.core)
            .field("client_id", &self.client_id)
            .field("event_handler", self.event_handler())
            .finish()
    }
}

#[derive(Debug)]
pub(crate) struct DiscordRef<'d, E>(ManuallyDrop<Discord<'d, E>>);

impl<'d, E> std::ops::Deref for DiscordRef<'d, E> {
    type Target = Discord<'d, E>;

    fn deref(&self) -> &Discord<'d, E> {
        &self.0
    }
}
