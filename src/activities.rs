use crate::error::*;
use crate::utils::*;
use crate::Discord;
use discord_game_sdk_sys as sys;
use std::os::raw::c_void;

/// Activities
impl Discord {
    pub fn register_launch_command<S>(&self, command: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let cstring = std::ffi::CString::new(command.as_ref()).map_err(DeveloperViolation::from)?;

        ffi!(self
            .get_activity_manager()
            .register_command(cstring.as_ptr()))
    }

    /// # Rate limit
    /// 5 updates per 20 seconds
    pub fn update_activity<F>(&self, activity_change: &ActivityChange, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let mut activity = activity_change.to_sys().unwrap();

        let _ = ffi!(self.get_activity_manager().update_activity(
            &mut activity as *mut _,
            &callback as *const _ as *mut _,
            Some(simple_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }

    pub fn clear_activity<F>(&self, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let _ = ffi!(self
            .get_activity_manager()
            .clear_activity(&callback as *const _ as *mut _, Some(simple_callback::<F>)))
        .map_err(|e| callback(Err(e)));
    }

    pub fn send_request_reply<F>(&self, user_id: i64, reply: RequestReply, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let _ = ffi!(self.get_activity_manager().send_request_reply(
            user_id,
            reply.to_sys(),
            &callback as *const _ as *mut _,
            Some(simple_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }

    pub fn send_invite<S, F>(&self, user_id: i64, action: Action, content: S, mut callback: F)
    where
        S: AsRef<str>,
        F: FnMut(Result<()>),
    {
        let _ = std::ffi::CString::new(content.as_ref())
            .map_err(DeveloperViolation::from)
            .map_err(Error::from)
            .and_then(|cstring| {
                ffi!(self.get_activity_manager().send_invite(
                    user_id,
                    action.to_sys(),
                    cstring.as_ptr(),
                    &callback as *const _ as *mut _,
                    Some(simple_callback::<F>)
                ))
            })
            .map_err(|e| callback(Err(e)));
    }

    pub fn accept_invite<F>(&self, user_id: i64, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let _ = ffi!(self.get_activity_manager().accept_invite(
            user_id,
            &callback as *const _ as *mut _,
            Some(simple_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Join,
    Spectate,
}

impl Action {
    fn to_sys(self) -> sys::EDiscordActivityActionType {
        match self {
            Action::Join => sys::DiscordActivityActionType_Join,
            Action::Spectate => sys::DiscordActivityActionType_Spectate,
        }
    }
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum RequestReply {
    Yes,
    No,
    Ignore,
}

impl RequestReply {
    fn to_sys(self) -> sys::EDiscordActivityJoinRequestReply {
        match self {
            RequestReply::Yes => sys::DiscordActivityJoinRequestReply_Yes,
            RequestReply::No => sys::DiscordActivityJoinRequestReply_No,
            RequestReply::Ignore => sys::DiscordActivityJoinRequestReply_Ignore,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ActivityChange<'a> {
    pub state: Option<&'a str>,
    pub details: Option<&'a str>,
    pub start_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub large_image_key: Option<&'a str>,
    pub large_image_tooltip: Option<&'a str>,
    pub small_image_key: Option<&'a str>,
    pub small_image_tooltip: Option<&'a str>,
    pub party_id: Option<&'a str>,
    pub party_amount: Option<i32>,
    pub party_capacity: Option<i32>,
    pub match_secret: Option<&'a str>,
    pub join_secret: Option<&'a str>,
    pub spectate_secret: Option<&'a str>,
}

impl<'a> ActivityChange<'a> {
    fn to_sys(&self) -> Result<sys::DiscordActivity> {
        let mut activity = sys::DiscordActivity::default();

        write_to_array(&self.state, &mut activity.state)?;
        write_to_array(&self.details, &mut activity.details)?;
        write_to_array(&self.large_image_key, &mut activity.assets.large_image)?;
        write_to_array(&self.large_image_tooltip, &mut activity.assets.large_text)?;
        write_to_array(&self.small_image_key, &mut activity.assets.small_image)?;
        write_to_array(&self.small_image_tooltip, &mut activity.assets.small_text)?;
        write_to_array(&self.party_id, &mut activity.party.id)?;
        write_to_array(&self.match_secret, &mut activity.secrets.match_)?;
        write_to_array(&self.join_secret, &mut activity.secrets.join)?;
        write_to_array(&self.spectate_secret, &mut activity.secrets.spectate)?;

        if let Some(start_time) = self.start_time {
            activity.timestamps.start = start_time.timestamp();
        }

        if let Some(end_time) = self.end_time {
            activity.timestamps.end = end_time.timestamp();
        }

        if let Some(party_amount) = self.party_amount {
            activity.party.size.current_size = party_amount;
        }

        if let Some(party_capacity) = self.party_capacity {
            activity.party.size.max_size = party_capacity;
        }

        Ok(activity)
    }
}

fn write_to_array(source: &Option<&str>, destination: &mut [i8]) -> Result<()> {
    if let &Some(src) = source {
        let cstring = std::ffi::CString::new(src).map_err(DeveloperViolation::from)?;
        let bytes: &[i8] = unsafe { std::mem::transmute(cstring.as_bytes_with_nul()) };
        if bytes.len() > destination.len() {
            Err(DeveloperViolation::StringTooLarge)?;
        }
        destination[..bytes.len()].copy_from_slice(bytes);
    }

    Ok(())
}
