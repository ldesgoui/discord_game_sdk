use crate::prelude::*;

pub(crate) extern "C" fn on_refresh(event_data: *mut c_void) {}

pub(crate) extern "C" fn on_relationship_update(
    event_data: *mut c_void,
    relationship: *mut sys::DiscordRelationship,
) {
}
