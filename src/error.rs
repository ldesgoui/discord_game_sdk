use crate::sys;

/// Alias for a `Result` with the error type [`discord_game_sdk::Error`]
///
/// [`discord_game_sdk::Error`]: enum.Error.html
pub type Result<T> = std::result::Result<T, Error>;

/// Discord Error
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/discord#data-models-result-enum)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, err_derive::Error)]
pub enum Error {
    /// Discord isn't working
    #[error(display = "service unavailable")]
    ServiceUnavailable,

    /// The SDK version is outdated
    #[error(display = "invalid version")]
    InvalidVersion,

    /// An internal erorr on transactional operations
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

    /// Could not fetch
    #[error(display = "not fetched")]
    NotFetched,

    /// Not found
    #[error(display = "not found")]
    NotFound,

    /// User already has network connection open on that channel
    #[error(display = "conflict")]
    Conflict,

    /// Activity secrets must be unique and not match party id
    #[error(display = "invalid secret")]
    InvalidSecret,

    /// Join request for that user does not exist
    #[error(display = "invalid join secret")]
    InvalidJoinSecret,

    /// Invalid Application ID in Activity payload (none should be set)
    #[error(display = "no eligible activity")]
    NoEligibleActivity,

    /// Invalid invite
    #[error(display = "invalid invite")]
    InvalidInvite,

    /// Not authenticated
    #[error(display = "not authenticated")]
    NotAuthenticated,

    /// The user's bearer token is invalid
    #[error(display = "invalid access token")]
    InvalidAccessToken,

    /// Access token belongs to another application
    #[error(display = "application mismatch")]
    ApplicationMismatch,

    /// Internal error fetching image data
    #[error(display = "invalid data URL")]
    InvalidDataUrl,

    /// Invalid base64 data
    #[error(display = "invalid base-64")]
    InvalidBase64,

    /// Trying to access data before it was filtered
    #[error(display = "not filtered")]
    NotFiltered,

    /// Lobby full
    #[error(display = "lobby full")]
    LobbyFull,

    /// Invalid lobby secret
    #[error(display = "invalid lobby secret")]
    InvalidLobbySecret,

    /// Filename is too long
    #[error(display = "invalid filename")]
    InvalidFilename,

    /// File is too big
    #[error(display = "invalid file size")]
    InvalidFileSize,

    /// Invalid entitlement
    #[error(display = "invalid entitlement")]
    InvalidEntitlement,

    /// Discord is not installed
    #[error(display = "not installed")]
    NotInstalled,

    /// Discord is not running
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
