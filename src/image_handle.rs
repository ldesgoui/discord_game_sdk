use crate::{sys, ImageKind, Snowflake, UserID};

/// Image Handle
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/images#data-models-imagehandle-struct)
#[derive(Clone, Copy, Eq, Hash, PartialEq, derive_more::From, derive_more::Into)]
#[repr(transparent)]
pub struct ImageHandle(pub(crate) sys::DiscordImageHandle);

impl ImageHandle {
    /// What sort of image it is
    pub fn kind(&self) -> ImageKind {
        self.0.type_.into()
    }

    /// A unique ID related to the image, when kind is User, it is the ID of said user
    pub fn id(&self) -> Snowflake {
        self.0.id
    }

    /// The resolution desired
    pub fn size(&self) -> u32 {
        self.0.size
    }

    /// Create new Image Handle
    pub fn from_user_id(user_id: UserID, size: u32) -> Self {
        Self(sys::DiscordImageHandle {
            type_: ImageKind::User.into(),
            id: user_id,
            size,
            ..Default::default()
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
