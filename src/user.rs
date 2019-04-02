use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
    pub bot: bool,
}

impl User {
    pub(crate) fn from_sys(source: &sys::DiscordUser) -> Result<Self> {
        let username =
            unsafe { std::ffi::CStr::from_ptr(&source.username as *const _ as *const _) }
                .to_str()
                .map_err(BindingsViolation::from)?
                .to_string();

        let discriminator =
            unsafe { std::ffi::CStr::from_ptr(&source.discriminator as *const _ as *const _) }
                .to_str()
                .map_err(BindingsViolation::from)?
                .to_string();

        let avatar = unsafe { std::ffi::CStr::from_ptr(&source.avatar as *const _ as *const _) }
            .to_str()
            .map_err(BindingsViolation::from)?
            .to_string();

        Ok(Self {
            id: source.id,
            username,
            discriminator,
            avatar,
            bot: source.bot,
        })
    }
}
