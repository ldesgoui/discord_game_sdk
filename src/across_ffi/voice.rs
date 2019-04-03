use crate::prelude::*;

pub(crate) extern "C" fn on_settings_update(core_ptr: *mut c_void) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    core.voice_channel
        .single_write(event::Voice::SettingsUpdated)
}
