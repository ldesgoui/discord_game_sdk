use crate::sys;

/// Discord Creation Flags
///
/// <https://discordapp.com/developers/docs/game-sdk/discord#data-models-createflags-enum>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CreateFlags {
    /// Requires Discord to be running to play the game
    Default,
    /// Does not require Discord to be running, use this on other platforms
    NoRequireDiscord,
}

impl Default for CreateFlags {
    fn default() -> Self {
        CreateFlags::Default
    }
}

#[doc(hidden)]
impl Into<sys::EDiscordCreateFlags> for CreateFlags {
    fn into(self) -> sys::EDiscordCreateFlags {
        match self {
            CreateFlags::Default => sys::DiscordCreateFlags_Default,
            CreateFlags::NoRequireDiscord => sys::DiscordCreateFlags_NoRequireDiscord,
        }
    }
}
