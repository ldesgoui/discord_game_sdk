use discord_game_sdk_sys as sys;

//

pub struct GameSDK {
    core_ptr: *mut sys::IDiscordCore,
}

impl GameSDK {
    #[allow(clippy::cast_possible_wrap)]
    pub fn new(client_id: i64, flags: &CreateFlags) -> Result<Self> {
        let mut params = sys::DiscordCreateParams::default();
        let mut core_ptr = std::ptr::null_mut();

        params.client_id = client_id;
        params.flags = u64::from(flags.to_sys());
        params.application_version = sys::DISCORD_APPLICATION_MANAGER_VERSION as i32;
        params.user_version = sys::DISCORD_USER_MANAGER_VERSION as i32;
        params.image_version = sys::DISCORD_IMAGE_MANAGER_VERSION as i32;
        params.activity_version = sys::DISCORD_ACTIVITY_MANAGER_VERSION as i32;
        params.relationship_version = sys::DISCORD_RELATIONSHIP_MANAGER_VERSION as i32;
        params.lobby_version = sys::DISCORD_LOBBY_MANAGER_VERSION as i32;
        params.network_version = sys::DISCORD_NETWORK_MANAGER_VERSION as i32;
        params.overlay_version = sys::DISCORD_OVERLAY_MANAGER_VERSION as i32;
        params.storage_version = sys::DISCORD_STORAGE_MANAGER_VERSION as i32;
        params.store_version = sys::DISCORD_STORE_MANAGER_VERSION as i32;
        params.voice_version = sys::DISCORD_VOICE_MANAGER_VERSION as i32;
        params.achievement_version = sys::DISCORD_ACHIEVEMENT_MANAGER_VERSION as i32;

        let res = unsafe {
            sys::DiscordCreate(
                sys::DISCORD_VERSION as i32,
                &mut params as *mut _,
                &mut core_ptr,
            )
        };

        Error::guard(res).map(|_| {
            debug_assert!(!core_ptr.is_null());

            Self { core_ptr }
        })
    }

    fn core(&self) -> &sys::IDiscordCore {
        unsafe { &*self.core_ptr }
    }

    pub fn set_log_hook(&mut self) {
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

impl Drop for GameSDK {
    fn drop(&mut self) {
        debug_assert!(self.core().destroy.is_some());
        unsafe {
            self.core().destroy.unwrap()(self.core_ptr);
        }
    }
}

unsafe extern "C" fn log_hook(
    hook_data: *mut std::ffi::c_void,
    level: u32,
    message: *const std::os::raw::c_char,
) {
    debug_assert!(hook_data.is_null());
    debug_assert!(!message.is_null());

    let level = match level {
        1 => log::Level::Error,
        2 => log::Level::Warn,
        3 => log::Level::Info,
        4 => log::Level::Debug,
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

pub type Result<T> = std::result::Result<T, Error>;

#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug)]
pub enum Error {
    ServiceUnavailable,
    InvalidVersion,
    LockFailed,
    InternalError,
    InvalidPayload,
    InvalidCommand,
    InvalidPermissions,
    NotFetched,
    NotFound,
    Conflict,
    InvalidSecret,
    InvalidJoinSecret,
    NoEligibleActivity,
    InvalidInvite,
    NotAuthenticated,
    InvalidAccessToken,
    ApplicationMismatch,
    InvalidDataUrl,
    InvalidBase64,
    NotFiltered,
    LobbyFull,
    InvalidLobbySecret,
    InvalidFilename,
    InvalidFileSize,
    InvalidEntitlement,
    NotInstalled,
    NotRunning,
    InsufficientBuffer,
    PurchaseCanceled,
    InvalidGuild,
    InvalidEvent,
    InvalidChannel,
    InvalidOrigin,
    RateLimited,
    OAuth2Error,
    SelectChannelTimeout,
    GetGuildTimeout,
    SelectVoiceForceRequired,
    CaptureShortcutAlreadyListening,
    UnauthorizedForAchievement,
    InvalidGiftCode,
    PurchaseError,
    TransactionAborted,
    /// 44..u32::MAX
    Undefined,
}

impl Error {
    fn guard(source: sys::EDiscordResult) -> Result<()> {
        use Error::*;

        Err(match source {
            sys::DiscordResult_Ok => return Ok(()),
            sys::DiscordResult_ServiceUnavailable => ServiceUnavailable,
            sys::DiscordResult_InvalidVersion => InvalidVersion,
            sys::DiscordResult_LockFailed => LockFailed,
            sys::DiscordResult_InternalError => InternalError,
            sys::DiscordResult_InvalidPayload => InvalidPayload,
            sys::DiscordResult_InvalidCommand => InvalidCommand,
            sys::DiscordResult_InvalidPermissions => InvalidPermissions,
            sys::DiscordResult_NotFetched => NotFetched,
            sys::DiscordResult_NotFound => NotFound,
            sys::DiscordResult_Conflict => Conflict,
            sys::DiscordResult_InvalidSecret => InvalidSecret,
            sys::DiscordResult_InvalidJoinSecret => InvalidJoinSecret,
            sys::DiscordResult_NoEligibleActivity => NoEligibleActivity,
            sys::DiscordResult_InvalidInvite => InvalidInvite,
            sys::DiscordResult_NotAuthenticated => NotAuthenticated,
            sys::DiscordResult_InvalidAccessToken => InvalidAccessToken,
            sys::DiscordResult_ApplicationMismatch => ApplicationMismatch,
            sys::DiscordResult_InvalidDataUrl => InvalidDataUrl,
            sys::DiscordResult_InvalidBase64 => InvalidBase64,
            sys::DiscordResult_NotFiltered => NotFiltered,
            sys::DiscordResult_LobbyFull => LobbyFull,
            sys::DiscordResult_InvalidLobbySecret => InvalidLobbySecret,
            sys::DiscordResult_InvalidFilename => InvalidFilename,
            sys::DiscordResult_InvalidFileSize => InvalidFileSize,
            sys::DiscordResult_InvalidEntitlement => InvalidEntitlement,
            sys::DiscordResult_NotInstalled => NotInstalled,
            sys::DiscordResult_NotRunning => NotRunning,
            sys::DiscordResult_InsufficientBuffer => InsufficientBuffer,
            sys::DiscordResult_PurchaseCanceled => PurchaseCanceled,
            sys::DiscordResult_InvalidGuild => InvalidGuild,
            sys::DiscordResult_InvalidEvent => InvalidEvent,
            sys::DiscordResult_InvalidChannel => InvalidChannel,
            sys::DiscordResult_InvalidOrigin => InvalidOrigin,
            sys::DiscordResult_RateLimited => RateLimited,
            sys::DiscordResult_OAuth2Error => OAuth2Error,
            sys::DiscordResult_SelectChannelTimeout => SelectChannelTimeout,
            sys::DiscordResult_GetGuildTimeout => GetGuildTimeout,
            sys::DiscordResult_SelectVoiceForceRequired => SelectVoiceForceRequired,
            sys::DiscordResult_CaptureShortcutAlreadyListening => CaptureShortcutAlreadyListening,
            sys::DiscordResult_UnauthorizedForAchievement => UnauthorizedForAchievement,
            sys::DiscordResult_InvalidGiftCode => InvalidGiftCode,
            sys::DiscordResult_PurchaseError => PurchaseError,
            sys::DiscordResult_TransactionAborted => TransactionAborted,
            val => {
                log::warn!(
                    "EDiscordResult could not be matched with our definitions: {}",
                    val
                );
                Undefined
            }
        })
    }
}

//

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        use std::io::Write;
        env_logger::Builder::from_env("TEST_LOG")
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{:>20}:{:<3} {:>5}: {}",
                    record.file().unwrap(),
                    record.line().unwrap(),
                    buf.default_styled_level(record.level()),
                    record.args()
                )
            })
            .init();

        let mut gsdk = GameSDK::new(1, &Default::default()).unwrap();
        gsdk.set_log_hook();
    }

}
