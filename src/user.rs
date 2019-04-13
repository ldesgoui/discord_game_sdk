use crate::{sys, ImageHandle};

#[derive(Clone, Copy, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
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

    pub fn image_handle(&self, size: u32) -> ImageHandle {
        debug_assert!([16, 32, 64, 128, 256].contains(&size));

        ImageHandle(sys::DiscordImageHandle {
            type_: sys::DiscordImageType_User,
            id: self.0.id,
            size: size,
        })
    }
}
