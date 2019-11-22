use crate::sys;

/// Activity Type
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#data-models-activitytype-enum>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActivityKind {
    Listening,
    Playing,
    Streaming,
    Watching,
    Undefined(sys::EDiscordActivityType),
}

#[doc(hidden)]
impl From<sys::EDiscordActivityType> for ActivityKind {
    fn from(source: sys::EDiscordActivityType) -> Self {
        match source {
            sys::DiscordActivityType_Listening => Self::Listening,
            sys::DiscordActivityType_Playing => Self::Playing,
            sys::DiscordActivityType_Streaming => Self::Streaming,
            sys::DiscordActivityType_Watching => Self::Watching,
            _ => Self::Undefined(source),
        }
    }
}

#[doc(hidden)]
impl Into<sys::EDiscordActivityType> for ActivityKind {
    fn into(self) -> sys::EDiscordActivityType {
        match self {
            Self::Listening => sys::DiscordActivityType_Listening,
            Self::Playing => sys::DiscordActivityType_Playing,
            Self::Streaming => sys::DiscordActivityType_Streaming,
            Self::Watching => sys::DiscordActivityType_Watching,
            Self::Undefined(n) => n,
        }
    }
}
