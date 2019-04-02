use crate::prelude::*;
use crate::presence::Presence;
use crate::user::User;

pub struct Relationship {
    pub kind: RelationshipKind,
    pub user: User,
    pub presence: Presence,
}

pub enum RelationshipKind {
    Blocked,
    Friend,
    Implicit,
    None,
    PendingIncoming,
    PendingOutgoing,
}
