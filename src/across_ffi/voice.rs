use crate::prelude::*;

pub(crate) extern "C" fn on_settings_update(senders: *mut c_void) {
    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .voice_settings_update
        .try_send(event::voice::SettingsUpdate)
        .unwrap()
}
