use discord_game_sdk_sys as sys;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, err_derive::Error)]
pub enum Error {
    #[error(display = "Discord SDK returned error")]
    DiscordError(#[error(cause)] DiscordError),
    #[error(display = "result was null")]
    NullResult,
    #[error(display = "pointer to method was null")]
    MissingMethod,
    #[error(display = "utf8 conversion error")]
    Utf8(#[error(cause)] std::str::Utf8Error),
}

#[rustfmt::skip]
impl From<DiscordError> for Error { fn from(e: DiscordError) -> Self { Error::DiscordError(e) } }

#[rustfmt::skip]
impl From<std::str::Utf8Error> for Error { fn from(e: std::str::Utf8Error) -> Self { Error::Utf8(e) } }

#[derive(Debug, err_derive::Error)]
pub enum DiscordError {
    #[error(display = "service unavailable")]
    ServiceUnavailable,
    #[error(display = "invalid version")]
    InvalidVersion,
    #[error(display = "lock failed")]
    LockFailed,
    #[error(display = "internal error")]
    InternalError,
    #[error(display = "invalid payload")]
    InvalidPayload,
    #[error(display = "invalid command")]
    InvalidCommand,
    #[error(display = "invalid permissions")]
    InvalidPermissions,
    #[error(display = "not fetched")]
    NotFetched,
    #[error(display = "not found")]
    NotFound,
    #[error(display = "conflict")]
    Conflict,
    #[error(display = "invalid secret")]
    InvalidSecret,
    #[error(display = "invalid join secret")]
    InvalidJoinSecret,
    #[error(display = "no eligible activity")]
    NoEligibleActivity,
    #[error(display = "invalid invite")]
    InvalidInvite,
    #[error(display = "not authenticated")]
    NotAuthenticated,
    #[error(display = "invalid access token")]
    InvalidAccessToken,
    #[error(display = "application mismatch")]
    ApplicationMismatch,
    #[error(display = "invalid data URL")]
    InvalidDataUrl,
    #[error(display = "invalid base-64")]
    InvalidBase64,
    #[error(display = "not filtered")]
    NotFiltered,
    #[error(display = "lobby full")]
    LobbyFull,
    #[error(display = "invalid lobby secret")]
    InvalidLobbySecret,
    #[error(display = "invalid filename")]
    InvalidFilename,
    #[error(display = "invalid file size")]
    InvalidFileSize,
    #[error(display = "invalid entitlement")]
    InvalidEntitlement,
    #[error(display = "not installed")]
    NotInstalled,
    #[error(display = "not running")]
    NotRunning,
    #[error(display = "insufficient buffer")]
    InsufficientBuffer,
    #[error(display = "purchase canceled")]
    PurchaseCanceled,
    #[error(display = "invalid guild")]
    InvalidGuild,
    #[error(display = "invalid event")]
    InvalidEvent,
    #[error(display = "invalid channel")]
    InvalidChannel,
    #[error(display = "invalid origin")]
    InvalidOrigin,
    #[error(display = "rate limited")]
    RateLimited,
    #[error(display = "OAuth 2.0 error")]
    OAuth2Error,
    #[error(display = "select channel timeout")]
    SelectChannelTimeout,
    #[error(display = "get guild timeout")]
    GetGuildTimeout,
    #[error(display = "select voice force required")]
    SelectVoiceForceRequired,
    #[error(display = "capture shortcut already listening")]
    CaptureShortcutAlreadyListening,
    #[error(display = "unauthorized for achievement")]
    UnauthorizedForAchievement,
    #[error(display = "invalid gift code")]
    InvalidGiftCode,
    #[error(display = "purchase error")]
    PurchaseError,
    #[error(display = "transaction aborted")]
    TransactionAborted,
    #[error(display = "undefined error")]
    #[doc(hidden)]
    _Undefined,
}

pub(crate) fn discord_result(source: sys::EDiscordResult) -> std::result::Result<(), DiscordError> {
    use DiscordError::*;

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
            log::warn!("EDiscordResult returned something undefined: {}", val);
            _Undefined
        }
    })
}
