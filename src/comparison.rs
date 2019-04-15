use crate::sys;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Comparison {
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    NotEqual,
}

impl Into<sys::EDiscordLobbySearchComparison> for Comparison {
    fn into(self) -> sys::EDiscordLobbySearchComparison {
        match self {
            Comparison::Equal => sys::DiscordLobbySearchComparison_Equal,
            Comparison::GreaterThan => sys::DiscordLobbySearchComparison_GreaterThan,
            Comparison::GreaterThanOrEqual => sys::DiscordLobbySearchComparison_GreaterThanOrEqual,
            Comparison::LessThan => sys::DiscordLobbySearchComparison_LessThan,
            Comparison::LessThanOrEqual => sys::DiscordLobbySearchComparison_LessThanOrEqual,
            Comparison::NotEqual => sys::DiscordLobbySearchComparison_NotEqual,
        }
    }
}
