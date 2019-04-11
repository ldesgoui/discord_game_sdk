use crate::prelude::*;

pub(crate) extern "C" fn on_entitlement_create(
    senders: *mut c_void,
    entitlement: *mut sys::DiscordEntitlement,
) {
    prevent_unwind!();

    let entitlement = unsafe { Entitlement::from_sys_ptr(entitlement) };

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .store_entitlement_create
        .try_send(event::store::EntitlementCreate { entitlement })
        .unwrap()
}

pub(crate) extern "C" fn on_entitlement_delete(
    senders: *mut c_void,
    entitlement: *mut sys::DiscordEntitlement,
) {
    prevent_unwind!();

    let entitlement = unsafe { Entitlement::from_sys_ptr(entitlement) };

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .store_entitlement_delete
        .try_send(event::store::EntitlementDelete { entitlement })
        .unwrap()
}
