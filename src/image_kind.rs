use crate::{panic_messages::INVALID_ENUM, sys};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ImageKind {
    User,
}

impl From<sys::EDiscordImageType> for ImageKind {
    fn from(source: sys::EDiscordImageType) -> Self {
        match source {
            sys::DiscordImageType_User => ImageKind::User,
            _ => panic!(INVALID_ENUM),
        }
    }
}

impl Into<sys::EDiscordImageType> for ImageKind {
    fn into(self) -> sys::EDiscordImageType {
        match self {
            ImageKind::User => sys::DiscordImageType_User,
        }
    }
}
