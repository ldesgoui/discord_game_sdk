//! Safe wrapper for the [Discord Game SDK](https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide).
//!
//! # Status
//!
//! This library is currently in very early stages, most of the API is missing.
//!
//! # "Legal" note
//!
//! This wrapper was informally allowed for publication and distribution by Discord Staff.
//! I cannot redistribute the SDK files until it is made open-source or is licensed for redistribution. You will have to follow some instructions when first setting up your project.
//! This also means that docs.rs will not be able to build the documentation.
//! Apologies for the inconvenience.
//!
//! If you're a part of Discord and wish to discuss this, please email `ldesgoui@gmail.com` or contact `twiikuu#0047`. I mean no harm.

use discord_game_sdk_sys as sys;

pub mod error;

pub use error::{Error, Result};
use std::os::raw::c_void;

//

pub struct GameSDK {
    core_ptr: *mut sys::IDiscordCore,
}

impl GameSDK {
    pub fn new(client_id: i64) -> Result<Self> {
        Self::with_create_flags(client_id, &Default::default())
    }

    pub fn with_create_flags(client_id: i64, flags: &CreateFlags) -> Result<Self> {
        let mut sdk = GameSDK {
            core_ptr: std::ptr::null_mut(),
        };
        let mut params = create_params(client_id, flags, &mut sdk as *mut _ as *mut c_void);

        let res = unsafe {
            sys::DiscordCreate(
                sys::DISCORD_VERSION,
                &mut params as *mut _,
                &mut sdk.core_ptr,
            )
        };

        Error::guard(res).map(|_| {
            debug_assert!(!sdk.core_ptr.is_null());

            sdk.set_log_hook();
            sdk
        })
    }

    fn set_log_hook(&self) {
        debug_assert!(self.core().set_log_hook.is_some());
        unsafe {
            self.core().set_log_hook.unwrap()(
                self.core_ptr,
                sys::DiscordLogLevel_Debug,
                std::ptr::null_mut(),
                Some(log_hook),
            );
        }
    }
}

fn create_params(
    client_id: i64,
    flags: &CreateFlags,
    event_data: *mut c_void,
) -> sys::DiscordCreateParams {
    sys::DiscordCreateParams {
        client_id: client_id,
        flags: flags.to_sys() as u64,
        events: std::ptr::null_mut(),
        event_data,
        application_version: sys::DISCORD_APPLICATION_MANAGER_VERSION,
        application_events: std::ptr::null_mut(),
        user_events: &mut USER_EVENTS as *mut _,
        user_version: sys::DISCORD_USER_MANAGER_VERSION,
        image_events: std::ptr::null_mut(),
        image_version: sys::DISCORD_IMAGE_MANAGER_VERSION,
        activity_events: &mut ACTIVITY_EVENTS as *mut _,
        activity_version: sys::DISCORD_ACTIVITY_MANAGER_VERSION,
        relationship_events: &mut RELATIONSHIP_EVENTS as *mut _,
        relationship_version: sys::DISCORD_RELATIONSHIP_MANAGER_VERSION,
        lobby_events: &mut LOBBY_EVENTS as *mut _,
        lobby_version: sys::DISCORD_LOBBY_MANAGER_VERSION,
        network_events: &mut NETWORK_EVENTS as *mut _,
        network_version: sys::DISCORD_NETWORK_MANAGER_VERSION,
        overlay_events: &mut OVERLAY_EVENTS as *mut _,
        overlay_version: sys::DISCORD_OVERLAY_MANAGER_VERSION,
        storage_events: std::ptr::null_mut(),
        storage_version: sys::DISCORD_STORAGE_MANAGER_VERSION,
        store_events: &mut STORE_EVENTS as *mut _,
        store_version: sys::DISCORD_STORE_MANAGER_VERSION,
        voice_events: &mut VOICE_EVENTS as *mut _,
        voice_version: sys::DISCORD_VOICE_MANAGER_VERSION,
        achievement_events: &mut ACHIEVEMENT_EVENTS as *mut _,
        achievement_version: sys::DISCORD_ACHIEVEMENT_MANAGER_VERSION,
    }
}

const USER_EVENTS: sys::IDiscordUserEvents = sys::IDiscordUserEvents {
    on_current_user_update: None,
};

const ACTIVITY_EVENTS: sys::IDiscordActivityEvents = sys::IDiscordActivityEvents {
    on_activity_join: None,
    on_activity_spectate: None,
    on_activity_join_request: None,
    on_activity_invite: None,
};

const RELATIONSHIP_EVENTS: sys::IDiscordRelationshipEvents = sys::IDiscordRelationshipEvents {
    on_refresh: None,
    on_relationship_update: None,
};

const LOBBY_EVENTS: sys::IDiscordLobbyEvents = sys::IDiscordLobbyEvents {
    on_lobby_update: None,
    on_lobby_delete: None,
    on_member_connect: None,
    on_member_update: None,
    on_member_disconnect: None,
    on_lobby_message: None,
    on_speaking: None,
    on_network_message: None,
};

const NETWORK_EVENTS: sys::IDiscordNetworkEvents = sys::IDiscordNetworkEvents {
    on_message: None,
    on_route_update: None,
};

const OVERLAY_EVENTS: sys::IDiscordOverlayEvents = sys::IDiscordOverlayEvents { on_toggle: None };

const STORE_EVENTS: sys::IDiscordStoreEvents = sys::IDiscordStoreEvents {
    on_entitlement_create: None,
    on_entitlement_delete: None,
};

const VOICE_EVENTS: sys::IDiscordVoiceEvents = sys::IDiscordVoiceEvents {
    on_settings_update: None,
};

const ACHIEVEMENT_EVENTS: sys::IDiscordAchievementEvents = sys::IDiscordAchievementEvents {
    on_user_achievement_update: None,
};

impl Drop for GameSDK {
    fn drop(&mut self) {
        debug_assert!(self.core().destroy.is_some());
        unsafe {
            self.core().destroy.unwrap()(self.core_ptr);
        }
    }
}

macro_rules! get_manager {
    ($name:ident, $typ:path, $func:ident) => {
        fn $name(&self) -> &$typ {
            debug_assert!(self.core().$func.is_some());
            unsafe {
                &*self.core().$func.unwrap()(self.core_ptr)
            }
        }
    }
}

#[rustfmt::skip]
#[allow(dead_code)]
impl GameSDK {
    fn core(&self) -> &sys::IDiscordCore {
        unsafe { &*self.core_ptr }
    }

    get_manager!(application, sys::IDiscordApplicationManager, get_application_manager);
    get_manager!(user, sys::IDiscordUserManager, get_user_manager);
    get_manager!(image, sys::IDiscordImageManager, get_image_manager);
    get_manager!(activity, sys::IDiscordActivityManager, get_activity_manager);
    get_manager!(relationship, sys::IDiscordRelationshipManager, get_relationship_manager);
    get_manager!(lobby, sys::IDiscordLobbyManager, get_lobby_manager);
    get_manager!(network, sys::IDiscordNetworkManager, get_network_manager);
    get_manager!(overlay, sys::IDiscordOverlayManager, get_overlay_manager);
    get_manager!(storage, sys::IDiscordStorageManager, get_storage_manager);
    get_manager!(store, sys::IDiscordStoreManager, get_store_manager);
    get_manager!(voice, sys::IDiscordVoiceManager, get_voice_manager);
    get_manager!(achievement, sys::IDiscordAchievementManager, get_achievement_manager);
}

unsafe extern "C" fn log_hook(
    hook_data: *mut std::ffi::c_void,
    level: sys::EDiscordLogLevel,
    message: *const std::os::raw::c_char,
) {
    debug_assert!(hook_data.is_null());
    debug_assert!(!message.is_null());

    let level = match level {
        sys::DiscordLogLevel_Error => log::Level::Error,
        sys::DiscordLogLevel_Warn => log::Level::Warn,
        sys::DiscordLogLevel_Info => log::Level::Info,
        sys::DiscordLogLevel_Debug => log::Level::Debug,
        _ => log::Level::Trace,
    };

    log::log!(
        level,
        "{}",
        std::ffi::CStr::from_ptr(message).to_str().unwrap()
    );
}

//

pub enum CreateFlags {
    Default,
    NoRequireDiscord,
}

impl Default for CreateFlags {
    fn default() -> Self {
        CreateFlags::Default
    }
}

impl CreateFlags {
    fn to_sys(&self) -> sys::EDiscordCreateFlags {
        match self {
            CreateFlags::Default => sys::DiscordCreateFlags_Default,
            CreateFlags::NoRequireDiscord => sys::DiscordCreateFlags_NoRequireDiscord,
        }
    }
}

//

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let _ = pretty_env_logger::try_init_custom_env("TEST_LOG");

        log::info!("running tests");
        let mut gsdk = GameSDK::new(1).unwrap();
        log::info!("bye");
    }

}
