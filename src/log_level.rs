use crate::sys;

/// Log level
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/discord#data-models-loglevel-enum)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LogLevel {
    /// Only errors
    Error,
    /// Warnings and errors
    Warning,
    /// Info, warnings and errors
    Info,
    /// Everything
    Debug,
    /// Safety net for missing definitions
    Undefined(sys::EDiscordLogLevel),
}

impl From<sys::EDiscordLogLevel> for LogLevel {
    fn from(source: sys::EDiscordLogLevel) -> Self {
        match source {
            sys::DiscordLogLevel_Error => Self::Error,
            sys::DiscordLogLevel_Warn => Self::Warning,
            sys::DiscordLogLevel_Info => Self::Info,
            sys::DiscordLogLevel_Debug => Self::Debug,
            n => Self::Undefined(n),
        }
    }
}

impl Into<log::Level> for LogLevel {
    fn into(self) -> log::Level {
        match self {
            Self::Error => log::Level::Error,
            Self::Warning => log::Level::Warn,
            Self::Info => log::Level::Info,
            _ => log::Level::Debug,
        }
    }
}
