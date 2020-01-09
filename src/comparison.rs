use crate::sys;

/// Lobby Search Comparison
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#data-models-lobbysearchcomparison-enum)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Comparison {
    /// Metadata must be equal to the search value
    Equal,
    /// Metadata must be greater than the search value
    GreaterThan,
    /// Metadata must be greater than or equal to the search value
    GreaterThanOrEqual,
    /// Metadata must be less than the search value
    LessThan,
    /// Metadata must be less than or equal to the search value
    LessThanOrEqual,
    /// Metadata must not be equal to the search value
    NotEqual,
}

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
