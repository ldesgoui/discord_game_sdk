use crate::{
    discord::{Discord, DiscordInner},
    sys,
    to_result::ToResult,
    utils::charptr_to_str,
    Activity, ClientID, CreateFlags, Entitlement, EventHandler, Relationship, Result, User,
    UserAchievement,
};
use std::{cell::UnsafeCell, convert::TryFrom, ffi::c_void, ops::DerefMut};

/// # Core
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/discord)
///
/// ```rust
/// # fn example() {
/// use discord_game_sdk::Discord;
///
/// # const DISCORD_CLIENT_ID: discord_game_sdk::ClientID = 0;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut discord = Discord::new(DISCORD_CLIENT_ID)?;
///
///     loop {
///         discord.run_callbacks()?;
///     }
///
///     Ok(())
/// }
/// # }
/// ```
impl Discord {
    /// Calls [`with_create_flags`] with [`CreateFlags::Default`].
    ///
    /// [`with_create_flags`]: #method.with_create_flags
    /// [`CreateFlags::Default`]: enum.CreateFlags.html#variant.Default
    pub fn new(client_id: ClientID) -> Result<Self> {
        Self::with_create_flags(client_id, CreateFlags::Default)
    }

    /// Creates an instance of the main interface with the Discord Game SDK.
    ///
    /// SDK log messages are forwarded to [`log`](https://docs.rs/log)
    ///
    /// > [`Create` in official docs](https://discordapp.com/developers/docs/game-sdk/discord#create)  
    /// > [`SetLogHook` in official docs](https://discordapp.com/developers/docs/game-sdk/discord#setloghook)
    pub fn with_create_flags(client_id: ClientID, flags: CreateFlags) -> Result<Self> {
        // This is a mess
        //
        // - We want to call `sys::DiscordCreate`, it gives us a `*mut sys::IDiscordCore`
        // - We provide `&mut EventHandler` and `&Discord` during event handlers
        // - That means we need to mutate `DiscordInner::event_handler`, which is fine via `UnsafeCell`
        // - That also means we need to pass the `Box` as raw pointer to duplicate it
        // - `sys::DiscordCreate` wants `sys::DiscordCreateParams` + `*mut *mut sys::IDiscordCore`
        // - `sys::DiscordCreateParams` wants our `event_data: *mut c_void`
        // - Our `event_data` is the raw `Box` pointer
        // - We need to build the `Box<DiscordInner>` first to pass a valid pointer

        log::debug!("instantiating with client ID {}", client_id);

        let mut instance = Discord(Box::new(DiscordInner {
            // SAFETY: overwritten by `sys::DiscordCreate`, not deref'd until then
            core: std::ptr::null_mut(),
            client_id,
            event_handler: UnsafeCell::new(None),
        }));

        // SAFETY: As described above, we're passing the Box as raw pointer
        // It'll be used to duplicate the `Box` but won't mutate or drop it
        let ptr = &mut *instance.0 as *mut DiscordInner as *mut std::ffi::c_void;
        let mut params = create_params(client_id, flags.into(), ptr);

        unsafe {
            sys::DiscordCreate(
                sys::DISCORD_VERSION,
                // XXX: *mut should be *const
                &mut params,
                &mut instance.0.core,
            )
            .to_result()?;
        }

        log::trace!("received pointer to {:p}", instance.0.core);

        instance.set_log_hook();
        instance.kickstart_managers();

        Ok(instance)
    }

    fn set_log_hook(&self) {
        unsafe {
            ffi!(self.set_log_hook(
                sys::DiscordLogLevel_Debug,
                // SAFETY: this is never used
                std::ptr::null_mut(),
                Some(log_hook),
            ));
        }
    }

    #[allow(clippy::cognitive_complexity)]
    fn kickstart_managers(&self) {
        unsafe {
            // Signal managers that we want events ASAP
            ffi!(self.get_network_manager());
            ffi!(self.get_overlay_manager());
            ffi!(self.get_relationship_manager());
            ffi!(self.get_user_manager());

            ffi!(self.get_achievement_manager());
            ffi!(self.get_activity_manager());
            ffi!(self.get_lobby_manager());
            ffi!(self.get_store_manager());
            ffi!(self.get_voice_manager());
        }
    }

    /// Runs all pending SDK callbacks.
    ///
    /// This should be called often, like in the main loop if you're writing a game.
    ///
    /// ## Errors
    ///
    /// If the Discord client was closed, [`Error::NotRunning`] will be returned.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/discord#runcallbacks)
    ///
    /// [emptied]: #method.empty_event_buffers
    /// [`Error::NotRunning`]: enum.Error.html#variant.NotRunning
    // We require &mut self to prevent calling during callbacks
    pub fn run_callbacks(&mut self) -> Result<()> {
        unsafe { ffi!(self.run_callbacks()).to_result() }
    }

    /// The Client ID that was supplied during creation
    pub fn client_id(&self) -> ClientID {
        self.0.client_id
    }

    /// Replaces the current event handler
    pub fn replace_event_handler(
        &mut self,
        event_handler: Box<dyn EventHandler>,
    ) -> Option<Box<dyn EventHandler>> {
        self.0.event_handler_mut().replace(event_handler)
    }

    /// Takes the current event handler, leaving `None` in its place
    pub fn take_event_handler(&mut self) -> Option<Box<dyn EventHandler>> {
        self.0.event_handler_mut().take()
    }

    /// Returns some mutable reference to the event handler if it is of type T, or None if it isn't.
    // We require &mut self to prevent calling during callbacks
    pub fn downcast_event_handler<T: std::any::Any>(&mut self) -> Option<&mut T> {
        self.0
            .event_handler_mut()
            .as_mut()
            .and_then(|e| e.downcast_mut())
    }
}

fn create_params(
    client_id: sys::DiscordClientId,
    flags: sys::EDiscordCreateFlags,
    event_data: *mut std::ffi::c_void,
) -> sys::DiscordCreateParams {
    sys::DiscordCreateParams {
        client_id,
        // XXX: u64 should be sys::EDiscordCreateFlags
        flags: u64::try_from(flags).unwrap(),

        // XXX: *mut should be *const
        events: std::ptr::null_mut(),
        event_data,

        // XXX: *mut should be *const
        achievement_events: ACHIEVEMENT as *const _ as *mut _,
        achievement_version: sys::DISCORD_ACHIEVEMENT_MANAGER_VERSION,

        // XXX: *mut should be *const
        activity_events: ACTIVITY as *const _ as *mut _,
        activity_version: sys::DISCORD_ACTIVITY_MANAGER_VERSION,

        // XXX: *mut should be *const
        application_events: std::ptr::null_mut(),
        application_version: sys::DISCORD_APPLICATION_MANAGER_VERSION,

        // XXX: *mut should be *const
        image_events: std::ptr::null_mut(),
        image_version: sys::DISCORD_IMAGE_MANAGER_VERSION,

        // XXX: *mut should be *const
        lobby_events: LOBBY as *const _ as *mut _,
        lobby_version: sys::DISCORD_LOBBY_MANAGER_VERSION,

        // XXX: *mut should be *const
        network_events: NETWORK as *const _ as *mut _,
        network_version: sys::DISCORD_NETWORK_MANAGER_VERSION,

        // XXX: *mut should be *const
        overlay_events: OVERLAY as *const _ as *mut _,
        overlay_version: sys::DISCORD_OVERLAY_MANAGER_VERSION,

        // XXX: *mut should be *const
        relationship_events: RELATIONSHIP as *const _ as *mut _,
        relationship_version: sys::DISCORD_RELATIONSHIP_MANAGER_VERSION,

        // XXX: *mut should be *const
        storage_events: std::ptr::null_mut(),
        storage_version: sys::DISCORD_STORAGE_MANAGER_VERSION,

        // XXX: *mut should be *const
        store_events: STORE as *const _ as *mut _,
        store_version: sys::DISCORD_STORE_MANAGER_VERSION,

        // XXX: *mut should be *const
        user_events: USER as *const _ as *mut _,
        user_version: sys::DISCORD_USER_MANAGER_VERSION,

        // XXX: *mut should be *const
        voice_events: VOICE as *const _ as *mut _,
        voice_version: sys::DISCORD_VOICE_MANAGER_VERSION,
    }
}

fn with_event_handler(inner: *mut c_void, callback: impl FnOnce(&mut dyn EventHandler, &Discord)) {
    prevent_unwind!();

    debug_assert!(!inner.is_null());

    // SAFETY:
    // We're duplicating the `Box<DiscordInner>`, this is safe:
    // - We're not mutating it, we're not dropping it
    // - No other part of the code will mutate it as `&mut Discord` is in the callstack
    let discord = unsafe { Discord(Box::from_raw(inner as *mut DiscordInner)) };

    // SAFETY: Mutation through immutable reference
    // - `discord.0.event_handler` is an `UnsafeCell`, inner mutation is legal
    // - No other part of the code can safely mutate it as they require `&mut DiscordInner`
    // - `EventHandler` can mutate itself during method but not `&Discord`
    let mut event_handler = unsafe { (*discord.0.event_handler.get()).take() };

    if let Some(event_handler) = event_handler.as_mut() {
        callback(event_handler.deref_mut(), &discord);
    }

    // SAFETY: See previous
    unsafe {
        (*discord.0.event_handler.get()) = event_handler;
    }

    // SAFETY: Not dropping our duplicated `Box<DiscordInner>`
    std::mem::forget(discord);
}

const ACHIEVEMENT: &sys::IDiscordAchievementEvents = &sys::IDiscordAchievementEvents {
    on_user_achievement_update: {
        extern "C" fn on_user_achievement_update(
            inner: *mut c_void,
            user_achievement: *mut sys::DiscordUserAchievement,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_user_achievement_update(discord, unsafe {
                    &*(user_achievement as *const UserAchievement)
                })
            })
        }

        Some(on_user_achievement_update)
    },
};

const ACTIVITY: &sys::IDiscordActivityEvents = &sys::IDiscordActivityEvents {
    on_activity_join: {
        extern "C" fn on_activity_join(inner: *mut c_void, secret: *const u8) {
            with_event_handler(inner, |eh, discord| {
                eh.on_activity_join(discord, charptr_to_str(secret))
            })
        }

        Some(on_activity_join)
    },

    on_activity_spectate: {
        extern "C" fn on_activity_spectate(inner: *mut c_void, secret: *const u8) {
            with_event_handler(inner, |eh, discord| {
                eh.on_activity_spectate(discord, charptr_to_str(secret))
            })
        }

        Some(on_activity_spectate)
    },

    on_activity_join_request: {
        extern "C" fn on_activity_join_request(inner: *mut c_void, user: *mut sys::DiscordUser) {
            with_event_handler(inner, |eh, discord| {
                eh.on_activity_join_request(discord, unsafe { &*(user as *const User) })
            })
        }

        Some(on_activity_join_request)
    },

    on_activity_invite: {
        extern "C" fn on_activity_invite(
            inner: *mut c_void,
            kind: sys::EDiscordActivityActionType,
            user: *mut sys::DiscordUser,
            activity: *mut sys::DiscordActivity,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_activity_invite(
                    discord,
                    kind.into(),
                    unsafe { &*(user as *const User) },
                    unsafe { &*(activity as *const Activity) },
                )
            })
        }

        Some(on_activity_invite)
    },
};

const LOBBY: &sys::IDiscordLobbyEvents = &sys::IDiscordLobbyEvents {
    on_lobby_update: {
        extern "C" fn on_lobby_update(inner: *mut c_void, lobby_id: sys::DiscordLobbyId) {
            with_event_handler(inner, |eh, discord| eh.on_lobby_update(discord, lobby_id))
        }

        Some(on_lobby_update)
    },

    on_lobby_delete: {
        extern "C" fn on_lobby_delete(
            inner: *mut c_void,
            lobby_id: sys::DiscordLobbyId,
            reason: u32,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_lobby_delete(discord, lobby_id, reason)
            })
        }

        Some(on_lobby_delete)
    },

    on_member_connect: {
        extern "C" fn on_member_connect(
            inner: *mut c_void,
            lobby_id: sys::DiscordLobbyId,
            member_id: sys::DiscordUserId,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_member_connect(discord, lobby_id, member_id)
            })
        }

        Some(on_member_connect)
    },

    on_member_update: {
        extern "C" fn on_member_update(
            inner: *mut c_void,
            lobby_id: sys::DiscordLobbyId,
            member_id: sys::DiscordUserId,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_member_update(discord, lobby_id, member_id)
            })
        }

        Some(on_member_update)
    },

    on_member_disconnect: {
        extern "C" fn on_member_disconnect(
            inner: *mut c_void,
            lobby_id: sys::DiscordLobbyId,
            member_id: sys::DiscordUserId,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_member_disconnect(discord, lobby_id, member_id)
            })
        }

        Some(on_member_disconnect)
    },

    on_lobby_message: {
        extern "C" fn on_lobby_message(
            inner: *mut c_void,
            lobby_id: sys::DiscordLobbyId,
            member_id: sys::DiscordUserId,
            data: *mut u8,
            data_len: u32,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_lobby_message(discord, lobby_id, member_id, unsafe {
                    std::slice::from_raw_parts(data, data_len as usize)
                })
            })
        }

        Some(on_lobby_message)
    },

    on_speaking: {
        extern "C" fn on_speaking(
            inner: *mut c_void,
            lobby_id: sys::DiscordLobbyId,
            member_id: sys::DiscordUserId,
            speaking: bool,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_speaking(discord, lobby_id, member_id, speaking)
            })
        }

        Some(on_speaking)
    },

    on_network_message: {
        extern "C" fn on_network_message(
            inner: *mut c_void,
            lobby_id: sys::DiscordLobbyId,
            member_id: sys::DiscordUserId,
            channel_id: sys::DiscordNetworkChannelId,
            data: *mut u8,
            data_len: u32,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_lobby_network_message(discord, lobby_id, member_id, channel_id, unsafe {
                    std::slice::from_raw_parts(data, data_len as usize)
                })
            })
        }

        Some(on_network_message)
    },
};

const NETWORK: &sys::IDiscordNetworkEvents = &sys::IDiscordNetworkEvents {
    on_message: {
        extern "C" fn on_message(
            inner: *mut c_void,
            peer_id: sys::DiscordNetworkPeerId,
            channel_id: sys::DiscordNetworkChannelId,
            data: *mut u8,
            data_len: u32,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_network_message(discord, peer_id, channel_id, unsafe {
                    std::slice::from_raw_parts(data, data_len as usize)
                })
            })
        }

        Some(on_message)
    },

    on_route_update: {
        extern "C" fn on_route_update(inner: *mut c_void, route: *const u8) {
            with_event_handler(inner, |eh, discord| {
                eh.on_network_route_update(discord, charptr_to_str(route))
            })
        }

        Some(on_route_update)
    },
};

const OVERLAY: &sys::IDiscordOverlayEvents = &sys::IDiscordOverlayEvents {
    on_toggle: {
        extern "C" fn on_toggle(inner: *mut c_void, locked: bool) {
            with_event_handler(inner, |eh, discord| eh.on_overlay_toggle(discord, !locked))
        }

        Some(on_toggle)
    },
};

const RELATIONSHIP: &sys::IDiscordRelationshipEvents = &sys::IDiscordRelationshipEvents {
    on_refresh: {
        extern "C" fn on_refresh(inner: *mut c_void) {
            with_event_handler(inner, |eh, discord| eh.on_relationships_refresh(discord))
        }

        Some(on_refresh)
    },

    on_relationship_update: {
        extern "C" fn on_relationship_update(
            inner: *mut c_void,
            relationship: *mut sys::DiscordRelationship,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_relationship_update(discord, unsafe {
                    &*(relationship as *const Relationship)
                })
            })
        }

        Some(on_relationship_update)
    },
};

const STORE: &sys::IDiscordStoreEvents = &sys::IDiscordStoreEvents {
    on_entitlement_create: {
        extern "C" fn on_entitlement_create(
            inner: *mut c_void,
            entitlement: *mut sys::DiscordEntitlement,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_entitlement_create(discord, unsafe { &*(entitlement as *const Entitlement) })
            })
        }

        Some(on_entitlement_create)
    },

    on_entitlement_delete: {
        extern "C" fn on_entitlement_delete(
            inner: *mut c_void,
            entitlement: *mut sys::DiscordEntitlement,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_entitlement_delete(discord, unsafe { &*(entitlement as *const Entitlement) })
            })
        }

        Some(on_entitlement_delete)
    },
};

const USER: &sys::IDiscordUserEvents = &sys::IDiscordUserEvents {
    on_current_user_update: {
        extern "C" fn on_current_user_update(inner: *mut c_void) {
            with_event_handler(inner, |eh, discord| eh.on_current_user_update(discord))
        }

        Some(on_current_user_update)
    },
};

const VOICE: &sys::IDiscordVoiceEvents = &sys::IDiscordVoiceEvents {
    on_settings_update: {
        extern "C" fn on_settings_update(inner: *mut c_void) {
            with_event_handler(inner, |eh, discord| eh.on_voice_settings_update(discord))
        }

        Some(on_settings_update)
    },
};

extern "C" fn log_hook(_: *mut std::ffi::c_void, level: sys::EDiscordLogLevel, message: *const u8) {
    prevent_unwind!();

    let level = match level {
        sys::DiscordLogLevel_Error => log::Level::Error,
        sys::DiscordLogLevel_Warn => log::Level::Warn,
        sys::DiscordLogLevel_Info => log::Level::Info,
        sys::DiscordLogLevel_Debug => log::Level::Debug,
        _ => log::Level::Trace,
    };

    log::log!(level, "SDK: {}", charptr_to_str(message));
}
