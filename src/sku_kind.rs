use crate::{panic_messages::INVALID_ENUM, sys};

/// SKU Type
///
/// <https://discordapp.com/developers/docs/game-sdk/store#data-models-skutype-enum>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SkuKind {
    /// SKU is a game
    Application,
    /// SKU is a bundle (comprising the 3 other types)
    Bundle,
    /// Bundle is a consumable (in-app purchase)
    Consumable,
    /// Bundle is a DLC
    DLC,
}

#[doc(hidden)]
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
