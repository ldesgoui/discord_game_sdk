use crate::{across_ffi, sys, to_result::ToResult, Discord, Relationship, Result};

/// # Relationships
///
/// Most methods will return empty/error until event::relationships::Refreshed
///
/// <https://discordapp.com/developers/docs/game-sdk/relationships>
impl<'a> Discord<'a> {
    /// <https://discordapp.com/developers/docs/game-sdk/relationships#get>
    pub fn relationship_with(&mut self, user_id: i64) -> Result<Relationship> {
        let mut relationship = sys::DiscordRelationship::default();

        unsafe {
            ffi!(self
                .get_relationship_manager()
                .get(user_id, &mut relationship))
        }
        .to_result()?;

        Ok(relationship.into())
    }

    /// <https://discordapp.com/developers/docs/game-sdk/relationships#filter>  
    /// <https://discordapp.com/developers/docs/game-sdk/relationships#getat>  
    /// <https://discordapp.com/developers/docs/game-sdk/relationships#count>
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
        let mut relationship = sys::DiscordRelationship::default();

        for index in 0..count {
            unsafe {
                ffi!(self
                    .get_relationship_manager()
                    .get_at(index as u32, &mut relationship))
            }
            .to_result()?;

            result.push(relationship.into())
        }

        Ok(result)
    }
}
