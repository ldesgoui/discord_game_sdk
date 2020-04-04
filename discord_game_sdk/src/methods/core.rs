use crate::{
    discord::{Discord, DiscordInner},
    events, sys,
    to_result::ToResult,
    utils, ClientID, CreateFlags, EventHandler, Result,
};
use std::{cell::UnsafeCell, convert::TryFrom, marker::PhantomData};

/// # Core
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/discord)
///
/// ```rust
/// # fn example() {
/// use discord_game_sdk::Discord;
///
/// # const DISCORD_CLIENT_ID: discord_game_sdk::ClientID = 0;
/// # #[derive(Debug, Default)] struct MyEventHandler;
/// # impl discord_game_sdk::EventHandler for MyEventHandler {}
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut discord = Discord::new(DISCORD_CLIENT_ID)?;
///     *discord.event_handler_mut() = Some(MyEventHandler::default());
///
///     loop {
///         discord.run_callbacks()?;
///     }
///
///     Ok(())
/// }
/// # }
/// ```
impl<E> Discord<'_, E> {
    /// Calls [`with_create_flags`] with [`CreateFlags::Default`].
    ///
    /// [`with_create_flags`]: #method.with_create_flags
    /// [`CreateFlags::Default`]: enum.CreateFlags.html#variant.Default
    pub fn new(client_id: ClientID) -> Result<Self>
    where
        E: EventHandler,
    {
        Self::with_create_flags(client_id, CreateFlags::Default)
    }

    /// Creates an instance of the main interface with the Discord Game SDK.
    ///
    /// SDK log messages are forwarded to [`log`](https://docs.rs/log)
    ///
    /// > [`Create` in official docs](https://discordapp.com/developers/docs/game-sdk/discord#create)  
    /// > [`SetLogHook` in official docs](https://discordapp.com/developers/docs/game-sdk/discord#setloghook)
    pub fn with_create_flags(client_id: ClientID, flags: CreateFlags) -> Result<Self>
    where
        E: EventHandler,
    {
        // This is a mess
        //
        // - We want to call `sys::DiscordCreate`, it gives us a `*mut sys::IDiscordCore`
        // - We provide `&mut EventHandler` and `&Discord` during event handlers
        // - That means we need to mutate `DiscordInner::event_handler`, which is fine via `UnsafeCell`
        // - That also means we need to duplicate the `*mut DiscordInner`
        // - `sys::DiscordCreate` wants `sys::DiscordCreateParams` + `*mut *mut sys::IDiscordCore`
        // - `sys::DiscordCreateParams` wants our `event_data: *mut c_void`
        // - Our `event_data` is `*mut DiscordInner`
        // - We need to build the `Discord` first to pass a valid pointer

        log::debug!("instantiating with client ID {}", client_id);

        let mut instance = Discord(Box::into_raw(Box::new(DiscordInner {
            _invariant_lifetime: PhantomData,

            // SAFETY: overwritten by `sys::DiscordCreate`, not deref'd until then
            core: std::ptr::null_mut(),
            client_id,
            event_handler: UnsafeCell::new(None),

            achievement_events: events::achievement::<E>(),
            activity_events: events::activity::<E>(),
            lobby_events: events::lobby::<E>(),
            network_events: events::network::<E>(),
            overlay_events: events::overlay::<E>(),
            relationship_events: events::relationship::<E>(),
            store_events: events::store::<E>(),
            user_events: events::user::<E>(),
            voice_events: events::voice::<E>(),
        })));

        let mut params = instance.create_params(flags.into());

        unsafe {
            sys::DiscordCreate(
                sys::DISCORD_VERSION,
                &mut params,
                &mut instance.inner_mut().core,
            )
            .to_result()?;
        }

        log::trace!("received pointer to {:p}", instance.inner().core);

        instance.set_log_hook();
        instance.kickstart_managers();

        Ok(instance)
    }

    pub(crate) fn create_params(
        &self,
        flags: sys::EDiscordCreateFlags,
    ) -> sys::DiscordCreateParams {
        sys::DiscordCreateParams {
            client_id: self.client_id(),
            flags: u64::try_from(flags).unwrap(),

            events: std::ptr::null_mut(),
            event_data: self.0 as *mut std::ffi::c_void,

            // SAFETY: pointers are safe
            // they last until `DiscordInner` is dropped,
            // and the SDK won't dereference them after that
            achievement_events: &self.inner().achievement_events as *const _ as *mut _,
            achievement_version: sys::DISCORD_ACHIEVEMENT_MANAGER_VERSION,

            activity_events: &self.inner().activity_events as *const _ as *mut _,
            activity_version: sys::DISCORD_ACTIVITY_MANAGER_VERSION,

            application_events: std::ptr::null_mut(),
            application_version: sys::DISCORD_APPLICATION_MANAGER_VERSION,

            image_events: std::ptr::null_mut(),
            image_version: sys::DISCORD_IMAGE_MANAGER_VERSION,

            lobby_events: &self.inner().lobby_events as *const _ as *mut _,
            lobby_version: sys::DISCORD_LOBBY_MANAGER_VERSION,

            network_events: &self.inner().network_events as *const _ as *mut _,
            network_version: sys::DISCORD_NETWORK_MANAGER_VERSION,

            overlay_events: &self.inner().overlay_events as *const _ as *mut _,
            overlay_version: sys::DISCORD_OVERLAY_MANAGER_VERSION,

            relationship_events: &self.inner().relationship_events as *const _ as *mut _,
            relationship_version: sys::DISCORD_RELATIONSHIP_MANAGER_VERSION,

            storage_events: std::ptr::null_mut(),
            storage_version: sys::DISCORD_STORAGE_MANAGER_VERSION,

            store_events: &self.inner().store_events as *const _ as *mut _,
            store_version: sys::DISCORD_STORE_MANAGER_VERSION,

            user_events: &self.inner().user_events as *const _ as *mut _,
            user_version: sys::DISCORD_USER_MANAGER_VERSION,

            voice_events: &self.inner().voice_events as *const _ as *mut _,
            voice_version: sys::DISCORD_VOICE_MANAGER_VERSION,
        }
    }

    fn set_log_hook(&self) {
        extern "C" fn log_hook(
            _: *mut std::ffi::c_void,
            level: sys::EDiscordLogLevel,
            message: *const u8,
        ) {
            utils::abort_on_panic(|| {
                let level = match level {
                    sys::DiscordLogLevel_Error => log::Level::Error,
                    sys::DiscordLogLevel_Warn => log::Level::Warn,
                    sys::DiscordLogLevel_Info => log::Level::Info,
                    sys::DiscordLogLevel_Debug => log::Level::Debug,
                    _ => log::Level::Trace,
                };

                log::log!(level, "SDK: {}", unsafe { utils::charptr_to_str(message) });
            })
        }

        unsafe {
            (*self.inner().core).set_log_hook.unwrap()(
                self.inner().core,
                sys::DiscordLogLevel_Debug,
                // SAFETY: this is never used
                std::ptr::null_mut(),
                Some(log_hook),
            );
        }
    }

    // To start producing events, the SDK must initialize the related manager
    // We initialize all managers that produce events to kickstart event passing
    fn kickstart_managers(&self) {
        unsafe {
            self.achievement_manager();
            self.activity_manager();
            self.lobby_manager();
            self.network_manager();
            self.overlay_manager();
            self.relationship_manager();
            self.store_manager();
            self.user_manager();
            self.voice_manager();
        }
    }

    /// Runs all pending SDK callbacks.
    ///
    /// This should be called often, like in the main loop if you're writing a game.
    ///
    /// Make sure to overwrite the [`EventHandler`](trait.EventHandler.html)
    /// (with [`event_handler_mut`](#method.event_handler_mut))
    /// before calling this method.
    ///
    /// ## Errors
    ///
    /// If the Discord client was closed, [`Error::NotRunning`](enum.Error.html#variant.NotRunning) will be returned.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/discord#runcallbacks)
    // We require &mut self to prevent calling during callbacks
    pub fn run_callbacks(&mut self) -> Result<()> {
        unsafe { (*self.inner().core).run_callbacks.unwrap()(self.inner().core).to_result() }
    }

    pub(crate) unsafe fn achievement_manager(&self) -> *mut sys::IDiscordAchievementManager {
        (*self.inner().core).get_achievement_manager.unwrap()(self.inner().core)
    }

    pub(crate) unsafe fn activity_manager(&self) -> *mut sys::IDiscordActivityManager {
        (*self.inner().core).get_activity_manager.unwrap()(self.inner().core)
    }

    pub(crate) unsafe fn application_manager(&self) -> *mut sys::IDiscordApplicationManager {
        (*self.inner().core).get_application_manager.unwrap()(self.inner().core)
    }

    pub(crate) unsafe fn image_manager(&self) -> *mut sys::IDiscordImageManager {
        (*self.inner().core).get_image_manager.unwrap()(self.inner().core)
    }

    pub(crate) unsafe fn lobby_manager(&self) -> *mut sys::IDiscordLobbyManager {
        (*self.inner().core).get_lobby_manager.unwrap()(self.inner().core)
    }

    pub(crate) unsafe fn network_manager(&self) -> *mut sys::IDiscordNetworkManager {
        (*self.inner().core).get_network_manager.unwrap()(self.inner().core)
    }

    pub(crate) unsafe fn overlay_manager(&self) -> *mut sys::IDiscordOverlayManager {
        (*self.inner().core).get_overlay_manager.unwrap()(self.inner().core)
    }

    pub(crate) unsafe fn relationship_manager(&self) -> *mut sys::IDiscordRelationshipManager {
        (*self.inner().core).get_relationship_manager.unwrap()(self.inner().core)
    }

    pub(crate) unsafe fn storage_manager(&self) -> *mut sys::IDiscordStorageManager {
        (*self.inner().core).get_storage_manager.unwrap()(self.inner().core)
    }

    pub(crate) unsafe fn store_manager(&self) -> *mut sys::IDiscordStoreManager {
        (*self.inner().core).get_store_manager.unwrap()(self.inner().core)
    }

    pub(crate) unsafe fn user_manager(&self) -> *mut sys::IDiscordUserManager {
        (*self.inner().core).get_user_manager.unwrap()(self.inner().core)
    }

    pub(crate) unsafe fn voice_manager(&self) -> *mut sys::IDiscordVoiceManager {
        (*self.inner().core).get_voice_manager.unwrap()(self.inner().core)
    }
}
