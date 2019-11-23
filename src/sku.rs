use crate::{
    sys,
    utils::{charbuf_len, charbuf_to_str},
    SkuKind,
};

/// SKU (stock keeping unit)
///
/// <https://discordapp.com/developers/docs/game-sdk/store#data-models-sku-struct>
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Sku {
    pub(crate) sys: sys::DiscordSku,
    name_len: usize,
    price_currency_len: usize,
}

impl Sku {
    /// The unique ID of the SKU
    pub fn id(&self) -> i64 {
        self.sys.id
    }

    /// What sort of SKU it is
    pub fn kind(&self) -> SkuKind {
        self.sys.type_.into()
    }

    /// The name of the SKU
    pub fn name(&self) -> &str {
        charbuf_to_str(&self.sys.name[..self.name_len])
    }

    /// The amount of money that the SKU costs
    pub fn price_amount(&self) -> u32 {
        self.sys.price.amount
    }

    /// The currency that [`price_amount`](#method.price_currency) is in
    pub fn price_currency(&self) -> &str {
        charbuf_to_str(&self.sys.price.currency[..self.price_currency_len])
    }
}

impl From<sys::DiscordSku> for Sku {
    fn from(sys: sys::DiscordSku) -> Self {
        Self {
            sys,
            name_len: charbuf_len(&sys.name[..]),
            price_currency_len: charbuf_len(&sys.price.currency[..]),
        }
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
