use crate::{panic_messages::INVALID_ENUM, sys};

/// Activity Type
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#data-models-activitytype-enum>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActivityKind {
    Listening,
    Playing,
    Streaming,
    Watching,
    Custom,
}

#[doc(hidden)]
impl From<sys::EDiscordActivityType> for ActivityKind {
    fn from(source: sys::EDiscordActivityType) -> Self {
        match source {
            sys::DiscordActivityType_Listening => ActivityKind::Listening,
            sys::DiscordActivityType_Playing => ActivityKind::Playing,
            sys::DiscordActivityType_Streaming => ActivityKind::Streaming,
            sys::DiscordActivityType_Watching => ActivityKind::Watching,
            4 => ActivityKind::Custom,
            _ => panic!(INVALID_ENUM),
        }
    }
}

#[doc(hidden)]
impl Into<sys::EDiscordActivityType> for ActivityKind {
    fn into(self) -> sys::EDiscordActivityType {
        match self {
            ActivityKind::Listening => sys::DiscordActivityType_Listening,
            ActivityKind::Playing => sys::DiscordActivityType_Playing,
            ActivityKind::Streaming => sys::DiscordActivityType_Streaming,
            ActivityKind::Watching => sys::DiscordActivityType_Watching,
            ActivityKind::Custom => 4,
        }
    }
}
