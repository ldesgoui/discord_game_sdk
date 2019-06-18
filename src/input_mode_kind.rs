use crate::{panic_messages::INVALID_ENUM, sys};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InputModeKind {
    PushToTalk,
    VoiceActivity,
}

#[doc(hidden)]
impl From<sys::EDiscordInputModeType> for InputModeKind {
    fn from(source: sys::EDiscordInputModeType) -> Self {
        match source {
            sys::DiscordInputModeType_PushToTalk => InputModeKind::PushToTalk,
            sys::DiscordInputModeType_VoiceActivity => InputModeKind::VoiceActivity,
            _ => panic!(INVALID_ENUM),
        }
    }
}
