use crate::prelude::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PremiumType {
    /// Not a Nitro subscriber
    None,
    /// Nitro Classic subscriber
    Tier1,
    /// Nitro subscriber
    Tier2,
}

impl PremiumType {
    pub(crate) fn from_sys(source: sys::EDiscordPremiumType) -> Result<Self> {
        Ok(match source {
            sys::DiscordPremiumType_None => PremiumType::None,
            sys::DiscordPremiumType_Tier1 => PremiumType::Tier1,
            sys::DiscordPremiumType_Tier2 => PremiumType::Tier2,
            _ => Err(BindingsViolation::Enum)?,
        })
    }
}
