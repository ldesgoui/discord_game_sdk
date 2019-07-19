use crate::{
    event,
    panic_messages::{NULL_PTR, SEND_FAIL},
    sys,
};
use std::ffi::c_void;

pub(crate) extern "C" fn on_user_achievement_update(
    senders: *mut c_void,
    achievement: *mut sys::DiscordUserAchievement,
) {
    prevent_unwind!();

    debug_assert!(!achievement.is_null());

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .achievements_update
        .try_send(event::achievements::Update {
            achievement: unsafe { *achievement }.into(),
        })
        .expect(SEND_FAIL)
}
