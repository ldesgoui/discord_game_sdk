use crate::{panic_messages::INVALID_ENUM, sys};

/// User Status
///
/// <https://discordapp.com/developers/docs/game-sdk/relationships#data-models-status-enum>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Status {
    DoNotDisturb,
    Idle,
    Offline,
    Online,
}

#[doc(hidden)]
impl From<sys::EDiscordStatus> for Status {
    fn from(source: sys::EDiscordStatus) -> Self {
        match source {
            sys::DiscordStatus_DoNotDisturb => Status::DoNotDisturb,
            sys::DiscordStatus_Idle => Status::Idle,
            sys::DiscordStatus_Offline => Status::Offline,
            sys::DiscordStatus_Online => Status::Online,
            _ => panic!(INVALID_ENUM),
        }
    }
}
