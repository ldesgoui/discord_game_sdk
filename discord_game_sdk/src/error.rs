use crate::sys;
use std::fmt;

/// Alias for a `Result` with the error type [`discord_game_sdk::Error`]
///
/// [`discord_game_sdk::Error`]: enum.Error.html
pub type Result<T> = std::result::Result<T, Error>;

/// Discord Error
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/discord#data-models-result-enum)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// Discord isn't working
    ServiceUnavailable,

    /// The SDK version is outdated
    InvalidVersion,

    /// An internal erorr on transactional operations
    LockFailed,

    /// Internal error
    Internal,

    /// Invalid payload
    InvalidPayload,

    /// Invalid command
    InvalidCommand,

    /// Invalid permissions
    InvalidPermissions,

    /// Could not fetch
    NotFetched,

    /// Not found
    NotFound,

    /// User already has network connection open on that channel
    Conflict,

    /// Activity secrets must be unique and not match party id
    InvalidSecret,

    /// Join request for that user does not exist
    InvalidJoinSecret,

    /// Invalid Application ID in Activity payload (none should be set)
    NoEligibleActivity,

    /// Invalid invite
    InvalidInvite,

    /// Not authenticated
    NotAuthenticated,

    /// The user's bearer token is invalid
    InvalidAccessToken,

    /// Access token belongs to another application
    ApplicationMismatch,

    /// Internal error fetching image data
    InvalidDataUrl,

    /// Invalid base64 data
    InvalidBase64,

    /// Trying to access data before it was filtered
    NotFiltered,

    /// Lobby full
    LobbyFull,

    /// Invalid lobby secret
    InvalidLobbySecret,

    /// Filename is too long
    InvalidFilename,

    /// File is too big
    InvalidFileSize,

    /// Invalid entitlement
    InvalidEntitlement,

    /// Discord is not installed
    NotInstalled,

    /// Discord is not running
    NotRunning,

    /// Insufficient buffer
    InsufficientBuffer,

    /// Purchase canceled
    PurchaseCanceled,

    /// Invalid guild
    InvalidGuild,

    /// Invalid event
    InvalidEvent,

    /// Invalid channel
    InvalidChannel,

    /// Invalid origin
    InvalidOrigin,

    /// Rate limited
    RateLimited,

    /// `OAuth2` error
    OAuth2,

    /// Select channel timeout
    SelectChannelTimeout,

    /// Get guild timeout
    GetGuildTimeout,

    /// Select voice force required
    SelectVoiceForceRequired,

    /// Capture shortcut already listening
    CaptureShortcutAlreadyListening,

    /// Unauthorized for achievement
    UnauthorizedForAchievement,

    /// Invalid gift code
    InvalidGiftCode,

    /// Purchase error
    Purchase,

    /// Transaction aborted
    TransactionAborted,

    /// Safety net for missing definitions
    Undefined(sys::EDiscordResult),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        let message = match self {
            ServiceUnavailable => "service unavailable",
            InvalidVersion => "invalid version",
            LockFailed => "lock failed",
            Internal => "internal error",
            InvalidPayload => "invalid payload",
            InvalidCommand => "invalid command",
            InvalidPermissions => "invalid permissions",
            NotFetched => "not fetched",
            NotFound => "not found",
            Conflict => "conflict",
            InvalidSecret => "invalid secret",
            InvalidJoinSecret => "invalid join secret",
            NoEligibleActivity => "no eligible activity",
            InvalidInvite => "invalid invite",
            NotAuthenticated => "not authenticated",
            InvalidAccessToken => "invalid access token",
            ApplicationMismatch => "application mismatch",
            InvalidDataUrl => "invalid data URL",
            InvalidBase64 => "invalid base-64",
            NotFiltered => "not filtered",
            LobbyFull => "lobby full",
            InvalidLobbySecret => "invalid lobby secret",
            InvalidFilename => "invalid filename",
            InvalidFileSize => "invalid file size",
            InvalidEntitlement => "invalid entitlement",
            NotInstalled => "not installed",
            NotRunning => "not running",
            InsufficientBuffer => "insufficient buffer",
            PurchaseCanceled => "purchase canceled",
            InvalidGuild => "invalid guild",
            InvalidEvent => "invalid event",
            InvalidChannel => "invalid channel",
            InvalidOrigin => "invalid origin",
            RateLimited => "rate limited",
            OAuth2 => "OAuth 2.0 error",
            SelectChannelTimeout => "select channel timeout",
            GetGuildTimeout => "get guild timeout",
            SelectVoiceForceRequired => "select voice force required",
            CaptureShortcutAlreadyListening => "capture shortcut already listening",
            UnauthorizedForAchievement => "unauthorized for achievement",
            InvalidGiftCode => "invalid gift code",
            Purchase => "purchase error",
            TransactionAborted => "transaction aborted",
            Undefined(n) => return write!(f, "undefined error {}", n),
        };

        write!(f, "{}", message)
    }
}

impl std::error::Error for Error {}
