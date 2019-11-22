use crate::sys;

/// Lobby Search Comparison
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#data-models-lobbysearchcomparison-enum>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Comparison {
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    NotEqual,
}

#[doc(hidden)]
impl Into<sys::EDiscordLobbySearchComparison> for Comparison {
    fn into(self) -> sys::EDiscordLobbySearchComparison {
        match self {
            Self::Equal => sys::DiscordLobbySearchComparison_Equal,
            Self::GreaterThan => sys::DiscordLobbySearchComparison_GreaterThan,
            Self::GreaterThanOrEqual => sys::DiscordLobbySearchComparison_GreaterThanOrEqual,
            Self::LessThan => sys::DiscordLobbySearchComparison_LessThan,
            Self::LessThanOrEqual => sys::DiscordLobbySearchComparison_LessThanOrEqual,
            Self::NotEqual => sys::DiscordLobbySearchComparison_NotEqual,
        }
    }
}
