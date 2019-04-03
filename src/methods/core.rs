use crate::prelude::*;

/// # Core
impl<'a> Discord<'a> {
    pub fn new(client_id: i64) -> Result<Self> {
        Self::with_create_flags(client_id, CreateFlags::default())
    }

    // pub fn with_config(client_id: i64, config: &Config) -> Result<Self> {
    pub fn with_create_flags(client_id: i64, flags: CreateFlags) -> Result<Self> {
        let mut sdk = Self {
            core: unsafe { std::mem::uninitialized() },
            client_id,
            activity_channel: shrev::EventChannel::new(),
            lobby_channel: shrev::EventChannel::new(),
            network_channel: shrev::EventChannel::new(),
            overlay_channel: shrev::EventChannel::new(),
            relationship_channel: shrev::EventChannel::new(),
            store_channel: shrev::EventChannel::new(),
            user_channel: shrev::EventChannel::new(),
            voice_channel: shrev::EventChannel::new(),
        };

        let mut params = create_params(client_id, flags, &mut sdk as *mut _);

        let mut core_ptr = std::ptr::null_mut();

        unsafe {
            sys::DiscordCreate(sys::DISCORD_VERSION, &mut params, &mut core_ptr).to_result()?
        };

        sdk.core = unsafe { core_ptr.as_mut() }.unwrap();
        sdk.set_log_hook();

        Ok(sdk)
    }

    fn set_log_hook(&mut self) {
        unsafe {
            ffi!(self.set_log_hook(
                sys::DiscordLogLevel_Debug,
                std::ptr::null_mut(),
                Some(across_ffi::callbacks::log),
            ))
        };
    }

    pub fn run_callbacks(&mut self) -> Result<()> {
        unsafe { ffi!(self.run_callbacks()) }.to_result()
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
    ptr: *mut Discord,
) -> sys::DiscordCreateParams {
    sys::DiscordCreateParams {
        client_id,
        flags: u64::from(flags.to_sys()),

        events: std::ptr::null_mut(),
        event_data: ptr as *mut c_void,

        application_version: sys::DISCORD_APPLICATION_MANAGER_VERSION,
        application_events: std::ptr::null_mut(),

        user_events: &mut USER,
        user_version: sys::DISCORD_USER_MANAGER_VERSION,

        image_events: std::ptr::null_mut(),
        image_version: sys::DISCORD_IMAGE_MANAGER_VERSION,

        activity_events: &mut ACTIVITY,
        activity_version: sys::DISCORD_ACTIVITY_MANAGER_VERSION,

        relationship_events: &mut RELATIONSHIP,
        relationship_version: sys::DISCORD_RELATIONSHIP_MANAGER_VERSION,

        lobby_events: &mut LOBBY,
        lobby_version: sys::DISCORD_LOBBY_MANAGER_VERSION,

        network_events: &mut NETWORK,
        network_version: sys::DISCORD_NETWORK_MANAGER_VERSION,

        overlay_events: &mut OVERLAY,
        overlay_version: sys::DISCORD_OVERLAY_MANAGER_VERSION,

        storage_events: std::ptr::null_mut(),
        storage_version: sys::DISCORD_STORAGE_MANAGER_VERSION,

        store_events: &mut STORE,
        store_version: sys::DISCORD_STORE_MANAGER_VERSION,

        voice_events: &mut VOICE,
        voice_version: sys::DISCORD_VOICE_MANAGER_VERSION,

        achievement_events: &mut sys::IDiscordAchievementEvents::default(),
        achievement_version: sys::DISCORD_ACHIEVEMENT_MANAGER_VERSION,
    }
}

const ACTIVITY: sys::IDiscordActivityEvents = sys::IDiscordActivityEvents {
    on_activity_join: Some(across_ffi::activity::on_activity_join),
    on_activity_spectate: Some(across_ffi::activity::on_activity_spectate),
    on_activity_join_request: Some(across_ffi::activity::on_activity_join_request),
    on_activity_invite: Some(across_ffi::activity::on_activity_invite),
};

const LOBBY: sys::IDiscordLobbyEvents = sys::IDiscordLobbyEvents {
    on_lobby_update: Some(across_ffi::lobby::on_lobby_update),
    on_lobby_delete: Some(across_ffi::lobby::on_lobby_delete),
    on_member_connect: Some(across_ffi::lobby::on_member_connect),
    on_member_update: Some(across_ffi::lobby::on_member_update),
    on_member_disconnect: Some(across_ffi::lobby::on_member_disconnect),
    on_lobby_message: Some(across_ffi::lobby::on_lobby_message),
    on_speaking: Some(across_ffi::lobby::on_speaking),
    on_network_message: Some(across_ffi::lobby::on_network_message),
};

const NETWORK: sys::IDiscordNetworkEvents = sys::IDiscordNetworkEvents {
    on_message: Some(across_ffi::network::on_message),
    on_route_update: Some(across_ffi::network::on_route_update),
};

const OVERLAY: sys::IDiscordOverlayEvents = sys::IDiscordOverlayEvents {
    on_toggle: Some(across_ffi::overlay::on_toggle),
};

const RELATIONSHIP: sys::IDiscordRelationshipEvents = sys::IDiscordRelationshipEvents {
    on_refresh: Some(across_ffi::relationship::on_refresh),
    on_relationship_update: Some(across_ffi::relationship::on_relationship_update),
};

const STORE: sys::IDiscordStoreEvents = sys::IDiscordStoreEvents {
    on_entitlement_create: Some(across_ffi::store::on_entitlement_create),
    on_entitlement_delete: Some(across_ffi::store::on_entitlement_delete),
};

const USER: sys::IDiscordUserEvents = sys::IDiscordUserEvents {
    on_current_user_update: Some(across_ffi::user::on_current_user_update),
};

const VOICE: sys::IDiscordVoiceEvents = sys::IDiscordVoiceEvents {
    on_settings_update: Some(across_ffi::voice::on_settings_update),
};
