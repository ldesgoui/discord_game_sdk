use crate::sys;

/// Relationship Type
///
/// <https://discordapp.com/developers/docs/game-sdk/relationships#data-models-relationshiptype-enum>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RelationshipKind {
    /// User is blocked
    Blocked,
    /// User is a friend
    Friend,
    /// Not a friend but interacts with current user often (frequency + recency)
    Implicit,
    /// User has no intrinsic relationship
    None,
    /// User has a pending incoming friend request to current user
    PendingIncoming,
    /// Current user has a pending outgoing friend request to user
    PendingOutgoing,
    Undefined(sys::EDiscordRelationshipType),
}

#[doc(hidden)]
impl From<sys::EDiscordRelationshipType> for RelationshipKind {
    fn from(source: sys::EDiscordRelationshipType) -> Self {
        match source {
            sys::DiscordRelationshipType_Blocked => Self::Blocked,
            sys::DiscordRelationshipType_Friend => Self::Friend,
            sys::DiscordRelationshipType_Implicit => Self::Implicit,
            sys::DiscordRelationshipType_None => Self::None,
            sys::DiscordRelationshipType_PendingIncoming => Self::PendingIncoming,
            sys::DiscordRelationshipType_PendingOutgoing => Self::PendingOutgoing,
            _ => Self::Undefined(source),
        }
    }
}
