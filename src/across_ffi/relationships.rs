use crate::{event, sys};
use std::ffi::c_void;

pub(crate) extern "C" fn on_refresh(senders: *mut c_void) {
    prevent_unwind!();

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
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .unwrap()
        .relationships_update
        .try_send(event::relationships::Update {
            relationship: unsafe { *relationship }.into(),
        })
        .unwrap()
}
