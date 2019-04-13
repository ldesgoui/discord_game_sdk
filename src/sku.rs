use crate::{sys, SkuKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Sku(pub(crate) sys::DiscordSku);

impl Sku {
    pub fn id(&self) -> i64 {
        self.0.id
    }

    pub fn kind(&self) -> SkuKind {
        self.0.type_.into()
    }

    str_field!(name, name);

    pub fn price_amount(&self) -> u32 {
        self.0.price.amount
    }

    str_field!(price_currency, price.currency);
}
