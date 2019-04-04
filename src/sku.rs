use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sku {
    pub id: i64,
    pub kind: SkuKind,
    pub name: String,
    pub price: u32,
    pub currency: String,
}

impl FromSys for Sku {
    type Source = sys::DiscordSku;

    fn from_sys(source: &Self::Source) -> Self {
        Self {
            id: source.id,
            kind: SkuKind::from_sys(&source.type_),
            name: unsafe { string_from_cstr(&source.name as *const _) },
            price: source.price.amount,
            currency: unsafe { string_from_cstr(&source.price.currency as *const _) },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SkuKind {
    Application,
    Bundle,
    Consumable,
    DLC,
}

impl FromSys for SkuKind {
    type Source = sys::EDiscordSkuType;

    fn from_sys(source: &Self::Source) -> Self {
        match *source {
            sys::DiscordSkuType_Application => SkuKind::Application,
            sys::DiscordSkuType_Bundle => SkuKind::Bundle,
            sys::DiscordSkuType_Consumable => SkuKind::Consumable,
            sys::DiscordSkuType_DLC => SkuKind::DLC,
            _ => panic!("enum"),
        }
    }
}
