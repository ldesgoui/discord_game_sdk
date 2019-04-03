use crate::prelude::*;

pub(crate) extern "C" fn on_current_user_update(event_data: *mut c_void) {
    let core: &mut Discord = unsafe { (event_data as *mut Discord).as_mut() }.unwrap();

    core.user_events
        .single_write(event::User::CurrentUserUpdated)
}
