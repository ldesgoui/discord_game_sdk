use crate::{panic_messages::INVALID_ENUM, sys};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PremiumKind {
    /// Not a Nitro subscriber
    None,
    /// Nitro Classic subscriber
    Tier1,
    /// Nitro subscriber
    Tier2,
}

impl From<sys::EDiscordPremiumType> for PremiumKind {
    fn from(source: sys::EDiscordPremiumType) -> Self {
        match source {
            sys::DiscordPremiumType_None => PremiumKind::None,
            sys::DiscordPremiumType_Tier1 => PremiumKind::Tier1,
            sys::DiscordPremiumType_Tier2 => PremiumKind::Tier2,
            _ => panic!(INVALID_ENUM),
        }
    }
}
