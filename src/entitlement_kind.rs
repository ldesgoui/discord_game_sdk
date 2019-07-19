use crate::{panic_messages::INVALID_ENUM, sys};

/// Entitlement Type
///
/// <https://discordapp.com/developers/docs/game-sdk/store#data-models-entitlementtype-enum>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EntitlementKind {
    /// Entitlement was gifted by a developer
    DeveloperGift,
    /// Entitlement was granted when the SKU was free
    FreePurchase,
    /// Entitlement was claimed by user for free as a Nitro Subscriber
    PremiumPurchase,
    /// Entitlement for a Discord Nitro subscription
    PremiumSubscription,
    /// Entitlement was purchased
    Purchase,
    /// Entitlement was purchased by a dev in application test mode<Paste>
    TestModePurchase,
    /// Entitlement was gifted by another user
    UserGift,
}

#[doc(hidden)]
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
