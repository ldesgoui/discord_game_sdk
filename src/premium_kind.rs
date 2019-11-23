use crate::sys;

/// Premium Type
///
/// <https://discordapp.com/developers/docs/game-sdk/users#data-models-premiumtype-enum>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PremiumKind {
    /// Not a Nitro subscriber
    None,
    /// Nitro Classic subscriber
    Tier1,
    /// Nitro subscriber
    Tier2,
    Undefined(sys::EDiscordPremiumType),
}

impl From<sys::EDiscordPremiumType> for PremiumKind {
    fn from(source: sys::EDiscordPremiumType) -> Self {
        match source {
            sys::DiscordPremiumType_None => Self::None,
            sys::DiscordPremiumType_Tier1 => Self::Tier1,
            sys::DiscordPremiumType_Tier2 => Self::Tier2,
            _ => Self::Undefined(source),
        }
    }
}
