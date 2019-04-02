use crate::create_flags::CreateFlags;
use crate::prelude::*;

/// # Core
impl Discord {
    pub fn new(client_id: i64) -> Result<Self> {
        Self::with_create_flags(client_id, CreateFlags::default())
    }

    pub fn with_create_flags(client_id: i64, flags: CreateFlags) -> Result<Self> {
        let mut sdk = Self {
            core_ptr: std::ptr::null_mut(),
            client_id,
            activity_events: shrev::EventChannel::new(),
            user_events: shrev::EventChannel::new(),
        };

        let mut params = create_params(
            client_id,
            flags,
            // We're not passing a reference because we don't want the borrow
            // checker to know what's happening with that pointer
            &mut sdk as *mut _,
        );

        unsafe {
            sys::DiscordCreate(
                sys::DISCORD_VERSION,
                &mut params as *mut _,
                &mut sdk.core_ptr,
            )
            .to_result()?;
        }

        sdk.set_log_hook()?;

        Ok(sdk)
    }

    fn set_log_hook(&self) -> Result<()> {
        ffi!(self.set_log_hook(
            sys::DiscordLogLevel_Debug,
            std::ptr::null_mut(),
            Some(log_hook_callback),
        ))?;

        Ok(())
    }

    pub fn run_callbacks(&mut self) -> Result<()> {
        ffi!(self.run_callbacks())?;

        Ok(())
    }
}

impl Drop for Discord {
    fn drop(&mut self) {
        if let Err(err) = ffi!(self.destroy()) {
            log::error!("error while dropping: {}", err);
        }
    }
}

fn create_params(
    client_id: i64,
    flags: CreateFlags,
    ptr: *mut Discord,
) -> sys::DiscordCreateParams {
    use crate::event::*;

    sys::DiscordCreateParams {
        client_id,
        flags: u64::from(flags.to_sys()),
        //
        events: std::ptr::null_mut(),
        event_data: ptr as *mut c_void,
        //
        application_version: sys::DISCORD_APPLICATION_MANAGER_VERSION,
        application_events: std::ptr::null_mut(),
        //
        user_events: &mut USER as *mut _,
        user_version: sys::DISCORD_USER_MANAGER_VERSION,
        //
        image_events: std::ptr::null_mut(),
        image_version: sys::DISCORD_IMAGE_MANAGER_VERSION,
        //
        activity_events: &mut ACTIVITY as *mut _,
        activity_version: sys::DISCORD_ACTIVITY_MANAGER_VERSION,
        //
        relationship_events: &mut RELATIONSHIP as *mut _,
        relationship_version: sys::DISCORD_RELATIONSHIP_MANAGER_VERSION,
        //
        lobby_events: &mut LOBBY as *mut _,
        lobby_version: sys::DISCORD_LOBBY_MANAGER_VERSION,
        //
        network_events: &mut NETWORK as *mut _,
        network_version: sys::DISCORD_NETWORK_MANAGER_VERSION,
        //
        overlay_events: &mut OVERLAY as *mut _,
        overlay_version: sys::DISCORD_OVERLAY_MANAGER_VERSION,
        //
        storage_events: std::ptr::null_mut(),
        storage_version: sys::DISCORD_STORAGE_MANAGER_VERSION,
        //
        store_events: &mut STORE as *mut _,
        store_version: sys::DISCORD_STORE_MANAGER_VERSION,
        //
        voice_events: &mut VOICE as *mut _,
        voice_version: sys::DISCORD_VOICE_MANAGER_VERSION,
        //
        achievement_events: &mut ACHIEVEMENT as *mut _,
        achievement_version: sys::DISCORD_ACHIEVEMENT_MANAGER_VERSION,
    }
}

extern "C" fn log_hook_callback(
    _: *mut c_void,
    level: sys::EDiscordLogLevel,
    message: *const std::os::raw::c_char,
) {
    if message.is_null() {
        log::error!("log_hook was passed a null pointer");
        return;
    }

    let level = match level {
        sys::DiscordLogLevel_Error => log::Level::Error,
        sys::DiscordLogLevel_Warn => log::Level::Warn,
        sys::DiscordLogLevel_Info => log::Level::Info,
        sys::DiscordLogLevel_Debug => log::Level::Debug,
        _ => {
            log::warn!("log_hook was passed an undefined log level: {:?}", level);
            log::Level::Trace
        }
    };

    let c_str = unsafe { std::ffi::CStr::from_ptr(message) };

    match c_str.to_str() {
        Ok(message) => log::log!(level, "{}", message),
        Err(err) => log::error!("log_hook was passed an invalid UTF8 string: {}", err),
    }
}
