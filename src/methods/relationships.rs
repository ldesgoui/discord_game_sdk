use crate::{across_ffi, sys, to_result::ToResult, Discord, DiscordResult, Relationship};

/// # Relationships
impl<'a> Discord<'a> {
    // tested
    // returns NotFound until event::relationships::Refreshed
    pub fn relationship_with(&mut self, user_id: i64) -> DiscordResult<Relationship> {
        let mut relationship = sys::DiscordRelationship::default();

        unsafe {
            ffi!(self
                .get_relationship_manager()
                .get(user_id, &mut relationship))
        }
        .to_result()?;

        Ok(Relationship::from(relationship))
    }

    // tested
    // returns vec![] until event::relationships::Refreshed
    pub fn all_relationships<F>(&mut self, filter: F) -> DiscordResult<Vec<Relationship>>
    where
        F: FnMut(Relationship) -> bool,
    {
        let filter_ptr = Box::into_raw(Box::new(filter));
        let _filter = unsafe { Box::from_raw(filter_ptr) };

        unsafe {
            ffi!(self.get_relationship_manager().filter(
                filter_ptr as *mut _,
                Some(across_ffi::callbacks::filter_relationship::<F>)
            ))
        }

        let mut count = 0;

        unsafe { ffi!(self.get_relationship_manager().count(&mut count)) }.to_result()?;

        let mut result = Vec::with_capacity(count as usize);
        let mut relationship = sys::DiscordRelationship::default();

        for index in 0..count {
            unsafe {
                ffi!(self
                    .get_relationship_manager()
                    .get_at(index as u32, &mut relationship))
            }
            .to_result()?;

            result.push(Relationship::from(relationship))
        }

        Ok(result)
    }
}
