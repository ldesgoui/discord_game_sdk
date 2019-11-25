use crate::{sys, utils::charbuf_to_str, ImageHandle};

/// User
///
/// <https://discordapp.com/developers/docs/game-sdk/users#data-models-user-struct>
#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
#[repr(transparent)]
pub struct User(pub(crate) sys::DiscordUser);

impl User {
    /// The unique ID of the user
    pub fn id(&self) -> i64 {
        self.0.id
    }

    /// Their name
    pub fn username(&self) -> &str {
        charbuf_to_str(&self.0.username)
    }

    /// The four digit unique discriminator, often attached to a `#`
    pub fn discriminator(&self) -> &str {
        charbuf_to_str(&self.0.discriminator)
    }

    /// The hash of the user's avatar
    pub fn avatar(&self) -> &str {
        charbuf_to_str(&self.0.avatar)
    }

    /// Whether the user is a bot
    pub fn is_bot(&self) -> bool {
        self.0.bot
    }

    /// Create an [Image Handle](struct.ImageHandle.html) targeting the user's avatar
    pub fn image_handle(&self, size: u32) -> ImageHandle {
        ImageHandle(sys::DiscordImageHandle {
            type_: sys::DiscordImageType_User,
            id: self.0.id,
            size,
        })
    }
}

impl std::fmt::Debug for User {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("User")
            .field("id", &self.id())
            .field("username", &self.username())
            .field("discriminator", &self.discriminator())
            .field("avatar", &self.avatar())
            .field("is_bot", &self.is_bot())
            .finish()
    }
}
