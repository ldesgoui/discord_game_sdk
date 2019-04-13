use crate::{sys, EntitlementKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
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
