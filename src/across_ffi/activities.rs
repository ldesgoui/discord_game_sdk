use crate::{event, sys};
use std::ffi::{c_void, CStr};

pub(crate) extern "C" fn on_activity_join(senders: *mut c_void, secret: *const i8) {
    prevent_unwind!();

    let secret = unsafe { CStr::from_ptr(secret) }
        .to_str()
        .unwrap()
        .to_string();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .activities_join
        .try_send(event::activities::Join { secret })
        .unwrap()
}

pub(crate) extern "C" fn on_activity_spectate(senders: *mut c_void, secret: *const i8) {
    prevent_unwind!();

    let secret = unsafe { CStr::from_ptr(secret) }
        .to_str()
        .unwrap()
        .to_string();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .activities_spectate
        .try_send(event::activities::Spectate { secret })
        .unwrap()
}

pub(crate) extern "C" fn on_activity_join_request(
    senders: *mut c_void,
    user: *mut sys::DiscordUser,
) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .activities_request
        .try_send(event::activities::Request {
            user: unsafe { *user }.into(),
        })
        .unwrap()
}

pub(crate) extern "C" fn on_activity_invite(
    senders: *mut c_void,
    action: sys::EDiscordActivityActionType,
    user: *mut sys::DiscordUser,
    activity: *mut sys::DiscordActivity,
) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .activities_invite
        .try_send(event::activities::Invite {
            action: action.into(),
            user: unsafe { *user }.into(),
            activity: unsafe { *activity }.into(),
        })
        .unwrap()
}
