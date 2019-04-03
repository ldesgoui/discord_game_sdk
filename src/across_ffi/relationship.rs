use crate::prelude::*;

pub(crate) extern "C" fn on_refresh(core_ptr: *mut c_void) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    core.relationship_channel
        .single_write(event::Relationship::Refresh)
}

pub(crate) extern "C" fn on_relationship_update(
    core_ptr: *mut c_void,
    relationship: *mut sys::DiscordRelationship,
) {
    let core: &mut Discord = unsafe { (core_ptr as *mut Discord).as_mut() }.unwrap();

    let relationship = unsafe { Relationship::from_sys_ptr(relationship) };

    core.relationship_channel
        .single_write(event::Relationship::Update { relationship })
}
