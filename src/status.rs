use crate::{panic_messages::INVALID_ENUM, sys};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    DoNotDisturb,
    Idle,
    Offline,
    Online,
}

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
