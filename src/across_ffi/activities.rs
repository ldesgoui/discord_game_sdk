use crate::prelude::*;

pub(crate) extern "C" fn on_activity_join(senders: *mut c_void, secret: *const i8) {
    let secret = unsafe { std::ffi::CStr::from_ptr(secret) }
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
    let secret = unsafe { std::ffi::CStr::from_ptr(secret) }
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
    let user = unsafe { User::from_sys_ptr(user) };

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .activities_request
        .try_send(event::activities::Request { user })
        .unwrap()
}

pub(crate) extern "C" fn on_activity_invite(
    senders: *mut c_void,
    action: sys::EDiscordActivityActionType,
    user: *mut sys::DiscordUser,
    activity: *mut sys::DiscordActivity,
) {
    let action = Action::from_sys(&action);
    let user = unsafe { User::from_sys_ptr(user) };
    let activity = unsafe { Activity::from_sys_ptr(activity) };

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .activities_invite
        .try_send(event::activities::Invite {
            action,
            user,
            activity,
        })
        .unwrap()
}
