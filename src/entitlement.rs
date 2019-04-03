use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entitlement {
    pub id: i64,
    pub kind: EntitlementKind,
    pub sku_id: i64,
}

impl FromSys for Entitlement {
    type Source = sys::DiscordEntitlement;

    fn from_sys(source: &Self::Source) -> Self {
        Self {
            id: source.id,
            kind: EntitlementKind::from_sys(&source.type_),
            sku_id: source.sku_id,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntitlementKind {
    DeveloperGift,
    FreePurchase,
    PremiumPurchase,
    PremiumSubscription,
    Purchase,
    TestModePurchase,
    UserGift,
}

impl FromSys for EntitlementKind {
    type Source = sys::EDiscordEntitlementType;

    fn from_sys(source: &Self::Source) -> Self {
        match *source {
            sys::DiscordEntitlementType_DeveloperGift => EntitlementKind::DeveloperGift,
            sys::DiscordEntitlementType_FreePurchase => EntitlementKind::FreePurchase,
            sys::DiscordEntitlementType_PremiumPurchase => EntitlementKind::PremiumPurchase,
            sys::DiscordEntitlementType_PremiumSubscription => EntitlementKind::PremiumSubscription,
            sys::DiscordEntitlementType_Purchase => EntitlementKind::Purchase,
            sys::DiscordEntitlementType_TestModePurchase => EntitlementKind::TestModePurchase,
            sys::DiscordEntitlementType_UserGift => EntitlementKind::UserGift,
            _ => panic!("enum"),
        }
    }
}
