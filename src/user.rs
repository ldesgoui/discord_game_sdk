use crate::{sys, ImageHandle};

#[derive(Clone, Copy, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct User(pub(crate) sys::DiscordUser);

impl User {
    pub fn id(&self) -> i64 {
        self.0.id
    }

    str_field!(username, username);
    str_field!(discriminator, discriminator);
    str_field!(avatar, avatar);

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
