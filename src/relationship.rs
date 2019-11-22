use crate::{sys, Presence, RelationshipKind, User};

/// Relationship
///
/// <https://discordapp.com/developers/docs/game-sdk/relationships#data-models-relationship-struct>
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Relationship {
    kind: RelationshipKind,
    user: User,
    presence: Presence,
}

impl Relationship {
    pub fn kind(&self) -> RelationshipKind {
        self.kind
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn presence(&self) -> &Presence {
        &self.presence
    }
}

impl From<sys::DiscordRelationship> for Relationship {
    fn from(sys: sys::DiscordRelationship) -> Self {
        Self {
            kind: sys.type_.into(),
            user: sys.user.into(),
            presence: sys.presence.into(),
        }
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
