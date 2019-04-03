use crate::prelude::*;

/// # Relationships
impl<'a> Discord<'a> {
    pub fn relationship_with(&mut self, user_id: i64) -> Result<Relationship> {
        let mut relationship = sys::DiscordRelationship::default();

        unsafe {
            ffi!(self
                .get_relationship_manager()
                .get(user_id, &mut relationship))
        }
        .to_result()?;

        Ok(Relationship::from_sys(&relationship))
    }

    pub fn relationships<F>(&mut self, mut filter: F) -> Result<Vec<Relationship>>
    where
        F: FnMut(Relationship) -> bool,
    {
        unsafe {
            ffi!(self.get_relationship_manager().filter(
                &mut filter as *mut _ as *mut _,
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

            result.push(Relationship::from_sys(&relationship))
        }

        Ok(result)
    }
}
