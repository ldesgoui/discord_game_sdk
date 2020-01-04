use crate::sys;

/// Input Mode Type
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/discord-voice#data-models-inputmodetype-enum)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InputModeKind {
    /// Voice is transmitted when a key is pushed
    PushToTalk,
    /// Voice is transmitted when detected by Discord
    VoiceActivity,
    /// Safety net for missing definitions
    Undefined(sys::EDiscordInputModeType),
}

impl From<sys::EDiscordInputModeType> for InputModeKind {
    fn from(source: sys::EDiscordInputModeType) -> Self {
        match source {
            sys::DiscordInputModeType_PushToTalk => Self::PushToTalk,
            sys::DiscordInputModeType_VoiceActivity => Self::VoiceActivity,
            _ => Self::Undefined(source),
        }
    }
}

impl Into<sys::EDiscordInputModeType> for InputModeKind {
    fn into(self) -> sys::EDiscordInputModeType {
        match self {
            Self::PushToTalk => sys::DiscordInputModeType_PushToTalk,
            Self::VoiceActivity => sys::DiscordInputModeType_VoiceActivity,
            Self::Undefined(n) => n,
        }
    }
}
