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
    pub(crate) fn from_sys(source: &sys::DiscordUser) -> Result<Self> {
        let username =
            unsafe { std::ffi::CStr::from_ptr(&source.username as *const _ as *const _) }
                .to_str()
                .map_err(BindingsViolation::from)?
                .to_string();

        let discriminator =
            unsafe { std::ffi::CStr::from_ptr(&source.discriminator as *const _ as *const _) }
                .to_str()
                .map_err(BindingsViolation::from)?
                .to_string();

        let avatar = unsafe { std::ffi::CStr::from_ptr(&source.avatar as *const _ as *const _) }
            .to_str()
            .map_err(BindingsViolation::from)?
            .to_string();

        Ok(Self {
            id: source.id,
            username,
            discriminator,
            avatar,
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
