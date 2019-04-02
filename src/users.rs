use crate::error::*;
use crate::events::UserEvent;
use crate::utils::*;
use crate::Discord;
use discord_game_sdk_sys as sys;
use std::os::raw::c_void;

/// Activities
impl Discord {
    pub fn get_current_user(&self) -> Result<User> {
        let mut user = sys::DiscordUser::default();

        ffi!(self
            .get_user_manager()
            .get_current_user(&mut user as *mut _))?;

        User::from_sys(&user)
    }

    pub fn get_user<F>(&self, user_id: i64, mut callback: F)
    where
        F: FnMut(Result<User>),
    {
        let _ = ffi!(self.get_user_manager().get_user(
            user_id,
            &callback as *const _ as *mut _,
            Some(get_user_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }

    pub fn get_current_user_premium_type(&self) -> Result<PremiumType> {
        let mut premium_type = sys::EDiscordPremiumType::default();

        ffi!(self
            .get_user_manager()
            .get_current_user_premium_type(&mut premium_type as *mut _))?;

        PremiumType::from_sys(premium_type)
    }

    pub fn user_events_reader(&mut self) -> shrev::ReaderId<UserEvent> {
        self.user_events.register_reader()
    }

    pub fn user_events(
        &self,
        reader: &mut shrev::ReaderId<UserEvent>,
    ) -> shrev::EventIterator<UserEvent> {
        self.user_events.read(reader)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
    pub bot: bool,
}

impl User {
    fn from_sys(source: &sys::DiscordUser) -> Result<Self> {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PremiumType {
    /// Not a Nitro subscriber
    None,
    /// Nitro Classic subscriber
    Tier1,
    /// Nitro subscriber
    Tier2,
}

impl PremiumType {
    fn from_sys(source: sys::EDiscordPremiumType) -> Result<Self> {
        Ok(match source {
            sys::DiscordPremiumType_None => PremiumType::None,
            sys::DiscordPremiumType_Tier1 => PremiumType::Tier1,
            sys::DiscordPremiumType_Tier2 => PremiumType::Tier2,
            _ => Err(BindingsViolation::Enum)?,
        })
    }
}

extern "C" fn get_user_callback<F>(
    data: *mut c_void,
    res: sys::EDiscordResult,
    user: *mut sys::DiscordUser,
) where
    F: FnMut(Result<User>),
{
    if data.is_null() {
        log::error!("SDK invoked callback with null");
        return;
    }
    let callback: &mut F = unsafe { &mut *(data as *mut _) };

    callback(
        res.to_result()
            .and_then(|_| unsafe { user.as_ref() }.ok_or(BindingsViolation::NullPointer.into()))
            .and_then(|user| User::from_sys(user)),
    )
}
