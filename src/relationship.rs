use crate::{sys, Presence, RelationshipKind, User};

#[derive(Clone, Copy, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Relationship(pub(crate) sys::DiscordRelationship);

impl Relationship {
    pub fn kind(&self) -> RelationshipKind {
        self.0.type_.into()
    }

    pub fn user(&self) -> &User {
        unsafe { std::mem::transmute(&self.0.user) }
    }

    pub fn presence(&self) -> &Presence {
        unsafe { std::mem::transmute(&self.0.presence) }
    }
}
