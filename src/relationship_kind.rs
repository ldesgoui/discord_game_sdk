use crate::{panic_messages::INVALID_ENUM, sys};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RelationshipKind {
    Blocked,
    Friend,
    Implicit,
    None,
    PendingIncoming,
    PendingOutgoing,
}

impl From<sys::EDiscordRelationshipType> for RelationshipKind {
    fn from(source: sys::EDiscordRelationshipType) -> Self {
        match source {
            sys::DiscordRelationshipType_Blocked => RelationshipKind::Blocked,
            sys::DiscordRelationshipType_Friend => RelationshipKind::Friend,
            sys::DiscordRelationshipType_Implicit => RelationshipKind::Implicit,
            sys::DiscordRelationshipType_None => RelationshipKind::None,
            sys::DiscordRelationshipType_PendingIncoming => RelationshipKind::PendingIncoming,
            sys::DiscordRelationshipType_PendingOutgoing => RelationshipKind::PendingOutgoing,
            _ => panic!(INVALID_ENUM),
        }
    }
}
