use discord_game_sdk_sys as sys;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
/// https://discordapp.com/developers/docs/game-sdk/discord#data-models-result-enum
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
    pub(crate) fn guard(source: sys::EDiscordResult) -> Result<()> {
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
