use crate::{iter, sys, to_result::ToResult, utils, Discord, Relationship, Result, UserID};
use std::convert::TryInto;

/// # Relationships
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/relationships)
impl<E> Discord<E> {
    /// Get the relationship between the current user and a given user by ID.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/relationships#get)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord<()>, user: User) -> Result<()> {
    /// let relationship = discord.relationship_with(user.id())?;
    /// # Ok(()) }
    pub fn relationship_with(&self, user_id: UserID) -> Result<Relationship> {
        let mut relationship = Relationship(sys::DiscordRelationship::default());

        self.with_relationship_manager(|mgr| unsafe {
            mgr.get.unwrap()(mgr, user_id, &mut relationship.0)
        })
        .to_result()?;

        Ok(relationship)
    }

    /// Filter all relationships by a given predicate.
    ///
    /// [`RelationshipsRefreshed`](event/relationships/struct.Refresh.html)
    /// must have fired first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/relationships#filter)  
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # const DISCORD_CLIENT_ID: ClientID = 0;
    /// # fn example(discord: Discord<()>) -> Result<()> {
    /// discord.filter_relationships(|relationship| {
    ///     relationship.presence().activity().application_id() == DISCORD_CLIENT_ID
    /// });
    /// # Ok(()) }
    pub fn filter_relationships<F: FnMut(&Relationship) -> bool>(&self, mut filter: F) {
        pub(crate) unsafe extern "C" fn filter_relationship<F>(
            callback_ptr: *mut std::ffi::c_void,
            relationship_ptr: *mut sys::DiscordRelationship,
        ) -> bool
        where
            F: FnMut(&Relationship) -> bool,
        {
            let _guard = utils::prevent_unwind();

            (*(callback_ptr as *mut F))(&*(relationship_ptr as *const Relationship))
        }

        self.with_relationship_manager(|mgr| unsafe {
            mgr.filter.unwrap()(
                mgr,
                &mut filter as *mut F as *mut std::ffi::c_void,
                Some(filter_relationship::<F>),
            )
        })
    }

    /// Returns the number of relationships matching the filter.
    ///
    /// [`RelationshipsRefreshed`](event/relationships/struct.Refresh.html)
    /// must have fired first.
    ///
    /// Prefer using [`iter_relationships`](#method.iter_relationships).
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/relationships#count)
    pub fn relationship_count(&self) -> Result<u32> {
        let mut count = 0;

        self.with_relationship_manager(|mgr| unsafe { mgr.count.unwrap()(mgr, &mut count) })
            .to_result()?;

        // XXX: i32 should be u32
        Ok(count.try_into().unwrap())
    }

    /// Returns the relationship matching the filter at a given index.
    ///
    /// [`RelationshipsRefreshed`](event/relationships/struct.Refresh.html)
    /// must have fired first.
    ///
    /// Prefer using [`iter_relationships`](#method.iter_relationships).
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/relationships#getat)  
    pub fn relationship_at(&self, index: u32) -> Result<Relationship> {
        let mut relationship = Relationship(sys::DiscordRelationship::default());

        self.with_relationship_manager(|mgr| unsafe {
            mgr.get_at.unwrap()(mgr, index, &mut relationship.0)
        })
        .to_result()?;

        Ok(relationship)
    }

    /// Returns an `Iterator` over the relationships matching the filter.
    ///
    /// [`RelationshipsRefreshed`](event/relationships/struct.Refresh.html)
    /// must have fired first.
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord<()>) -> Result<()> {
    /// for relationship in discord.iter_relationships()? {
    ///     let relationship = relationship?;
    ///     // ..
    /// }
    /// # Ok(()) }
    pub fn iter_relationships<'d>(
        &'d self,
    ) -> Result<
        impl 'd
            + Iterator<Item = Result<Relationship>>
            + DoubleEndedIterator
            + ExactSizeIterator
            + std::iter::FusedIterator,
    > {
        Ok(iter::Collection::new(
            self,
            Self::relationship_at,
            self.relationship_count()?,
        ))
    }
}
