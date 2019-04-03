use crate::prelude::*;

pub(crate) extern "C" fn on_entitlement_create(
    core_ptr: *mut c_void,
    entitlement: *mut sys::DiscordEntitlement,
) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    let entitlement = unsafe { Entitlement::from_sys_ptr(entitlement) };

    core.store_channel
        .single_write(event::Store::EntitlementCreate { entitlement })
}

pub(crate) extern "C" fn on_entitlement_delete(
    core_ptr: *mut c_void,
    entitlement: *mut sys::DiscordEntitlement,
) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    let entitlement = unsafe { Entitlement::from_sys_ptr(entitlement) };

    core.store_channel
        .single_write(event::Store::EntitlementDelete { entitlement })
}
