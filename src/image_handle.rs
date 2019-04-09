use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImageHandle {
    pub kind: ImageKind,
    pub id: i64,
    pub size: u32,
}

impl FromSys for ImageHandle {
    type Source = sys::DiscordImageHandle;

    fn from_sys(source: &Self::Source) -> Self {
        Self {
            kind: ImageKind::from_sys(&source.type_),
            id: source.id,
            size: source.size,
        }
    }
}

impl ImageHandle {
    pub(crate) fn to_sys(&self) -> sys::DiscordImageHandle {
        sys::DiscordImageHandle {
            type_: self.kind.to_sys(),
            id: self.id,
            size: self.size,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageKind {
    User,
}

impl FromSys for ImageKind {
    type Source = sys::EDiscordImageType;

    fn from_sys(source: &Self::Source) -> Self {
        match *source {
            sys::DiscordImageType_User => ImageKind::User,
            _ => panic!("enum"),
        }
    }
}

impl ImageKind {
    pub(crate) fn to_sys(self) -> sys::EDiscordImageType {
        match self {
            ImageKind::User => sys::DiscordImageType_User,
        }
    }
}
