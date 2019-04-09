use crate::prelude::*;

pub(crate) extern "C" fn on_refresh(senders: *mut c_void) {
    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .relationships_refresh
        .try_send(event::relationships::Refresh)
        .unwrap()
}

pub(crate) extern "C" fn on_relationship_update(
    senders: *mut c_void,
    relationship: *mut sys::DiscordRelationship,
) {
    let relationship = unsafe { Relationship::from_sys_ptr(relationship) };

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .relationships_update
        .try_send(event::relationships::Update { relationship })
        .unwrap()
}
