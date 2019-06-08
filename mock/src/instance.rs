use crate::interfaces;
use crate::prelude::*;

#[repr(C, packed)]
pub struct Instance {
    pub interfaces: Interfaces,
    pub state: State,
}

impl Instance {
    pub fn new(version: sys::DiscordVersion, params: &sys::DiscordCreateParams) -> Self {
        Self {
            interfaces: interfaces::INTERFACES,
            state: State::new(version, params),
        }
    }

    pub fn log(&self, level: sys::EDiscordLogLevel, message: &str) {
        log::log!(
            match level {
                1 => log::Level::Error,
                2 => log::Level::Warn,
                3 => log::Level::Info,
                4 => log::Level::Debug,
                _ => log::Level::Trace,
            },
            "{}",
            message
        );

        if self.state.log_hook.is_none() || level > self.state.log_min_level {
            return;
        }

        let c_str = std::ffi::CString::new(message).unwrap();

        unsafe {
            self.state.log_hook.unwrap()(self.state.log_hook_data, level, c_str.as_ptr());
        }
    }
}

#[allow(dead_code)]
#[rustfmt::skip]
impl Instance {
    from_ptr!(Instance, from_core, sys::IDiscordCore, interfaces.core);
    from_ptr!(Instance, from_application, sys::IDiscordApplicationManager, interfaces.applications);
    from_ptr!(Instance, from_user, sys::IDiscordUserManager, interfaces.users);
    from_ptr!(Instance, from_image, sys::IDiscordImageManager, interfaces.images);
    from_ptr!(Instance, from_activity, sys::IDiscordActivityManager, interfaces.activities);
    from_ptr!(Instance, from_relationship, sys::IDiscordRelationshipManager, interfaces.relationships);
    from_ptr!(Instance, from_lobby, sys::IDiscordLobbyManager, interfaces.lobbies);
    from_ptr!(Instance, from_network, sys::IDiscordNetworkManager, interfaces.networking);
    from_ptr!(Instance, from_overlay, sys::IDiscordOverlayManager, interfaces.overlay);
    from_ptr!(Instance, from_storage, sys::IDiscordStorageManager, interfaces.storage);
    from_ptr!(Instance, from_store, sys::IDiscordStoreManager, interfaces.store);
    from_ptr!(Instance, from_voice, sys::IDiscordVoiceManager, interfaces.voice);
    from_ptr!(Instance, from_achievement, sys::IDiscordAchievementManager, interfaces.achievements);
}
