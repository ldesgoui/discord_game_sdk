use crate::prelude::*;

/// # Relationships
impl Discord {
    pub fn get_relationship(&self, user_id: i64) -> Result<Relationship> {
        let mut relationship = sys::DiscordRelationship::default();

        ffi!(self
            .get_relationship_manager()
            .get(user_id, &mut relationship))?;

        Relationship::from_sys(&relationship)
    }

    pub fn relationship_filter<F>(&self, mut filter: F) -> Result<()>
    where
        F: FnMut(&Relationship) -> bool,
    {
        ffi!(self
            .get_relationship_manager()
            .filter(&mut filter as *mut _ as *mut _, Some(filter_callback::<F>)))
    }
}

extern "C" fn filter_callback<F>(
    data: *mut c_void,
    relationship: *mut sys::DiscordRelationship,
) -> bool
where
    F: FnMut(&Relationship) -> bool,
{
    if data.is_null() {
        log::error!("SDK invoked callback with null");
        return false;
    }
    let callback: &mut F = unsafe { &mut *(data as *mut _) };

    Relationship::from_sys_ptr(relationship)
        .map(|r| callback(&r))
        .unwrap_or(false)
}
