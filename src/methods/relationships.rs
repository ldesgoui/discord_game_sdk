use crate::{across_ffi, event, iter, sys, to_result::ToResult, Discord, Relationship, Result};

/// # Relationships
///
/// <https://discordapp.com/developers/docs/game-sdk/relationships>
impl<'a> Discord<'a> {
    /// Get the relationship between the current user and a given user by ID.
    ///
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

    /// Filter all relationships by a given predicate.
    ///
    /// The event [`relationships::Refreshed`](event/relationships/struct.Refresh.html)
    /// must be fired before using this method.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/relationships#filter>  
    pub fn filter_relationships<F: FnMut(Relationship) -> bool>(&mut self, mut filter: F) {
        unsafe {
            ffi!(self.get_relationship_manager().filter(
                &mut filter as *mut _ as *mut _,
                Some(across_ffi::callbacks::filter_relationship::<F>)
            ))
        }
    }

    /// <https://discordapp.com/developers/docs/game-sdk/relationships#count>
    pub fn relationship_count(&mut self) -> Result<i32> {
        let mut count = 0;

        unsafe { ffi!(self.get_relationship_manager().count(&mut count)) }.to_result()?;

        Ok(count)
    }

    /// <https://discordapp.com/developers/docs/game-sdk/relationships#getat>  
    pub fn relationship_at(&mut self, index: i32) -> Result<Relationship> {
        let mut relationship = sys::DiscordRelationship::default();

        unsafe {
            ffi!(self
                .get_relationship_manager()
                .get_at(index as u32, &mut relationship))
        }
        .to_result()?;

        Ok(relationship.into())
    }

    pub fn iter_relationships<'b>(
        &'b mut self,
    ) -> Result<iter::GenericIter<'a, 'b, Result<Relationship>>> {
        let count = self.relationship_count()?;

        Ok(iter::GenericIter::new(
            self,
            Box::new(|d, i| d.relationship_at(i)),
            count,
        ))
    }

    /// Fires at initialization when Discord has cached a snapshot of all your relationships.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/relationships#onrefresh>
    pub fn recv_relationships_refresh(
        &self,
    ) -> impl '_ + Iterator<Item = event::relationships::Refresh> {
        self.receivers.relationships_refresh.try_iter()
    }

    /// Fires when a relationship in the filtered list changes, like an updated presence or user attribute.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/relationships#onrelationshipupdate>
    pub fn recv_relationships_update(
        &self,
    ) -> impl '_ + Iterator<Item = event::relationships::Update> {
        self.receivers.relationships_update.try_iter()
    }
}
