use crate::interfaces;
use crate::prelude::*;
use std::mem::size_of;

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
    const OFFSET_CORE: usize = 0;
    const OFFSET_ACHIEVEMENT: usize = Self::OFFSET_CORE + size_of::<sys::IDiscordCore>();
    const OFFSET_ACTIVITY: usize = Self::OFFSET_ACHIEVEMENT + size_of::<sys::IDiscordAchievementManager>();
    const OFFSET_APPLICATION: usize = Self::OFFSET_ACTIVITY + size_of::<sys::IDiscordActivityManager>();
    const OFFSET_IMAGE: usize = Self::OFFSET_APPLICATION + size_of::<sys::IDiscordApplicationManager>();
    const OFFSET_LOBBY: usize = Self::OFFSET_IMAGE + size_of::<sys::IDiscordImageManager>();
    const OFFSET_NETWORK: usize = Self::OFFSET_LOBBY + size_of::<sys::IDiscordLobbyManager>();
    const OFFSET_OVERLAY: usize = Self::OFFSET_NETWORK + size_of::<sys::IDiscordNetworkManager>();
    const OFFSET_RELATIONSHIP: usize = Self::OFFSET_OVERLAY + size_of::<sys::IDiscordOverlayManager>();
    const OFFSET_STORAGE: usize = Self::OFFSET_RELATIONSHIP + size_of::<sys::IDiscordRelationshipManager>();
    const OFFSET_STORE: usize = Self::OFFSET_STORAGE + size_of::<sys::IDiscordStorageManager>();
    const OFFSET_USER: usize = Self::OFFSET_STORE + size_of::<sys::IDiscordStoreManager>();
    const OFFSET_VOICE: usize = Self::OFFSET_USER + size_of::<sys::IDiscordUserManager>();

    pub unsafe fn from_core<'a>(ptr: *mut sys::IDiscordCore) -> &'a mut Self {
        ((ptr as *const u8).sub(Self::OFFSET_CORE) as *mut Self).as_mut().unwrap()
    }

    pub unsafe fn from_achievement<'a>(ptr: *mut sys::IDiscordAchievementManager) -> &'a mut Self {
        ((ptr as *const u8).sub(Self::OFFSET_ACHIEVEMENT) as *mut Self).as_mut().unwrap()
    }

    pub unsafe fn from_activity<'a>(ptr: *mut sys::IDiscordActivityManager) -> &'a mut Self {
        ((ptr as *const u8).sub(Self::OFFSET_ACTIVITY) as *mut Self).as_mut().unwrap()
    }

    pub unsafe fn from_application<'a>(ptr: *mut sys::IDiscordApplicationManager) -> &'a mut Self {
        ((ptr as *const u8).sub(Self::OFFSET_APPLICATION) as *mut Self).as_mut().unwrap()
    }

    pub unsafe fn from_image<'a>(ptr: *mut sys::IDiscordImageManager) -> &'a mut Self {
        ((ptr as *const u8).sub(Self::OFFSET_IMAGE) as *mut Self).as_mut().unwrap()
    }

    pub unsafe fn from_lobby<'a>(ptr: *mut sys::IDiscordLobbyManager) -> &'a mut Self {
        ((ptr as *const u8).sub(Self::OFFSET_LOBBY) as *mut Self).as_mut().unwrap()
    }

    pub unsafe fn from_network<'a>(ptr: *mut sys::IDiscordNetworkManager) -> &'a mut Self {
        ((ptr as *const u8).sub(Self::OFFSET_NETWORK) as *mut Self).as_mut().unwrap()
    }

    pub unsafe fn from_overlay<'a>(ptr: *mut sys::IDiscordOverlayManager) -> &'a mut Self {
        ((ptr as *const u8).sub(Self::OFFSET_OVERLAY) as *mut Self).as_mut().unwrap()
    }

    pub unsafe fn from_relationship<'a>(ptr: *mut sys::IDiscordRelationshipManager) -> &'a mut Self {
        ((ptr as *const u8).sub(Self::OFFSET_RELATIONSHIP) as *mut Self).as_mut().unwrap()
    }

    pub unsafe fn from_storage<'a>(ptr: *mut sys::IDiscordStorageManager) -> &'a mut Self {
        ((ptr as *const u8).sub(Self::OFFSET_STORAGE) as *mut Self).as_mut().unwrap()
    }

    pub unsafe fn from_store<'a>(ptr: *mut sys::IDiscordStoreManager) -> &'a mut Self {
        ((ptr as *const u8).sub(Self::OFFSET_STORE) as *mut Self).as_mut().unwrap()
    }

    pub unsafe fn from_user<'a>(ptr: *mut sys::IDiscordUserManager) -> &'a mut Self {
        ((ptr as *const u8).sub(Self::OFFSET_USER) as *mut Self).as_mut().unwrap()
    }

    pub unsafe fn from_voice<'a>(ptr: *mut sys::IDiscordVoiceManager) -> &'a mut Self {
        ((ptr as *const u8).sub(Self::OFFSET_VOICE) as *mut Self).as_mut().unwrap()
    }
}
