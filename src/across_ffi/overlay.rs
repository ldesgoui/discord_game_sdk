use crate::{
    event,
    panic_messages::{NULL_PTR, SEND_FAIL},
};
use std::ffi::c_void;

pub(crate) extern "C" fn on_toggle(senders: *mut c_void, locked: bool) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .overlay_toggle
        .try_send(event::overlay::Toggle { opened: !locked })
        .expect(SEND_FAIL)
}
