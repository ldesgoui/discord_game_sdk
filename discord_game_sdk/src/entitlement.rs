use crate::{sys, EntitlementKind, Snowflake};

/// Proof that user has made a purchase
///
/// This must then be consumed by your game's backend
///
/// > [Struct in official docs](https://discordapp.com/developers/docs/game-sdk/store#data-models-entitlement-struct)
#[derive(Clone, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct Entitlement(pub(crate) sys::DiscordEntitlement);

impl Entitlement {
    /// The unique ID of the entitlement
    pub fn id(&self) -> Snowflake {
        self.0.id
    }

    /// The kind of entitlement it is
    pub fn kind(&self) -> EntitlementKind {
        self.0.type_.into()
    }

    /// The ID of the SKU to which the user is entitled
    pub fn sku_id(&self) -> Snowflake {
        self.0.sku_id
    }
}

impl std::fmt::Debug for Entitlement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Entitlement")
            .field("id", &self.id())
            .field("kind", &self.kind())
            .field("sku_id", &self.sku_id())
            .finish()
    }
}
