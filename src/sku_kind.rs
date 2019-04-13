use crate::{panic_messages::INVALID_ENUM, sys};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SkuKind {
    Application,
    Bundle,
    Consumable,
    DLC,
}

impl From<sys::EDiscordSkuType> for SkuKind {
    fn from(source: sys::EDiscordSkuType) -> Self {
        match source {
            sys::DiscordSkuType_Application => SkuKind::Application,
            sys::DiscordSkuType_Bundle => SkuKind::Bundle,
            sys::DiscordSkuType_Consumable => SkuKind::Consumable,
            sys::DiscordSkuType_DLC => SkuKind::DLC,
            _ => panic!(INVALID_ENUM),
        }
    }
}
