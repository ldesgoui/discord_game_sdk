use crate::{
    channels, event,
    panic_messages::{NULL_PTR, SEND_FAIL},
    sys,
};
use std::ffi::c_void;

pub(crate) extern "C" fn on_entitlement_create(
    senders: *mut c_void,
    entitlement: *mut sys::DiscordEntitlement,
) {
    prevent_unwind!();

    debug_assert!(!entitlement.is_null());

    unsafe { (senders as *mut channels::Senders).as_ref() }
        .expect(NULL_PTR)
        .store_entitlement_create
        .try_send(event::StoreEntitlementCreate {
            entitlement: unsafe { *entitlement }.into(),
        })
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn on_entitlement_delete(
    senders: *mut c_void,
    entitlement: *mut sys::DiscordEntitlement,
) {
    prevent_unwind!();

    debug_assert!(!entitlement.is_null());

    unsafe { (senders as *mut channels::Senders).as_ref() }
        .expect(NULL_PTR)
        .store_entitlement_delete
        .try_send(event::StoreEntitlementDelete {
            entitlement: unsafe { *entitlement }.into(),
        })
        .expect(SEND_FAIL)
}
