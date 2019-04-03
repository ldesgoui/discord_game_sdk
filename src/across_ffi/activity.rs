use crate::prelude::*;

pub(crate) extern "C" fn on_activity_join(core_ptr: *mut c_void, secret: *const c_char) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    let secret = unsafe { std::ffi::CStr::from_ptr(secret) }
        .to_str()
        .unwrap()
        .to_string();

    core.activity_channel
        .single_write(event::Activity::Join { secret });
}

pub(crate) extern "C" fn on_activity_spectate(core_ptr: *mut c_void, secret: *const c_char) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    let secret = unsafe { std::ffi::CStr::from_ptr(secret) }
        .to_str()
        .unwrap()
        .to_string();

    core.activity_channel
        .single_write(event::Activity::Spectate { secret });
}

pub(crate) extern "C" fn on_activity_join_request(
    core_ptr: *mut c_void,
    user: *mut sys::DiscordUser,
) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    let user = unsafe { User::from_sys_ptr(user) };

    core.activity_channel
        .single_write(event::Activity::Request { user });
}

pub(crate) extern "C" fn on_activity_invite(
    core_ptr: *mut c_void,
    ty: sys::EDiscordActivityActionType,
    user: *mut sys::DiscordUser,
    activity: *mut sys::DiscordActivity,
) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    let user = unsafe { User::from_sys_ptr(user) };
    let activity = unsafe { Activity::from_sys_ptr(activity) };

    core.activity_channel
        .single_write(event::Activity::Invite { user, activity });
}
