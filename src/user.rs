use crate::{
    sys,
    utils::{charbuf_len, charbuf_to_str},
    ImageHandle,
};

/// User
///
/// <https://discordapp.com/developers/docs/game-sdk/users#data-models-user-struct>
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct User {
    pub(crate) sys: sys::DiscordUser,
    username_len: usize,
    discriminator_len: usize,
    avatar_len: usize,
}

impl User {
    pub fn id(&self) -> i64 {
        self.sys.id
    }

    pub fn username(&self) -> &str {
        charbuf_to_str(&self.sys.username[..self.username_len])
    }

    pub fn discriminator(&self) -> &str {
        charbuf_to_str(&self.sys.discriminator[..self.discriminator_len])
    }

    pub fn avatar(&self) -> &str {
        charbuf_to_str(&self.sys.avatar[..self.avatar_len])
    }

    pub fn is_bot(&self) -> bool {
        self.sys.bot
    }

    /// Create a new [Image Handle]
    ///
    /// `size` must be 16, 32, 64, 128 or 256
    ///
    /// [Image Handle]: ./struct.ImageHandle.html
    pub fn image_handle(&self, size: u32) -> ImageHandle {
        debug_assert!([16, 32, 64, 128, 256].contains(&size));

        ImageHandle(sys::DiscordImageHandle {
            type_: sys::DiscordImageType_User,
            id: self.sys.id,
            size,
        })
    }
}

impl From<sys::DiscordUser> for User {
    fn from(sys: sys::DiscordUser) -> Self {
        Self {
            sys,
            username_len: charbuf_len(&sys.username),
            discriminator_len: charbuf_len(&sys.discriminator),
            avatar_len: charbuf_len(&sys.avatar),
        }
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
