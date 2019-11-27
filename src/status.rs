use crate::sys;

/// User Status
///
/// <https://discordapp.com/developers/docs/game-sdk/relationships#data-models-status-enum>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Status {
    DoNotDisturb,
    Idle,
    Offline,
    Online,
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
