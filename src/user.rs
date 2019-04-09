use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
    pub bot: bool,
}

impl FromSys for User {
    type Source = sys::DiscordUser;

    fn from_sys(source: &Self::Source) -> Self {
        Self {
            id: source.id,
            username: unsafe { string_from_cstr(&source.username as *const _) },
            discriminator: unsafe { string_from_cstr(&source.discriminator as *const _) },
            avatar: unsafe { string_from_cstr(&source.avatar as *const _) },
            bot: source.bot,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PremiumKind {
    /// Not a Nitro subscriber
    None,
    /// Nitro Classic subscriber
    Tier1,
    /// Nitro subscriber
    Tier2,
}

impl FromSys for PremiumKind {
    type Source = sys::EDiscordPremiumType;

    fn from_sys(source: &Self::Source) -> Self {
        match *source {
            sys::DiscordPremiumType_None => PremiumKind::None,
            sys::DiscordPremiumType_Tier1 => PremiumKind::Tier1,
            sys::DiscordPremiumType_Tier2 => PremiumKind::Tier2,
            _ => panic!("enum"),
        }
    }
}
