use crate::{sys, Presence, RelationshipKind, User};

#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Relationship(pub(crate) sys::DiscordRelationship);

impl Relationship {
    pub fn kind(&self) -> RelationshipKind {
        self.0.type_.into()
    }

    pub fn user(&self) -> &User {
        unsafe { &*(&self.0.user as *const _ as *const User) }
    }

    pub fn presence(&self) -> &Presence {
        unsafe { &*(&self.0.presence as *const _ as *const Presence) }
    }
}

impl std::fmt::Debug for Relationship {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Relationship")
            .field("kind", &self.kind())
            .field("user", &self.user())
            .field("presence", &self.presence())
            .finish()
    }
}
