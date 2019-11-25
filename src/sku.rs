use crate::{sys, utils::charbuf_to_str, SkuKind};

/// SKU (stock keeping unit)
///
/// <https://discordapp.com/developers/docs/game-sdk/store#data-models-sku-struct>
#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Sku(pub(crate) sys::DiscordSku);

impl Sku {
    /// The unique ID of the SKU
    pub fn id(&self) -> i64 {
        self.0.id
    }

    /// What sort of SKU it is
    pub fn kind(&self) -> SkuKind {
        self.0.type_.into()
    }

    /// The name of the SKU
    pub fn name(&self) -> &str {
        charbuf_to_str(&self.0.name)
    }

    /// The amount of money that the SKU costs
    pub fn price_amount(&self) -> u32 {
        self.0.price.amount
    }

    /// The currency that [`price_amount`](#method.price_currency) is in
    pub fn price_currency(&self) -> &str {
        charbuf_to_str(&self.0.price.currency)
    }
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
