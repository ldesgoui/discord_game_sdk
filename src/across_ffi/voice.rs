use crate::{
    channels, event,
    panic_messages::{NULL_PTR, SEND_FAIL},
};
use std::ffi::c_void;

pub(crate) extern "C" fn on_settings_update(senders: *mut c_void) {
    prevent_unwind!();

    unsafe { (senders as *mut channels::Senders).as_ref() }
        .expect(NULL_PTR)
        .voice_settings_update
        .try_send(event::VoiceSettingsUpdate)
        .expect(SEND_FAIL)
}
