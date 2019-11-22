use crate::sys;

/// Image Type
///
/// <https://discordapp.com/developers/docs/game-sdk/images#data-models-imagetype-enum>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ImageKind {
    /// User Avatar
    User,
    Undefined(sys::EDiscordImageType),
}

#[doc(hidden)]
impl From<sys::EDiscordImageType> for ImageKind {
    fn from(source: sys::EDiscordImageType) -> Self {
        match source {
            sys::DiscordImageType_User => Self::User,
            _ => Self::Undefined(source),
        }
    }
}

#[doc(hidden)]
impl Into<sys::EDiscordImageType> for ImageKind {
    fn into(self) -> sys::EDiscordImageType {
        match self {
            Self::User => sys::DiscordImageType_User,
            Self::Undefined(n) => n,
        }
    }
}
