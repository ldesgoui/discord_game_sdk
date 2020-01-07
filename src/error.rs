use crate::sys;

/// Alias for a `Result` with the error type [`discord_game_sdk::Error`]
///
/// [`discord_game_sdk::Error`]: enum.Error.html
pub type Result<T> = std::result::Result<T, Error>;

/// Discord Error
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/discord#data-models-result-enum)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, thiserror::Error)]
pub enum Error {
    /// Discord isn't working
    #[error("service unavailable")]
    ServiceUnavailable,

    /// The SDK version is outdated
    #[error("invalid version")]
    InvalidVersion,

    /// An internal erorr on transactional operations
    #[error("lock failed")]
    LockFailed,

    /// Internal error
    #[error("internal error")]
    InternalError,

    /// Invalid payload
    #[error("invalid payload")]
    InvalidPayload,

    /// Invalid command
    #[error("invalid command")]
    InvalidCommand,

    /// Invalid permissions
    #[error("invalid permissions")]
    InvalidPermissions,

    /// Could not fetch
    #[error("not fetched")]
    NotFetched,

    /// Not found
    #[error("not found")]
    NotFound,

    /// User already has network connection open on that channel
    #[error("conflict")]
    Conflict,

    /// Activity secrets must be unique and not match party id
    #[error("invalid secret")]
    InvalidSecret,

    /// Join request for that user does not exist
    #[error("invalid join secret")]
    InvalidJoinSecret,

    /// Invalid Application ID in Activity payload (none should be set)
    #[error("no eligible activity")]
    NoEligibleActivity,

    /// Invalid invite
    #[error("invalid invite")]
    InvalidInvite,

    /// Not authenticated
    #[error("not authenticated")]
    NotAuthenticated,

    /// The user's bearer token is invalid
    #[error("invalid access token")]
    InvalidAccessToken,

    /// Access token belongs to another application
    #[error("application mismatch")]
    ApplicationMismatch,

    /// Internal error fetching image data
    #[error("invalid data URL")]
    InvalidDataUrl,

    /// Invalid base64 data
    #[error("invalid base-64")]
    InvalidBase64,

    /// Trying to access data before it was filtered
    #[error("not filtered")]
    NotFiltered,

    /// Lobby full
    #[error("lobby full")]
    LobbyFull,

    /// Invalid lobby secret
    #[error("invalid lobby secret")]
    InvalidLobbySecret,

    /// Filename is too long
    #[error("invalid filename")]
    InvalidFilename,

    /// File is too big
    #[error("invalid file size")]
    InvalidFileSize,

    /// Invalid entitlement
    #[error("invalid entitlement")]
    InvalidEntitlement,

    /// Discord is not installed
    #[error("not installed")]
    NotInstalled,

    /// Discord is not running
    #[error("not running")]
    NotRunning,

    /// Insufficient buffer
    #[error("insufficient buffer")]
    InsufficientBuffer,

    /// Purchase canceled
    #[error("purchase canceled")]
    PurchaseCanceled,

    /// Invalid guild
    #[error("invalid guild")]
    InvalidGuild,

    /// Invalid event
    #[error("invalid event")]
    InvalidEvent,

    /// Invalid channel
    #[error("invalid channel")]
    InvalidChannel,

    /// Invalid origin
    #[error("invalid origin")]
    InvalidOrigin,

    /// Rate limited
    #[error("rate limited")]
    RateLimited,

    /// `OAuth2` error
    #[error("OAuth 2.0 error")]
    OAuth2Error,

    /// Select channel timeout
    #[error("select channel timeout")]
    SelectChannelTimeout,

    /// Get guild timeout
    #[error("get guild timeout")]
    GetGuildTimeout,

    /// Select voice force required
    #[error("select voice force required")]
    SelectVoiceForceRequired,

    /// Capture shortcut already listening
    #[error("capture shortcut already listening")]
    CaptureShortcutAlreadyListening,

    /// Unauthorized for achievement
    #[error("unauthorized for achievement")]
    UnauthorizedForAchievement,

    /// Invalid gift code
    #[error("invalid gift code")]
    InvalidGiftCode,

    /// Purchase Error
    #[error("purchase error")]
    PurchaseError,

    /// Transaction aborted
    #[error("transaction aborted")]
    TransactionAborted,

    /// Safety net for missing definitions
    #[error("undefined error {0}")]
    Undefined(sys::EDiscordResult),
}
