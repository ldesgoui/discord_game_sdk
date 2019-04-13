use crate::{event, sys};
use std::ffi::c_void;

pub(crate) extern "C" fn on_entitlement_create(
    senders: *mut c_void,
    entitlement: *mut sys::DiscordEntitlement,
) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .store_entitlement_create
        .try_send(event::store::EntitlementCreate {
            entitlement: unsafe { *entitlement }.into(),
        })
        .unwrap()
}

pub(crate) extern "C" fn on_entitlement_delete(
    senders: *mut c_void,
    entitlement: *mut sys::DiscordEntitlement,
) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .store_entitlement_delete
        .try_send(event::store::EntitlementDelete {
            entitlement: unsafe { *entitlement }.into(),
        })
        .unwrap()
}
