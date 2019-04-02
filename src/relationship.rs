use crate::prelude::*;
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

pub struct Presence {
    pub status: Status,
    pub activity: Activity,
}

pub enum Status {
    DoNotDisturb,
    Idle,
    Offline,
    Online,
}
