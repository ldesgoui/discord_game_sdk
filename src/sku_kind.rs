use crate::sys;

/// SKU Type
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/store#data-models-skutype-enum)
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
    /// Safety net for missing definitions
    Undefined(sys::EDiscordSkuType),
}

impl From<sys::EDiscordSkuType> for SkuKind {
    fn from(source: sys::EDiscordSkuType) -> Self {
        match source {
            sys::DiscordSkuType_Application => Self::Application,
            sys::DiscordSkuType_Bundle => Self::Bundle,
            sys::DiscordSkuType_Consumable => Self::Consumable,
            sys::DiscordSkuType_DLC => Self::DLC,
            _ => Self::Undefined(source),
        }
    }
}
