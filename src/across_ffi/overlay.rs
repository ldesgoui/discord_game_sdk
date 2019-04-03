use crate::prelude::*;

pub(crate) extern "C" fn on_toggle(core_ptr: *mut c_void, locked: bool) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    core.overlay_channel.single_write(if locked {
        event::Overlay::Opened
    } else {
        event::Overlay::Closed
    })
}
