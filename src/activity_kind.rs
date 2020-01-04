use crate::sys;

/// Activity Type
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/activities#data-models-activitytype-enum)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActivityKind {
    /// Listening to music (only Spotify as of Jan 2020)
    Listening,
    /// Playing a game
    Playing,
    /// Live streaming (only Twitch as of Jan 2020)
    Streaming,
    /// Watching a live stream
    Watching,
    /// Safety net for missing definitions
    Undefined(sys::EDiscordActivityType),
}

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
