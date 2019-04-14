use crate::{sys, SkuKind};

#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Sku(pub(crate) sys::DiscordSku);

impl Sku {
    pub fn id(&self) -> i64 {
        self.0.id
    }

    pub fn kind(&self) -> SkuKind {
        self.0.type_.into()
    }

    get_str!(name, name);

    pub fn price_amount(&self) -> u32 {
        self.0.price.amount
    }

    get_str!(price_currency, price.currency);
}

impl std::fmt::Debug for Sku {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Sku")
            .field("id", &self.id())
            .field("kind", &self.kind())
            .field("name", &self.name())
            .field("price_amount", &self.price_amount())
            .field("price_currency", &self.price_currency())
            .finish()
    }
}
