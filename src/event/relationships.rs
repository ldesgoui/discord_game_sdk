use crate::Relationship;

/// On Relationships Refresh
///
/// <https://discordapp.com/developers/docs/game-sdk/relationships#onrefresh>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Refresh;

/// On Relationship Update
///
/// <https://discordapp.com/developers/docs/game-sdk/relationships#onrelationshipupdate>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Update {
    pub relationship: Relationship,
}
