use crate::{
    channels, event,
    panic_messages::{NOT_UTF8, NULL_PTR, SEND_FAIL},
    sys,
};
use std::ffi::{c_void, CStr};

pub(crate) extern "C" fn on_activity_join(senders: *mut c_void, secret: *const i8) {
    prevent_unwind!();

    debug_assert!(!secret.is_null());

    let secret = unsafe { CStr::from_ptr(secret) }
        .to_str()
        .expect(NOT_UTF8)
        .to_string();

    unsafe { (senders as *mut channels::Senders).as_ref() }
        .expect(NULL_PTR)
        .activities_join
        .try_send(event::ActivityJoin { secret })
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn on_activity_spectate(senders: *mut c_void, secret: *const i8) {
    prevent_unwind!();

    debug_assert!(!secret.is_null());

    let secret = unsafe { CStr::from_ptr(secret) }
        .to_str()
        .expect(NOT_UTF8)
        .to_string();

    unsafe { (senders as *mut channels::Senders).as_ref() }
        .expect(NULL_PTR)
        .activities_spectate
        .try_send(event::ActivitySpectate { secret })
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn on_activity_join_request(
    senders: *mut c_void,
    user: *mut sys::DiscordUser,
) {
    prevent_unwind!();

    debug_assert!(!user.is_null());

    unsafe { (senders as *mut channels::Senders).as_ref() }
        .expect(NULL_PTR)
        .activities_request
        .try_send(event::ActivityRequest {
            user: unsafe { *user }.into(),
        })
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn on_activity_invite(
    senders: *mut c_void,
    action: sys::EDiscordActivityActionType,
    user: *mut sys::DiscordUser,
    activity: *mut sys::DiscordActivity,
) {
    prevent_unwind!();

    debug_assert!(!user.is_null());
    debug_assert!(!activity.is_null());

    unsafe { (senders as *mut channels::Senders).as_ref() }
        .expect(NULL_PTR)
        .activities_invite
        .try_send(event::ActivityInvite {
            action: action.into(),
            user: unsafe { *user }.into(),
            activity: unsafe { *activity }.into(),
        })
        .expect(SEND_FAIL)
}
