pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, err_derive::Error)]
pub enum Error {
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
}
