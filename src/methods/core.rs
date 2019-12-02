use crate::{
    across_ffi::*, channels, event, sys, to_result::ToResult, CreateFlags, Discord, Result,
};
use std::ffi::c_void;

/// # Core
///
/// <https://discordapp.com/developers/docs/game-sdk/discord>
impl<'a> Discord<'a> {
    /// Calls [`with_create_flags`](#method.with_create_flags)
    /// with [`CreateFlags::Default`](enum.CreateFlags.html#variant.Default).
    pub fn new(client_id: i64) -> Result<Self> {
        Self::with_create_flags(client_id, CreateFlags::Default)
    }

    /// Creates an instance of the main interface with the Discord Game SDK.
    /// It also forwards all log messages to [`log`](https://docs.rs/log)
    /// and kickstarts all managers that produce events when started.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/discord#create>  
    /// <https://discordapp.com/developers/docs/game-sdk/discord#setloghook>
    pub fn with_create_flags(client_id: i64, flags: CreateFlags) -> Result<Self> {
        let (senders, receivers) = channels::create_channels();
        let senders_ptr = Box::into_raw(Box::new(senders));
        let senders = unsafe { Box::from_raw(senders_ptr) };

        let mut params = create_params(client_id, flags, senders_ptr as *mut _);

        let mut core = std::ptr::null_mut();

        unsafe { sys::DiscordCreate(sys::DISCORD_VERSION, &mut params, &mut core) }.to_result()?;

        log::trace!("received pointer to {:p}", core);

        let instance = Self {
            core,
            client_id,
            senders,
            receivers,
            callbacks: Default::default(),
        };

        instance.set_log_hook();
        instance.kickstart_managers();

        Ok(instance)
    }

    fn set_log_hook(&self) {
        unsafe {
            ffi!(self.set_log_hook(
                sys::DiscordLogLevel_Debug,
                std::ptr::null_mut(),
                Some(callbacks::log),
            ))
        };
    }

    fn kickstart_managers(&self) {
        unsafe {
            // In this order to prioritize managers that instantly generate events
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
    /// Will return [`Error::NotRunning`](enum.Error.html#variant.NotRunning)
    /// if the Discord client was closed.
    ///
    /// ## Attention
    /// Event buffers will grow large if `run_callbacks` is not ran often, it is recommended to run
    /// this in the game loop.
    pub fn run_callbacks(&mut self) -> Result<()> {
        unsafe { ffi!(self.run_callbacks()) }.to_result()?;

        let callbacks = unsafe { &mut *self.callbacks.get() };

        // https://github.com/rust-lang/rust/issues/43244
        let mut i = 0;
        while i < callbacks.len() {
            if callbacks[i].is_ready() {
                let mut callback = callbacks.remove(i);
                callback.run(self);
            } else {
                i += 1;
            }
        }

        Ok(())
    }

    /// This will discard all events that have not been used.
    ///
    /// ## Attention
    /// Event buffers will grow large if not all used or emptied.
    ///
    /// As a rule of thumb, call `empty_event_buffers` before every `run_callbacks`.
    pub fn empty_event_buffers(&self) {
        self.receivers.empty_channels()
    }
}

impl<'a> Drop for Discord<'a> {
    fn drop(&mut self) {
        unsafe { ffi!(self.destroy()) }
    }
}

fn create_params(
    client_id: i64,
    flags: CreateFlags,
    event_data: *mut c_void,
) -> sys::DiscordCreateParams {
    let flags: sys::EDiscordCreateFlags = flags.into();

    sys::DiscordCreateParams {
        client_id,
        flags: flags as u64,

        events: std::ptr::null_mut(),
        event_data,

        application_events: ACHIEVEMENT as *const _ as *mut _,
        application_version: sys::DISCORD_APPLICATION_MANAGER_VERSION,

        user_events: USER as *const _ as *mut _,
        user_version: sys::DISCORD_USER_MANAGER_VERSION,

        image_events: std::ptr::null_mut(),
        image_version: sys::DISCORD_IMAGE_MANAGER_VERSION,

        activity_events: ACTIVITY as *const _ as *mut _,
        activity_version: sys::DISCORD_ACTIVITY_MANAGER_VERSION,

        relationship_events: RELATIONSHIP as *const _ as *mut _,
        relationship_version: sys::DISCORD_RELATIONSHIP_MANAGER_VERSION,

        lobby_events: LOBBY as *const _ as *mut _,
        lobby_version: sys::DISCORD_LOBBY_MANAGER_VERSION,

        network_events: NETWORK as *const _ as *mut _,
        network_version: sys::DISCORD_NETWORK_MANAGER_VERSION,

        overlay_events: OVERLAY as *const _ as *mut _,
        overlay_version: sys::DISCORD_OVERLAY_MANAGER_VERSION,

        storage_events: std::ptr::null_mut(),
        storage_version: sys::DISCORD_STORAGE_MANAGER_VERSION,

        store_events: STORE as *const _ as *mut _,
        store_version: sys::DISCORD_STORE_MANAGER_VERSION,

        voice_events: VOICE as *const _ as *mut _,
        voice_version: sys::DISCORD_VOICE_MANAGER_VERSION,

        achievement_events: std::ptr::null_mut(),
        achievement_version: sys::DISCORD_ACHIEVEMENT_MANAGER_VERSION,
    }
}

const ACHIEVEMENT: &sys::IDiscordAchievementEvents = &sys::IDiscordAchievementEvents {
    on_user_achievement_update: Some(
        event_handlers::ptr::<event::UserAchievementUpdate, sys::DiscordUserAchievement>,
    ),
};

const ACTIVITY: &sys::IDiscordActivityEvents = &sys::IDiscordActivityEvents {
    on_activity_join: Some(event_handlers::string::<event::ActivityJoin>),
    on_activity_spectate: Some(event_handlers::string::<event::ActivitySpectate>),
    on_activity_join_request: Some(event_handlers::ptr::<event::ActivityRequest, sys::DiscordUser>),
    on_activity_invite: Some(
        event_handlers::plain_ptr_ptr::<
            event::ActivityInvite,
            sys::EDiscordActivityActionType,
            sys::DiscordUser,
            sys::DiscordActivity,
        >,
    ),
};

const LOBBY: &sys::IDiscordLobbyEvents = &sys::IDiscordLobbyEvents {
    on_lobby_update: Some(event_handlers::plain::<event::LobbyUpdate, sys::DiscordLobbyId>),
    on_lobby_delete: Some(
        event_handlers::plain_plain::<event::LobbyDelete, sys::DiscordLobbyId, u32>,
    ),
    on_member_connect: Some(
        event_handlers::plain_plain::<
            event::LobbyMemberConnect,
            sys::DiscordLobbyId,
            sys::DiscordUserId,
        >,
    ),
    on_member_update: Some(
        event_handlers::plain_plain::<
            event::LobbyMemberUpdate,
            sys::DiscordLobbyId,
            sys::DiscordUserId,
        >,
    ),
    on_member_disconnect: Some(
        event_handlers::plain_plain::<
            event::LobbyMemberDisconnect,
            sys::DiscordLobbyId,
            sys::DiscordUserId,
        >,
    ),
    on_lobby_message: Some(
        event_handlers::plain_plain_buffer::<
            event::LobbyMessage,
            sys::DiscordLobbyId,
            sys::DiscordUserId,
        >,
    ),
    on_speaking: Some(
        event_handlers::plain_plain_plain::<
            event::LobbySpeaking,
            sys::DiscordLobbyId,
            sys::DiscordUserId,
            bool,
        >,
    ),
    on_network_message: Some(
        event_handlers::plain_plain_plain_buffer::<
            event::LobbyNetworkMessage,
            sys::DiscordLobbyId,
            sys::DiscordUserId,
            sys::DiscordNetworkChannelId,
        >,
    ),
};

const NETWORK: &sys::IDiscordNetworkEvents = &sys::IDiscordNetworkEvents {
    on_message: Some(
        event_handlers::plain_plain_buffer::<
            event::NetworkMessage,
            sys::DiscordNetworkPeerId,
            sys::DiscordNetworkChannelId,
        >,
    ),
    on_route_update: Some(event_handlers::string::<event::NetworkRouteUpdate>),
};

const OVERLAY: &sys::IDiscordOverlayEvents = &sys::IDiscordOverlayEvents {
    on_toggle: Some(event_handlers::plain::<event::OverlayToggle, bool>),
};

const RELATIONSHIP: &sys::IDiscordRelationshipEvents = &sys::IDiscordRelationshipEvents {
    on_refresh: Some(event_handlers::empty::<event::RelationshipsRefresh>),
    on_relationship_update: Some(
        event_handlers::ptr::<event::RelationshipUpdate, sys::DiscordRelationship>,
    ),
};

const STORE: &sys::IDiscordStoreEvents = &sys::IDiscordStoreEvents {
    on_entitlement_create: Some(
        event_handlers::ptr::<event::StoreEntitlementCreate, sys::DiscordEntitlement>,
    ),
    on_entitlement_delete: Some(
        event_handlers::ptr::<event::StoreEntitlementDelete, sys::DiscordEntitlement>,
    ),
};

const USER: &sys::IDiscordUserEvents = &sys::IDiscordUserEvents {
    on_current_user_update: Some(event_handlers::empty::<event::CurrentUserUpdate>),
};

const VOICE: &sys::IDiscordVoiceEvents = &sys::IDiscordVoiceEvents {
    on_settings_update: Some(event_handlers::empty::<event::VoiceSettingsUpdate>),
};
