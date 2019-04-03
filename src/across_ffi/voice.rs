use crate::prelude::*;

pub(crate) extern "C" fn on_settings_update(event_data: *mut c_void) {
    let core: &mut Discord = unsafe { (event_data as *mut Discord).as_mut() }.unwrap();

    core.voice_events
        .single_write(event::Voice::SettingsUpdated)
}
