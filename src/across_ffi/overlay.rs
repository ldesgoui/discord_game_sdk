use crate::prelude::*;

pub(crate) extern "C" fn on_toggle(event_data: *mut c_void, locked: bool) {
    let core: &mut Discord = unsafe { (event_data as *mut Discord).as_mut() }.unwrap();

    core.overlay_events.single_write(if locked {
        event::Overlay::Opened
    } else {
        event::Overlay::Closed
    })
}
