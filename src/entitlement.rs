use crate::{sys, EntitlementKind};

#[derive(Clone, Copy, Eq, Hash, PartialEq, derive_more::From, derive_more::Into)]
pub struct Entitlement(pub(crate) sys::DiscordEntitlement);

impl Entitlement {
    pub fn id(&self) -> i64 {
        self.0.id
    }

    pub fn kind(&self) -> EntitlementKind {
        self.0.type_.into()
    }

    pub fn sku_id(&self) -> i64 {
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
