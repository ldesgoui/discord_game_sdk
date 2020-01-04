use crate::sys;

/// Alias for a `Result` with the error type [`discord_game_sdk::Error`]
///
/// [`discord_game_sdk::Error`](struct.Error.html)
pub type Result<T> = std::result::Result<T, Error>;

/// Discord Error
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/discord#data-models-result-enum)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, err_derive::Error)]
pub enum Error {
    /// Service unavailable
    #[error(display = "service unavailable")]
    ServiceUnavailable,

    /// Invalid version
    #[error(display = "invalid version")]
    InvalidVersion,

    /// Lock failed
    #[error(display = "lock failed")]
    LockFailed,

    /// Internal error
    #[error(display = "internal error")]
    InternalError,

    /// Invalid payload
    #[error(display = "invalid payload")]
    InvalidPayload,

    /// Invalid command
    #[error(display = "invalid command")]
    InvalidCommand,

    /// Invalid permissions
    #[error(display = "invalid permissions")]
    InvalidPermissions,

    /// Not fetched
    #[error(display = "not fetched")]
    NotFetched,

    /// Not found
    #[error(display = "not found")]
    NotFound,

    /// Conflict
    #[error(display = "conflict")]
    Conflict,

    /// Invalid secret
    #[error(display = "invalid secret")]
    InvalidSecret,

    /// Invalid join secret
    #[error(display = "invalid join secret")]
    InvalidJoinSecret,

    /// No eligible activity
    #[error(display = "no eligible activity")]
    NoEligibleActivity,

    /// Invalid invite
    #[error(display = "invalid invite")]
    InvalidInvite,

    /// Not authenticated
    #[error(display = "not authenticated")]
    NotAuthenticated,

    /// Invalid access token
    #[error(display = "invalid access token")]
    InvalidAccessToken,

    /// Application mismatch
    #[error(display = "application mismatch")]
    ApplicationMismatch,

    /// Invalid data URL
    #[error(display = "invalid data URL")]
    InvalidDataUrl,

    /// Invalid base-64
    #[error(display = "invalid base-64")]
    InvalidBase64,

    /// Not filtered
    #[error(display = "not filtered")]
    NotFiltered,

    /// Lobby full
    #[error(display = "lobby full")]
    LobbyFull,

    /// Invalid lobby secret
    #[error(display = "invalid lobby secret")]
    InvalidLobbySecret,

    /// Invalid filename
    #[error(display = "invalid filename")]
    InvalidFilename,

    /// Invalid file size
    #[error(display = "invalid file size")]
    InvalidFileSize,

    /// Invalid entitlement
    #[error(display = "invalid entitlement")]
    InvalidEntitlement,

    /// Not installed
    #[error(display = "not installed")]
    NotInstalled,

    /// Not running
    #[error(display = "not running")]
    NotRunning,

    /// Insufficient buffer
    #[error(display = "insufficient buffer")]
    InsufficientBuffer,

    /// Purchase canceled
    #[error(display = "purchase canceled")]
    PurchaseCanceled,

    /// Invalid guild
    #[error(display = "invalid guild")]
    InvalidGuild,

    /// Invalid event
    #[error(display = "invalid event")]
    InvalidEvent,

    /// Invalid channel
    #[error(display = "invalid channel")]
    InvalidChannel,

    /// Invalid origin
    #[error(display = "invalid origin")]
    InvalidOrigin,

    /// Rate limited
    #[error(display = "rate limited")]
    RateLimited,

    /// `OAuth2` error
    #[error(display = "OAuth 2.0 error")]
    OAuth2Error,

    /// Select channel timeout
    #[error(display = "select channel timeout")]
    SelectChannelTimeout,

    /// Get guild timeout
    #[error(display = "get guild timeout")]
    GetGuildTimeout,

    /// Select voice force required
    #[error(display = "select voice force required")]
    SelectVoiceForceRequired,

    /// Capture shortcut already listening
    #[error(display = "capture shortcut already listening")]
    CaptureShortcutAlreadyListening,

    /// Unauthorized for achievement
    #[error(display = "unauthorized for achievement")]
    UnauthorizedForAchievement,

    /// Invalid gift code
    #[error(display = "invalid gift code")]
    InvalidGiftCode,

    /// Purchase Error
    #[error(display = "purchase error")]
    PurchaseError,

    /// Transaction aborted
    #[error(display = "transaction aborted")]
    TransactionAborted,

    /// Safety net for missing definitions
    #[error(display = "undefined error {}", _0)]
    Undefined(sys::EDiscordResult),
}
