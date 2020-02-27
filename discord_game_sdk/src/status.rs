use crate::sys;

/// User Status
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/relationships#data-models-status-enum)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Status {
    /// User does not want to be disturbed (red dot)
    DoNotDisturb,
    /// User is idle (yellow dot)
    Idle,
    /// User is offline (grey dot)
    Offline,
    /// User is online (green dot)
    Online,
    /// Safety net for missing definitions
    Undefined(sys::EDiscordStatus),
}

impl From<sys::EDiscordStatus> for Status {
    fn from(source: sys::EDiscordStatus) -> Self {
        match source {
            sys::DiscordStatus_DoNotDisturb => Self::DoNotDisturb,
            sys::DiscordStatus_Idle => Self::Idle,
            sys::DiscordStatus_Offline => Self::Offline,
            sys::DiscordStatus_Online => Self::Online,
            _ => Self::Undefined(source),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                Self::DoNotDisturb => "do not disturb",
                Self::Idle => "idle",
                Self::Offline => "offline",
                Self::Online => "online",
                Self::Undefined(n) => return write!(fmt, "undefined status ({})", n),
            }
        )
    }
}
