use crate::prelude::*;

pub(crate) extern "C" fn on_entitlement_create(
    event_data: *mut c_void,
    entitlement: *mut sys::DiscordEntitlement,
) {
}

pub(crate) extern "C" fn on_entitlement_delete(
    event_data: *mut c_void,
    entitlement: *mut sys::DiscordEntitlement,
) {
}
