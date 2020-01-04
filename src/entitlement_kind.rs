use crate::sys;

/// Entitlement Type
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/store#data-models-entitlementtype-enum)
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
    /// Safety net for missing definitions
    Undefined(sys::EDiscordEntitlementType),
}

impl From<sys::EDiscordEntitlementType> for EntitlementKind {
    fn from(source: sys::EDiscordEntitlementType) -> Self {
        match source {
            sys::DiscordEntitlementType_DeveloperGift => Self::DeveloperGift,
            sys::DiscordEntitlementType_FreePurchase => Self::FreePurchase,
            sys::DiscordEntitlementType_PremiumPurchase => Self::PremiumPurchase,
            sys::DiscordEntitlementType_PremiumSubscription => Self::PremiumSubscription,
            sys::DiscordEntitlementType_Purchase => Self::Purchase,
            sys::DiscordEntitlementType_TestModePurchase => Self::TestModePurchase,
            sys::DiscordEntitlementType_UserGift => Self::UserGift,
            _ => Self::Undefined(source),
        }
    }
}
