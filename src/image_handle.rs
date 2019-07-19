use crate::{sys, ImageKind};

/// Image Handle
///
/// <https://discordapp.com/developers/docs/game-sdk/images#data-models-imagehandle-struct>
#[derive(Clone, Copy, Eq, Hash, PartialEq, derive_more::From, derive_more::Into)]
pub struct ImageHandle(pub(crate) sys::DiscordImageHandle);

impl ImageHandle {
    pub fn kind(&self) -> ImageKind {
        self.0.type_.into()
    }

    pub fn id(&self) -> i64 {
        self.0.id
    }

    pub fn size(&self) -> u32 {
        self.0.size
    }

    /// Create new Image Handle
    ///
    /// `size` must be 16, 32, 64, 128 or 256.
    pub fn from_user_id(user_id: i64, size: u32) -> Self {
        debug_assert!([16, 32, 64, 128, 256].contains(&size));

        Self(sys::DiscordImageHandle {
            type_: ImageKind::User.into(),
            id: user_id,
            size,
        })
    }
}

impl std::fmt::Debug for ImageHandle {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("ImageHandle")
            .field("kind", &self.kind())
            .field("id", &self.id())
            .field("size", &self.size())
            .finish()
    }
}
