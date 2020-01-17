use crate::{
    discord::{Discord, DiscordInner},
    sys,
    to_result::ToResult,
    utils::{charptr_to_str, VoidEvents},
    Activity, ClientID, CreateFlags, Entitlement, EventHandler, Relationship, Result, User,
    UserAchievement,
};
use std::{convert::TryFrom, ops::Deref};

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
    /// > [`Create` in official docs](https://discordapp.com/developers/docs/game-sdk/discord#create)  
    /// > [`SetLogHook` in official docs](https://discordapp.com/developers/docs/game-sdk/discord#setloghook)
    #[allow(clippy::cognitive_complexity)]
    pub fn with_create_flags(client_id: ClientID, flags: CreateFlags) -> Result<Self> {
        let mut inner = Box::new(DiscordInner {
            core: std::ptr::null_mut(),
            client_id,
            event_handler: Box::new(VoidEvents),
        });

        // SAFETY: This is the pointer we use in event handler code
        let ptr = inner.deref() as *const DiscordInner as *mut std::ffi::c_void;

        let params = create_params(client_id, flags.into(), ptr);

        unsafe {
            sys::DiscordCreate(
                sys::DISCORD_VERSION,
                // XXX: *mut should be *const
                &params as *const _ as *mut _,
                // XXX: *mut *mut should be *mut *const
                &mut inner.core,
            )
        }
        .to_result()?;

        log::trace!("received pointer to {:p}", inner.core);

        let instance = Discord(inner);

        #[allow(unused_results)]
        unsafe {
            ffi!(instance.set_log_hook(
                sys::DiscordLogLevel_Debug,
                ptr,
                event_handler!(|level: sys::EDiscordLogLevel, message: *const u8| {
                    EventHandler::on_log_message(level.into(), charptr_to_str(message))
                })
            ));

            // Signal managers that we want events ASAP
            ffi!(instance.get_network_manager());
            ffi!(instance.get_overlay_manager());
            ffi!(instance.get_relationship_manager());
            ffi!(instance.get_user_manager());

            ffi!(instance.get_achievement_manager());
            ffi!(instance.get_activity_manager());
            ffi!(instance.get_lobby_manager());
            ffi!(instance.get_store_manager());
            ffi!(instance.get_voice_manager());
        }

        Ok(instance)
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
    pub fn run_callbacks(&mut self) -> Result<()> {
        unsafe { ffi!(self.run_callbacks()) }.to_result()
    }

    /// The Client ID that was supplied during creation
    pub fn client_id(&self) -> ClientID {
        self.0.client_id
    }

    /// Sets a new Event Handler, returning the previous one
    pub fn set_event_handler<'a>(
        &'a mut self,
        event_handler: Box<dyn EventHandler>,
    ) -> Box<dyn EventHandler> {
        std::mem::replace(&mut self.0.event_handler, event_handler)
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

const ACHIEVEMENT: &sys::IDiscordAchievementEvents = &sys::IDiscordAchievementEvents {
    on_user_achievement_update: event_handler!(
        |user_achievement: *mut sys::DiscordUserAchievement| {
            EventHandler::on_user_achievement_update(&*(user_achievement as *mut UserAchievement))
        }
    ),
};

const ACTIVITY: &sys::IDiscordActivityEvents = &sys::IDiscordActivityEvents {
    on_activity_join: event_handler!(|secret: *const u8| {
        EventHandler::on_activity_join(charptr_to_str(secret))
    }),

    on_activity_spectate: event_handler!(|secret: *const u8| {
        EventHandler::on_activity_spectate(charptr_to_str(secret))
    }),

    on_activity_join_request: event_handler!(|user: *mut sys::DiscordUser| {
        EventHandler::on_activity_join_request(&*(user as *mut User))
    }),

    on_activity_invite: event_handler!(
        |kind: sys::EDiscordActivityActionType,
         user: *mut sys::DiscordUser,
         activity: *mut sys::DiscordActivity| {
            EventHandler::on_activity_invite(
                kind.into(),
                &*(user as *mut User),
                &*(activity as *mut Activity),
            )
        }
    ),
};

const LOBBY: &sys::IDiscordLobbyEvents = &sys::IDiscordLobbyEvents {
    on_lobby_update: event_handler!(|lobby_id: sys::DiscordLobbyId| {
        EventHandler::on_lobby_update(lobby_id)
    }),

    on_lobby_delete: event_handler!(|lobby_id: sys::DiscordLobbyId, reason: u32| {
        EventHandler::on_lobby_delete(lobby_id, reason)
    }),

    on_member_connect: event_handler!(
        |lobby_id: sys::DiscordLobbyId, member_id: sys::DiscordUserId| {
            EventHandler::on_member_connect(lobby_id, member_id)
        }
    ),

    on_member_update: event_handler!(
        |lobby_id: sys::DiscordLobbyId, member_id: sys::DiscordUserId| {
            EventHandler::on_member_update(lobby_id, member_id)
        }
    ),

    on_member_disconnect: event_handler!(
        |lobby_id: sys::DiscordLobbyId, member_id: sys::DiscordUserId| {
            EventHandler::on_member_disconnect(lobby_id, member_id)
        }
    ),

    on_lobby_message: event_handler!(|lobby_id: sys::DiscordLobbyId,
                                      member_id: sys::DiscordUserId,
                                      data: *mut u8,
                                      data_len: u32| {
        EventHandler::on_lobby_message(
            lobby_id,
            member_id,
            std::slice::from_raw_parts(data, data_len as usize),
        )
    }),

    on_speaking: event_handler!(|lobby_id: sys::DiscordLobbyId,
                                 member_id: sys::DiscordUserId,
                                 speaking: bool| {
        EventHandler::on_speaking(lobby_id, member_id, speaking)
    }),

    on_network_message: event_handler!(|lobby_id: sys::DiscordLobbyId,
                                        member_id: sys::DiscordUserId,
                                        channel_id: sys::DiscordNetworkChannelId,
                                        data: *mut u8,
                                        data_len: u32| {
        EventHandler::on_lobby_network_message(
            lobby_id,
            member_id,
            channel_id,
            std::slice::from_raw_parts(data, data_len as usize),
        )
    }),
};

const NETWORK: &sys::IDiscordNetworkEvents = &sys::IDiscordNetworkEvents {
    on_message: event_handler!(|peer_id: sys::DiscordNetworkPeerId,
                                channel_id: sys::DiscordNetworkChannelId,
                                data: *mut u8,
                                data_len: u32| {
        EventHandler::on_network_message(
            peer_id,
            channel_id,
            std::slice::from_raw_parts(data, data_len as usize),
        )
    }),

    on_route_update: event_handler!(|route: *const u8| {
        EventHandler::on_network_route_update(charptr_to_str(route))
    }),
};

const OVERLAY: &sys::IDiscordOverlayEvents = &sys::IDiscordOverlayEvents {
    on_toggle: event_handler!(|locked: bool| EventHandler::on_overlay_toggle(!locked)),
};

const RELATIONSHIP: &sys::IDiscordRelationshipEvents = &sys::IDiscordRelationshipEvents {
    on_refresh: event_handler!(|| EventHandler::on_relationships_refresh()),

    on_relationship_update: event_handler!(|relationship: *mut sys::DiscordRelationship| {
        EventHandler::on_relationship_update(&*(relationship as *mut Relationship))
    }),
};

const STORE: &sys::IDiscordStoreEvents = &sys::IDiscordStoreEvents {
    on_entitlement_create: event_handler!(|entitlement: *mut sys::DiscordEntitlement| {
        EventHandler::on_entitlement_create(&*(entitlement as *mut Entitlement))
    }),

    on_entitlement_delete: event_handler!(|entitlement: *mut sys::DiscordEntitlement| {
        EventHandler::on_entitlement_delete(&*(entitlement as *mut Entitlement))
    }),
};

const USER: &sys::IDiscordUserEvents = &sys::IDiscordUserEvents {
    on_current_user_update: event_handler!(|| EventHandler::on_current_user_update()),
};

const VOICE: &sys::IDiscordVoiceEvents = &sys::IDiscordVoiceEvents {
    on_settings_update: event_handler!(|| EventHandler::on_voice_settings_update()),
};
