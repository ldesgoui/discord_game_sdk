use crate::{sys, Presence, RelationshipKind, User};

/// Relationship
///
/// > [Struct in official docs](https://discordapp.com/developers/docs/game-sdk/relationships#data-models-relationship-struct)
#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
#[repr(transparent)]
pub struct Relationship(pub(crate) sys::DiscordRelationship);

impl Relationship {
    /// What sort of relationship it is
    pub fn kind(&self) -> RelationshipKind {
        self.0.type_.into()
    }

    /// The target of the relationship
    pub fn user(&self) -> &User {
        unsafe { &*(&self.0.user as *const sys::DiscordUser as *const User) }
    }

    /// The target's current presence
    pub fn presence(&self) -> &Presence {
        unsafe { &*(&self.0.presence as *const sys::DiscordPresence as *const Presence) }
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
