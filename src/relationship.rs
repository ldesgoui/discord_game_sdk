use crate::prelude::*;
use crate::user::User;
use crate::utils::FromSys;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Relationship {
    pub kind: RelationshipKind,
    pub user: User,
    pub presence: Presence,
}

impl FromSys for Relationship {
    type Source = sys::DiscordRelationship;

    fn from_sys(source: &Self::Source) -> Result<Self> {
        Ok(Self {
            kind: RelationshipKind::from_sys(&source.type_)?,
            user: User::from_sys(&source.user)?,
            presence: Presence::from_sys(&source.presence)?,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationshipKind {
    Blocked,
    Friend,
    Implicit,
    None,
    PendingIncoming,
    PendingOutgoing,
}

impl FromSys for RelationshipKind {
    type Source = sys::EDiscordRelationshipType;

    fn from_sys(source: &Self::Source) -> Result<Self> {
        Ok(match *source {
            sys::DiscordRelationshipType_Blocked => RelationshipKind::Blocked,
            sys::DiscordRelationshipType_Friend => RelationshipKind::Friend,
            sys::DiscordRelationshipType_Implicit => RelationshipKind::Implicit,
            sys::DiscordRelationshipType_None => RelationshipKind::None,
            sys::DiscordRelationshipType_PendingIncoming => RelationshipKind::PendingIncoming,
            sys::DiscordRelationshipType_PendingOutgoing => RelationshipKind::PendingOutgoing,
            _ => Err(BindingsViolation::Enum)?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Presence {
    pub status: Status,
    pub activity: Activity,
}

impl FromSys for Presence {
    type Source = sys::DiscordPresence;

    fn from_sys(source: &Self::Source) -> Result<Self> {
        Ok(Self {
            status: Status::from_sys(&source.status)?,
            activity: Activity::from_sys(&source.activity)?,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    DoNotDisturb,
    Idle,
    Offline,
    Online,
}

impl FromSys for Status {
    type Source = sys::EDiscordStatus;

    fn from_sys(source: &Self::Source) -> Result<Self> {
        Ok(match *source {
            sys::DiscordStatus_DoNotDisturb => Status::DoNotDisturb,
            sys::DiscordStatus_Idle => Status::Idle,
            sys::DiscordStatus_Offline => Status::Offline,
            sys::DiscordStatus_Online => Status::Online,
            _ => Err(BindingsViolation::Enum)?,
        })
    }
}
