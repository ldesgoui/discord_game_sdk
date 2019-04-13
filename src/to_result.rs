use crate::{sys, DiscordError, DiscordResult};

pub(crate) trait ToResult: Sized {
    fn to_result(self) -> DiscordResult<()>;
}

impl ToResult for sys::EDiscordResult {
    fn to_result(self) -> DiscordResult<()> {
        use DiscordError::*;

        Err(match self {
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
            _ => panic!("enum"),
        })
    }
}
