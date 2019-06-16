use crate::{across_ffi, sys, to_result::ToResult, Discord, Relationship, Result};

/// # Relationships
/// https://discordapp.com/developers/docs/game-sdk/relationships
impl<'a> Discord<'a> {
    // tested
    // returns NotFound until event::relationships::Refreshed
    pub fn relationship_with(&mut self, user_id: i64) -> Result<Relationship> {
        let mut relationship = Relationship(sys::DiscordRelationship::default());

        unsafe {
            ffi!(self
                .get_relationship_manager()
                .get(user_id, &mut relationship.0))
        }
        .to_result()?;

        Ok(relationship)
    }

    // tested
    // returns vec![] until event::relationships::Refreshed
    pub fn all_relationships<F: FnMut(Relationship) -> bool>(
        &mut self,
        filter: F,
    ) -> Result<Vec<Relationship>> {
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
        let mut relationship = Relationship(sys::DiscordRelationship::default());

        for index in 0..count {
            unsafe {
                ffi!(self
                    .get_relationship_manager()
                    .get_at(index as u32, &mut relationship.0))
            }
            .to_result()?;

            result.push(relationship)
        }

        Ok(result)
    }
}
