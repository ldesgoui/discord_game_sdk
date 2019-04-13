use crate::{panic_messages::INVALID_ENUM, sys};

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

impl From<sys::EDiscordEntitlementType> for EntitlementKind {
    fn from(source: sys::EDiscordEntitlementType) -> Self {
        match source {
            sys::DiscordEntitlementType_DeveloperGift => EntitlementKind::DeveloperGift,
            sys::DiscordEntitlementType_FreePurchase => EntitlementKind::FreePurchase,
            sys::DiscordEntitlementType_PremiumPurchase => EntitlementKind::PremiumPurchase,
            sys::DiscordEntitlementType_PremiumSubscription => EntitlementKind::PremiumSubscription,
            sys::DiscordEntitlementType_Purchase => EntitlementKind::Purchase,
            sys::DiscordEntitlementType_TestModePurchase => EntitlementKind::TestModePurchase,
            sys::DiscordEntitlementType_UserGift => EntitlementKind::UserGift,
            _ => panic!(INVALID_ENUM),
        }
    }
}
