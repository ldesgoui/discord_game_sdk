use crate::{panic_messages::INVALID_ENUM, sys};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActivityKind {
    Listening,
    Playing,
    Streaming,
    Watching,
}

impl From<sys::EDiscordActivityType> for ActivityKind {
    fn from(source: sys::EDiscordActivityType) -> Self {
        match source {
            sys::DiscordActivityType_Listening => ActivityKind::Listening,
            sys::DiscordActivityType_Playing => ActivityKind::Playing,
            sys::DiscordActivityType_Streaming => ActivityKind::Streaming,
            sys::DiscordActivityType_Watching => ActivityKind::Watching,
            _ => panic!(INVALID_ENUM),
        }
    }
}

impl Into<sys::EDiscordActivityType> for ActivityKind {
    fn into(self) -> sys::EDiscordActivityType {
        match self {
            ActivityKind::Listening => sys::DiscordActivityType_Listening,
            ActivityKind::Playing => sys::DiscordActivityType_Playing,
            ActivityKind::Streaming => sys::DiscordActivityType_Streaming,
            ActivityKind::Watching => sys::DiscordActivityType_Watching,
        }
    }
}
