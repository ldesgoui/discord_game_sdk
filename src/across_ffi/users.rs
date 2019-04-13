use crate::event;
use std::ffi::c_void;

pub(crate) extern "C" fn on_current_user_update(senders: *mut c_void) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .current_user_update
        .try_send(event::users::CurrentUserUpdate)
        .unwrap()
}
