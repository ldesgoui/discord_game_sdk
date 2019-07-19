use crate::{sys, ImageHandle};

/// User
///
/// <https://discordapp.com/developers/docs/game-sdk/users#data-models-user-struct>
#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct User(pub(crate) sys::DiscordUser);

impl User {
    pub fn id(&self) -> i64 {
        self.0.id
    }

    get_str!(username, username);
    get_str!(discriminator, discriminator);
    get_str!(avatar, avatar);

    pub fn is_bot(&self) -> bool {
        self.0.bot
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
