use crate::{
    event,
    panic_messages::{NULL_PTR, SEND_FAIL},
    sys,
};
use std::ffi::c_void;

pub(crate) extern "C" fn on_refresh(senders: *mut c_void) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .relationships_refresh
        .try_send(event::relationships::Refresh)
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn on_relationship_update(
    senders: *mut c_void,
    relationship: *mut sys::DiscordRelationship,
) {
    prevent_unwind!();

    debug_assert!(!relationship.is_null());

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .relationships_update
        .try_send(event::relationships::Update {
            relationship: unsafe { *relationship }.into(),
        })
        .expect(SEND_FAIL)
}
