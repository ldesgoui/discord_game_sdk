use crate::{
    discord::{Discord, DiscordInner},
    events, sys,
    to_result::ToResult,
    utils, ClientID, CreateFlags, EventHandler, Result,
};
use std::{cell::UnsafeCell, convert::TryFrom};

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
impl<E: EventHandler> Discord<E> {
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

            achievement_events: events::achievement::<E>(),
            activity_events: events::activity::<E>(),
            lobby_events: events::lobby::<E>(),
            network_events: events::network::<E>(),
            overlay_events: events::overlay::<E>(),
            relationship_events: events::relationship::<E>(),
            store_events: events::store::<E>(),
            user_events: events::user::<E>(),
            voice_events: events::voice::<E>(),
        }));

        let mut params = instance.create_params(flags.into());

        unsafe {
            sys::DiscordCreate(sys::DISCORD_VERSION, &mut params, &mut instance.0.core)
                .to_result()?;
        }

        log::trace!("received pointer to {:p}", instance.0.core);

        instance.set_log_hook();
        instance.kickstart_managers();

        Ok(instance)
    }

    fn create_params(&self, flags: sys::EDiscordCreateFlags) -> sys::DiscordCreateParams {
        // SAFETY: As described above, we're passing the Box as raw pointer
        // It'll be used to duplicate the `Box` but won't mutate or drop it
        let event_data = &*self.0 as *const DiscordInner<E> as *mut std::ffi::c_void;

        sys::DiscordCreateParams {
            client_id: self.client_id(),
            flags: u64::try_from(flags).unwrap(),

            events: std::ptr::null_mut(),
            event_data,

            // SAFETY: pointers are safe
            // they last until `DiscordInner` is dropped,
            // and the SDK won't dereference them after that
            achievement_events: &self.0.achievement_events as *const _ as *mut _,
            achievement_version: sys::DISCORD_ACHIEVEMENT_MANAGER_VERSION,

            activity_events: &self.0.activity_events as *const _ as *mut _,
            activity_version: sys::DISCORD_ACTIVITY_MANAGER_VERSION,

            application_events: std::ptr::null_mut(),
            application_version: sys::DISCORD_APPLICATION_MANAGER_VERSION,

            image_events: std::ptr::null_mut(),
            image_version: sys::DISCORD_IMAGE_MANAGER_VERSION,

            lobby_events: &self.0.lobby_events as *const _ as *mut _,
            lobby_version: sys::DISCORD_LOBBY_MANAGER_VERSION,

            network_events: &self.0.network_events as *const _ as *mut _,
            network_version: sys::DISCORD_NETWORK_MANAGER_VERSION,

            overlay_events: &self.0.overlay_events as *const _ as *mut _,
            overlay_version: sys::DISCORD_OVERLAY_MANAGER_VERSION,

            relationship_events: &self.0.relationship_events as *const _ as *mut _,
            relationship_version: sys::DISCORD_RELATIONSHIP_MANAGER_VERSION,

            storage_events: std::ptr::null_mut(),
            storage_version: sys::DISCORD_STORAGE_MANAGER_VERSION,

            store_events: &self.0.store_events as *const _ as *mut _,
            store_version: sys::DISCORD_STORE_MANAGER_VERSION,

            user_events: &self.0.user_events as *const _ as *mut _,
            user_version: sys::DISCORD_USER_MANAGER_VERSION,

            voice_events: &self.0.voice_events as *const _ as *mut _,
            voice_version: sys::DISCORD_VOICE_MANAGER_VERSION,
        }
    }

    fn set_log_hook(&self) {
        extern "C" fn log_hook(
            _: *mut std::ffi::c_void,
            level: sys::EDiscordLogLevel,
            message: *const u8,
        ) {
            prevent_unwind!();

            let level = match level {
                sys::DiscordLogLevel_Error => log::Level::Error,
                sys::DiscordLogLevel_Warn => log::Level::Warn,
                sys::DiscordLogLevel_Info => log::Level::Info,
                sys::DiscordLogLevel_Debug => log::Level::Debug,
                _ => log::Level::Trace,
            };

            log::log!(level, "SDK: {}", unsafe { utils::charptr_to_str(message) });
        }

        self.with_core(|core| unsafe {
            core.set_log_hook.unwrap()(
                core,
                sys::DiscordLogLevel_Debug,
                // SAFETY: this is never used
                std::ptr::null_mut(),
                Some(log_hook),
            )
        });
    }

    // To start producing events, the SDK must initialize the related manager
    // We initialize all managers that produce events to kickstart event passing
    fn kickstart_managers(&self) {
        self.with_achievement_manager(|_| {});
        self.with_activity_manager(|_| {});
        self.with_lobby_manager(|_| {});
        self.with_network_manager(|_| {});
        self.with_overlay_manager(|_| {});
        self.with_relationship_manager(|_| {});
        self.with_store_manager(|_| {});
        self.with_user_manager(|_| {});
        self.with_voice_manager(|_| {});
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
        self.with_core(|core| unsafe { core.run_callbacks.unwrap()(core) })
            .to_result()
    }

    /// The Client ID that was supplied during creation
    pub fn client_id(&self) -> ClientID {
        self.0.client_id
    }

    /// The [`EventHandler`](trait.EventHandler.html)
    pub fn event_handler_mut(&mut self) -> &mut Option<E> {
        self.0.event_handler_mut()
    }
}

#[rustfmt::skip]
impl<E> Discord<E> {
    pub(crate) fn with_core<T>(&self, callback: impl FnOnce(&mut sys::IDiscordCore) -> T) -> T {
        utils::with_tx(self.0.core, callback)
    }

    with_manager!(with_achievement_manager, get_achievement_manager, sys::IDiscordAchievementManager);
    with_manager!(with_activity_manager, get_activity_manager, sys::IDiscordActivityManager);
    with_manager!(with_application_manager, get_application_manager, sys::IDiscordApplicationManager);
    with_manager!(with_image_manager, get_image_manager, sys::IDiscordImageManager);
    with_manager!(with_lobby_manager, get_lobby_manager, sys::IDiscordLobbyManager);
    with_manager!(with_network_manager, get_network_manager, sys::IDiscordNetworkManager);
    with_manager!(with_overlay_manager, get_overlay_manager, sys::IDiscordOverlayManager);
    with_manager!(with_relationship_manager, get_relationship_manager, sys::IDiscordRelationshipManager);
    with_manager!(with_storage_manager, get_storage_manager, sys::IDiscordStorageManager);
    with_manager!(with_store_manager, get_store_manager, sys::IDiscordStoreManager);
    with_manager!(with_user_manager, get_user_manager, sys::IDiscordUserManager);
    with_manager!(with_voice_manager, get_voice_manager, sys::IDiscordVoiceManager);
}
