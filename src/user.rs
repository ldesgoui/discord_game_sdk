use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
    pub bot: bool,
}

impl User {
    pub(crate) fn from_sys(source: *const sys::DiscordUser) -> Result<Self> {
        let source = unsafe { source.as_ref() }.ok_or(BindingsViolation::NullPointer)?;

        Ok(Self {
            id: source.id,
            username: from_cstr(&source.username as *const _)?.to_string(),
            discriminator: from_cstr(&source.discriminator as *const _)?.to_string(),
            avatar: from_cstr(&source.avatar as *const _)?.to_string(),
            bot: source.bot,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
