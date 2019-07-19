use crate::sys;

/// Lobby Search Max Distance
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#data-models-lobbysearchdistance-enum>
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

#[doc(hidden)]
impl Into<sys::EDiscordLobbySearchDistance> for Distance {
    fn into(self) -> sys::EDiscordLobbySearchDistance {
        match self {
            Distance::Default => sys::DiscordLobbySearchDistance_Default,
            Distance::Extended => sys::DiscordLobbySearchDistance_Extended,
            Distance::Global => sys::DiscordLobbySearchDistance_Global,
            Distance::Local => sys::DiscordLobbySearchDistance_Local,
        }
    }
}
