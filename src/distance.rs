use crate::sys;

/// Lobby Search Max Distance
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#data-models-lobbysearchdistance-enum)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Distance {
    /// Within the same region
    Local,
    /// Within the same and adjacent regions
    Default,
    /// Far distances, like US to EU
    Extended,
    /// All regions
    Global,
}

impl Into<sys::EDiscordLobbySearchDistance> for Distance {
    fn into(self) -> sys::EDiscordLobbySearchDistance {
        match self {
            Self::Default => sys::DiscordLobbySearchDistance_Default,
            Self::Extended => sys::DiscordLobbySearchDistance_Extended,
            Self::Global => sys::DiscordLobbySearchDistance_Global,
            Self::Local => sys::DiscordLobbySearchDistance_Local,
        }
    }
}
