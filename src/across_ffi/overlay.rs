use crate::event;
use std::ffi::c_void;

pub(crate) extern "C" fn on_toggle(senders: *mut c_void, locked: bool) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .overlay_toggle
        .try_send(event::overlay::Toggle { opened: !locked })
        .unwrap()
}
